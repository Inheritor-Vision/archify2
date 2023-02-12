use crate::{conf::*, spotify::authentication::AppToken};
use crate::spotify::authentication::Token;

use log::info;
use rusqlite::{params,Connection};
use serde_json::Value;

pub struct Playlist {
	pub id:  String,
	pub sha256:  [u8; 32],
	pub timestamp: u64,
	pub data: Value
}

pub type Playlists = Vec<Playlist>;


pub struct Database {
	client: Connection
}


impl Database {
	pub fn new() -> Self {
		let con = Connection::open(CONF_DATABASE_PATH).unwrap();		

		let db  = Database { 
			client: con 
		};

		db.create_tables();

		db
	}

	fn create_tables(&self){
		self.client.execute("CREATE TABLE IF NOT EXISTS spotify_tokens (user_id TEXT, access_token TEXT, refresh_token TEXT, token_type TEXT, expires_in BIGINT, received_at TIMESTAMP, PRIMARY KEY (user_id))", ()).unwrap();
		self.client.execute("CREATE TABLE IF NOT EXISTS playlists (playlist_id TEXT, playlist_SHA256 BLOB, timestamp TIMESTAMP, playlist_data TEXT, PRIMARY KEY (playlist_id, timestamp))", ()).unwrap();
	}

	pub fn set_unique_empty_playlist(&self, playlist_id: &String){

		let res = self.client.query_row(
			"SELECT * FROM playlists WHERE playlist_id = ?1",
			params![
				playlist_id
			],
			|_| Result::Ok(0) 
		);

		match res {
			Result::Ok(_) => info!(" Playlist {} is already present.", playlist_id),
			Result::Err(_) => {
				self.client.execute(
					"INSERT INTO playlists (playlist_id, timestamp) VALUES (?1, ?2)", 
					params![
						playlist_id.as_str(),
						CONF_TIME_BIG_BANG
					]
				).unwrap();
				info!("Empty playlist {} inserted.", playlist_id);
			}
		};
	}

	pub fn set_playlist(&self, playlist: &Playlist){
		self.client.execute(
			"INSERT INTO playlists (playlist_id, playlist_SHA256, timestamp, playlist_data) VALUES (?1, ?2, ?3, ?4)",
			params![
				playlist.id,
				playlist.sha256,
				playlist.timestamp,
				playlist.data
			]
		).unwrap();
		info!("Playlist {} inserted.", playlist.id);
	}

	pub fn delete_playlist(&self, playlist_id: &String){
		self.client.execute(
			"DELETE FROM playlists WHERE playlist_id = ?1",
			params![playlist_id]
		).unwrap();
		info!("Playlist(s) {} deleted.", playlist_id);
	}

	pub fn update_app_token(&self, token: &Token){
		self.client.execute(
			"INSERT OR REPLACE INTO spotify_tokens (access_token, token_type, expires_in, received_at, user_id) VALUES (?1, ?2, ?3, ?4, ?5)", 
			params![
				token.token.access_token.as_str(),
				token.token.token_type.as_str(),
				token.token.expires_in,
				token.received_at,
				CONF_ARCHIFY_ID
			]
		).unwrap();
		info!("App token replaced.");
	}

	pub fn get_app_token(&self) -> Option<Token> {
		let res = self.client.query_row(
			"SELECT * FROM spotify_tokens WHERE user_id = ?1", params![CONF_ARCHIFY_ID],
			|row| {
				Result::Ok(Option::Some(Token{
					token: AppToken{
						access_token: row.get("access_token")?,
						expires_in: row.get("expires_in")?,
						token_type: row.get("token_type")?
					},
					received_at: row.get("received_at")?,
					client_id: String::from(CONF_ARCHIFY_ID)
				}))
			}
		);

		match res {
			Ok(r) => {info!("App token retreived."); r},
			Err(_) => {info!("App token not found."); Option::None}
		}
	}

	pub fn get_latest_unique_playlists(&self) -> Playlists {
		let mut playlists = Playlists::new();

		let mut query = self.client.prepare("SELECT playlist_id, playlist_sha256, MAX(timestamp) as timestamp, playlist_data FROM playlists GROUP BY playlist_id").unwrap();
		let p_iter = query.query_map([], |row| {
			Ok(
				Playlist {
					id: row.get("playlist_id")?,
					sha256: row.get("playlist_sha256").unwrap_or_else(|_| {CONF_SHA256_NULL}),
					timestamp: row.get("timestamp").unwrap_or_else(|_| {CONF_TIMESTAMP_NULL}),
					data: row.get("playlist_data").unwrap_or_else(|_| {CONF_NULL_PLAYLIST_DATA})
				}
			)
		});

		match p_iter {
			Ok(ps) => {
				for p in ps {
					playlists.push(p.unwrap());
				}
				info!("{} playlists retreived.", playlists.len());
				playlists
			},
			Err(_) => {info!("No playlist found."); playlists}
			
		}
	}

}