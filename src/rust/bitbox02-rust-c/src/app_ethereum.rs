#[no_mangle]
pub extern "C" fn rust_ethereum_keypath_is_valid_address(
    keypath: *const u32,
    keypath_len: usize,
    expected_coin: u32,
) -> bool {
    let keypath = unsafe { core::slice::from_raw_parts(keypath, keypath_len) };
    ethereum::keypath::is_valid_keypath_address(keypath, expected_coin)
}
