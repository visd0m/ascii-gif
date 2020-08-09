use crate::ascii::symbol::AsciiSymbol;

pub mod downscaling;

pub trait PostProcessor {
    fn process(&self, display_data: DisplayData) -> DisplayData;
}

pub struct DisplayData {
    pub buffer: Vec<AsciiSymbol>,
    pub width: u16,
    pub height: u16,
}
