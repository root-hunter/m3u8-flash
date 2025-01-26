use std::path::{self, Path};
use std::process::Command;
use std::error::Error;

use crate::protocol::media::Media;
use crate::protocol::stream::Stream;

pub fn convert_ts_to_mp4(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let status = Command::new("ffmpeg")
        .args(&["-i", input, "-c:v", "copy", "-c:a", "copy", output])
        .status()?;

    if status.success() {
        println!("Conversione completata con successo: {} -> {}", input, output);
    } else {
        println!("Errore nella conversione.");
    }

    Ok(())
}

pub struct Export {
    pub video: Stream,
    pub audio: Option<Media>,
}

impl Export {
    pub fn save(self: &mut Self, target_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("START EXPORT");
        //println!("Stream: {:?}", self.stream);
        let export_folder = target_file.parent().unwrap();
        let output_path = self.video.output_path.as_str();
        
        std::fs::create_dir_all(export_folder).unwrap();

        match convert_ts_to_mp4(output_path, target_file.to_str().unwrap()) {
            Ok(_) => println!("Conversione completata con successo."),
            Err(e) => eprintln!("Errore durante la conversione: {}", e),
        }
        std::fs::remove_file(output_path).unwrap();

        println!("ALL FINISH");

        Ok(())
    }
}