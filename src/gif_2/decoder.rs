use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::from_utf8;

pub struct Decoder {}

impl Decoder {
    pub fn decode<R>(self, source: &mut R) -> Result<(), Box<dyn std::error::Error>>
    where
        R: Read,
    {
        let bytes: &mut Vec<u8> = &mut Vec::new();
        source.read_to_end(bytes)?;

        // == SIGNATURE
        let signature = from_utf8(&bytes[0..=5]).unwrap();

        // == SCREEN DESCRIPTOR
        let screen_descriptor = &bytes[6..=12];

        let screen_width = ((screen_descriptor[1] as u16) << 8) | screen_descriptor[0] as u16;
        let screen_height = ((screen_descriptor[3] as u16) << 8) | screen_descriptor[2] as u16;

        let flags: u8 = screen_descriptor[4];
        let m: bool = flags & (1 << 7) != 0;
        let cr: u8 = (flags << 1) >> 5;
        let pixel: u8 = (flags << 5) >> 5;

        let background: u8 = screen_descriptor[5];
        let map: u8 = screen_descriptor[6];

        dbg!(signature);
        dbg!(screen_width);
        dbg!(screen_height);
        dbg!(flags);
        dbg!(m);
        dbg!(cr);
        dbg!(pixel);

        Ok(())
    }
}

#[test]
pub fn should_decode() {
    let d = Decoder {};

    let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
    d.decode::<File>(file).unwrap();
}

#[test]
pub fn bytes() {
    let byte: u8 = 2;

    dbg!(byte & (1 << 1));
}
