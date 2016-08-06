use std::io;
use std::str;

extern crate base64;

fn encode(file: &mut io::Read) {
    let mut buf: [u8; 1024 * 4] = [0; 1024 * 4];

    loop {
        match file.read(&mut buf[..]) {
            Ok(count) => {
                if count == 0 {
                    print!("\n");
                    break;
                }
                let res = base64::encode(&buf);
                print!("{}", str::from_utf8(&res[..count + 1]).unwrap());
            },
            Err(err) => panic!(err),
        }
    }
}

fn main() {
    encode(&mut io::stdin());
}
