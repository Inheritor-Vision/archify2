mod arguments;
mod conf;
mod database;
mod spotify;

use conf::*;
use rspotify::model::PlaylistId;
use rspotify::prelude::Id;
use spotify::get_spotify_client_from_client_credentials;

use std::env;
use std::str::FromStr;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::time::{UNIX_EPOCH, Duration};

use chrono::{Local, DateTime};
use env_logger::Builder;
use env_logger::fmt::Color;
use log::{Level, error, info};
use serde_json::Value;
use single_instance::SingleInstance;
use tokio::runtime::Runtime;
use url::Url;

pub struct ArchifyConf{
	archify_id: String,
	archify_secret: String
}

fn extract_configuration() -> ArchifyConf{
	let mut buf = String::new();

	File::open("data/config.json")
	.unwrap()
	.read_to_string(&mut buf)
	.unwrap();

	let json_api: Value = serde_json::from_str(&*buf).unwrap();

	if json_api[CONF_ARCHIFY_ID].is_null() || json_api[CONF_ARCHIFY_SECRET].is_null(){
		error!("Configuration file cannot be parsed correctly!");
		exit(-1);
	}

	ArchifyConf { 
		archify_id: String::from_str(json_api[CONF_ARCHIFY_ID].as_str().unwrap()).unwrap(), 
		archify_secret: String::from_str(json_api[CONF_ARCHIFY_SECRET].as_str().unwrap()).unwrap() 
	}

}

fn verify_single_instance() -> SingleInstance{
	let instance = SingleInstance::new("archify").unwrap();

	if !instance.is_single(){
		error!("Only one instance of archify must run at the same time!");
		exit(-1);
	}

	instance
}

fn parse_url(url: &String) -> Option<String>{
	let parsed_url = Url::parse(url).unwrap();
	let segments = parsed_url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();

	if segments[0] == "playlist" {
		Option::Some(String::from(segments[1]))
	}else{
		None
	}

}

fn add_playlist(db: &database::Database, playlist_ids: Vec<String>){
	for p in playlist_ids{
		match parse_url(&p){
			Some(p_url) => db.set_unique_empty_playlist(&PlaylistId::from_id(p_url).unwrap()),
			None => ()
		}
	}
}

fn delete_playlist(db: &database::Database, playlist_ids: Vec<String>){
	for p in playlist_ids{
		match parse_url(&p){
			Some(p_url) => db.delete_playlist(&PlaylistId::from_id(p_url).unwrap()),
			None => ()
		}
	}
}

async fn update_playlists(db: &database::Database, conf: &ArchifyConf){
	let playlists = db.get_latest_unique_playlists();
	let client = get_spotify_client_from_client_credentials(&conf).await;

	for p in playlists{
		let fresh_p = spotify::get_public_playlists(&client, &p.id).await;

		if p.sha256 != fresh_p.sha256{
			db.set_playlist(&fresh_p);
		}else{
			info!("Playlist {} SHA matching, not pushed to db.", p.id.id());
		}
	}
}

async fn list_playlists(db: &database::Database){
	let playlists = db.get_latest_unique_playlists();
	println!("List of tracked playlist:");
	for p in playlists{
		match p.data {
			Some(data) => println!("[{}]: {} ({} version(s))", p.id.id(), data.name, p.count - 1),
			None => println!("[{}]: ! Name not available, please --update first !", p.id.id())
		}
	}
}

fn list_tracked_versions(db: &database::Database, playlist_id: &String){
	let p_id ;
	if !PlaylistId::id_is_valid(playlist_id.as_str()){
		p_id = PlaylistId::from_id(parse_url(playlist_id).unwrap()).unwrap();
	}else{
		p_id = PlaylistId::from_id(playlist_id).unwrap();
	}

	let playlists = db.get_all_tracked_versions(&p_id);

	if playlists.is_empty(){
		error!("No playlist with this id are recorded!");
	}else if playlists.len() == 1 {
		error!("Playlist has not been updated yet! Do an --update first.")
	}else{
		println!("List of tracked versions for [{}] - {}:", p_id.id(), playlists.get(1).unwrap().data.as_ref().unwrap().name);
		for p in playlists{
			if p.count != 0{
				let date = DateTime::<Local>::from(UNIX_EPOCH + Duration::from_secs(p.timestamp));
				let format_date = format!("{}", date.format("%v %X"));	
				println!("[{}]: {}", p.count, format_date);
			}
		}
	}
}

async fn export_playlist(db: &database::Database, playlist_id: &String, index: u64, conf: &ArchifyConf){
	let client = &spotify::get_spotify_client_from_user(conf).await;

	let p_id ;
	if !PlaylistId::id_is_valid(playlist_id.as_str()){
		p_id = PlaylistId::from_id(parse_url(playlist_id).unwrap()).unwrap();
	}else{
		p_id = PlaylistId::from_id(playlist_id).unwrap();
	}

	let playlist = db.get_playlist_from_tracked_index(&p_id, index);

	match playlist {
		Some(p) => spotify::export_playlist_to_user(client, &p).await,
		None =>	error!("No playlist with this id & index are stored. Check --tracked.")
	}

}

fn main() {
	println!("Welcome to archify!");

	#[cfg(debug_assertions)]
	env::set_var("RUST_LOG", "debug");

	#[cfg(feature = "proxy")]{
		env::set_var("HTTP_PROXY", REQWEST_ENV_HTTP_PROXY);
		env::set_var("HTTPS_PROXY", REQWEST_ENV_HTTPS_PROXY);
	}

	Builder::from_default_env().format(|buf, record|{
		let mut style = buf.style();
		let level = match record.level() {
			Level::Error => style.set_color(Color::Red).value(Level::Error),
			Level::Warn => style.set_color(Color::Yellow).value(Level::Warn),
			Level::Info => style.set_color(Color::Green).value(Level::Info),
			Level::Debug => style.set_color(Color::Blue).value(Level::Debug),
			Level::Trace => style.set_color(Color::Cyan).value(Level::Trace),
		};
		let mut style = buf.style();
		style.set_color(Color::Magenta);
		writeln!(
			buf, 
			"[{}] {: <5} - {} ({})",
			Local::now().format("%v %X"),
			level,
			record.args(),
			style.value(record.module_path().unwrap_or_else(|| {"Uknown file"}))
		)
	}).init();


	let _instance  = verify_single_instance();

	let args = arguments::parse_args();
	let conf = extract_configuration();

	let db = database::Database::new();


	match args{
		arguments::Args::NewPlaylist(playlists) => add_playlist(&db, playlists),
		arguments::Args::Update => Runtime::new().unwrap().block_on(update_playlists(&db, &conf)),
		arguments::Args::DeletePlaylist(playlists) => delete_playlist(&db, playlists),
		arguments::Args::List => Runtime::new().unwrap().block_on(list_playlists(&db)),
		arguments::Args::Tracked(playlist_id) => list_tracked_versions(&db, &playlist_id),
		arguments::Args::Export(export) => Runtime::new().unwrap().block_on(export_playlist(&db, &export.playlist_id, export.index, &conf))
	}

}
