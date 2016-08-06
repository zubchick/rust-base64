use std::str;

extern crate base64;

fn main() {
    let res = base64::encode("Hello world".as_bytes());
    println!("{}", str::from_utf8(&res).unwrap());

    match base64::decode(&res) {
        Ok(data) => {
            println!("{}", str::from_utf8(&data).unwrap());
        },
        Err(err) => {
            println!("error: {:?}", err);
        }
    };
}
