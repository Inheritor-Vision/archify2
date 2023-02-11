use crate::conf::*;

use rusqlite::Connection;

pub struct Database {
	pub client: Connection
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
		self.client.execute("CREATE TABLE IF NOT EXISTS spotify_app_tokens (access_token TEXT, token_type TEXT, duration BIGINT, received_at TIMESTAMP)", ()).unwrap();
		self.client.execute("CREATE TABLE IF NOT EXISTS playlists (playlist_id TEXT, playlist_SHA256 BYTEA, timestamp TIMESTAMP, playlist_data TEXT, PRIMARY KEY (playlist_id, timestamp))", ()).unwrap();
	}
}