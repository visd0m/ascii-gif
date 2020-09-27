use crate::gif_2;
use std::collections::HashMap;

pub fn decode(
    bytes: &Vec<u8>,
    pixel: u8,
    m: bool,
    cursor: usize,
) -> (Option<gif_2::ColorMap>, usize) {
    if m {
        let mut map = HashMap::new();

        let map_entries = 3 * 2i32.pow(pixel as u32 + 1);
        let to_index = cursor + map_entries as usize;
        let entries = &bytes[cursor..to_index];

        entries
            .chunks(3 as usize)
            .map(|rgb| gif_2::Rgb {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
            })
            .enumerate()
            .for_each(|(index, rgb)| {
                map.insert(index, rgb);
            });

        (Some(map), to_index)
    } else {
        (None, cursor)
    }
}
