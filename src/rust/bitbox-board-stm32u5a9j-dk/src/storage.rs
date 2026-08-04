// SPDX-License-Identifier: Apache-2.0

use bitbox_platform_stm32u5::flash::FlashStorage;

pub type UserStorage =
    FlashStorage<{ crate::memory::USER_DATA_ADDR }, { crate::memory::USER_DATA_LEN }>;
pub type VendorStorage =
    FlashStorage<{ crate::memory::VENDOR_DATA_ADDR }, { crate::memory::VENDOR_DATA_LEN }>;
