use crate::ascii::symbol::Symbol;
use crate::postprocessor::{DisplayData, PostProcessor};

pub struct Downscaling {
    pub target_width: u16,
    pub target_height: u16,
}

impl Downscaling {
    pub fn new(target_width: u16, target_height: u16) -> Self {
        Self {
            target_width,
            target_height,
        }
    }
}

impl PostProcessor for Downscaling {
    fn process(&self, display_data: DisplayData) -> DisplayData {
        let rg = display_data.width as f64 / display_data.height as f64;
        let rs = self.target_width as f64 / self.target_height as f64;

        let (scaled_width, scaled_height) = if rs > rg {
            (
                display_data.width * self.target_height / display_data.height,
                self.target_height,
            )
        } else {
            (
                self.target_width,
                display_data.height * self.target_width / display_data.width,
            )
        };

        if scaled_height == display_data.height && scaled_width == display_data.width {
            return display_data;
        }

        let scaled_height = if display_data.height < scaled_height {
            display_data.height
        } else {
            scaled_height
        };

        let scaled_width = if display_data.width < scaled_width {
            display_data.width
        } else {
            scaled_width
        };

        let pixels_to_remove_width = display_data.width - scaled_width;
        let pixels_to_remove_height = display_data.height - scaled_height;

        let remove_one_width_every =
            (display_data.width as f64 / pixels_to_remove_width as f64).ceil() as usize;
        let remove_one_height_every =
            (display_data.height as f64 / pixels_to_remove_height as f64).ceil() as usize;

        let mut lines = display_data
            .buffer
            .chunks(display_data.width as usize)
            .into_iter()
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<Symbol>>>();

        for i in 0..pixels_to_remove_height {
            lines.remove(((i * remove_one_height_every as u16) % lines.len() as u16) as usize);
        }

        let buffer = lines
            .into_iter()
            .flat_map(|line| {
                let mut line = line.clone();
                for i in 0..pixels_to_remove_width {
                    line.remove(((i * remove_one_width_every as u16) % line.len() as u16) as usize);
                }
                line
            })
            .collect::<Vec<Symbol>>();

        DisplayData {
            buffer,
            width: scaled_width,
            height: scaled_height,
        }
    }
}
