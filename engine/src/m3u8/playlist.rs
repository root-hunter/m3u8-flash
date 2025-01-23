use std::{
    fmt::format,
    fs::{self, DirEntry, File, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
    path,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use aes::Aes128;
use cbc::Decryptor;
use cipher::{block_padding::Pkcs7, generic_array::GenericArray, BlockDecryptMut, KeyIvInit};
use hex::decode;
use regex::Regex;
use reqwest;
use serde::Serialize;
use std::thread;
use url::Url;

use crate::{m3u8::segment::Segment, output::export::convert_ts_to_mp4};

#[derive(Debug, Clone, Serialize)]
pub enum PlaylistType {
    VOD,
    EVENT,
    ERROR,
}

#[derive(Debug, Clone, Serialize)]
pub enum PlaylistCryptMethod {
    Aes128,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlaylistCrypt {
    method: PlaylistCryptMethod,
    url: String,
    bytes: Vec<u8>,
    iv: Option<Vec<u8>>,
}

impl PlaylistCrypt {
    pub fn new() -> Self {
        return PlaylistCrypt {
            url: String::new(),
            method: PlaylistCryptMethod::Aes128,
            bytes: Vec::new(),
            iv: None,
        };
    }

    pub fn set_iv(self: &mut Self, hex: &str) -> Result<(), Box<dyn std::error::Error>> {
        let cleaned_hex = hex.trim_start_matches("0x");
        self.iv = Some(decode(cleaned_hex).unwrap());

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Playlist {
    pub base_url: String,
    pub url: String,
    pub bandwidth: usize,
    pub codecs: String,
    pub resolution: String,
    pub version: usize,
    pub _type: PlaylistType,
    pub target_duration: usize,
    pub key: Option<PlaylistCrypt>,
    pub segments: Vec<Segment>,
}

impl Playlist {
    pub fn new() -> Self {
        return Playlist {
            base_url: String::new(),
            url: String::new(),
            _type: PlaylistType::EVENT,
            target_duration: 0,
            version: 0,
            key: None,
            bandwidth: 0,
            codecs: String::new(),
            resolution: String::new(),
            segments: Vec::new(),
        };
    }
    pub fn scan(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        let reg_m3u8_file_start = Regex::new(r"#EXTM3U").unwrap();
        let reg_m3u8_version = Regex::new(r"#EXT-X-VERSION:(\d{1,})").unwrap();
        let reg_m3u8_playlist_type = Regex::new(r"#EXT-X-PLAYLIST-TYPE:(VOD|EVENT)").unwrap();
        let reg_m3u8_target_duration = Regex::new(r"#EXT-X-TARGETDURATION:(\d{1,})").unwrap();
        let reg_m3u8_seg_duration = Regex::new(r"#EXTINF:(\d+),").unwrap();
        let reg_m3u8_seg_http = Regex::new(r"http").unwrap();
        let reg_m3u8_key = Regex::new(r"#EXT-X-KEY:").unwrap();

        println!("{}", self.url);
        let response = reqwest::blocking::get(self.url.clone())?.text()?;

        let rows = response.split('\n');

        let mut index = 0;
        let mut segment = Segment::new();

        for row in rows {
            if index == 0 && !reg_m3u8_file_start.is_match(row) {
                panic!("Not valid M3U8 file");
            } else {
                if reg_m3u8_seg_duration.is_match(row) {
                    let row = row.replace(',', "");
                    let parts: Vec<&str> = row.split(':').collect();
                    let value = usize::from_str_radix(parts[1], 10).unwrap();

                    segment.duration = value
                }
                if reg_m3u8_seg_http.is_match(row) {
                    segment.url = row.to_string();
                    self.segments.push(segment.clone());
                    segment = Segment::new();
                } else if reg_m3u8_playlist_type.is_match(row) {
                    let parts: Vec<&str> = row.split(':').collect();

                    if parts[1] == "VOD" {
                        self._type = PlaylistType::VOD;
                        println!("{:#?}", self);
                    } else if parts[1] == "EVENT" {
                        self._type = PlaylistType::EVENT;
                    } else {
                        self._type = PlaylistType::ERROR;
                    }
                } else if reg_m3u8_target_duration.is_match(row) {
                    let parts: Vec<&str> = row.split(':').collect();
                    self.target_duration = usize::from_str_radix(parts[1], 10).unwrap();
                } else if reg_m3u8_version.is_match(row) {
                    let parts: Vec<&str> = row.split(':').collect();
                    self.version = usize::from_str_radix(parts[1], 10).unwrap();
                } else if reg_m3u8_key.is_match(row) {
                    let parts: Vec<&str> = row.split(':').collect();
                    let parts: Vec<&str> = parts[1].split(',').collect();
                    self.key = Some(PlaylistCrypt {
                        method: PlaylistCryptMethod::Aes128,
                        url: String::new(),
                        bytes: Vec::new(),
                        iv: None,
                    });

                    if let Some(ref mut key) = self.key {
                        for part in parts {
                            let tmp: Vec<&str> = part.split('=').collect();

                            if tmp[0] == "METHOD" {
                                if tmp[1] == "AES-128" {
                                    key.method = PlaylistCryptMethod::Aes128;
                                }
                            } else if tmp[0] == "URI" {
                                let key_uri = tmp[1].replace('"', "").to_string();
                                let base_url = Url::parse(&self.base_url.as_str()).unwrap();
                                let key_url = base_url.join(&key_uri).unwrap();

                                key.url = key_url.as_str().to_string();

                                let bytes = reqwest::blocking::get(key.clone().url)
                                    .unwrap()
                                    .bytes()
                                    .unwrap();
                                key.bytes = bytes.to_vec();
                            } else if tmp[0] == "IV" {
                                key.set_iv(tmp[1]).unwrap();
                            }
                        }
                    }
                }
            }

            //println!("{}", row);
            index += 1;
        }

        Ok(())
    }

    pub fn save(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH);
        let folder_path = format!("./generated/{:?}", now.unwrap());
        let folder = path::Path::new(folder_path.as_str());
        let key = self.key.clone().unwrap();

        let segments = self.segments.clone();
        let chunk_size = (segments.len() / 8).max(1);
        let chunks: Vec<_> = segments.chunks(chunk_size).map(|c| c.to_vec()).collect();
        let mut handles = vec![];
        let reg_temp_file = Regex::new(r"temp_(\d+).part").unwrap();

        std::fs::create_dir(folder).unwrap();

        let mut j: usize = 0;
        for chunk in chunks {
            let file_path = folder.join(format!("temp_{}.part", j));
            let file = OpenOptions::new()
                .create(true) // Crea il file se non esiste
                .append(true) // Aggiungi i dati invece di sovrascrivere
                .write(true) // Permetti la scrittura
                .open(file_path)?; // Nome del file di destinazione
            let writer = Arc::new(Mutex::new(BufWriter::new(file)));

            let cipher =
                Decryptor::<Aes128>::new_from_slices(&key.bytes, &key.iv.clone().unwrap()).unwrap();
            let writer_clone = Arc::clone(&writer);
            let handle = thread::spawn(move || {
                for segment in chunk {
                    match reqwest::blocking::get(&segment.url) {
                        Ok(response) => {
                            let mut data = response.bytes().unwrap().to_vec();
                            match cipher.clone().decrypt_padded_mut::<Pkcs7>(&mut data) {
                                Ok(decrypted_data) => {
                                    let mut writer_lock = writer_clone.lock().unwrap();
                                    writer_lock.write_all(decrypted_data).unwrap();
                                }
                                Err(e) => eprintln!("Errore di padding: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Errore nel download: {}", e),
                    }
                }
            });
            handles.push(handle);
            j += 1;
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let output_path = folder.join("output.ts");
        let mut output = BufWriter::new(File::create(output_path.clone())?);

        let mut files: Vec<_> = fs::read_dir(folder)?
            .filter_map(|entry| entry.ok())
            .filter(|e| {
                let file_path = e.path();
                let file_name = file_path.file_name().unwrap();
                let mut value = e.path().is_file();

                value = value && reg_temp_file.is_match(file_name.to_str().unwrap());
                return value;
            })
            .collect();
        files.sort_by_key(|f| {
            let file_name = f.file_name();
            let parts: Vec<&str> = file_name.to_str().unwrap().split('_').collect();
            let value = parts[1].replace(".part", "");

            return usize::from_str_radix(value.as_str(), 10).unwrap();
        });

        let files: Vec<&DirEntry> = files.iter().collect();
        for file in files {
            let file_path = file.path();
            println!("{:?}", file_path.as_os_str());

            let mut input = BufReader::new(File::open(&file_path)?);
            let mut buffer = Vec::new();
            input.read_to_end(&mut buffer).unwrap();
            output.write_all(&buffer).unwrap();
            std::fs::remove_file(file_path).unwrap();

        }

        output.flush()?; // Assicura che tutti i dati siano scritti

        let output_path = output_path.to_str().unwrap();
        let final_path = output_path.replace("ts", "mp4");

        match convert_ts_to_mp4(output_path, final_path.as_str()) {
            Ok(_) => println!("Conversione completata con successo."),
            Err(e) => eprintln!("Errore durante la conversione: {}", e),
        }
        std::fs::remove_file(output_path).unwrap();

        println!("ALL FINISH");
        Ok(())
    }
}
