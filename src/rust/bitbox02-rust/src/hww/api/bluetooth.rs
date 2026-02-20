// SPDX-License-Identifier: Apache-2.0

use super::Error;
use super::pb;
use crate::hal::ui::{ConfirmParams, Progress};

use hex_lit::hex;

use pb::bluetooth_request::Request;
use pb::bluetooth_response::Response;

use sha2::{Digest, Sha256};

use crate::hal::{Memory, Ui, memory as hal_memory};

use alloc::vec::Vec;

use bitbox02::spi_mem;

// See also bitbox-da14531-firmware.bin.sha256.
const ALLOWED_HASH: [u8; 32] =
    hex!("1e4aa8364e935c0785e4f891208307d832f788172e4bf61621de6df9ec3c215f");

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

async fn _process_upgrade<M: Memory>(
    memory: &mut M,
    funcs: &mut impl Funcs,
    progress: &mut impl Progress,
    request: &pb::BluetoothUpgradeInitRequest,
    allowed_hash: &[u8; 32],
) -> Result<Response, Error> {
    if request.firmware_length == 0 || request.firmware_length > spi_mem::BLE_FIRMWARE_MAX_SIZE {
        return Err(Error::InvalidInput);
    }

    let mut ble_metadata = memory.ble_get_metadata();

    // We work on the inactive firmware memory area.
    let inactive_index: u8 = if ble_metadata.active_index == 0 { 1 } else { 0 };
    let inactive_slot = if inactive_index == 0 {
        hal_memory::BleFirmwareSlot::First
    } else {
        hal_memory::BleFirmwareSlot::Second
    };

    let mut firmware_hasher = Sha256::new();
    let mut firmware_checksum = 0u8;

    let flash_chunk_size = M::BLE_FW_FLASH_CHUNK_SIZE;

    // The host needs to send this many chunks.
    let num_chunks = request.firmware_length.div_ceil(flash_chunk_size);

    // Stream chunks from host.
    for chunk_index in 0..num_chunks {
        let chunk_offset = chunk_index * flash_chunk_size;
        let chunk_length = core::cmp::min(flash_chunk_size, request.firmware_length - chunk_offset);
        let chunk: Vec<u8> = funcs.get_fw_chunk(chunk_offset, chunk_length).await?;
        if chunk.len() != chunk_length as usize {
            return Err(Error::InvalidInput);
        }
        firmware_hasher.update(&chunk);
        for &byte in &chunk {
            firmware_checksum ^= byte;
        }

        memory.ble_firmware_flash_chunk(inactive_slot, chunk_index, &chunk)?;

        // Update progress.
        progress.set((chunk_index + 1) as f32 / (num_chunks as f32));
    }

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

    memory.set_ble_metadata(&ble_metadata)?;

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
        .confirm(&ConfirmParams {
            title: "Bluetooth",
            body: "Upgrade\nfirmware?",
            longtouch: true,
            ..Default::default()
        })
        .await?;

    let mut progress = hal.ui().progress_create("Upgrading...");
    let response = _process_upgrade(
        hal.memory(),
        &mut RealFuncs,
        &mut progress,
        request,
        &ALLOWED_HASH,
    )
    .await;
    drop(progress);

    if response.is_ok() {
        hal.ui().status("Upgrade\nsuccessful", true).await;
        bitbox02::reset_ble();
        if crate::communication_mode::ble_enabled(hal) {
            // Since the Bluetooth host will not be there anymore to read this response, this task
            // will not be cleared by the executor. We do it manually to make space for the next
            // task upon reconnection.
            crate::async_usb::cancel();
        }
    } else {
        hal.ui().status("Upgrade failed", false).await;
    }
    response
}

async fn process_toggle_enabled(hal: &mut impl crate::hal::Hal) -> Result<Response, Error> {
    let enabled = hal.memory().ble_enabled();
    let body = if enabled {
        "Disable Bluetooth?"
    } else {
        "Enable Bluetooth?"
    };

    hal.ui()
        .confirm(&ConfirmParams {
            body,
            longtouch: true,
            ..Default::default()
        })
        .await?;

    hal.memory()
        .ble_enable(!enabled)
        .map_err(|_| Error::Memory)?;

    let status_text = if enabled {
        "Bluetooth\ndisabled"
    } else {
        "Bluetooth\nenabled"
    };
    hal.ui().status(status_text, true).await;

    Ok(pb::bluetooth_response::Response::Success(
        pb::BluetoothSuccess {},
    ))
}

/// Processes a Bluetooth API call (BB02+ only).
pub async fn process_api(
    hal: &mut impl crate::hal::Hal,
    request: &Request,
) -> Result<Response, Error> {
    if !matches!(
        hal.memory().get_platform().map_err(|_| Error::Memory)?,
        hal_memory::Platform::BitBox02Plus
    ) {
        return Err(Error::Disabled);
    }
    match request {
        Request::UpgradeInit(request) => process_upgrade(hal, request).await,
        // These are streamed asynchronously using the `next_request()` primitive are not handled
        // directly.
        Request::Chunk(_) => Err(Error::InvalidInput),
        Request::ToggleEnabled(_) => process_toggle_enabled(hal).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bitbox02::testing::mock_memory;
    use util::bb02_async::block_on;

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
            let mut memory = crate::hal::testing::TestingMemory::new();
            let mut mock_funcs = MockFuncs {
                chunk_requests: vec![],
            };
            let allowed_hash: [u8; 32] =
                Sha256::digest(vec![0; test.firmware_length as usize]).into();
            assert!(
                block_on(_process_upgrade(
                    &mut memory,
                    &mut mock_funcs,
                    &mut crate::hal::testing::ui::NoopProgress,
                    &pb::BluetoothUpgradeInitRequest {
                        firmware_length: test.firmware_length,
                    },
                    &allowed_hash,
                ))
                .is_ok()
            );
            assert_eq!(mock_funcs.chunk_requests, test.expected_chunk_requests);
        }
    }
}
