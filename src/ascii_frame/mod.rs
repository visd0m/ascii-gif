use gif::Frame;
use std::borrow::Cow;

pub struct AsciiFrame {
    pub width: u16,
    pub height: u16,
    pub buffer: Vec<String>,
}

impl From<&Frame<'_>> for AsciiFrame {
    fn from(frame: &Frame) -> Self {
        Self {
            width: frame.width,
            height: frame.height,
            buffer: to_text_frame(&frame.buffer),
        }
    }
}

impl AsciiFrame {
    pub fn to_string(&self) -> String {
        self.buffer
            .chunks(self.width as usize)
            .map(|x| format!("{}", x.join("")))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn to_text_frame(buffer: &Cow<[u8]>) -> Vec<String> {
    buffer
        .chunks(4)
        .map(|bytes| (bytes[0], bytes[1], bytes[2], bytes[3]))
        .map(|(r, g, b, _a)| gray_value(r, g, b))
        .map(to_ascii)
        .collect::<Vec<String>>()
}

fn gray_value(r: u8, g: u8, b: u8) -> u8 {
    ((0.3 * (r as f32)) + (0.59 * (g as f32)) + (0.11 * (b as f32))) as u8
}

fn to_ascii(value: u8) -> String {
    if value < 10 {
        "@".to_string()
    } else if value > 10 && value < 50 {
        "#".to_string()
    } else if value > 50 && value < 100 {
        "*".to_string()
    } else if value >= 100 && value < 150 {
        "+".to_string()
    } else if value >= 150 && value < 200 {
        "~".to_string()
    } else if value >= 200 && value < 250 {
        "-".to_string()
    } else {
        ".".to_string()
    }
}
