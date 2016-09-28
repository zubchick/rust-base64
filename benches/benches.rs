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

#[bench]
fn bench_decoder(b: &mut test::Bencher) {
    let mut out = [0u8; 1024];
    let count = base64::encode(TEST_PHRASE.as_bytes(), &mut out);

    let mut decode_out = [0u8; 1024];

    b.iter(|| {
        base64::decode(&out[..count], &mut decode_out)
    })
}

#[bench]
fn bench_encoder(b: &mut test::Bencher) {
    let mut out = [0u8; 1024];

    b.iter(|| {
        base64::encode(TEST_PHRASE.as_bytes(), &mut out)
    })
}
