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

use super::pb;
use super::Error;

use pb::bluetooth_request::Request;
use pb::bluetooth_response::Response;

use sha2::{Digest, Sha256};

use crate::hal::Ui;
use crate::workflow::confirm;

use alloc::vec::Vec;

use bitbox02::{memory, spi_mem};

const ALLOWED_HASH: &[u8; 32] = b"\xb1\xe6\x92\x94\x04\xea\xd1\xca\x02\x29\x25\x48\x97\x5d\xc8\x57\x10\x82\x5b\x91\xf8\x03\x84\x59\xc8\x79\xaf\x37\x00\xf8\x1f\x05";

// We want to write FW to the memory chip in erase-size chunks, so that we don't repeatedly need to
// read-erase-write the same sector.
const SPI_ERASE_SIZE: u32 = 4096;

/// Like `hww::next_request`, but for Bluetooth requests/responses.
async fn next_request(response: Response) -> Result<Request, Error> {
    let request =
        crate::hww::next_request(pb::response::Response::Bluetooth(pb::BluetoothResponse {
            response: Some(response),
        }))
        .await?;
    match request {
        pb::request::Request::Bluetooth(pb::BluetoothRequest {
            request: Some(request),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_fw_chunk_from_host(offset: u32, length: u32) -> Result<Vec<u8>, Error> {
    let request = next_request(Response::RequestChunk(pb::BluetoothRequestChunkResponse {
        offset,
        length,
    }))
    .await?;
    match request {
        Request::Chunk(pb::BluetoothChunkRequest { data }) => Ok(data),
        _ => Err(Error::InvalidInput),
    }
}

trait Funcs {
    async fn get_fw_chunk(&mut self, offset: u32, length: u32) -> Result<Vec<u8>, Error>;
}

async fn _process_upgrade(
    funcs: &mut impl Funcs,
    request: &pb::BluetoothUpgradeInitRequest,
    allowed_hash: &[u8; 32],
) -> Result<Response, Error> {
    if request.firmware_length == 0 || request.firmware_length > spi_mem::BLE_FIRMWARE_MAX_SIZE {
        return Err(Error::InvalidInput);
    }

    let mut ble_metadata = memory::get_ble_metadata();

    // We work on the inactive firmware memory area.
    let inactive_index: u8 = if ble_metadata.active_index == 0 { 1 } else { 0 };
    let inactive_ble_fw_address = if inactive_index == 0 {
        spi_mem::BLE_FIRMWARE_1_ADDR
    } else {
        spi_mem::BLE_FIRMWARE_2_ADDR
    };

    let mut firmware_hasher = Sha256::new();
    let mut firmware_checksum = 0u8;

    // The host needs to send this many chunks.
    let num_chunks = (request.firmware_length + SPI_ERASE_SIZE - 1) / SPI_ERASE_SIZE;

    // Show progress
    let mut progress_component = bitbox02::ui::progress_create("Upgrading...");
    progress_component.screen_stack_push();

    // Stream chunks from host.
    for chunk_index in 0..num_chunks {
        let chunk_offset = chunk_index * SPI_ERASE_SIZE;
        let chunk_length = core::cmp::min(SPI_ERASE_SIZE, request.firmware_length - chunk_offset);
        let chunk: Vec<u8> = funcs.get_fw_chunk(chunk_offset, chunk_length).await?;
        if chunk.len() != chunk_length as usize {
            return Err(Error::InvalidInput);
        }
        firmware_hasher.update(&chunk);
        for &byte in &chunk {
            firmware_checksum ^= byte;
        }

        spi_mem::write(inactive_ble_fw_address + chunk_offset, &chunk)
            .map_err(|_| Error::Memory)?;

        // Update progress.
        bitbox02::ui::progress_set(
            &mut progress_component,
            (chunk_index + 1) as f32 / (num_chunks as f32),
        );
    }

    drop(progress_component);

    let firmware_hash: [u8; 32] = firmware_hasher.finalize().into();
    if &firmware_hash != allowed_hash {
        return Err(Error::InvalidInput);
    }

    // TODO: read back and verify FW bytes

    // Activate!
    ble_metadata.active_index = inactive_index;
    ble_metadata.allowed_firmware_hash = firmware_hash;
    // Conversion safe because the length is checked to be below the maximum allowed length above.
    ble_metadata.firmware_sizes[inactive_index as usize] = request.firmware_length as u16;
    ble_metadata.firmware_checksums[inactive_index as usize] = firmware_checksum;

    memory::set_ble_metadata(&ble_metadata).map_err(|_| Error::Memory)?;

    Ok(pb::bluetooth_response::Response::Success(
        pb::BluetoothSuccess {},
    ))
}

async fn process_upgrade(
    hal: &mut impl crate::hal::Hal,
    request: &pb::BluetoothUpgradeInitRequest,
) -> Result<Response, Error> {
    struct RealFuncs;
    impl Funcs for RealFuncs {
        async fn get_fw_chunk(&mut self, offset: u32, length: u32) -> Result<Vec<u8>, Error> {
            get_fw_chunk_from_host(offset, length).await
        }
    }

    hal.ui()
        .confirm(&confirm::Params {
            title: "Bluetooth",
            body: "Upgrade\nfirmware?",
            longtouch: true,
            ..Default::default()
        })
        .await?;

    let response = _process_upgrade(&mut RealFuncs, request, ALLOWED_HASH).await;

    if response.is_ok() {
        hal.ui().status("Upgrade\nsuccessful", true).await;
    } else {
        hal.ui().status("Upgrade failed", false).await;
    }

    response
}

/// Processes a Bluetooth API call (BB02+ only).
pub async fn process_api(
    hal: &mut impl crate::hal::Hal,
    request: &Request,
) -> Result<Response, Error> {
    if !matches!(memory::get_platform()?, memory::Platform::BitBox02Plus) {
        return Err(Error::Disabled);
    }
    match request {
        Request::UpgradeInit(ref request) => process_upgrade(hal, request).await,
        // These are streamed asynchronously using the `next_request()` primitive are not handled
        // directly.
        Request::Chunk(_) => Err(Error::InvalidInput),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bb02_async::block_on;
    use bitbox02::testing::mock_memory;

    #[test]
    fn test_chunk_streaming() {
        mock_memory();

        struct MockFuncs {
            chunk_requests: Vec<(u32, u32)>,
        }

        impl Funcs for MockFuncs {
            async fn get_fw_chunk(&mut self, offset: u32, length: u32) -> Result<Vec<u8>, Error> {
                self.chunk_requests.push((offset, length));
                Ok(vec![0; length as usize])
            }
        }

        struct Test<'a> {
            firmware_length: u32,
            expected_chunk_requests: &'a [(u32, u32)],
        }

        let test_cases = vec![
            Test {
                firmware_length: 1,
                expected_chunk_requests: &[(0, 1)],
            },
            Test {
                firmware_length: 4095,
                expected_chunk_requests: &[(0, 4095)],
            },
            Test {
                firmware_length: 4096,
                expected_chunk_requests: &[(0, 4096)],
            },
            Test {
                firmware_length: 4097,
                expected_chunk_requests: &[(0, 4096), (4096, 1)],
            },
            Test {
                firmware_length: 5 * 4096,
                expected_chunk_requests: &[
                    (0, 4096),
                    (4096, 4096),
                    (8192, 4096),
                    (12288, 4096),
                    (16384, 4096),
                ],
            },
            Test {
                firmware_length: 5 * 4096 + 1,
                expected_chunk_requests: &[
                    (0, 4096),
                    (4096, 4096),
                    (8192, 4096),
                    (12288, 4096),
                    (16384, 4096),
                    (20480, 1),
                ],
            },
        ];

        for test in test_cases {
            let mut mock_funcs = MockFuncs {
                chunk_requests: vec![],
            };
            let allowed_hash: [u8; 32] =
                Sha256::digest(vec![0; test.firmware_length as usize]).into();
            assert!(block_on(_process_upgrade(
                &mut mock_funcs,
                &pb::BluetoothUpgradeInitRequest {
                    firmware_length: test.firmware_length,
                },
                &allowed_hash,
            ))
            .is_ok());
            assert_eq!(mock_funcs.chunk_requests, test.expected_chunk_requests);
        }
    }
}
