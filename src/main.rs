use std::{fs::File, io::Read};

use serde_json::Value;

struct ArchifyApi{
	archify_id: String,
	archify_secret: String
}

fn extract_configuration() -> ArchifyApi{
	let mut buf = String::new();

	File::open("data/secrets.json")
	.unwrap()
	.read_to_string(&mut buf)
	.unwrap();

	let json_api: Value = serde_json::from_str(&*buf).unwrap();

	ArchifyApi { 
		archify_id: json_api["archify_id"].to_string(), 
		archify_secret: json_api["archify_secret"].to_string()
	}
}

fn main() {
   println!("Welcome to archify!");

   let api = extract_configuration();

}
