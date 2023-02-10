mod arguments;

use std::fs::File;
use std::io::Read;

use serde_json::Value;
use single_instance::SingleInstance;
use reqwest::header;

static APP_USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"/",
	env!("CARGO_PKG_VERSION")
);

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

fn create_client(default_header: header::HeaderMap) -> reqwest::Client {
	let client: reqwest::ClientBuilder;


	#[cfg(not(feature = "proxy"))]{
		client = reqwest::Client::builder()
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

fn main() {
   println!("Welcome to archify!");
   let _instance  = verify_single_instance();

   let conf = extract_configuration();
   let default_spot_header = create_spotify_api_header();

   let spotify_client = create_client(default_spot_header);

   let args = arguments::parse_args();

   match args{
	   arguments::Args::NewPlaylist(playlists) => println!("Not available yet!"),
	   arguments::Args::Update => println!("Not available yet!"),
	   arguments::Args::DeletePlaylist(playlists) => println!("Not available yet!")
   }

}
