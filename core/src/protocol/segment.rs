use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Segment {
    pub duration: usize,
    pub url: String,
    pub data: Vec<u8>
}

impl Segment {
    pub fn new() -> Self {
        return Segment {
            url: String::new(),
            duration: 0,
            data: Vec::new()
        };
    }
}