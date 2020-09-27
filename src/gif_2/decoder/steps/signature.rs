use std::str::from_utf8;

pub fn decode(bytes: &Vec<u8>, cursor: usize) -> (String, usize) {
    let to_index = cursor + 6;
    let signature = from_utf8(&bytes[cursor..to_index]).unwrap();
    (signature.to_string(), to_index)
}
