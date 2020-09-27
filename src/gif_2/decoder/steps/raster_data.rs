use lzw::LsbReader;

pub fn decode(bytes: &Vec<u8>, cursor: usize) -> (Vec<u8>, usize) {
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
