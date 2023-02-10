use std::{fs::File, io::Read};

use serde_json::Value;
use reqwest::header;

struct ArchifyApi{
	archify_id: String,
}

fn extract_configuration() -> ArchifyApi{
	let mut buf = String::new();

	File::open("data/config.json")
	.unwrap()
	.read_to_string(&mut buf)
	.unwrap();

	let json_api: Value = serde_json::from_str(&*buf).unwrap();

	ArchifyApi { 
		archify_id: json_api["archify_id"].to_string(), 
	}
}



fn main() {
   println!("Welcome to archify!");

   let api = extract_configuration();

}
