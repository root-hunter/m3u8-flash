use regex::Regex;
use reqwest;
use serde::Serialize;
use url::Url;

use super::{media::{Media, MediaType}, stream::Stream};

#[derive(Debug, Clone, Serialize)]
pub struct Playlist {
    pub uid: String,
    pub url: String,
    pub audios: Vec<Media>,
    pub subtitles: Vec<Media>,
    pub playlists: Vec<Stream>,
}

impl Playlist {
    pub fn new(uid: String, url: String) -> Self {
        return Playlist {
            uid: uid,
            url: url,
            audios: Vec::new(),
            subtitles: Vec::new(),
            playlists: Vec::new(),
        };
    }

    pub fn scan(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        let reg_m3u8_file_start = Regex::new(r"#EXTM3U").unwrap();
        let reg_m3u8_stream = Regex::new(r"#EXT-X-STREAM-INF:").unwrap();
        let reg_m3u8_media = Regex::new(r"#EXT-X-MEDIA:").unwrap();
        let reg_m3u8_http = Regex::new(r"^http").unwrap();

        let response = reqwest::blocking::get(self.url.clone())?.text()?;

        let rows = response.split('\n');

        let mut index = 0;
        let mut stream = Stream::new();

        for row in rows {
            if index == 0 && !reg_m3u8_file_start.is_match(row) {
                panic!("Not valid M3U8 file");
            } else if reg_m3u8_media.is_match(row) {
                let parts = row.split_once(':').unwrap();
                let parts: Vec<&str> = parts.1.split(',').collect();
                
                let mut media = Media::new();

                for part in parts {
                    let tmp = part.split_once('=').unwrap();
                    
                    if tmp.0 == "TYPE" {
                        let _type_string = tmp.1.to_string().replace('"', "");
                        
                        if _type_string == "AUDIO" {
                            media._type = MediaType::Audio;
                        } else if _type_string == "SUBTITLES" {
                            media._type = MediaType::Subtitles;
                        }
                    } else if tmp.0 == "URI" {
                        media.url = tmp.1.to_string().replace('"', "");
                    } else if tmp.0 == "GROUP-ID" {
                        media.group_id = tmp.1.to_string().replace('"', "");
                    } else if tmp.0 == "LANGUAGE" {
                        media.language = tmp.1.to_string().replace('"', "");
                    } else if tmp.0 == "DEFAULT" {
                        let value = tmp.1.to_string().replace('"', "");

                        if value == "YES" {
                            media.default = true;
                        } else {
                            media.default = false;
                        }
                    } else if tmp.0 == "AUTOSELECT" {
                        let value = tmp.1.to_string();

                        if value == "YES" {
                            media.auto_select = true;
                        } else {
                            media.auto_select = false;
                        }
                    } else if tmp.0 == "FORCED" {
                        let value = tmp.1.to_string();

                        if value == "YES" {
                            media.forced = true;
                        } else {
                            media.forced = false;
                        }
                    }
                }

                media.scan().unwrap();

                if media._type == MediaType::Audio {
                    media.stream.output_path = "./generated/audio.ts".into();
                    self.audios.push(media.clone());
                } else if media._type == MediaType::Subtitles {
                    media.stream.output_path = "./generated/subtitle.ts".into();
                    self.subtitles.push(media.clone());
                }
            } else if reg_m3u8_stream.is_match(row) {
                let parts: Vec<&str> = row.split(':').collect();
                let parts: Vec<&str> = parts[1].split(',').collect();

                for part in parts {
                    let tmp = part.split_once('=').unwrap_or(("", ""));

                    if tmp.0 == "BANDWIDTH" {
                        stream.bandwidth = usize::from_str_radix(tmp.1, 10).unwrap()
                    } else if tmp.0 == "CODECS" {
                        stream.codecs = tmp.1.to_string().replace('"', "");
                    } else if tmp.0 == "RESOLUTION" {
                        stream.resolution = tmp.1.to_string();
                    }
                }
            } else if reg_m3u8_http.is_match(row) {
                let url = Url::parse(row).unwrap();
                stream.url = url.to_string();
                stream.base_url = format!(
                    "{}://{}{}",
                    url.scheme(),
                    url.domain().unwrap_or_default(),
                    url.port().map_or("".to_string(), |p| format!(":{}", p))
                );
                stream.scan().unwrap();
                stream.output_path = "./generated/video.ts".into();
                self.playlists.push(stream.clone());

                stream = Stream::new();
            }

            index += 1;
        }
        Ok(())
    }
}
