use crate::ArchifyConf;

use std::{time::{SystemTime, UNIX_EPOCH}};

use base64::{Engine as _, engine::general_purpose};
use reqwest::{blocking::Client, header};
use serde::{Deserialize};

static FORM_TOKEN: [(&str, &str); 1] = 
	[
		("grant_type", "client_credentials"), 
	];

#[derive(Deserialize, Clone)]
pub struct AppToken {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

#[derive(Clone)]
pub struct Token {
	pub token: AppToken,
	pub received_at: u64,
	pub client_id: String,
}

fn add_app_authorization(headers: &mut header::HeaderMap, conf: &ArchifyConf){
	let auth_value = format!(
		"Basic {}", general_purpose::STANDARD.encode(
			format!(
				"{}:{}",
				conf.archify_id.as_str(),
				conf.archify_secret.as_str()
			)
		)
	);
	
	let mut auth = header::HeaderValue::from_str(&auth_value).unwrap();
	auth.set_sensitive(true);

	headers.insert(header::AUTHORIZATION, auth);
}

pub fn get_app_token(client: &mut Client, conf: &ArchifyConf) -> Token {
	let mut headers = header::HeaderMap::new();
	
	add_app_authorization(&mut headers, &conf);

	let time = SystemTime::now()
	.duration_since(UNIX_EPOCH)
	.unwrap()
	.as_secs();

	let body = client.post("https://accounts.spotify.com/api/token")
	.form(&FORM_TOKEN)
	.headers(headers)
	.send()
	.unwrap()
	.text()
	.unwrap();

	let app_token: AppToken = serde_json::from_str(&body).unwrap();

	let token = Token {
		token: app_token,
		received_at: time,
		client_id: conf.archify_id.clone() 
	};

	token
}