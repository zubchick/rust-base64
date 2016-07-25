use std::str;

const ALPHABET: &'static [u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                      abcdefghijklmnopqrstuvwxyz\
                                      0123456789\
                                      +/";

/// get three u8 characters and convert to 4 indexes of ALPHABET table
fn encode_block(block: &[u8]) -> [u8; 4] {
    let mut bitvec: u32 = 0xff_00_00_00;

    // first char place in second octet of bitvec
    // second char to third, etc.
    for (i, chr) in block.iter().enumerate() {
        let x: u32 = (*chr as u32) << (8 * (2 - i));
        bitvec |= x;
    }

    //                              <-----> six bytes
    let mut mask: u32 = 0b0000_0000_1111_1100_0000_0000_0000_0000;
    let mut res: [u8; 4] = [0; 4];

    // divide three octets of bitvec (2, 3, 4) to four six-bytes integers
    for i in 0..4 {
        res[i] = ((bitvec & mask) >> (6 * (3 - i))) as u8;
        mask = mask >> 6;
    }
    res
}

/// get translation table and str data, return encoded string
fn encode(table: &[u8], data: &[u8]) -> Vec<u8> {
    let mut data = data.iter();
    let mut block: [u8; 3] = [0; 3];
    let mut res = Vec::new();
    let mut done = false;
    let mut count;

    while !done {
        count = 0;

        // fill block with chars
        // count only those symbols that were actually added
        for i in 0..3 {
            block[i] = match data.next() {
                Some(chr) => {
                    count += 1;
                    *chr
                },
                None => {
                    done = true;
                    0
                },
            }
        }

        // in case of empty iterator
        if count == 0 {
            break
        }

        for idx in &encode_block(&block) {
            if count + 1 != 0 {
                res.push(table[*idx as usize]);
                count -= 1;
            } else {
                res.push('=' as u8);
            }
        }
    }
    res
}


/// get 4 indexes of ALPHABET table and return 3 ACII charaters
fn decode_block(block: &[u8]) -> [u8; 3] {
    let mut bitvec: u32 = 0xff_00_00_00;

    for (i, chr) in block.iter().enumerate() {
        let x: u32 = (*chr as u32) << (6 * (3 - i));
        bitvec |= x;
    }

    let mut res: [u8; 3] = [0; 3];
    let mut mask: u32 = 0x00_ff_00_00;

    for i in 0..3 {
        res[i] = ((bitvec & mask) >> (8 * (2 - i))) as u8;
        mask = mask >> 8;
    }
    res
}

fn decode(table: &[u8], data: &[u8]) -> Vec<u8> {
    let mut data = data.iter();
    let mut block: [u8; 4] = [0; 4];
    let mut res = Vec::new();
    let mut errcount = 0;
    let mut skip;

    loop {
        skip = 0;

        // fill block with chars
        for i in 0..4 {
            block[i] = match data.next() {
                Some(chr) => {
                    let idx = table[*chr as usize];
                    let chr = *chr as char;

                    if chr == '=' {
                        skip += 1;
                    }

                    if idx == 0 && (chr != 'A' || chr != '=') {
                        // invalid symbol
                        continue;
                    };

                    idx
                },
                None => {
                    errcount += 1;
                    0
                },
            }
        }

        if 0 < errcount && errcount <= 4 {
            // invalid padding or empty string
            break;
        }

        let decoded = &decode_block(&block)[..3 - skip];
        for idx in decoded {
            res.push(*idx);
        }
    }
    res
}

fn main() {
    let res = encode(ALPHABET, "Hello worl".as_bytes());
    println!("{}", str::from_utf8(&res).unwrap());

    let mut decode_table: [u8; 256] = [0; 256];
    for (i, byte) in ALPHABET.iter().enumerate() {
        decode_table[*byte as usize] = i as u8;
    }

    println!("{}", str::from_utf8(&decode(&decode_table, &res)).unwrap());
}
