#[alloc_error_handler]
#[cfg(not(test))]
// Function name is arbitrary.
fn bitbox02_alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("memory allocation of {} bytes failed", layout.size())
}
