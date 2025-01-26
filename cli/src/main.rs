use std::path::Path;

use clap::{Arg, Command};
use log::{error, info, warn};
use m3u8_flash_core::{output::{self, export::Export}, protocol::playlist::Playlist};

fn main() {
    env_logger::init();

    let matches = Command::new("M3U8 Flash - CLI")
        .version("1.0")
        .author("Antonio Ricciardi")
        .about("Esempio di CLI con Clap")
        .arg(
            Arg::new("playlist")
                .short('p')
                .long("playlist")
                .value_name("PLAYLIST_URL")
                .help("Insert the playlist URL")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_PATH")
                .help("Insert the output PATH")
                .required(true),
        )
        .get_matches();

    let playlist_url = matches
        .get_one::<String>("playlist")
        .expect("Playlist URL is required");

    info!("Playlist URL: {}", playlist_url);

    let output_path = matches
        .get_one::<String>("output")
        .expect("Output PATH is required");

    let output_path = Path::new(output_path);
    let output_path_base = output_path.parent().unwrap();

    info!("Output PATH: {:?}", output_path);

    let mut library = Playlist::new(playlist_url.to_string());
    library.scan().unwrap();

    let mut video = library.playlists[1].clone();
    let folder_path = output_path_base.join("video");
    
    let mut audio = None;

    if library.audios.len() > 0 {
        audio = Some(library.audios[0].clone());
    }

    let target_filename = "video.ts".to_string();
    video.save(&folder_path, target_filename).unwrap();
    
    if audio.is_some() {
        let mut audio = audio.clone().unwrap();
        audio.save(output_path_base).unwrap();
    }

    let mut export = Export {
        video: video,
        audio: audio,
    };

    export.save(output_path).unwrap();
}
