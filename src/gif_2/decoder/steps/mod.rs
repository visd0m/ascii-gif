pub mod color_map;
pub mod extension_block;
pub mod image_descriptor;
pub mod raster_data;
pub mod screen_descriptor;
pub mod signature;

pub fn nth_bit(byte: u8, nth: usize) -> bool {
    byte & (1 << nth) != 0
}
