use crate::ascii::symbol::{Encoding, Symbol};

pub struct Frame {
    pub width: u16,
    pub height: u16,
    pub buffer: Vec<Symbol>,
    pub delay: u16,
    pub top: u16,
    pub left: u16,
}

impl From<(&yaged::types::Frame, &Encoding)> for Frame {
    fn from((frame, encoding): (&yaged::types::Frame, &Encoding)) -> Self {
        Self {
            width: frame.image_descriptor().image_width(),
            height: frame.image_descriptor().image_height(),
            buffer: to_text_frame(&frame.rgba_raster_data().as_ref().unwrap(), encoding),
            delay: frame
                .graphic_control_extension()
                .as_ref()
                .map(|block| block.delay_time())
                .unwrap_or(100u16),
            top: frame.image_descriptor().image_top(),
            left: frame.image_descriptor().image_left(),
        }
    }
}

fn to_text_frame(buffer: &Vec<u8>, encoding: &Encoding) -> Vec<Symbol> {
    buffer
        .chunks(4)
        .map(|bytes| (bytes[0], bytes[1], bytes[2], bytes[3]))
        .map(|rgba| Symbol::from((rgba, encoding)))
        .collect::<Vec<Symbol>>()
}
