use frame::Frame;

pub mod frame;
pub mod player;

pub struct Gif {
    pub width: u16,
    pub height: u16,
    pub frames: Vec<Frame>,
}

impl Gif {
    pub fn new(frames: Vec<Frame>, width: u16, height: u16) -> Self {
        Self {
            frames,
            width,
            height,
        }
    }
}
