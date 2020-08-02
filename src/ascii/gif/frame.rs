use crate::ascii::symbol::{to_string, AsciiSymbol};
use std::borrow::Cow;

pub struct AsciiGifFrame {
    pub width: u16,
    pub height: u16,
    pub buffer: Vec<AsciiSymbol>,
    pub delay: u16,
    pub top: u16,
    pub left: u16,
}

impl From<&gif::Frame<'_>> for AsciiGifFrame {
    fn from(frame: &gif::Frame<'_>) -> Self {
        Self {
            width: frame.width,
            height: frame.height,
            buffer: to_text_frame(&frame.buffer),
            delay: frame.delay,
            top: frame.top,
            left: frame.left,
        }
    }
}

impl AsciiGifFrame {
    pub fn to_string(&self) -> String {
        to_string(
            &self.buffer,
            self.height as usize,
            self.width as usize,
            self.height as usize,
            self.width as usize,
        )
    }
}

fn to_text_frame(buffer: &Cow<[u8]>) -> Vec<AsciiSymbol> {
    buffer
        .chunks(4)
        .map(|bytes| (bytes[0], bytes[1], bytes[2], bytes[3]))
        .map(|rgba| AsciiSymbol::from(rgba))
        .collect::<Vec<AsciiSymbol>>()
}