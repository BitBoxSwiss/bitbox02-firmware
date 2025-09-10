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

// Same as `core::sync::SyncUnsafeCell`, which is still in nightly. Can remove once it is stable.
pub struct SyncUnsafeCell<T: ?Sized> {
    value: core::cell::UnsafeCell<T>,
}

// Implement Sync if the wrapped type is Sync.
unsafe impl<T: ?Sized + Sync> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    pub const fn new(val: T) -> Self {
        SyncUnsafeCell {
            value: core::cell::UnsafeCell::new(val),
        }
    }

    /// Reads the value from `self` without moving it. This leaves the
    /// memory in `self` unchanged.
    ///
    /// # Safety
    ///
    /// This is unsafe because it allows accessing the interior value without
    /// synchronization. The caller must ensure no other code is currently
    /// writing to this cell during the read operation.
    pub unsafe fn read(&self) -> T
    where
        T: Sized,
    {
        unsafe { self.value.get().read() }
    }

    /// Overwrites a memory location with the given value without reading or
    /// dropping the old value.
    ///
    /// # Safety
    ///
    /// This is unsafe because it allows accessing the interior value without
    /// synchronization. The caller must ensure no other code is currently
    /// accessing this cell (either reading or writing) during the write operation.
    pub unsafe fn write(&self, val: T)
    where
        T: Sized,
    {
        unsafe { self.value.get().write(val) }
    }
}
