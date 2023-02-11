use crate::{conf::*, spotify::authentication::AppToken};
use crate::spotify::authentication::Token;

use rusqlite::{params,Connection};

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
		self.client.execute("CREATE TABLE IF NOT EXISTS spotify_tokens (user_id TEXT, access_token TEXT, refresh_token TEXT, token_type TEXT, expires_in BIGINT, received_at TIMESTAMP, PRIMARY KEY (user_id))", ()).unwrap();
		self.client.execute("CREATE TABLE IF NOT EXISTS playlists (playlist_id TEXT, playlist_SHA256 BYTEA, timestamp TIMESTAMP, playlist_data TEXT, PRIMARY KEY (playlist_id, timestamp))", ()).unwrap();
	}

	pub fn update_app_token(&self, token: &Token){
		println!("Token updated!");
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
			Ok(r) => r,
			Err(_) => Option::None
		}
	}
}