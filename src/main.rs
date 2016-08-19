extern crate base64;
extern crate docopt;
extern crate rustc_serialize;

use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::str;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const BUF_SIZE: usize = 4 * 1024;

const USAGE: &'static str = "
Base64 encode or decode FILE, or standard input, to standard output.

Usage: base64 [options] [<file>]
       base64 (--help | --version)

Options:
  -d --decode   decode data
  -h --help     display this help and exit
  --version     output version information and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
    flag_decode: bool,
    flag_help: bool,
    flag_version: bool,
}

fn process_file<F>(func: F, file: &mut io::Read)
where F: Fn(&mut [u8; BUF_SIZE], usize) {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    loop {
        match file.read(&mut buf[..]) {
            Ok(count) => func(&mut buf, count),
            Err(err) => panic!(err),
        }
    }
}

fn encode(file: &mut io::Read) {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let mut stdout = io::stdout();

    loop {
        match file.read(&mut buf[..]) {
            Ok(count) => {
                if count == 0 {
                    print!("\n");
                    break;
                }
                let res = base64::encode(&buf);
                stdout.write(&res[..count + 1]).unwrap();
            },
            Err(err) => panic!(err),
        }
    }
}

fn decode(file: &mut io::Read) {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    loop {
        match file.read(&mut buf[..]) {
            Ok(count) => {
                if count == 0 {
                     break;
                }
                println!("{:?}", str::from_utf8(&buf[..count])); // DEBUG
                match base64::decode(&buf[..count]) {
                    Ok(res) => {
                        stdout.write(&res).unwrap();
                    },
                    Err(msg) => {
                        stderr.write(msg.as_bytes()).unwrap();

                    },
                }
            },
            Err(err) => panic!(err),
        }
    }
}

fn process(file: &mut io::Read, is_decode: bool) {
    if is_decode {
        decode(file);
    } else {
        encode(file);
    }
}

fn main() {
    /*
    match base64::decode(b"cXdlCg==") {
        Err(msg) => println!("{}", msg),
        Ok(data) => println!("{}", str::from_utf8(&data).unwrap()),
    }
    */
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("base64 v{}", VERSION);
        return;
    }

    if args.arg_file != "" {
        let path = Path::new(&args.arg_file);
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}",
                               path.display(), why.description()),
            Ok(file) => file,
        };
        process(&mut file, args.flag_decode);
    } else {
        process(&mut io::stdin(), args.flag_decode);
    }
}
