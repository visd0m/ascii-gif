use crate::ascii::symbol::{Encoding, Symbol};
use crate::gif_2;
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

impl From<(&gif_2::Frame, &Encoding)> for Frame {
    fn from((frame, encoding): (&gif_2::Frame, &Encoding)) -> Self {
        Self {
            width: frame.image_descriptor.image_width,
            height: frame.image_descriptor.image_height,
            buffer: to_text_frame_2(&frame.raster_data, encoding),
            delay: frame
                .graphic_control_extension
                .as_ref()
                .expect("graphic control extension not found")
                .delay_time,
            top: frame.image_descriptor.image_top,
            left: frame.image_descriptor.image_left,
        }
    }
}

fn to_text_frame_2(buffer: &Vec<u8>, encoding: &Encoding) -> Vec<Symbol> {
    buffer
        .chunks(4)
        .map(|bytes| (bytes[0], bytes[1], bytes[2], bytes[3]))
        .map(|rgba| Symbol::from((rgba, encoding)))
        .collect::<Vec<Symbol>>()
}

fn to_text_frame(buffer: &Cow<[u8]>, encoding: &Encoding) -> Vec<Symbol> {
    buffer
        .chunks(4)
        .map(|bytes| (bytes[0], bytes[1], bytes[2], bytes[3]))
        .map(|rgba| Symbol::from((rgba, encoding)))
        .collect::<Vec<Symbol>>()
}
