use crate::ascii::gif::Gif;
use crate::ascii::symbol::{to_string, Symbol};
use crate::postprocessor::{DisplayData, PostProcessor};
use std::thread::sleep;
use tokio::time::Duration;

pub struct Player {
    pub max_lines: u16,
    pub max_columns: u16,
    pub gif_width: u16,
    pub gif_height: u16,
    pub display_buffer: Vec<Symbol>,
    pub post_processors: Vec<Box<dyn PostProcessor>>,
}

impl Player {
    pub fn new(
        max_lines: u16,
        max_columns: u16,
        post_processors: Vec<Box<dyn PostProcessor>>,
    ) -> Self {
        Self {
            max_columns,
            max_lines,
            gif_width: max_columns,
            gif_height: max_lines,
            display_buffer: vec![
                Symbol {
                    symbol: " ".to_string(),
                    alpha: 0
                };
                (max_columns * max_lines) as usize
            ],
            post_processors,
        }
    }

    pub fn play(&mut self, gif: Gif, r#loop: bool) {
        self.gif_width = gif.width;
        self.gif_height = gif.height;

        self.display_buffer = vec![
            Symbol {
                symbol: " ".to_string(),
                alpha: 0
            };
            (gif.width as u32 * gif.height as u32) as usize
        ];
        // clear screen
        print!("{esc}[2J", esc = 27 as char);

        let ref_gif = &gif;

        if r#loop {
            loop {
                self.do_play(ref_gif);
            }
        } else {
            self.do_play(ref_gif)
        }
    }

    fn do_play(&mut self, gif: &Gif) {
        for frame in gif.frames.iter() {
            self.update_display_buffer(frame.top, frame.left, &frame.buffer, frame.width);

            self.display();

            sleep(Duration::from_millis((frame.delay * 10) as u64));
        }
    }

    pub fn display(&self) {
        print!("{esc}[1;1H", esc = 27 as char);

        let display_data = self.post_processors.iter().fold(
            DisplayData {
                buffer: self.display_buffer.clone(),
                height: self.gif_height,
                width: self.gif_width,
            },
            |acc, p| p.process(acc),
        );

        print!(
            "{}",
            to_string(
                &display_data.buffer,
                display_data.height as usize,
                display_data.width as usize,
                self.max_lines as usize,
                self.max_columns as usize,
            )
        );
    }

    fn update_display_buffer(&mut self, top: u16, left: u16, new_buffer: &Vec<Symbol>, width: u16) {
        new_buffer
            .chunks(width as usize)
            .enumerate()
            .for_each(|(line_index, line)| {
                line.to_vec()
                    .into_iter()
                    .enumerate()
                    .for_each(|(column_index, symbol)| {
                        let position: usize = (((top) as usize + line_index)
                            * self.gif_width as usize)
                            + ((left) as usize)
                            + (column_index as usize);

                        if symbol.alpha == 255 {
                            self.display_buffer[position as usize] = symbol;
                        }
                    })
            });
    }
}
