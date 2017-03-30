# cli base64 encoder/decoder #

[![Build Status](https://travis-ci.org/zubchick/rust-base64.svg?branch=master)](https://travis-ci.org/zubchick/rust-base64)

Fast comand line base64 encoder/decoder.

### Usage
```
Base64 encode or decode FILE, or standard input, to standard output.

Usage: base64 [options] [<file>]
       base64 (--help | --version)

Options:
  -d --decode      decode data
  -w --wrap COLS   wrap encoded lines after COLS character (default 76).
                   Use 0 to disable line wrapping

  -h --help     display this help and exit
  --version     output version information and exit
```

### Development
* `cargo build --release` to build release
* `make test` to run tests
* `make bench` to run benchmarks
