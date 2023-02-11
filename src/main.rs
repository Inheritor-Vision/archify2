mod arguments;
mod conf;
mod database;
mod spotify;

use conf::*;

use std::{fs::File, str::FromStr};
use std::io::Read;
use std::process::exit;

use serde_json::Value;
use single_instance::SingleInstance;
use reqwest::{blocking::Client, blocking::ClientBuilder, header};

static APP_USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"/",
	env!("CARGO_PKG_VERSION")
);

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
		eprintln!("ERROR: Configuration file cannot be parsed correctly!");
		exit(-1);
	}

	ArchifyConf { 
		archify_id: String::from_str(json_api[CONF_ARCHIFY_ID].as_str().unwrap()).unwrap(), 
		archify_secret: String::from_str(json_api[CONF_ARCHIFY_SECRET].as_str().unwrap()).unwrap() 
	}

}

fn create_spotify_api_header() -> header::HeaderMap{
	let mut spotify_http_header = header::HeaderMap::new();

	spotify_http_header.insert(
		header::ACCEPT,
		header::HeaderValue::from_static("application/json")
	);

	spotify_http_header

}

fn verify_single_instance() -> SingleInstance{
	let instance = SingleInstance::new("archify").unwrap();

	if !instance.is_single(){
		panic!("Only one instance of archify must run at the same time!")
	}

	instance
}

fn create_client(default_header: header::HeaderMap) -> Client {
	let client: ClientBuilder;


	#[cfg(not(feature = "proxy"))]{
		client = Client::builder()
			.user_agent(APP_USER_AGENT)
			.default_headers(default_header);
	}
	
	#[cfg(feature = "proxy")]{
		client = Client::builder()
			.user_agent(APP_USER_AGENT)
			.default_headers(default_header)
			.proxy(reqwest::Proxy::http("http://127.0.0.1:8080").unwrap())
			.proxy(reqwest::Proxy::https("http://127.0.0.1:8080").unwrap())
			.add_root_certificate(get_certificate());
	} 

	client.build().unwrap()
}

fn add_playlist(db: &database::Database, playlist_ids: Vec<String>){

		for p_id in playlist_ids{
			db.set_unique_empty_playlist(&p_id);
		}
}

fn main() {
	println!("Welcome to archify!");
	let _instance  = verify_single_instance();

	let args = arguments::parse_args();
	let conf = extract_configuration();
	let default_spot_header = create_spotify_api_header();
	let db = database::Database::new();

	let mut spotify_client = create_client(default_spot_header);

	let token = db.get_app_token();
	let _token = match token {
		Some(t) => {
			if spotify::authentication::is_access_token_expired(&t){
				let l_t = spotify::authentication::get_app_token(&mut spotify_client, &conf);
				db.update_app_token(&l_t);
				l_t
			}else{
				t
			}
		},
		None => {
			let l_t = spotify::authentication::get_app_token(&mut spotify_client, &conf);
			db.update_app_token(&l_t);
			l_t
		}
	};
	let _app_token = spotify::authentication::get_app_token(&mut spotify_client, &conf);

	match args{
		arguments::Args::NewPlaylist(playlists) => add_playlist(&db, playlists),
		arguments::Args::Update => println!("Not available yet!"),
		arguments::Args::DeletePlaylist(_playlists) => println!("Not available yet!")
	}

}
