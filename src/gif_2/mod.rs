use std::collections::HashMap;

pub mod decoder;

type ColorMap = HashMap<usize, Rgb>;

#[derive(Debug)]
pub struct Gif {
    pub signature: String,
    pub screen_descriptor: ScreenDescriptor,
    pub global_color_map: Option<ColorMap>,
    pub frames: Vec<Frame>,
}

#[derive(Debug)]
pub struct ScreenDescriptor {
    pub width: u16,
    pub height: u16,
    pub m: bool,
    pub cr: u8,
    pub pixel: u8,
    pub background: u8,
}

#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub struct Frame {
    pub image_descriptor: ImageDescriptor,
    pub local_color_map: Option<ColorMap>,
    pub raster_data: Vec<u8>,
}

#[derive(Debug)]
pub struct ImageDescriptor {
    pub image_left: u16,
    pub image_top: u16,
    pub image_width: u16,
    pub image_height: u16,
    pub m: bool,
    pub i: bool,
    pub pixel: u8,
}
