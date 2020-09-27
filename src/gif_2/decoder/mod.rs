use crate::gif_2;
#[cfg(test)]
use std::fs::File;
use std::io::Read;
#[cfg(test)]
use std::path::Path;

pub mod steps;

#[derive(PartialEq)]
pub enum ColorOutput {
    RGBA,
    ColorMap,
}

pub fn decode(
    mut source: impl Read,
    color_output: ColorOutput,
) -> Result<gif_2::Gif, Box<dyn std::error::Error>> {
    let bytes: &mut Vec<u8> = &mut Vec::new();
    source.read_to_end(bytes)?;

    let (signature, cursor) = steps::signature::decode(&bytes, 0);
    let (mut screen_descriptor, cursor) = steps::screen_descriptor::decode(&bytes, cursor);
    let (mut global_color_map, cursor) =
        steps::color_map::decode(&bytes, screen_descriptor.pixel, screen_descriptor.m, cursor);
    let (mut frames, _cursor) = frames(bytes, cursor);

    if color_output == ColorOutput::RGBA {
        for frame in &mut frames {
            let rgba_raster_data = rgba_raster_data(&frame, global_color_map.as_ref());
            frame.raster_data = rgba_raster_data;
            frame.local_color_map = None;
            frame.image_descriptor.m = false;
        }
        screen_descriptor.m = false;
        global_color_map = None;
    }

    Ok(gif_2::Gif {
        signature: signature.to_string(),
        screen_descriptor,
        global_color_map,
        frames,
    })
}

pub fn rgba_raster_data(
    frame: &gif_2::Frame,
    global_color_map: Option<&gif_2::ColorMap>,
) -> Vec<u8> {
    let color_map = if frame.image_descriptor.m {
        frame
            .local_color_map
            .as_ref()
            .expect("expected local color map not present")
    } else {
        global_color_map.expect("expected global color map not present")
    };

    frame
        .raster_data
        .iter()
        .map(|index| {
            table_index_to_rgba(
                *index,
                color_map,
                frame
                    .graphic_control_extension
                    .as_ref()
                    .and_then(|ext| ext.transparent_color_index),
            )
        })
        .flatten()
        .collect()
}

pub fn table_index_to_rgba(
    index: u8,
    color_map: &gif_2::ColorMap,
    maybe_transparent_color_index: Option<u8>,
) -> Vec<u8> {
    let rgba = color_map
        .get(&(index as usize))
        .expect("pixel index not found in color map");
    let alpha = maybe_transparent_color_index
        .filter(|alpha_index| index == *alpha_index)
        .map(|_| 0x00u8)
        .unwrap_or(0xFFu8);
    vec![rgba.r, rgba.g, rgba.b, alpha]
}

pub fn frames(bytes: &Vec<u8>, cursor: usize) -> (Vec<gif_2::Frame>, usize) {
    let mut mut_index = cursor;
    let mut frames: Vec<gif_2::Frame> = Vec::new();

    while bytes[mut_index] != 0x3b {
        let (frame, index) = frame(bytes, mut_index);
        mut_index = index;
        frames.push(frame);
    }

    (frames, mut_index)
}

pub fn frame(bytes: &Vec<u8>, cursor: usize) -> (gif_2::Frame, usize) {
    let mut index = cursor;
    let mut graphic_control_extension: Option<gif_2::GraphicControlExtension> = None;

    while bytes[index] != 0x2c {
        if bytes[index] == 0x21 {
            let (block, cursor) = steps::extension_block::decode(bytes, index);
            index = cursor;

            match block {
                Some(gif_2::ExtensionBlock::GraphicControlExtension(extension)) => {
                    graphic_control_extension = Some(extension);
                }
                _ => {}
            }
        } else {
            index += 1;
        }
    }

    let (image_descriptor, index) = steps::image_descriptor::decode(bytes, index);
    let (color_map, index) =
        steps::color_map::decode(bytes, image_descriptor.pixel, image_descriptor.m, index);
    let (raster_data, index) = steps::raster_data::decode(bytes, index);

    (
        gif_2::Frame {
            image_descriptor,
            local_color_map: color_map,
            raster_data,
            graphic_control_extension,
        },
        index,
    )
}

#[test]
pub fn should_decode() {
    let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
    let gif = decode(file, ColorOutput::ColorMap).unwrap();

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
