// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod pb {
    include!("./generated/shiftcrypto.bitbox02.rs");
}

pub mod pb_backup {
    include!("./generated/shiftcrypto.bitbox02.backups.rs");
}
