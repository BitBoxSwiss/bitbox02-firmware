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

pub mod api;
pub mod noise;

use alloc::vec::Vec;

const OP_UNLOCK: u8 = b'u';
const OP_ATTESTATION: u8 = b'a';

const OP_STATUS_SUCCESS: u8 = 0;
const OP_STATUS_FAILURE: u8 = 1;
const OP_STATUS_FAILURE_UNINITIALIZED: u8 = 2;

/// Must be called during the execution of a usb task. This sends out the response to the host and
/// awaits the next request. If the request is not a valid noise encrypted protofbuf api request
/// message, `Err(Error::InvalidInput)` is returned.
#[cfg(not(feature = "testing"))]
pub async fn next_request(
    response: crate::pb::response::Response,
) -> Result<crate::pb::request::Request, api::error::Error> {
    let mut out = [OP_STATUS_SUCCESS].to_vec();
    noise::encrypt(&api::encode(response), &mut out).or(Err(api::error::Error::NoiseEncrypt))?;
    let request = crate::async_usb::next_request(out).await;
    match request.split_first() {
        Some((&noise::OP_NOISE_MSG, encrypted_request)) => {
            let decrypted_request =
                noise::decrypt(encrypted_request).or(Err(api::error::Error::NoiseDecrypt))?;
            api::decode(&decrypted_request[..])
        }
        _ => Err(api::error::Error::InvalidInput),
    }
}

#[cfg(feature = "testing")]
pub struct SafeData<T>(T);
// Safety: must not be accessed concurrently.
#[cfg(feature = "testing")]
unsafe impl<T> Sync for SafeData<T> {}

#[cfg(feature = "testing")]
pub static MOCK_NEXT_REQUEST: SafeData<
    core::cell::RefCell<
        Option<
            alloc::boxed::Box<
                dyn Fn(
                    crate::pb::response::Response,
                ) -> Result<crate::pb::request::Request, api::error::Error>,
            >,
        >,
    >,
> = SafeData(core::cell::RefCell::new(None));

/// Set `MOCK_NEXT_REQUEST` to mock requests from the host.
#[cfg(feature = "testing")]
pub async fn next_request(
    response: crate::pb::response::Response,
) -> Result<crate::pb::request::Request, api::error::Error> {
    let func = MOCK_NEXT_REQUEST.0.borrow();
    func.as_ref().unwrap()(response)
}

/// Process OP_UNLOCK.
async fn api_unlock(hal: &mut impl crate::hal::Hal) -> Vec<u8> {
    match crate::workflow::unlock::unlock(hal).await {
        Ok(()) => [OP_STATUS_SUCCESS].to_vec(),
        Err(()) => [OP_STATUS_FAILURE_UNINITIALIZED].to_vec(),
    }
}

/// Process OP_ATTESTATION.
///
/// On failure, returns < 1 >.
///
/// On success, returns < 0 | bootloader_hash 32 | device_pubkey 64 |
/// certificate 64 | root_pubkey_identifier 32 | challenge_signature 64>
fn api_attestation(usb_in: &[u8]) -> Vec<u8> {
    use core::convert::TryInto;

    let usb_in: [u8; 32] = match usb_in.try_into() {
        Ok(usb_in) => usb_in,
        Err(_) => return [OP_STATUS_FAILURE].to_vec(),
    };

    let result = match crate::attestation::perform(usb_in) {
        Ok(result) => result,
        Err(()) => return [OP_STATUS_FAILURE].to_vec(),
    };

    let mut out = Vec::with_capacity(257);
    out.push(OP_STATUS_SUCCESS);
    out.extend_from_slice(&result.bootloader_hash[..]);
    out.extend_from_slice(&result.device_pubkey[..]);
    out.extend_from_slice(&result.certificate[..]);
    out.extend_from_slice(&result.root_pubkey_identifier[..]);
    out.extend_from_slice(&result.challenge_signature[..]);
    out
}

async fn _process_packet(hal: &mut impl crate::hal::Hal, usb_in: Vec<u8>) -> Vec<u8> {
    match usb_in.split_first() {
        Some((&OP_UNLOCK, b"")) => return api_unlock(hal).await,
        Some((&OP_ATTESTATION, rest)) => return api_attestation(rest),
        _ => (),
    }

    let mut out = [OP_STATUS_SUCCESS].to_vec();
    match noise::process(hal, usb_in, &mut out).await {
        Ok(()) => out,
        Err(noise::Error) => [OP_STATUS_FAILURE].to_vec(),
    }
}

