use crate::ascii::symbol::{to_string, Encoding, Symbol};
use std::borrow::Cow;

pub struct Frame {
    pub width: u16,
    pub height: u16,
    pub buffer: Vec<Symbol>,
    pub delay: u16,
    pub top: u16,
    pub left: u16,
}

impl From<(&gif::Frame<'_>, &Encoding)> for Frame {
    fn from((frame, encoding): (&gif::Frame<'_>, &Encoding)) -> Self {
        Self {
            width: frame.width,
            height: frame.height,
            buffer: to_text_frame(&frame.buffer, encoding),
            delay: frame.delay,
            top: frame.top,
            left: frame.left,
        }
    }
}

impl Frame {
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

fn to_text_frame(buffer: &Cow<[u8]>, encoding: &Encoding) -> Vec<Symbol> {
    buffer
        .chunks(4)
        .map(|bytes| (bytes[0], bytes[1], bytes[2], bytes[3]))
        .map(|rgba| Symbol::from((rgba, encoding)))
        .collect::<Vec<Symbol>>()
}
