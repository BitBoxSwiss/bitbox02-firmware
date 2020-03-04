#[alloc_error_handler]
#[cfg(not(test))]
// Function name is arbitrary.
fn bitbox02_alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("memory allocation of {} bytes failed", layout.size())
}

struct BB02Allocator;

extern "C" {
    pub fn malloc(size: util::c_types::size_t) -> *mut util::c_types::c_void;
    pub fn free(p: *mut util::c_types::c_void);
}

unsafe impl core::alloc::GlobalAlloc for BB02Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        malloc(layout.size()) as _
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        free(ptr as _)
    }
}

#[global_allocator]
static BB02_ALLOCATOR: BB02Allocator = BB02Allocator;

#[cfg(test)]
mod tests {
    extern crate std;
    use std::prelude::v1::*;

    #[test]
    fn test_alloc_dealloc() {
        unsafe {
            let layout = core::alloc::Layout::new::<u32>();
            let ptr = std::alloc::alloc(layout);
            assert!(!ptr.is_null());
            std::alloc::dealloc(ptr, layout);
        }
    }
}
