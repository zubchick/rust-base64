use std::result;

pub type Result<T> = result::Result<T, &'static str>;

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


pub fn decode(table: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let mut data = data.iter();
    let mut block: [u8; 4] = [0; 4];
    let mut res = Vec::new();

    loop {
        let mut skip = 0;

        // fill block with chars
        for i in 0..4 {
            block[i] = match data.next() {
                Some(&chr) => {
                    let idx = table[chr as usize];
                    let chr = chr as char;

                    if chr == '=' {
                        skip += 1;
                    }

                    if idx == 0 && !(chr == 'A' || chr == '=') {
                        return Err("Invalid character");
                    };

                    idx
                },
                None => {
                    if i != 0 {
                        return Err("Invalid padding");
                    } else {
                        return Ok(res);
                    }
                },
            }
        }
        let decoded = &decode_block(&block);
        for &idx in decoded[..3 - skip].iter() {
            res.push(idx);
        }
    }
}
