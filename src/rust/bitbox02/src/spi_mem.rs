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

extern crate alloc;

pub use bitbox02_sys::MEMORY_SPI_BLE_FIRMWARE_1_ADDR as BLE_FIRMWARE_1_ADDR;
pub use bitbox02_sys::MEMORY_SPI_BLE_FIRMWARE_2_ADDR as BLE_FIRMWARE_2_ADDR;
pub use bitbox02_sys::MEMORY_SPI_BLE_FIRMWARE_MAX_SIZE as BLE_FIRMWARE_MAX_SIZE;

pub fn write(address: u32, data: &[u8]) -> Result<(), ()> {
    match unsafe { bitbox02_sys::spi_mem_write(address, data.as_ptr(), data.len()) } {
        true => Ok(()),
        false => Err(()),
    }
}
