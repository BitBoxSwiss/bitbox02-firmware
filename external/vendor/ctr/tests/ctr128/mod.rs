use aes::{Aes128, Aes256};
use ctr::{Ctr128BE, CtrCore, flavors};

cipher::stream_cipher_test!(aes128_ctr_core, "aes128-ctr", Ctr128BE<Aes128>);
cipher::stream_cipher_test!(aes256_ctr_core, "aes256-ctr", Ctr128BE<Aes256>);
cipher::stream_cipher_seek_test!(aes128_ctr_seek, Ctr128BE<Aes128>);
cipher::stream_cipher_seek_test!(aes256_ctr_seek, Ctr128BE<Aes256>);
cipher::iv_state_test!(
    aes128_ctr_iv_state,
    CtrCore<Aes128, flavors::Ctr128BE>,
    apply_ks,
);

#[test]
fn set_iv() {
    use ctr::cipher::{IvState, KeyIvInit, SetIvState, StreamCipherCore};

    let key = Default::default();
    let iv = Default::default();
    let mut mode = CtrCore::<Aes128, flavors::Ctr128BE>::new(&key, &iv);

    let mut blocks = [Default::default(); 16];

    mode.apply_keystream_blocks(&mut blocks);
    let iv = mode.iv_state();

    let mut buf1 = blocks;
    let mut buf2 = blocks;

    mode.peek(|m| m.apply_keystream_blocks(&mut buf1));
    assert_eq!(mode.iv_state(), iv);

    mode.apply_keystream_blocks(&mut blocks);
    let iv2 = mode.iv_state();

    mode.set_iv(&iv);
    mode.apply_keystream_blocks(&mut buf2);

    assert_eq!(blocks, buf1);
    assert_eq!(blocks, buf2);
    assert_eq!(mode.iv_state(), iv2);
}
