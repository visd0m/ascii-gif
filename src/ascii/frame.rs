use crate::ascii::symbol::Symbol;
use gif::Frame;
use std::borrow::Cow;

pub struct AsciiFrame {
    pub width: u16,
    pub height: u16,
    pub buffer: Vec<Symbol>,
    pub delay: u16,
    pub top: u16,
    pub left: u16,
}

impl From<&Frame<'_>> for AsciiFrame {
    fn from(frame: &Frame) -> Self {
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

impl AsciiFrame {
    pub fn to_string(&self) -> String {
        self.buffer
            .chunks(self.width as usize)
            .map(|x| x.iter().map(|s| s.symbol.clone()).collect::<Vec<String>>())
            .map(|x| format!("{}", x.join("")))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn to_text_frame(buffer: &Cow<[u8]>) -> Vec<Symbol> {
    buffer
        .chunks(4)
        .map(|bytes| (bytes[0], bytes[1], bytes[2], bytes[3]))
        .map(|rgba| Symbol::from(rgba))
        .collect::<Vec<Symbol>>()
}
