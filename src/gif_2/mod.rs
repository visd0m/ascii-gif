pub mod decoder;

pub struct Gif {
    pub signature: String,
    pub width: u16,
    pub height: u16,
}
