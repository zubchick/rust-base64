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


pub fn decode(table: &[u8], data: &[u8], out: &mut [u8]) -> Result<usize> {
    let mut data = data.iter().filter(|&&x| x != '\n' as u8);
    let mut block: [u8; 4] = [0; 4];
    let mut total_count: usize = 0;

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
                        return Ok(total_count);
                    }
                },
            }
        }
        let decoded = &decode_block(&block);
        for &idx in decoded[..3 - skip].iter() {
            out[total_count] = idx;
            total_count += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{decode_block, decode};

    #[test]
    fn test_block() {
        let examples = [
            ([0u8, 0, 0], [0u8, 0, 0, 0]),
            ([1, 0, 0], [0, 16, 0, 0]),
            ([0, 1, 0], [0, 0, 4, 0]),
            ([0, 0, 1], [0, 0, 0, 1]),
        ];
        for &(res, block) in examples.iter() {
            assert_eq!(res, decode_block(&block));
        }
    }

    #[test]
    fn test_decode() {
        let mut table = [1u8; 256];
        let mut out = [0u8; 256];

        table[0] = 0;
        table['=' as usize] = 0;

        let examples = [
            ("AAAA", vec![4u8, 16, 65]),
            ("AA\nAA", vec![4u8, 16, 65]),
            ("A\n\nA\nAA\n", vec![4u8, 16, 65]),

            ("AAA=", vec![4u8, 16]),
            ("AA==", vec![4u8]),
            ("", vec![]),
        ];

        for &(data, ref res) in examples.iter() {
            let count = decode(&table, data.as_bytes(), &mut out)
                .unwrap();
            assert_eq!(
                out[..count].iter().collect::<Vec<_>>(),
                res.iter().collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_decode_fail() {
        let mut table = [1u8; 256];
        let mut out = [0u8; 256];

        table[0] = 0;
        table['=' as usize] = 0;
        table['+' as usize] = 0;

        let pad_err = "Invalid padding";
        let chr_err = "Invalid character";
        let examples = [
            ("A==", pad_err),
            ("AA", pad_err),
            ("A", pad_err),
            ("A=", pad_err),

            ("A+A=", chr_err),
            ("++", chr_err),
        ];

        for &(data, msg) in examples.iter() {
            match decode(&table, data.as_bytes(), &mut out) {
                Ok(_) => panic!("This test expect wrong input data"),
                Err(err) => assert_eq!(err, msg),
            }
        }
    }
}
