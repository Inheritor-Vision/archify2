use crate::conf::*;

use rusqlite::Connection;

pub struct Database {
	pub client: Connection
}


impl Database {
	pub fn new() -> Self {
		let con = Connection::open(CONF_DATABASE_PATH).unwrap();

		Database { 
			client: con 
		}
	}
}