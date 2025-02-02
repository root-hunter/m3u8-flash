use std::path::Path;

use serde::Serialize;
use url::Url;

use super::stream::Stream;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum MediaType {
    Audio,
    Subtitles
}

#[derive(Debug, Clone, Serialize)]
pub struct Media {
    pub _type: MediaType,
    pub url: String,
    pub group_id: String,   
    pub name: String,   
    pub default: bool,   
    pub auto_select: bool,   
    pub forced: bool,
    pub language: String,

    pub stream: Stream,
}

impl Media {
    pub fn new() -> Self {
        return Media{
            _type: MediaType::Audio,
            auto_select: false,
            default: false,
            forced: false,
            group_id: String::new(),
            language: String::new(),
            name: String::new(),
            url: String::new(),
            stream: Stream::new(),
        };
    }

    pub fn scan(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse(self.url.as_str()).unwrap();
  
        self.stream.url = url.to_string();
        self.stream.base_url = format!(
            "{}://{}{}",
            url.scheme(),
            url.domain().unwrap_or_default(),
            url.port().map_or("".to_string(), |p| format!(":{}", p))
        );

        self.stream.scan().unwrap();

        Ok(())
    }

    pub fn save(self: &mut Self, base_folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let folder_path = base_folder.join("audio");
        let target_filename = "audio.ts".to_string();
        
        self.stream.save(&folder_path, target_filename.clone()).unwrap();
        self.stream.output_path = base_folder.join(target_filename).to_str().unwrap().into();

        Ok(())
    }
}