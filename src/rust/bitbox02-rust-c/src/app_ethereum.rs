#[no_mangle]
pub unsafe extern "C" fn rust_ethereum_keypath_is_valid_xpub(
    keypath: *const u32,
    keypath_len: usize,
    expected_coin: u32,
) -> bool {
    let keypath = core::slice::from_raw_parts(keypath, keypath_len);
    ethereum::keypath::is_valid_keypath_xpub(keypath, expected_coin)
}

#[no_mangle]
pub unsafe extern "C" fn rust_ethereum_keypath_is_valid_address(
    keypath: *const u32,
    keypath_len: usize,
    expected_coin: u32,
) -> bool {
    let keypath = core::slice::from_raw_parts(keypath, keypath_len);
    ethereum::keypath::is_valid_keypath_address(keypath, expected_coin)
}
