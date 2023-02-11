use crate::conf::*;
use crate::database::Playlist;
use crate::spotify::authentication::Token;

use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::{header, blocking::Client};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Sha256, Digest};

#[derive(Deserialize)]
struct Id{
	id: String
}

#[derive(Deserialize)]
struct Track {
	track: Id
}

#[derive(Deserialize)]
struct Items {
	items: Vec<Track>
}

#[derive(Deserialize)]
struct Tracks{
	tracks: Items
}

pub fn get_playlist_content_from_playlist_id(client: &Client, token: &Token, playlist_id: &String) -> Playlist {
	let api_uri = format!("{}/{}", CONF_SPOTIFY_PLAYLIST_ENDPOINT, &playlist_id);
	let auth_value = format!("{} {}", token.token.token_type, token.token.access_token);
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs();

	let body = client.get(api_uri)
		.query(&[("fields", "tracks.items(track(id))")])
		.header(header::AUTHORIZATION, &auth_value)
		.send()
		.unwrap()
		.text()
		.unwrap();

	let json: Tracks = serde_json::from_str(body.as_str()).unwrap();
	let json_raw: Value = serde_json::from_str(body.as_str()).unwrap();
	
	let mut hasher = Sha256::new();

	for i in &json.tracks.items{
		hasher.update(i.track.id.as_bytes());
	}

	let sha256 = hasher.finalize();

	Playlist { 
		id: playlist_id.clone(), 
		sha256: sha256.into(), 
		timestamp: timestamp, 
		data: json_raw 
	}
}