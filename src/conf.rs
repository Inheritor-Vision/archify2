use rspotify::model::FullPlaylist;


// Dynamic Configuration Fields
pub const CONF_ARCHIFY_ID: &str								= "archify_id";
pub const CONF_ARCHIFY_SECRET: &str							= "archify_secret";

// Paths
pub const CONF_DATABASE_PATH: &str							= "data/db.sqlite";

// RSPOTIFY
pub const RSPOTIFY_ENV_CLIENT_ID: &str						= "RSPOTIFY_CLIENT_ID";
pub const RSPOTIFY_ENV_CLIENT_SECRET: &str					= "RSPOTIFY_CLIENT_SECRET";
pub const RSPOTIFY_CLIENT_TOKEN_PATH: &str					= "data/client_token.json";
pub const RSPOTIFY_USER_TOKEN_PATH: &str					= "data/user_token.json";
pub const RSPOTIFY_REDIRECT_URI: &str						= "http://localhost:8888/callback";
pub const RSPOTIFY_SCOPES: [&str; 3]						= ["playlist-modify-public", "playlist-modify-private", "ugc-image-upload"];
pub const RSPOTIFY_PLAYLIST_DESCRIPTION: &str				= "Playlist automatically created by Archify following an export of the archivied playlist.";

// Proxy
#[cfg(feature = "proxy")]
pub const REQWEST_ENV_HTTP_PROXY: &str						= "http://127.0.0.1:8080";
#[cfg(feature = "proxy")]
pub const REQWEST_ENV_HTTPS_PROXY: &str						= REQWEST_ENV_HTTP_PROXY;


// Default Values
pub const CONF_TIME_BIG_BANG: i64							= 0;
pub const CONF_SHA256_NULL: [u8; 32]						= [0;32];
pub const CONF_TIMESTAMP_NULL: u64							= 0;
pub const CONF_NULL_PLAYLIST_DATA: Option<FullPlaylist> 	= None;
pub const CONF_NULL_STRING: String							= String::new();
pub const CONF_DEFAULT_COUNT: u64							= 0;