/// Async HWW api processing main entry point.
/// `usb_in` - api request bytes.
/// Returns the usb response bytes.
pub async fn process_packet(usb_in: Vec<u8>) -> Vec<u8> {
    let hal = &mut crate::hal::BitBox02Hal::new();
    _process_packet(hal, usb_in).await
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    use crate::bb02_async::block_on;
    use crate::hal::testing::TestingHal;
    use crate::workflow::testing::Screen;
    use bitbox02::testing::{mock_memory, mock_sd};

    use prost::Message;

    use alloc::boxed::Box;
    use alloc::string::String;
    use core::convert::TryInto;

    /// Make a new noise channel by invoking the noise handshake. Returns a request function which
    /// encrypts the message going in and decrypts the message coming out.
    fn init_noise<H: crate::hal::Hal>() -> Box<dyn FnMut(&mut H, &[u8]) -> Result<Vec<u8>, ()>> {
        assert_eq!(
            block_on(_process_packet(&mut TestingHal::new(), b"h".to_vec())),
            [OP_STATUS_SUCCESS].to_vec()
        );
        let mut host_noise = bitbox02_noise::testing::make_host();
        let host_handshake_1 = host_noise.write_message_vec(b"").unwrap();
        let bb02_handshake_1 = {
            let result = block_on(_process_packet(&mut TestingHal::new(), {
                let mut m = b"H".to_vec(); // handshake opcode
                m.extend_from_slice(&host_handshake_1);
                m
            }));
            match result.split_first() {
                Some((&OP_STATUS_SUCCESS, rest)) => rest.to_vec(),
                _ => panic!("noise handshake failed"),
            }
        };

        let host_handshake_2 = {
            let payload = host_noise.read_message_vec(&bb02_handshake_1).unwrap();
            host_noise.write_message_vec(&payload).unwrap()
        };

        let response = block_on(_process_packet(&mut TestingHal::new(), {
            let mut m = b"H".to_vec(); // handshake opcode
            m.extend_from_slice(&host_handshake_2);
            m
        }));
        let verification_required = match response[..] {
            // OP_STATUS_SUCCESS and verification required byte.
            [OP_STATUS_SUCCESS, 0x01] => true,
            [OP_STATUS_SUCCESS, 0x00] => false,
            _ => panic!("handshake failed"),
        };
        if verification_required {
            // Verify pairing code.

            // Handshake hash as computed by the host. Should be the same as computed on the
            // device. The pairing code is derived from that.
            let handshake_hash: bitbox02_noise::HandshakeHash =
                host_noise.get_hash().try_into().unwrap();

            let mut mock_hal = TestingHal::new();
            assert_eq!(
                block_on(_process_packet(&mut mock_hal, b"v".to_vec())),
                [OP_STATUS_SUCCESS].to_vec()
            );
            assert_eq!(
                mock_hal.ui.screens,
                vec![Screen::Confirm {
                    title: "Pairing code".into(),
                    body: crate::workflow::pairing::format_hash(&handshake_hash),
                    longtouch: false,
                }]
            );
        }

        let (mut host_send, mut host_recv) = host_noise.get_ciphers();
        Box::new(move |hal, msg| -> Result<Vec<u8>, ()> {
            let msg_encrypted = host_send.encrypt_vec(msg);
            let response_encrypted = block_on(_process_packet(hal, {
                let mut m = b"n".to_vec(); // message opcode
                m.extend_from_slice(&msg_encrypted);
                m
            }));
            match response_encrypted.split_first() {
                Some((&OP_STATUS_SUCCESS, rest)) => Ok(host_recv.decrypt_vec(rest).unwrap()),
                _ => Err(()),
            }
        })
    }

    /// Can't unlock when the device is not initialized yet (not seeded).
    #[test]
    fn test_cant_unlock() {
        mock_memory();
        assert_eq!(
            block_on(_process_packet(&mut TestingHal::new(), vec![OP_UNLOCK])),
            [OP_STATUS_FAILURE_UNINITIALIZED].to_vec()
        );
    }

    /// Test establishing a noise channel and sending/receiving an API request over it.
    #[test]
    fn test_noise() {
        mock_memory();
        mock_sd();
        let mut make_request = init_noise();
        let request = crate::pb::Request {
            request: Some(crate::pb::request::Request::ListBackups(
                crate::pb::ListBackupsRequest {},
            )),
        };
        let response_encoded =
            make_request(&mut TestingHal::new(), &request.encode_to_vec()).unwrap();
        let response = crate::pb::Response::decode(&response_encoded[..]).unwrap();
        assert_eq!(
            response,
            crate::pb::Response {
                response: Some(crate::pb::response::Response::ListBackups(
                    crate::pb::ListBackupsResponse { info: vec![] }
                ))
            }
        );
    }

    /// Can initiate noise and send the Reboot protobuf request when the device is not seeded.
    #[test]
    fn test_reboot_when_unitialized() {
        mock_memory();

        let mut make_request = init_noise();
        let request = crate::pb::Request {
            request: Some(crate::pb::request::Request::Reboot(
                crate::pb::RebootRequest {
                    purpose: crate::pb::reboot_request::Purpose::Upgrade as _,
                },
            )),
        };
        let request_encoded = request.encode_to_vec();
        let mut mock_hal = TestingHal::new();
        let reboot_called = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            make_request(&mut mock_hal, &request_encoded).unwrap();
        }));
        match reboot_called {
            Ok(()) => panic!("reboot was not called"),
            Err(msg) => assert_eq!(msg.downcast_ref::<&str>(), Some(&"reboot called")),
        }
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "".into(),
                body: "Proceed to upgrade?".into(),
                longtouch: false,
            }]
        );
    }

    /// Can initiate noise and send the Reboot protobuf request when the device is seeded.
    #[test]
    fn test_reboot_when_seeded() {
        mock_memory();

        let mut make_request = init_noise();

        let mut mock_hal = TestingHal::new();
        mock_hal
            .ui
            .set_enter_string(Box::new(|_params| Ok("password".into())));
        make_request(
            &mut mock_hal,
            (crate::pb::Request {
                request: Some(crate::pb::request::Request::SetPassword(
                    crate::pb::SetPasswordRequest {
                        entropy: b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_vec(),
                    },
                )),
            })
            .encode_to_vec()
            .as_ref(),
        )
        .unwrap();
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Status {
                title: "Success".into(),
                success: true,
            }]
        );

        assert!(!bitbox02::keystore::is_locked());
        assert!(bitbox02::memory::is_seeded());
        assert!(!bitbox02::memory::is_initialized());

        let reboot_request = crate::pb::Request {
            request: Some(crate::pb::request::Request::Reboot(
                crate::pb::RebootRequest {
                    purpose: crate::pb::reboot_request::Purpose::Upgrade as _,
                },
            )),
        };

        // Can reboot when seeded and locked. This happens when the user sets a password and then
        // reconnects the device.
        bitbox02::keystore::lock();
        let mut mock_hal = TestingHal::new();
        let reboot_called = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            make_request(&mut mock_hal, reboot_request.encode_to_vec().as_ref()).unwrap();
        }));
        match reboot_called {
            Ok(()) => panic!("reboot was not called"),
            Err(msg) => assert_eq!(msg.downcast_ref::<&str>(), Some(&"reboot called")),
        }
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "".into(),
                body: "Proceed to upgrade?".into(),
                longtouch: false,
            }]
        );
    }

    /// Can initiate noise and send the Reboot protobuf request when the device is initialized.
    #[test]
    fn test_reboot_when_initialized() {
        mock_memory();

        let mut make_request = init_noise();

        bitbox02::keystore::lock();
        let mut mock_hal = TestingHal::new();
        mock_hal.sd.inserted = Some(true);
        mock_hal
            .ui
            .set_enter_string(Box::new(|_params| Ok("password".into())));
        make_request(
            &mut mock_hal,
            (crate::pb::Request {
                request: Some(crate::pb::request::Request::SetPassword(
                    crate::pb::SetPasswordRequest {
                        entropy: b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_vec(),
                    },
                )),
            })
            .encode_to_vec()
            .as_ref(),
        )
        .unwrap();
        assert!(!bitbox02::keystore::is_locked());
        assert!(!bitbox02::memory::is_initialized());
        let mut mock_hal = TestingHal::new();
        mock_hal.sd.inserted = Some(true);
        make_request(
            &mut mock_hal,
            (crate::pb::Request {
                request: Some(crate::pb::request::Request::CreateBackup(
                    crate::pb::CreateBackupRequest {
                        timestamp: 1601281809,
                        timezone_offset: 18000,
                    },
                )),
            })
            .encode_to_vec()
            .as_ref(),
        )
        .unwrap();
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Is today?".into(),
                    body: "Mon 2020-09-28".into(),
                    longtouch: false,
                },
                Screen::Status {
                    title: "Backup created".into(),
                    success: true,
                }
            ]
        );
        assert!(bitbox02::memory::is_initialized());

        let reboot_request = crate::pb::Request {
            request: Some(crate::pb::request::Request::Reboot(
                crate::pb::RebootRequest {
                    purpose: crate::pb::reboot_request::Purpose::Upgrade as _,
                },
            )),
        };

        // Can't reboot when initialized but locked.
        bitbox02::keystore::lock();
        let mut mock_hal = TestingHal::new();
        let response_encoded =
            make_request(&mut mock_hal, &reboot_request.encode_to_vec()).unwrap();
        let response = crate::pb::Response::decode(&response_encoded[..]).unwrap();
        assert_eq!(mock_hal.ui.screens, vec![]);
        assert_eq!(
            response,
            crate::pb::Response {
                response: Some(api::error::make_error(api::error::Error::InvalidState))
            },
        );

        // Unlock.
        let mut mock_hal = TestingHal::new();
        mock_hal
            .ui
            .set_enter_string(Box::new(|_params| Ok("password".into())));
        assert_eq!(
            block_on(_process_packet(&mut mock_hal, vec![OP_UNLOCK])),
            [OP_STATUS_SUCCESS].to_vec()
        );
        assert!(!bitbox02::keystore::is_locked());

        // Since in the previous request the msg was encrypted but not decrypted (query was
        // rejected), the noise states are out of sync and we need to make a new channel.
        let mut make_request = init_noise();
        let mut mock_hal = TestingHal::new();
        let reboot_called = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            make_request(&mut mock_hal, reboot_request.encode_to_vec().as_ref()).unwrap();
        }));
        match reboot_called {
            Ok(()) => panic!("reboot was not called"),
            Err(msg) => assert_eq!(msg.downcast_ref::<&str>(), Some(&"reboot called")),
        }
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "".into(),
                body: "Proceed to upgrade?".into(),
                longtouch: false,
            }]
        );
    }

    /// Test creating a seed, backing it up on SD, checking the backup, and restoring from the that backup.
    #[test]
    fn test_backup_create_check_list_restore() {
        // Test everything with a 32 and 16 byte seed (determined by the host entropy when creating the seed).
        for host_entropy in &[
            &b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"[..],
            &b"aaaaaaaaaaaaaaaa"[..],
        ] {
            bitbox02::keystore::lock();
            mock_memory();
            mock_sd();

            bitbox02::memory::set_device_name("test device name").unwrap();

            let mut make_request = init_noise();
            let mut mock_hal = TestingHal::new();
            mock_hal.sd.inserted = Some(true);
            mock_hal
                .ui
                .set_enter_string(Box::new(|_params| Ok("password".into())));
            make_request(
                &mut mock_hal,
                (crate::pb::Request {
                    request: Some(crate::pb::request::Request::SetPassword(
                        crate::pb::SetPasswordRequest {
                            entropy: host_entropy.to_vec(),
                        },
                    )),
                })
                .encode_to_vec()
                .as_ref(),
            )
            .unwrap();
            assert_eq!(
                mock_hal.ui.screens,
                vec![Screen::Status {
                    title: "Success".into(),
                    success: true,
                }]
            );

            let mut mock_hal = TestingHal::new();
            mock_hal.sd.inserted = Some(true);
            make_request(
                &mut mock_hal,
                (crate::pb::Request {
                    request: Some(crate::pb::request::Request::CreateBackup(
                        crate::pb::CreateBackupRequest {
                            timestamp: 1601281809,
                            timezone_offset: 18000,
                        },
                    )),
                })
                .encode_to_vec()
                .as_ref(),
            )
            .unwrap();
            assert_eq!(
                mock_hal.ui.screens,
                vec![
                    Screen::Confirm {
                        title: "Is today?".into(),
                        body: "Mon 2020-09-28".into(),
                        longtouch: false,
                    },
                    Screen::Status {
                        title: "Backup created".into(),
                        success: true,
                    }
                ]
            );

            let seed = bitbox02::keystore::copy_seed().unwrap();
            assert_eq!(seed.len(), host_entropy.len());
            let mut mock_hal = TestingHal::new();
            assert!(matches!(
                crate::pb::Response::decode(
                    make_request(
                        &mut mock_hal,
                        (crate::pb::Request {
                            request: Some(crate::pb::request::Request::CheckBackup(
                                crate::pb::CheckBackupRequest { silent: true },
                            )),
                        })
                        .encode_to_vec()
                        .as_ref(),
                    )
                    .unwrap()
                    .as_slice(),
                )
                .unwrap(),
                crate::pb::Response {
                    response: Some(crate::pb::response::Response::CheckBackup(
                        crate::pb::CheckBackupResponse { .. }
                    )),
                }
            ));
            assert_eq!(mock_hal.ui.screens, vec![]);

            let mut mock_hal = TestingHal::new();
            make_request(
                &mut mock_hal,
                (crate::pb::Request {
                    request: Some(crate::pb::request::Request::Reset(
                        crate::pb::ResetRequest {},
                    )),
                })
                .encode_to_vec()
                .as_ref(),
            )
            .unwrap();
            assert_eq!(
                mock_hal.ui.screens,
                vec![Screen::Confirm {
                    title: "RESET".into(),
                    body: "Proceed to\nfactory reset?".into(),
                    longtouch: true,
                }]
            );

            let mut mock_hal = TestingHal::new();
            assert!(matches!(
                crate::pb::Response::decode(
                    make_request(
                        &mut mock_hal,
                        (crate::pb::Request {
                            request: Some(crate::pb::request::Request::CheckBackup(
                                crate::pb::CheckBackupRequest { silent: true },
                            )),
                        })
                        .encode_to_vec()
                        .as_ref(),
                    )
                    .unwrap()
                    .as_slice(),
                )
                .unwrap(),
                crate::pb::Response {
                    response: Some(crate::pb::response::Response::Error(
                        crate::pb::Error { .. }
                    )),
                }
            ));
            assert_eq!(mock_hal.ui.screens, vec![]);

            let mut mock_hal = TestingHal::new();
            let backup_id = match crate::pb::Response::decode(
                make_request(
                    &mut mock_hal,
                    (crate::pb::Request {
                        request: Some(crate::pb::request::Request::ListBackups(
                            crate::pb::ListBackupsRequest {},
                        )),
                    })
                    .encode_to_vec()
                    .as_ref(),
                )
                .unwrap()
                .as_slice(),
            )
            .unwrap()
            {
                crate::pb::Response {
                    response:
                        Some(crate::pb::response::Response::ListBackups(
                            crate::pb::ListBackupsResponse { info },
                        )),
                } => match info.as_slice() {
                    &[crate::pb::BackupInfo {
                        ref id, ref name, ..
                    }] => {
                        assert_eq!(name.as_str(), "test device name");
                        id.clone()
                    }
                    _ => panic!("unexpected response"),
                },
                _ => panic!("unexpected response"),
            };
            assert_eq!(mock_hal.ui.screens, vec![]);

            let mut mock_hal = TestingHal::new();
            mock_hal
                .ui
                .set_enter_string(Box::new(|_params| Ok("password".into())));
            assert!(matches!(
                crate::pb::Response::decode(
                    make_request(
                        &mut mock_hal,
                        (crate::pb::Request {
                            request: Some(crate::pb::request::Request::RestoreBackup(
                                crate::pb::RestoreBackupRequest {
                                    id: backup_id.clone(),
                                    timestamp: 1601281809,
                                    timezone_offset: 18000,
                                },
                            )),
                        })
                        .encode_to_vec()
                        .as_ref(),
                    )
                    .unwrap()
                    .as_slice(),
                )
                .unwrap(),
                crate::pb::Response {
                    response: Some(crate::pb::response::Response::Success(
                        crate::pb::Success { .. }
                    )),
                }
            ));
            assert_eq!(
                mock_hal.ui.screens,
                vec![
                    Screen::Confirm {
                        title: "".into(),
                        body: "Restore backup?".into(),
                        longtouch: false,
                    },
                    Screen::Confirm {
                        title: "".into(),
                        body: format!("Name: test device name. ID: {}", backup_id.as_str()),
                        longtouch: false,
                    },
                    Screen::Confirm {
                        title: "Is now?".into(),
                        body: "Mon 2020-09-28\n13:30".into(),
                        longtouch: false,
                    },
                    Screen::Status {
                        title: "Success".into(),
                        success: true,
                    }
                ]
            );

            // Restored seed is the same as the seed that was backed up.
            assert_eq!(seed, bitbox02::keystore::copy_seed().unwrap());
        }
    }
}
