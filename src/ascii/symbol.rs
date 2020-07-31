use std::cmp::min;

#[derive(Clone)]
pub struct AsciiSymbol {
    pub symbol: String,
    pub alpha: u8,
}

impl From<(u8, u8, u8, u8)> for AsciiSymbol {
    fn from((r, g, b, alpha): (u8, u8, u8, u8)) -> Self {
        let g_value = gray_value(r, g, b);

        match g_value {
            0..=9 => Self {
                symbol: "@".to_string(),
                alpha,
            },
            10..=19 => Self {
                symbol: "0".to_string(),
                alpha,
            },
            20..=29 => Self {
                symbol: "A".to_string(),
                alpha,
            },
            30..=39 => Self {
                symbol: "W".to_string(),
                alpha,
            },
            40..=49 => Self {
                symbol: "#".to_string(),
                alpha,
            },
            50..=59 => Self {
                symbol: "ù".to_string(),
                alpha,
            },
            60..=69 => Self {
                symbol: "à".to_string(),
                alpha,
            },
            70..=79 => Self {
                symbol: "è".to_string(),
                alpha,
            },
            80..=89 => Self {
                symbol: "*".to_string(),
                alpha,
            },
            90..=99 => Self {
                symbol: "\"".to_string(),
                alpha,
            },
            100..=149 => Self {
                symbol: "+".to_string(),
                alpha,
            },
            160..=169 => Self {
                symbol: ";".to_string(),
                alpha,
            },
            170..=179 => Self {
                symbol: ":".to_string(),
                alpha,
            },
            180..=189 => Self {
                symbol: "~".to_string(),
                alpha,
            },
            190..=199 => Self {
                symbol: "'".to_string(),
                alpha,
            },

            200..=249 => Self {
                symbol: "-".to_string(),
                alpha,
            },
            _ => Self {
                symbol: ".".to_string(),
                alpha,
            },
        }
    }
}

fn gray_value(r: u8, g: u8, b: u8) -> u8 {
    ((0.3 * (r as f32)) + (0.59 * (g as f32)) + (0.11 * (b as f32))) as u8
}

pub fn to_string(
    symbols: &Vec<AsciiSymbol>,
    lines: usize,
    columns: usize,
    max_lines: usize,
    max_columns: usize,
) -> String {
    symbols
        .iter()
        .map(|x| x.symbol.clone())
        .collect::<Vec<String>>()
        .chunks(columns)
        .map(|x| x[0..min(max_columns as usize, columns as usize)].to_vec())
        .map(|x| format!("{}", x.join("")))
        .take(min(max_lines as usize, lines as usize))
        .collect::<Vec<String>>()
        .join("\n")
}
