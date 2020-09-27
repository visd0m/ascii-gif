use crate::gif_2;
use crate::gif_2::decoder::steps::nth_bit;

pub fn decode(bytes: &Vec<u8>, cursor: usize) -> (Option<gif_2::ExtensionBlock>, usize) {
    let label = bytes[cursor + 1];
    match label {
        0xf9 => {
            let (block, cursor) = graphic_control_extension(bytes, cursor + 2);
            (
                Some(gif_2::ExtensionBlock::GraphicControlExtension(block)),
                cursor,
            )
        }
        _ => (None, cursor + 2),
    }
}

pub fn graphic_control_extension(
    bytes: &Vec<u8>,
    cursor: usize,
) -> (gif_2::GraphicControlExtension, usize) {
    let flags = bytes[cursor + 1];
    let disposal_method = (flags << 3) >> 5;
    let user_input_flag = nth_bit(flags, 1);
    let transparent_color_flag = nth_bit(flags, 0);

    let delay_time = ((bytes[cursor + 3] as u16) << 8) | bytes[cursor + 2] as u16;
    let transparent_color_index = if transparent_color_flag {
        Some(bytes[cursor + 4])
    } else {
        None
    };

    (
        gif_2::GraphicControlExtension {
            disposal_method,
            user_input: user_input_flag,
            transparent_color: transparent_color_flag,
            delay_time,
            transparent_color_index,
        },
        cursor + if transparent_color_flag { 6 } else { 5 },
    )
}
