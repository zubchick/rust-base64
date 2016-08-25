extern crate base64;
use std::str;

#[test]
fn test_encode_decode_valid() {
    let examples = [
        // empty
        ("", ""),
        // no padding chars
        ("qwe", "cXdl"),
        // one padding char
        ("qw", "cXc="),
        // two apdding chars
        ("q", "cQ=="),
        // all bytes from 0 to 127
        ("\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\x11\
          \x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f !\"#$%&\'\
          ()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefgh\
          ijklmnopqrstuvwxyz{|}~\x7f",
         "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8gISIjJCUmJygpKissLS4vMDEy\
          MzQ1Njc4OTo7PD0+P0BBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWltcXV5fYGFiY2Rl\
          ZmdoaWprbG1ub3BxcnN0dXZ3eHl6e3x9fn8="
        ),
    ];

    for &(data, encoded) in examples.iter() {
        assert_eq!(
            str::from_utf8(&base64::encode(data.as_bytes())),
            str::from_utf8(&encoded.as_bytes())
        );
        assert_eq!(
            str::from_utf8(&base64::decode(encoded.as_bytes()).unwrap()),
            str::from_utf8(&data.as_bytes())
        )
    }
}

#[test]
fn test_decode() {
    let examples = [
        (Ok("qw"), "\n\n\nc\nX\nc\n="),
        (Err("Invalid padding"), "cXc"),
        (Err("Invalid character"), "cXc&"),
    ];

    for &(data, encoded) in examples.iter() {
        let res = base64::decode(encoded.as_bytes());
        match (res, data) {
            (Ok(res), Ok(data)) => assert_eq!(
                str::from_utf8(&res).unwrap(),
                data
            ),
            (Ok(_), Err(msg)) => panic!(
                "base64::decode return Ok when Err({}) expected",
                msg
            ),
            (Err(msg), Ok(_)) => panic!(
                "base64::decode return Err({}) when it not expected",
                msg
            ),
            (Err(msg), Err(expected)) => assert_eq!(
                msg, expected
            )
        }
    }
}
