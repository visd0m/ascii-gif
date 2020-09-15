use crate::gif_2;
use lzw::LsbReader;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::from_utf8;

pub struct Decoder {}

impl Decoder {
    pub fn decode(source: &mut impl Read) -> Result<gif_2::Gif, Box<dyn std::error::Error>> {
        let bytes: &mut Vec<u8> = &mut Vec::new();
        source.read_to_end(bytes)?;

        let (signature, cursor) = signature(&bytes, 0);
        let (screen_descriptor, cursor) = screen_descriptor(&bytes, cursor);
        let (global_color_map, cursor) =
            color_map(&bytes, screen_descriptor.pixel, screen_descriptor.m, cursor);
        let (frames, _cursor) = frames(bytes, cursor);

        Ok(gif_2::Gif {
            signature: signature.to_string(),
            screen_descriptor,
            global_color_map,
            frames,
        })
    }
}

pub fn signature(bytes: &Vec<u8>, cursor: usize) -> (String, usize) {
    let to_index = cursor + 6;
    let signature = from_utf8(&bytes[cursor..to_index]).unwrap();
    (signature.to_string(), to_index)
}

pub fn screen_descriptor(bytes: &Vec<u8>, cursor: usize) -> (gif_2::ScreenDescriptor, usize) {
    let to_index = cursor + 7;
    let screen_descriptor = &bytes[cursor..to_index];

    let screen_width = ((screen_descriptor[1] as u16) << 8) | screen_descriptor[0] as u16;
    let screen_height = ((screen_descriptor[3] as u16) << 8) | screen_descriptor[2] as u16;

    let flags: u8 = screen_descriptor[4];
    let m: bool = nth_bit(flags, 7);
    let cr: u8 = (flags << 1) >> 5;
    let pixel: u8 = (flags << 5) >> 5;

    let background: u8 = screen_descriptor[5];

    (
        gif_2::ScreenDescriptor {
            width: screen_width,
            height: screen_height,
            m,
            cr,
            pixel,
            background,
        },
        to_index,
    )
}

pub fn color_map(
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

pub fn frames(bytes: &Vec<u8>, cursor: usize) -> (Vec<gif_2::Frame>, usize) {
    let mut mut_index = cursor;
    let mut frames: Vec<gif_2::Frame> = Vec::new();
    while let Some(index) = find_frame_index(bytes, mut_index) {
        let (frame, index) = frame(bytes, index);
        mut_index = index;
        frames.push(frame);
    }

    (frames, mut_index)
}

pub fn find_frame_index(bytes: &Vec<u8>, cursor: usize) -> Option<usize> {
    let mut found: bool = false;
    let mut search_index = cursor;
    let mut found_index = Some(cursor);
    while !found {
        match bytes[search_index] {
            0x2c => {
                found = true;
                found_index = Some(search_index);
            }
            0x3b => {
                found = true;
                found_index = None;
            }
            _ => {
                search_index += 1;
            }
        }
    }
    found_index
}

pub fn frame(bytes: &Vec<u8>, cursor: usize) -> (gif_2::Frame, usize) {
    let (image_descriptor, index) = image_descriptor(bytes, cursor);
    let (color_map, index) = color_map(bytes, image_descriptor.pixel, image_descriptor.m, index);
    let (raster_data, index) = raster_data(bytes, index);
    (
        gif_2::Frame {
            image_descriptor,
            local_color_map: color_map,
            raster_data,
        },
        index,
    )
}

pub fn image_descriptor(bytes: &Vec<u8>, cursor: usize) -> (gif_2::ImageDescriptor, usize) {
    let to_index = cursor + 10;
    let image_descriptor = &bytes[cursor..to_index];

    let image_left = ((image_descriptor[2] as u16) << 8) | image_descriptor[1] as u16;
    let image_top = ((image_descriptor[4] as u16) << 8) | image_descriptor[3] as u16;
    let image_width = ((image_descriptor[6] as u16) << 8) | image_descriptor[5] as u16;
    let image_height = ((image_descriptor[8] as u16) << 8) | image_descriptor[7] as u16;

    let flags: u8 = image_descriptor[9];
    let m: bool = nth_bit(flags, 7);
    let i: bool = nth_bit(flags, 6);
    let pixel: u8 = (flags << 5) >> 5;

    (
        gif_2::ImageDescriptor {
            image_left,
            image_top,
            image_width,
            image_height,
            m,
            i,
            pixel,
        },
        to_index,
    )
}

pub fn raster_data(bytes: &Vec<u8>, cursor: usize) -> (Vec<u8>, usize) {
    // data is LZW compressed
    let code_size = bytes[cursor];
    let mut lzw_decoder = lzw::Decoder::new(lzw::LsbReader::new(), code_size);

    let decoded: &mut Vec<u8> = &mut vec![];

    let mut block_index: usize = cursor + 1;

    while bytes[block_index] != 0b00000000 {
        let decoded_index = decode_block(bytes, block_index, &mut lzw_decoder, decoded);
        block_index = decoded_index;
    }

    (decoded.to_vec(), block_index + 1)
}

pub fn decode_block(
    bytes: &Vec<u8>,
    cursor: usize,
    decoder: &mut lzw::Decoder<LsbReader>,
    decoded: &mut Vec<u8>,
) -> usize {
    let block_size = bytes[cursor] as usize;
    let mut left = block_size;

    let mut to_decode_index = cursor + 1;
    while left > 0 {
        let inp = &bytes[to_decode_index..to_decode_index + left];
        let (consumed, bytes) = decoder.decode_bytes(inp).expect("S'è rott tutt!");
        to_decode_index += consumed;
        left -= consumed;
        decoded.append(&mut bytes.to_vec())
    }

    to_decode_index
}

#[test]
pub fn should_decode() {
    let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
    let gif = Decoder::decode(file).unwrap();

    assert_eq!("GIF89a", gif.signature);
    assert_eq!(106, gif.frames.len());
    gif.frames.iter().for_each(|frame| {
        assert_eq!(
            frame.raster_data.len(),
            (frame.image_descriptor.image_width as u32 * frame.image_descriptor.image_height as u32)
                as usize
        );

        if frame.image_descriptor.m {
            assert!(frame.local_color_map.is_some())
        } else {
            assert!(frame.local_color_map.is_none())
        }
    });
}

pub fn nth_bit(byte: u8, nth: usize) -> bool {
    byte & (1 << nth) != 0
}

#[test]
pub fn bytes() {
    let byte: u8 = 0b11111111;
    let result = (byte << 5) >> 5;
    dbg!(format!("{:b}", result));
}
