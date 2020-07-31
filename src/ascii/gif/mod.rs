use crate::ascii::frame::AsciiFrame;

pub mod player;

pub struct AsciiGif {
    pub width: u16,
    pub height: u16,
    pub frames: Vec<AsciiFrame>,
}

#[derive(Debug)]
pub enum Error {
    NoIndex(ErrorPayload),
}

#[derive(Debug)]
pub struct ErrorPayload {
    pub message: String,
}

impl AsciiGif {
    pub fn new(frames: Vec<AsciiFrame>, width: u16, height: u16) -> Self {
        Self {
            frames,
            width,
            height,
        }
    }
}
