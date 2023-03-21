// Copyright 2020 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

struct BB02Allocator;

extern "C" {
    pub fn malloc(size: usize) -> *mut util::c_types::c_void;
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
