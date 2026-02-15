// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::hal::{Memory, SecureChip, memory as hal_memory, securechip};
use crate::pb;

use bitbox02::{memory, spi_mem};
use pb::response::Response;

pub fn process(hal: &mut impl crate::hal::Hal) -> Result<Response, Error> {
    let bluetooth = match hal.memory().get_platform().map_err(|_| Error::Memory)? {
        hal_memory::Platform::BitBox02Plus => {
            let ble_metadata = memory::get_ble_metadata();
            Some(pb::device_info_response::Bluetooth {
                firmware_hash: ble_metadata.allowed_firmware_hash.to_vec(),
                firmware_version: spi_mem::get_active_ble_firmware_version()
                    .map_err(|_| Error::Memory)?,
                enabled: hal.memory().ble_enabled(),
            })
        }
        hal_memory::Platform::BitBox02 => None,
    };
    // We display the stretching algo that is used on a seeded device, or the algo that would be
    // used for a new seed on an unseeded device.
    let password_stretching_algo = if hal.memory().is_seeded() {
        let (_, password_stretching_algo) = hal
            .memory()
            .get_encrypted_seed_and_hmac()
            .map_err(|_| Error::Memory)?;
        password_stretching_algo
    } else {
        crate::keystore::default_password_stretch_algo(hal).map_err(|err| match err {
            crate::keystore::Error::Memory => Error::Memory,
            _ => Error::Generic,
        })?
    };
    Ok(Response::DeviceInfo(pb::DeviceInfoResponse {
        name: hal.memory().get_device_name(),
        initialized: hal.memory().is_initialized(),
        version: crate::version::FIRMWARE_VERSION_SHORT.into(),
        mnemonic_passphrase_enabled: hal.memory().is_mnemonic_passphrase_enabled(),
        monotonic_increments_remaining: hal.securechip().monotonic_increments_remaining()?,
        securechip_model: match hal.securechip().model()? {
            securechip::Model::Atecc608A => "ATECC608A".into(),
            securechip::Model::Atecc608B => "ATECC608B".into(),
            securechip::Model::OptigaTrustM3 => "OPTIGA_TRUST_M_V3".into(),
        },
        bluetooth,
        password_stretching_algo: match password_stretching_algo {
            hal_memory::PasswordStretchAlgo::V0 => "V1".into(),
            hal_memory::PasswordStretchAlgo::V1 => "V2".into(),
        },
    }))
}
