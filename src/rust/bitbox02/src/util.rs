/// Must be given a null-terminated string
pub unsafe fn strlen_ptr(ptr: *const u8) -> isize {
    let mut end = ptr;
    loop {
        if *end == 0 {
            // Can be changed to (*const u8).offset_from() when stabilized
            return isize::wrapping_sub(end as _, ptr as _);
        }
        end = end.offset(1);
    }
}

pub fn strlen_slice(input: &[u8]) -> usize {
    if let Some(nullidx) = input.iter().position(|&c| c == 0) {
        nullidx
    } else {
        input.len()
    }
}

/// Parses a utf-8 string out of a null terminated fixed length array
pub fn str_from_null_terminated(input: &[u8]) -> Result<&str, core::str::Utf8Error> {
    let len = strlen_slice(input);
    core::str::from_utf8(&input[0..len])
}
