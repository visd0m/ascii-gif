use crate::ascii::symbol::Symbol;

pub mod downscaling;

pub trait PostProcessor {
    fn process(&self, display_data: DisplayData) -> DisplayData;
}

pub struct DisplayData {
    pub buffer: Vec<Symbol>,
    pub width: u16,
    pub height: u16,
}
