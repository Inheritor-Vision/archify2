use crate::conf::*;

use log::info;
use rspotify::model::{FullPlaylist, PlaylistId};
use rspotify::prelude::Id;
use rusqlite::{params,Connection};

pub struct Playlist {
	pub id: PlaylistId<'static>,
	pub sha256:  [u8; 32],
	pub timestamp: u64,
	pub count: u64,
	pub data: Option<FullPlaylist>
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
		self.client.execute("CREATE TABLE IF NOT EXISTS playlists (playlist_id TEXT, playlist_SHA256 BLOB, timestamp TIMESTAMP, playlist_data TEXT, PRIMARY KEY (playlist_id, timestamp))", ()).unwrap();
	}

	pub fn set_unique_empty_playlist(&self, playlist_id: &PlaylistId){
		let serialized_id = serde_json::to_string(playlist_id).unwrap();

		let res = self.client.query_row(
			"SELECT * FROM playlists WHERE playlist_id = ?1",
			params![
				serialized_id
			],
			|_| Result::Ok(0) 
		);

		match res {
			Result::Ok(_) => info!("Playlist {} is already present.", playlist_id),
			Result::Err(_) => {
				self.client.execute(
					"INSERT INTO playlists (playlist_id, timestamp, playlist_data) VALUES (?1, ?2, ?3)", 
					params![
						serialized_id,
						CONF_TIME_BIG_BANG,
						serde_json::to_string(&CONF_NULL_PLAYLIST_DATA).unwrap()
					]
				).unwrap();
				info!("Empty playlist {} inserted.", playlist_id);
			}
		};
	}

	pub fn set_playlist(&self, playlist: &Playlist){
		let serialized_id = serde_json::to_string(&playlist.id).unwrap();
		let serialized_data = serde_json::to_string(&playlist.data).unwrap();
		self.client.execute(
			"INSERT INTO playlists (playlist_id, playlist_SHA256, timestamp, playlist_data) VALUES (?1, ?2, ?3, ?4)",
			params![
				serialized_id,
				playlist.sha256,
				playlist.timestamp,
				serialized_data
			]
		).unwrap();
		info!("Playlist {} inserted.", playlist.id.id());
	}

	pub fn delete_playlist(&self, playlist_id: &PlaylistId){
		let serialized_id = serde_json::to_string(playlist_id).unwrap();
		self.client.execute(
			"DELETE FROM playlists WHERE playlist_id = ?1",
			params![serialized_id]
		).unwrap();
		info!("Playlist(s) {} deleted.", playlist_id);
	}

	pub fn get_latest_unique_playlists(&self) -> Playlists {
		let mut playlists = Playlists::new();

		let mut query = self.client.prepare("SELECT playlist_id, playlist_sha256, MAX(timestamp) as timestamp, COUNT(playlist_id) as count, playlist_data FROM playlists GROUP BY playlist_id").unwrap();
		let p_iter = query.query_map([], |row| {
			Ok(
				Playlist {
					id: {
						let res: String = row.get("playlist_id").unwrap();
						serde_json::from_str(res.as_str()).unwrap()
					},
					sha256: row.get("playlist_sha256").unwrap_or_else(|_| {CONF_SHA256_NULL}),
					timestamp: row.get("timestamp").unwrap_or_else(|_| {CONF_TIMESTAMP_NULL}),
					count: row.get("count").unwrap(),
					data: {
						let res: String = row.get("playlist_data").unwrap_or_else(|_| {CONF_NULL_STRING});
						if !res.is_empty() {
							serde_json::from_str(res.as_str()).unwrap()
						} else {
							CONF_NULL_PLAYLIST_DATA
						}
					}
				}
			)
		});

		match p_iter {
			Ok(ps) => {
				for p in ps {
					playlists.push(p.unwrap());
				}
				info!("{} latest unique playlists retreived.", playlists.len());
				playlists
			},
			Err(_) => {info!("No playlist found in db."); playlists}
			
		}
	}

}