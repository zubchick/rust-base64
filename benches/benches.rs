#![feature(test)]

extern crate test;
extern crate base64;

const TEST_PHRASE: &'static str = "Lorem ipsum dolor sit amet, consectetur \
                                   adipiscing elit, sed do eiusmod tempor \
                                   incididunt ut labore et dolore magna aliqua\
                                   . Ut enim ad minim veniam, quis nostrud \
                                   exercitation ullamco laboris nisi ut aliquip\
                                   ex ea commodo consequat. Duis aute irure \
                                   dolor in reprehenderit in voluptate velit \
                                   esse cillum dolore eu fugiat nulla pariatur\
                                   . Excepteur sint occaecat cupidatat non \
                                   proident, sunt in culpa qui officia \
                                   deserunt mollit anim id est laborum.";

const TEST_ENCODED: &'static str = "TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbn\
                                    NlY3RldHVyIGFkaXBpc2NpbmcgZWxpdCwgc2VkIGRv\
                                    IGVpdXNtb2QgdGVtcG9yIGluY2lkaWR1bnQgdXQgbG\
                                    Fib3JlIGV0IGRvbG9yZSBtYWduYSBhbGlxdWEuIFV0\
                                    IGVuaW0gYWQgbWluaW0gdmVuaWFtLCBxdWlzIG5vc3\
                                    RydWQgZXhlcmNpdGF0aW9uIHVsbGFtY28gbGFib3Jp\
                                    cyBuaXNpIHV0IGFsaXF1aXAgZXggZWEgY29tbW9kby\
                                    Bjb25zZXF1YXQuIER1aXMgYXV0ZSBpcnVyZSBkb2xv\
                                    ciBpbiByZXByZWhlbmRlcml0IGluIHZvbHVwdGF0ZS\
                                    B2ZWxpdCBlc3NlIGNpbGx1bSBkb2xvcmUgZXUgZnVn\
                                    aWF0IG51bGxhIHBhcmlhdHVy";


#[bench]
fn bench_decoder(b: &mut test::Bencher) {
    let mut out = [0u8; 1024];

    b.iter(|| {
        base64::decode(TEST_ENCODED.as_bytes(), &mut out)
    })
}

#[bench]
fn bench_encoder(b: &mut test::Bencher) {
    let mut out = [0u8; 1024];

    b.iter(|| {
        base64::encode(TEST_PHRASE.as_bytes(), &mut out)
    })
}
