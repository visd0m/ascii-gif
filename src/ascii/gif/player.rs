use crate::ascii::frame::AsciiFrame;
use crate::ascii::gif::AsciiGif;
use crate::ascii::symbol::{to_string, AsciiSymbol};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use tokio::time::Duration;

pub struct AsciiGifPlayer {
    pub max_lines: u16,
    pub max_columns: u16,
    pub gif_width: u16,
    pub gif_height: u16,
    pub display_buffer: Vec<AsciiSymbol>,
}

impl AsciiGifPlayer {
    pub fn new(max_lines: u16, max_columns: u16) -> Self {
        Self {
            max_columns,
            max_lines,
            gif_width: max_columns,
            gif_height: max_lines,
            display_buffer: vec![
                AsciiSymbol {
                    symbol: " ".to_string(),
                    alpha: 0
                };
                (max_columns * max_lines) as usize
            ],
        }
    }

    pub fn play(&mut self, gif: &AsciiGif, r#loop: bool) {
        self.gif_width = gif.width;
        self.gif_height = gif.height;
        self.display_buffer = vec![
            AsciiSymbol {
                symbol: " ".to_string(),
                alpha: 0
            };
            (gif.width as u32 * gif.height as u32) as usize
        ];
        // clear screen
        print!("{esc}[2J", esc = 27 as char);

        if r#loop {
            loop {
                self.do_play(gif)
            }
        } else {
            self.do_play(gif)
        }
    }

    fn do_play(&mut self, gif: &AsciiGif) {
        for (index, frame) in gif.frames.iter().enumerate() {
            self.update_display_buffer(frame.top, frame.left, &frame.buffer, frame.width);

            debug_frame(index as u16, frame);

            self.display();

            sleep(Duration::from_millis((frame.delay * 10) as u64));
        }
    }

    pub fn display(&self) {
        print!("{esc}[1;1H", esc = 27 as char);

        print!(
            "{}",
            to_string(
                &self.display_buffer,
                self.gif_height as usize,
                self.gif_width as usize,
                self.max_lines as usize,
                self.max_columns as usize,
            )
        );
    }

    fn update_display_buffer(
        &mut self,
        top: u16,
        left: u16,
        new_buffer: &Vec<AsciiSymbol>,
        width: u16,
    ) {
        new_buffer
            .chunks(width as usize)
            .enumerate()
            .for_each(|(line_index, line)| {
                line.iter().enumerate().for_each(|(column_index, symbol)| {
                    let position: usize = (((top) as usize + line_index) * self.gif_width as usize)
                        + ((left) as usize)
                        + (column_index as usize);

                    if symbol.alpha == 255 {
                        self.display_buffer[position as usize] = symbol.clone();
                    }
                })
            })
    }
}

fn debug_frame(index: u16, frame: &AsciiFrame) {
    let ascii_frame_as_string: String = frame.to_string();
    let mut file = File::create(Path::new(&format!("./debug_frames/frame_{}.txt", index))).unwrap();
    file.write(ascii_frame_as_string.as_bytes()).unwrap();
}
