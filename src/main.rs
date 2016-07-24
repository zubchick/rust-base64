const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789\
                                +/";

/// get three u8 characters and convert to 4 indexes of ALPHABET table
fn encode_chunk(chunk: &[u8]) -> [u8; 4] {
    let mut bitvec: u32 = 0xff_00_00_00;

    for (i, chr) in chunk.iter().enumerate() {
        let x: u32 = (*chr as u32) << (8 * (2 - i));
        bitvec |= x;
    };

    let mut mask: u32 = 0b0000_0000_1111_1100_0000_0000_0000_0000;
    let mut res: [u8; 4] = [0; 4];

    for i in 0..4 {
        res[i] = ((bitvec & mask) >> (6 * (3 - i))) as u8;
        mask = mask >> 6;
    }
    res
}

/// get translation table and str data, return encoded string
fn encode(table: &[u8], data: &str) -> String {
    let mut data = data.bytes();
    let mut chunk: [u8; 3] = [0; 3];
    let mut res = String::new();
    let mut done = false;
    let mut count;

    while !done {
        count = 0;

        // fill chunk with chars, count actualy added chars
        for i in 0..3 {
            chunk[i] = match data.next() {
                Some(chr) => {
                    count += 1;
                    chr
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

        for idx in &encode_chunk(&chunk) {
            if count + 1 != 0 {
                res.push(table[*idx as usize] as char);
                count -= 1;
            } else {
                res.push('=');
            }
        }
    }
    res
}

fn main() {
    let table: Vec<u8> = ALPHABET.bytes().collect();
    let mut res = encode(&table, "Hello world");
    res.push('\n');
    println!("{}", res);
}
