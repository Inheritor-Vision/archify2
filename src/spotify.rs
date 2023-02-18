use crate::ArchifyConf;
use crate::conf::*;
use crate::database::Playlist;

use std::env;
use std::path::PathBuf;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::DateTime;
use chrono::Local;
use log::error;
use log::info;
use rspotify::Token;
use rspotify::{AuthCodeSpotify, OAuth};
use rspotify::model::{PlaylistId, PlayableItem};
use rspotify::prelude::{BaseClient,Id, OAuthClient, PlayableId};
use rspotify::scopes;
use rspotify::{Credentials, ClientCredsSpotify, Config, DEFAULT_API_PREFIX, DEFAULT_PAGINATION_CHUNKS};
use sha2::{Digest, Sha256};



pub async fn get_spotify_client_from_client_credentials(app_conf: &ArchifyConf) -> ClientCredsSpotify{
	env::set_var(RSPOTIFY_ENV_CLIENT_ID, &app_conf.archify_id);
	env::set_var(RSPOTIFY_ENV_CLIENT_SECRET, &app_conf.archify_secret);

	let creds = Credentials::from_env().unwrap();

	env::remove_var(RSPOTIFY_ENV_CLIENT_ID);
	env::remove_var(RSPOTIFY_ENV_CLIENT_SECRET);

	let mut path = PathBuf::new();
	path.push(RSPOTIFY_CLIENT_TOKEN_PATH);

	let token_exists = path.exists();

	let config = Config{
		prefix: String::from(DEFAULT_API_PREFIX),
		cache_path: path,
		pagination_chunks: DEFAULT_PAGINATION_CHUNKS,
		token_cached: true,
		token_refreshing: true
	};

	let spot_client = ClientCredsSpotify::with_config(creds, config);

	let tok = match token_exists {
		true => spot_client.read_token_cache().await.unwrap(),
		false => None
	};

	match tok {
		Some(token) => {
			*spot_client.get_token().lock().await.unwrap() = Some(token);
			info!("Client token already cached.");
		}
			,
		None => {
			spot_client.request_token().await.unwrap();
			info!("Client token NOT cached. Retreived from Spotify API.");
		}
	}

	spot_client
}

pub async fn get_spotify_client_from_user(app_conf: &ArchifyConf) -> AuthCodeSpotify{
	let creds = Credentials::new(&app_conf.archify_id, &app_conf.archify_secret);

	let oauth = OAuth{
		redirect_uri: String::from(RSPOTIFY_REDIRECT_URI),
		scopes: scopes!(RSPOTIFY_SCOPES[0], RSPOTIFY_SCOPES[1], RSPOTIFY_SCOPES[2]),
		#[cfg(feature = "proxy")]
		proxies: Some(String::from(REQWEST_ENV_HTTP_PROXY)),
		..Default::default()
	};

	let mut path = PathBuf::new();
	path.push(RSPOTIFY_USER_TOKEN_PATH);

	let config = Config{
		cache_path: path,
		token_cached: true,
		..Default::default()
	};

	let client = AuthCodeSpotify::with_config(creds, oauth, config);

	let url = client.get_authorize_url(false).unwrap();
	client.prompt_for_token(&url).await.unwrap();
	client.refresh_token().await.unwrap();
	client.write_token_cache().await.unwrap();

	client

}

pub async fn get_public_playlists(client: &ClientCredsSpotify, playlist_id: &PlaylistId<'static>) -> Playlist {
	let fplaylist = client.playlist(playlist_id.clone_static(), None, None).await.unwrap();

	info!("Playlist {playlist_id} retreived, with {} tracks", fplaylist.tracks.items.len());
	// To verbose
	// #[cfg(debug_assertions)]{
	// 	let l_p = fplaylist.clone();
	// 	debug!("Content retreived: {l_p:#?}");
	// }

	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs();

	let mut hasher = Sha256::new();

	for item in &fplaylist.tracks.items{
		match &item.track {
			Some(pi) => {
				match pi {
					PlayableItem::Episode(_) => {
						let id = playlist_id.id();
						info!("Untracked Podcast in {id}");
					},
					PlayableItem::Track(track) => {
						match &track.id {
							Some(id) => hasher.update(id.id().as_bytes()),
							None => {
								let id = playlist_id.id();
								info!("Untracked NON null and local item in {id}");
							} 
						}
					}
				}
			},
			None => {
				if item.is_local{ 
					let id = playlist_id.id();
					info!("Untracked null and local item in {id}");
				}else{
					let id = playlist_id.id();
					error!("Null playable item but not local for playlist {id}");
				}
			} 
		}
	}

	let sha256 = hasher.finalize();

	let playlist  = Playlist{
		id: playlist_id.clone_static(),
		sha256: sha256.into(),
		timestamp: timestamp,
		count: CONF_DEFAULT_COUNT,
		data: Some(fplaylist)
	};


	playlist
}

pub async fn export_playlist_to_user(client: &AuthCodeSpotify, playlist: &Playlist){
	let user_id = playlist.data.as_ref().unwrap().owner.id.clone_static();

	let date = DateTime::<Local>::from(UNIX_EPOCH + Duration::from_secs(playlist.timestamp));
	let format_date = format!("{}", date.format("%v %X"));	
	let name = format!(
		"Archify - {} - {}",
		playlist.data.as_ref().unwrap().name,
		format_date
	);

	let new_p = client.user_playlist_create(user_id, name.as_str(), Some(true), Some(false), Some(RSPOTIFY_PLAYLIST_DESCRIPTION)).await.unwrap();

	let mut tracks: Vec<PlayableId> = Vec::new();
	for p in &playlist.data.as_ref().unwrap().tracks.items{
		match &p.track {
			Some(item) => match &item.id() {
				Some(id) => tracks.push(id.clone_static()),
				None => ()
			},
			None => ()
		}
	}

	let _ = client.playlist_add_items(new_p.id, tracks , Some(0)).await.unwrap();

}