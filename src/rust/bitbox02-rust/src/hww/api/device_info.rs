// Copyright 2020 Shift Crypto AG
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

use super::Error;
use crate::hal::SecureChip;
use crate::pb;

use pb::response::Response;

use bitbox02::{memory, securechip, spi_mem};

pub fn process(hal: &mut impl crate::hal::Hal) -> Result<Response, Error> {
    let bluetooth = match memory::get_platform().map_err(|_| Error::Memory)? {
        memory::Platform::BitBox02Plus => {
            let ble_metadata = memory::get_ble_metadata();
            Some(pb::device_info_response::Bluetooth {
                firmware_hash: ble_metadata.allowed_firmware_hash.to_vec(),
                firmware_version: spi_mem::get_active_ble_firmware_version()
                    .map_err(|_| Error::Memory)?,
                enabled: memory::ble_enabled(),
            })
        }
        memory::Platform::BitBox02 => None,
    };
    Ok(Response::DeviceInfo(pb::DeviceInfoResponse {
        name: memory::get_device_name(),
        initialized: memory::is_initialized(),
        version: crate::version::FIRMWARE_VERSION_SHORT.into(),
        mnemonic_passphrase_enabled: memory::is_mnemonic_passphrase_enabled(),
        monotonic_increments_remaining: hal.securechip().monotonic_increments_remaining()?,
        securechip_model: match securechip::model()? {
            securechip::Model::ATECC_ATECC608A => "ATECC608A".into(),
            securechip::Model::ATECC_ATECC608B => "ATECC608B".into(),
            securechip::Model::OPTIGA_TRUST_M_V3 => "OPTIGA_TRUST_M_V3".into(),
        },
        bluetooth,
    }))
}
