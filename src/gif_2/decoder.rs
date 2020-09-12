use crate::gif_2;
use crate::gif_2::ScreenDescriptor;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::from_utf8;
use weezl::{BitOrder, BufferResult, LzwError, LzwStatus};

pub struct Decoder {}

impl Decoder {
    pub fn decode(self, source: &mut impl Read) -> Result<gif_2::Gif, Box<dyn std::error::Error>> {
        let bytes: &mut Vec<u8> = &mut Vec::new();
        source.read_to_end(bytes)?;

        let (signature, index) = signature(&bytes, 0);
        let (screen_descriptor, index) = screen_descriptor(&bytes, index);
        let (global_color_map, index) =
            color_map(&bytes, screen_descriptor.pixel, screen_descriptor.m, index);
        let (frames, _index) = frames(bytes, index);

        Ok(gif_2::Gif {
            signature: signature.to_string(),
            screen_descriptor,
            global_color_map,
            frames,
        })
    }
}

pub fn signature(bytes: &Vec<u8>, index: usize) -> (String, usize) {
    let to_index = index + 6;
    let signature = from_utf8(&bytes[index..to_index]).unwrap();
    (signature.to_string(), to_index)
}

pub fn screen_descriptor(bytes: &Vec<u8>, index: usize) -> (ScreenDescriptor, usize) {
    let to_index = index + 7;
    let screen_descriptor = &bytes[index..to_index];

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
    index: usize,
) -> (Option<gif_2::ColorMap>, usize) {
    if m {
        let mut map = HashMap::new();

        let map_entries = 3 * 2i32.pow(pixel as u32 + 1);
        dbg!(map_entries);

        let to_index = index + map_entries as usize;

        let entries = &bytes[index..to_index];

        dbg!(entries.len());

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
        (None, index)
    }
}

pub fn frames(bytes: &Vec<u8>, index: usize) -> (Vec<gif_2::Frame>, usize) {
    let mut mut_index = index;
    let mut frames: Vec<gif_2::Frame> = Vec::new();
    while let Some(index) = find_frame_index(bytes, mut_index) {
        dbg!(index);
        let (frame, index) = frame(bytes, index);
        mut_index = index;
        frames.push(frame);
    }

    (frames, mut_index)
}

pub fn find_frame_index(bytes: &Vec<u8>, index: usize) -> Option<usize> {
    let mut found: bool = false;
    let mut search_index = index;
    let mut found_index = Some(index);
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

pub fn frame(bytes: &Vec<u8>, index: usize) -> (gif_2::Frame, usize) {
    let (image_descriptor, index) = image_descriptor(bytes, index);
    let (color_map, index) = color_map(bytes, image_descriptor.pixel, image_descriptor.m, index);
    let (raster_data, index) = raster_data(
        bytes,
        image_descriptor.image_width,
        image_descriptor.image_height,
        index,
    );
    (
        gif_2::Frame {
            image_descriptor,
            local_color_map: color_map,
            raster_data,
        },
        index,
    )
}

pub fn image_descriptor(bytes: &Vec<u8>, index: usize) -> (gif_2::ImageDescriptor, usize) {
    let to_index = index + 10;
    let image_descriptor = &bytes[index..to_index];

    let image_left = ((image_descriptor[2] as u16) << 8) | image_descriptor[1] as u16;
    let image_top = ((image_descriptor[4] as u16) << 8) | image_descriptor[3] as u16;
    let image_width = ((image_descriptor[6] as u16) << 8) | image_descriptor[5] as u16;
    let image_height = ((image_descriptor[8] as u16) << 8) | image_descriptor[7] as u16;

    let flags: u8 = image_descriptor[9];
    let m: bool = nth_bit(flags, 7);
    let i: bool = nth_bit(flags, 6);
    let pixel: u8 = (flags << 5) >> 5;
    dbg!(pixel);

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

pub fn raster_data(bytes: &Vec<u8>, width: u16, height: u16, index: usize) -> (Vec<u8>, usize) {
    // data is LZW compressed
    let code_size = bytes[index];
    let mut lzw_decoder = weezl::decode::Decoder::new(BitOrder::Lsb, code_size);

    let decoded: &mut Vec<u8> = &mut vec![0; (width as u32 * height as u32) as usize];

    let mut block_index: usize = index + 1;

    let mut done = false;
    while !done || bytes[block_index] != 0b00000000 {
        dbg!(block_index);
        // dbg!(&decoded);
        let decoded_index = decode_block(bytes, block_index, &mut lzw_decoder, decoded);
        block_index = decoded_index;
    }

    (decoded.to_vec(), block_index + 1)
}

pub fn decode_block(
    bytes: &Vec<u8>,
    index: usize,
    decoder: &mut weezl::decode::Decoder,
    mut decoded: &mut Vec<u8>,
) -> usize {
    let block_size = bytes[index] as usize;
    let mut left = block_size;

    let mut to_decode_index = index + 1;
    while left > 0 {
        dbg!(left);
        dbg!(to_decode_index);

        let inp = &bytes[to_decode_index..to_decode_index + left];

        dbg!(inp.len());
        let result = decoder.decode_bytes(inp, decoded);

        dbg!(result.consumed_in);
        dbg!(result.consumed_out);
        dbg!(result.status);

        match result.status {
            Ok(ok) => match ok {
                LzwStatus::Ok => {
                    left -= result.consumed_in;
                }
                LzwStatus::NoProgress => panic!("S'è rott tutt!"),
                LzwStatus::Done => panic!("S'è rott tutt!"),
            },
            Err(error) => match error {
                LzwError::InvalidCode => panic!("S'è rott tutt!"),
            },
        }
    }

    dbg!(decoded);
    to_decode_index
}

#[test]
pub fn should_decode() {
    let d = Decoder {};

    let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
    let gif = d.decode(file).unwrap();

    assert_eq!("GIF89a", gif.signature);
    // dbg!(gif);
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
