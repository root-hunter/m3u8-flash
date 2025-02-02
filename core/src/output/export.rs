use std::error::Error;
use std::path::{self, Path};
use std::process::Command;

use crate::protocol::media::Media;
use crate::protocol::stream::Stream;

pub fn convert_ts_to_mp4(
    video: &str,
    output: &str,
    audio: Option<String>,
) -> Result<(), Box<dyn Error>> {

    Ok(())
}

pub struct Export {
    pub video: Stream,
    pub audio: Option<Media>,
}

impl Export {
    pub fn save(self: &mut Self, target_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("START EXPORT");
        let export_folder = target_file.parent().unwrap();
        std::fs::create_dir_all(export_folder).unwrap();

        let video_path = self.video.output_path.as_str();
        let mut audio_path: Option<String> = None;
        let output_path = target_file.to_str().unwrap();

        let mut params: Vec<String> = Vec::from([
            "-i".into(), video_path.into()
        ]);

        if self.audio.is_some() {
            let audio = self.audio.clone().unwrap();
            println!("{:?}", audio);
            audio_path = Some(audio.stream.output_path);
            
            params.push("-i".into());
            params.push(audio_path.clone().unwrap().into());
            //params.push("./generated/audio.ts".into());
        }

        params.push("-c:v".into());
        params.push("copy".into());

        params.push("-c:a".into());
        params.push("aac".into());
        
        params.push("-strict".into());
        params.push("experimental".into());
        
        params.push(output_path.into());
    
        println!("{:#?}", params);

        let status = Command::new("ffmpeg").args(&params).status()?;
    
        if status.success() {
            println!(
                "Conversione completata con successo: {} -> {}",
                video_path, output_path
            );
        } else {
            println!("Errore nella conversione.");
        }
    
        std::fs::remove_file(video_path).unwrap();

        if audio_path.is_some() {
            std::fs::remove_file(audio_path.unwrap()).unwrap();
        }

        Ok(())
    }
}
