use clap::{Parser, error::ErrorKind, CommandFactory};

#[derive(Parser)]
#[clap(author="Inheritor-Vision")]
#[clap(version)]
#[clap(about = "It is aimed at keeping record of temporary playlist like Weekly Discovery, made by Spotify, or public playlist.", long_about = None)]
#[clap(arg_required_else_help = true)]
struct Cli {
	/// Add public playlist to archive
	#[arg(short, long, value_parser, num_args(1..))]
	add_playlist: Option<Vec<String>>,
	/// Update playlists stored in database
	#[arg(short,long,action,value_parser)]
	update: bool,
	/// List tracked playlists
	#[arg(short,long,action,value_parser)]
	list: bool,
	/// List versions of a single tracked playlist
	#[arg(short,long,action,value_parser)]
	tracked: Option<String>,
	/// Delete a playlist
	#[arg(short,long,value_parser, num_args(1..))] 
	delete_playlist: Option<Vec<String>>,
}

pub enum Args {
	NewPlaylist(Vec<String>),
	DeletePlaylist(Vec<String>),
	Update,
	List,
	Tracked(String)
}

pub fn parse_args() -> Args{
	let cli = Cli::parse();
	let res;

	if cli.update {
		res = Args::Update;
	}else if cli.add_playlist != None {
		res = Args::NewPlaylist(cli.add_playlist.unwrap());
	}else if cli.delete_playlist != None {
		res = Args::DeletePlaylist(cli.delete_playlist.unwrap());
	}else if cli.list {
		res = Args::List;
	} else if cli.tracked != None {
		res = Args::Tracked(cli.tracked.unwrap());
	}else{
		let mut cmd = Cli::command();
		cmd.error(
			ErrorKind::DisplayHelp,
			""
		).exit()

	}
	res
}