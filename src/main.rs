mod arguments;

use std::fs::File;
use std::io::Read;

use serde_json::Value;
use single_instance::SingleInstance;
use reqwest::header;

struct ArchifyConf{
	archify_id: String,
}

fn extract_configuration() -> ArchifyConf{
	let mut buf = String::new();

	File::open("data/config.json")
	.unwrap()
	.read_to_string(&mut buf)
	.unwrap();

	let json_api: Value = serde_json::from_str(&*buf).unwrap();

	ArchifyConf { 
		archify_id: json_api["archify_id"].to_string(), 
	}
}

fn create_spotify_api_header() -> header::HeaderMap{
	let mut spotify_HTTP_header = header::HeaderMap::new();

	spotify_HTTP_header.insert(
		header::ACCEPT,
		header::HeaderValue::from_static("application/json")
	);

	spotify_HTTP_header

}

fn verify_single_instance() -> SingleInstance{
	let instance = SingleInstance::new("archify").unwrap();

	if !instance.is_single(){
		panic!("Only one instance of archify must run at the same time!")
	}

	instance
}

fn main() {
   println!("Welcome to archify!");
   let _instance  = verify_single_instance();

   let conf = extract_configuration();
   let default_spot_header = create_spotify_api_header();

   let args = arguments::parse_args();

   match args{
	   arguments::Args::NewPlaylist(playlists) => println!("Not available yet!"),
	   arguments::Args::Update => println!("Not available yet!"),
	   arguments::Args::DeletePlaylist(playlists) => println!("Not available yet!")
   }

}
