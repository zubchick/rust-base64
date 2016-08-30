mod encode;
mod decode;

pub use decode::Result;

const ALPHABET: &'static [u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                      abcdefghijklmnopqrstuvwxyz\
                                      0123456789\
                                      +/";

fn make_decode_table(alphabet: &[u8; 64]) -> [u8; 256] {
    let mut table: [u8; 256] = [0; 256];

    for (i, &byte) in alphabet.iter().enumerate() {
        table[byte as usize] = i as u8;
    }
    table
}

pub fn encode(data: &[u8], out: &mut [u8]) -> usize {
    return encode::encode(ALPHABET, data, out)
}

pub fn decode(data: &[u8]) -> Result<Vec<u8>> {
    let table: [u8; 256] = make_decode_table(ALPHABET);
    decode::decode(&table, data)
}
