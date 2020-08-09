use once_cell::sync::Lazy;
use std::cmp::min;
use std::str::FromStr;

static ASCII_MAPPING_70: Lazy<[String; 70]> = Lazy::new(|| {
    [
        "$".to_string(),
        "@".to_string(),
        "B".to_string(),
        "%".to_string(),
        "8".to_string(),
        "&".to_string(),
        "W".to_string(),
        "M".to_string(),
        "#".to_string(),
        "*".to_string(),
        "o".to_string(),
        "a".to_string(),
        "h".to_string(),
        "k".to_string(),
        "b".to_string(),
        "d".to_string(),
        "p".to_string(),
        "q".to_string(),
        "w".to_string(),
        "m".to_string(),
        "Z".to_string(),
        "O".to_string(),
        "0".to_string(),
        "Q".to_string(),
        "L".to_string(),
        "C".to_string(),
        "J".to_string(),
        "U".to_string(),
        "Y".to_string(),
        "X".to_string(),
        "z".to_string(),
        "c".to_string(),
        "v".to_string(),
        "u".to_string(),
        "n".to_string(),
        "x".to_string(),
        "r".to_string(),
        "j".to_string(),
        "f".to_string(),
        "t".to_string(),
        "/".to_string(),
        "\\".to_string(),
        "|".to_string(),
        "(".to_string(),
        ")".to_string(),
        "1".to_string(),
        "{".to_string(),
        "}".to_string(),
        "[".to_string(),
        "]".to_string(),
        "?".to_string(),
        "-".to_string(),
        "_".to_string(),
        "+".to_string(),
        "~".to_string(),
        "<".to_string(),
        ">".to_string(),
        "i".to_string(),
        "!".to_string(),
        "l".to_string(),
        "I".to_string(),
        ";".to_string(),
        ":".to_string(),
        ",".to_string(),
        "\"".to_string(),
        "^".to_string(),
        "`".to_string(),
        "'".to_string(),
        ".".to_string(),
        " ".to_string(),
    ]
});

static ASCII_MAPPING_10: Lazy<[String; 10]> = Lazy::new(|| {
    let mut symbols = [
        " ".to_string(),
        ".".to_string(),
        ":".to_string(),
        "-".to_string(),
        "=".to_string(),
        "+".to_string(),
        "*".to_string(),
        "#".to_string(),
        "%".to_string(),
        "@".to_string(),
    ];
    symbols.reverse();
    symbols
});

#[derive(Debug)]
pub enum AsciiSymbolEncodingError {
    UnhandledEncodingError(UnhandledEncodingError),
}

impl ToString for AsciiSymbolEncodingError {
    fn to_string(&self) -> String {
        "Error encoding to ascii".to_string()
    }
}

#[derive(Debug)]
pub struct UnhandledEncodingError {
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct AsciiSymbol {
    pub symbol: String,
    pub alpha: u8,
}

#[derive(Debug)]
pub enum AsciiSymbolEncoding {
    Symbols70,
    Symbols10,
}

impl FromStr for AsciiSymbolEncoding {
    type Err = AsciiSymbolEncodingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "10" => Ok(Self::Symbols10),
            "70" => Ok(Self::Symbols70),
            _ => Err(AsciiSymbolEncodingError::UnhandledEncodingError(
                UnhandledEncodingError {
                    message: format!("unhandled encoding={}", s),
                },
            )),
        }
    }
}

impl From<((u8, u8, u8, u8), &AsciiSymbolEncoding)> for AsciiSymbol {
    fn from(((r, g, b, alpha), encoding): ((u8, u8, u8, u8), &AsciiSymbolEncoding)) -> Self {
        let g_value = gray_value(r, g, b);

        let symbol = match encoding {
            AsciiSymbolEncoding::Symbols70 => map_to_69_ascii_chars(g_value),
            AsciiSymbolEncoding::Symbols10 => map_to_10_ascii_chars(g_value),
        };

        Self { symbol, alpha }
    }
}

fn gray_value(r: u8, g: u8, b: u8) -> u8 {
    ((0.3 * (r as f32)) + (0.6 * (g as f32)) + (0.11 * (b as f32))) as u8
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

fn map_to_10_ascii_chars(g_value: u8) -> String {
    let index = ((g_value as f64 / 25.5).floor() - 1 as f64) as usize;
    let symbol = ASCII_MAPPING_10[index as usize].clone();
    symbol
}

fn map_to_69_ascii_chars(g_value: u8) -> String {
    let index = ((g_value as f64 / 3.64).floor() - 1 as f64) as usize;
    let symbol = ASCII_MAPPING_70[index as usize].clone();
    symbol
}
