// Copyright 2025 Shift Crypto AG
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

// Link library to provide symbols necessary for critical section even if code isn't called
// explicitly. See
// https://github.com/rust-embedded/critical-section/blob/bc0d1736c3b14e340c1b0c29042edb3e6bffe6b8/README.md#undefined-reference-errors
#[cfg(not(any(feature = "testing", feature = "c-unit-testing")))]
#[allow(unused_imports)]
use cortex_m as _;

// A cell useful for global mutable state.
pub struct SyncCell<T: ?Sized> {
    value: core::cell::UnsafeCell<T>,
}

// Implement Sync if the wrapped type is Sync.
unsafe impl<T: ?Sized + Sync> Sync for SyncCell<T> {}

impl<T> SyncCell<T> {
    pub const fn new(val: T) -> Self {
        SyncCell {
            value: core::cell::UnsafeCell::new(val),
        }
    }

    /// Reads the value from `self` without moving it. This leaves the
    /// memory in `self` unchanged.
    pub fn read(&self) -> T
    where
        T: Sized + Copy,
    {
        critical_section::with(|_| unsafe { self.value.get().read() })
    }

    /// Overwrites a memory location with the given value without reading or
    /// dropping the old value.
    pub fn write(&self, val: T)
    where
        T: Sized + Copy,
    {
        critical_section::with(|_| unsafe { self.value.get().write(val) })
    }
}
