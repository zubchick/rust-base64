/// get three u8 characters and convert to 4 indexes of ALPHABET table
fn encode_block(block: &[u8]) -> [u8; 4] {
    let mut bitvec: u32 = 0xff_00_00_00;

    // first char place in second octet of bitvec
    // second char to third, etc.
    for (i, chr) in block.iter().enumerate() {
        let x: u32 = (*chr as u32) << (8 * (2 - i));
        bitvec |= x;
    }

    //                              <-----> six bits
    let mut mask: u32 = 0b0000_0000_1111_1100_0000_0000_0000_0000;
    let mut res: [u8; 4] = [0; 4];

    // divide three octets (2, 3, 4) of bitvec to four six-bits integers
    for i in 0..4 {
        res[i] = ((bitvec & mask) >> (6 * (3 - i))) as u8;
        mask = mask >> 6;
    }
    res
}

/// get translation table and str data, return encoded string
pub fn encode(table: &[u8], data: &[u8]) -> Vec<u8> {
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


#[cfg(test)]
mod tests {
    use super::{encode_block, encode};

    #[test]
    fn test_block() {
        let examples = [
            ([0u8, 0, 0], [0u8, 0, 0, 0]),
            ([1, 0, 0], [0, 16, 0, 0]),
            ([0, 1, 0], [0, 0, 4, 0]),
            ([0, 0, 1], [0, 0, 0, 1]),
        ];
        for &(block, res) in examples.iter() {
            assert_eq!(res, encode_block(&block));
        }
    }

    #[test]
    fn test_encode() {
        let c = '+' as u8;
        let eq = '=' as u8;
        let table = [c; 64];

        assert_eq!([c; 4].iter().collect::<Vec<_>>(),
                   encode(&table, b"qwe").iter().collect::<Vec<_>>());

        let examples = [
            ("qwe", vec![c; 4]),
            ("qweqwe", vec![c; 8]),
            ("q", vec![c, c, eq, eq]),
            ("qw", vec![c, c, c, eq]),
        ];

        for &(data, ref res) in examples.iter() {
            assert_eq!(
                res.iter().collect::<Vec<_>>(),
                encode(&table, data.as_bytes()).iter().collect::<Vec<_>>()
            );
        }
    }
}
