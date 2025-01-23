use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
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
            url: String::new()
        };
    }
}