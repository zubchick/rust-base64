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

// MAGIC (!) buf size divisible by 3 for encode and by 4 for decode
const BUF_SIZE: usize = 4092;

const USAGE: &'static str = "
Base64 encode or decode FILE, or standard input, to standard output.

Usage: base64 [options] [<file>]
       base64 (--help | --version)

Options:
  -d --decode      decode data
  -w --wrap COLS   wrap encoded lines after COLS character (default 76).
                   Use 0 to disable line wrapping

  -h --help     display this help and exit
  --version     output version information and exit
";


#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
    flag_decode: bool,
    flag_help: bool,
    flag_version: bool,
    flag_wrap: Option<usize>,
}

struct WrapedOutput<'a> {
    out: &'a mut io::Write,
    wrap: usize,
}

impl<'a> Write for WrapedOutput<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let buf_len = buf.len();
        let mut writed: usize = 0;

        loop {
            let bound = if self.wrap == 0 {
                buf_len
            } else if (writed + self.wrap) >= buf_len {
                buf_len
            } else {
                writed + self.wrap
            };

            writed += try!(self.out.write(&buf[writed..bound]));

            if writed >= buf_len {
                return Ok(writed);
            }
            try!(self.out.write(b"\n"));
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}

fn encode(file: &mut io::Read, wrap: usize) -> io::Result<()> {
    let mut buf = [0u8; BUF_SIZE];
    let mut out_buf = [0u8; BUF_SIZE * 4 / 3];

    let mut out = WrapedOutput {
        out: &mut io::stdout(),
        wrap: wrap,
    };

    loop {
        let count = try!(file.read(&mut buf[..]));

        if count == 0 {
            try!(out.write(b"\n"));
            break;
        }

        let encode_count = base64::encode(
            &buf[..count], &mut out_buf
        );

        try!(out.write(&out_buf[..encode_count]));
    }

    Ok(())
}

fn decode(file: &mut io::Read) -> io::Result<()> {
    let mut buf = [0u8; BUF_SIZE];
    let mut out_buf = [0u8; BUF_SIZE * 3 / 4];
    let mut stdout = io::stdout();

    loop {
        let count = try!(file.read(&mut buf[..]));
        if count == 0 {
            break;
        }

        match base64::decode(&buf[..count], &mut out_buf) {
            Ok(count) => {
                stdout.write(&mut out_buf[..count]).unwrap();
            },
            Err(msg) => {
                panic!(msg.as_bytes());
            },
        }
    }

    Ok(())
}

fn process(file: &mut io::Read, is_decode: bool, wrap: usize) {
    if is_decode {
        decode(file).unwrap();
    } else {
        encode(file, wrap).unwrap();
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("base64 v{}", VERSION);
        return;
    }

    let wrap = match args.flag_wrap {
        Some(count) => count,
        None        => 76,
    };

    if args.arg_file != "" {
        let filename = args.arg_file;

        let path = Path::new(&filename);
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}",
                               path.display(), why.description()),
            Ok(file) => file,
        };
        process(&mut file, args.flag_decode, wrap);
    } else {
        process(&mut io::stdin(), args.flag_decode, wrap);
    }
}


#[cfg(test)]
mod tests {
    use super::{WrapedOutput};
    use std::io::{Write};

    fn check_write(wrap: usize, data: &str) {
        let mut file = Vec::new();
        {
            let mut out = WrapedOutput { out: &mut file, wrap: wrap };
            let _ = out.write(data.as_bytes());
        }
        assert_eq!(file, _wrap(wrap, data));
    }

    fn _wrap(wrap: usize, data: &str) -> Vec<u8> {
        let mut res = Vec::new();
        for (i, &chr) in data.as_bytes().iter().enumerate() {
            if wrap != 0 &&
                i != 0 &&
                i % wrap == 0
            {
                res.push('\n' as u8);
            }
            res.push(chr);
        }
        res
    }

    #[test]
    fn test_write_wrapper() {
        let data = [
            "",
            "q",
            "test",
            "\n\n\n\n",
        ];

        for text in data.iter() {
            for i in 0..text.len() {
                check_write(i, text);
            }
        }
    }
}
