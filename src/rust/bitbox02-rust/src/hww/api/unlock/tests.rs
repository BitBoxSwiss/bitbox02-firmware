// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::async_usb;
use crate::hal::testing::TestingHal;
use crate::hal::testing::ui::Screen;
use crate::hww::SafeData;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::future::poll_fn;
use core::task::Poll;
use hex_lit::hex;
use prost::Message;

// These tests exercise real Noise and async_usb continuations, including time spent between
// requests, rather than the synchronous MOCK_NEXT_REQUEST shortcut. Run with --test-threads 1.
static HAL: SafeData<RefCell<Option<TestingHal<'static>>>> = SafeData(RefCell::new(None));
const SEED: [u8; 32] = hex!("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c");

async fn task(packet: Vec<u8>) -> Vec<u8> {
    let mut hal = HAL.0.borrow_mut().take().unwrap();
    let response = crate::hww::process_packet(&mut hal, packet).await;
    HAL.0.borrow_mut().replace(hal);
    response
}

struct EntryGuard(Rc<Cell<usize>>);
impl Drop for EntryGuard {
    fn drop(&mut self) {
        self.0.set(self.0.get() - 1);
    }
}

struct Host {
    encrypt: Box<dyn FnMut(&[u8]) -> Vec<u8>>,
    decrypt: Box<dyn FnMut(&[u8]) -> Vec<u8>>,
    input: Rc<RefCell<Option<String>>>,
    entries: Rc<Cell<usize>>,
    active: Rc<Cell<usize>>,
}

impl Host {
    async fn new(enabled: bool, abort_screen: Option<usize>) -> Self {
        async_usb::cancel();
        crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut().take();
        crate::keystore::lock();
        let (encrypt, decrypt) = crate::hww::tests::init_noise_ciphers();
        let mut hal = TestingHal::new();
        crate::keystore::encrypt_and_store_seed(&mut hal, &SEED, "password")
            .await
            .unwrap();
        hal.memory.set_initialized().unwrap();
        hal.memory.set_mnemonic_passphrase_enabled(enabled).unwrap();
        crate::keystore::lock();
        if let Some(screen) = abort_screen {
            hal.ui.abort_nth(screen);
        }
        let host = Self {
            encrypt: Box::new(encrypt),
            decrypt: Box::new(decrypt),
            input: Rc::new(RefCell::new(None)),
            entries: Rc::new(Cell::new(0)),
            active: Rc::new(Cell::new(0)),
        };
        let input = host.input.clone();
        let entries = host.entries.clone();
        let active = host.active.clone();
        hal.ui.set_enter_string_async(Box::new(move |params| {
            if !params.passphrase {
                return Box::pin(async { Ok("password".into()) });
            }
            entries.set(entries.get() + 1);
            active.set(active.get() + 1);
            let guard = EntryGuard(active.clone());
            let input = input.clone();
            Box::pin(async move {
                let _guard = guard;
                poll_fn(|_| match input.borrow_mut().take() {
                    Some(input) => Poll::Ready(Ok(input)),
                    None => Poll::Pending,
                })
                .await
            })
        }));
        HAL.0.borrow_mut().replace(hal);
        host
    }

    fn send(&mut self, request: Request) {
        let encoded = pb::Request {
            request: Some(request),
        }
        .encode_to_vec();
        let packet = (self.encrypt)(&encoded);
        if async_usb::is_idle() {
            async_usb::spawn(task, &packet);
        } else {
            assert!(async_usb::waiting_for_next_request());
            async_usb::on_next_request(&packet);
        }
    }

    fn receive(&mut self) -> Response {
        for _ in 0..10_000 {
            async_usb::spin();
            if let Ok(packet) = async_usb::take_response() {
                assert_eq!(packet[0], crate::hww::OP_STATUS_SUCCESS);
                let plain = (self.decrypt)(&packet[1..]);
                return pb::Response::decode(plain.as_slice())
                    .unwrap()
                    .response
                    .unwrap();
            }
        }
        panic!("no response");
    }

    fn query(&mut self, request: Request) -> Response {
        self.send(request);
        self.receive()
    }

    fn start(&mut self) -> Response {
        self.query(Request::Unlock(pb::UnlockRequest {}))
    }

    fn poll(&mut self, request_host_entry: bool) -> Response {
        self.query(Request::UnlockContinue(pb::UnlockContinueRequest {
            request_host_entry,
        }))
    }

    fn submit(&mut self, passphrase: Option<&str>) -> Response {
        self.query(Request::UnlockHostInfo(pb::UnlockHostInfoRequest {
            passphrase: passphrase.map(Into::into),
        }))
    }

    async fn check_wallet(&self, passphrase: &str) {
        assert!(!crate::keystore::is_locked());
        let mut hal = HAL.0.borrow_mut().take().unwrap();
        assert_eq!(
            crate::keystore::copy_bip39_seed(&mut hal)
                .await
                .unwrap()
                .as_slice(),
            bip39::Mnemonic::from_entropy(&SEED)
                .unwrap()
                .to_seed_normalized(passphrase)
                .as_slice()
        );
        HAL.0.borrow_mut().replace(hal);
    }

    fn screens(&self) -> Vec<Screen> {
        HAL.0.borrow().as_ref().unwrap().ui.screens.clone()
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        async_usb::cancel();
        HAL.0.borrow_mut().take();
        crate::keystore::lock();
    }
}

fn pending() -> Response {
    response(State::PassphrasePending)
}
fn ready() -> Response {
    response(State::HostEntryReady)
}
fn entered() -> Response {
    response(State::PassphraseEntered)
}
fn done() -> Response {
    response(State::Done)
}

#[async_test::test]
async fn test_process_device_completion_wins_click() {
    let mut host = Host::new(true, None).await;
    assert_eq!(host.start(), pending());
    for _ in 0..3 {
        assert_eq!(host.poll(false), pending());
        assert_eq!(host.entries.get(), 1);
        assert_eq!(host.active.get(), 1);
    }
    host.input.borrow_mut().replace("device value".into());
    async_usb::spin();
    assert!(async_usb::waiting_for_next_request());
    assert!(matches!(
        async_usb::take_response(),
        Err(async_usb::CopyResponseErr::NotReady)
    ));
    assert_eq!(host.poll(true), entered());
    assert_eq!(host.active.get(), 0);
    assert!(crate::keystore::is_locked());
    // A second late host-entry request cannot interrupt device confirmation either.
    assert_eq!(host.poll(true), done());
    host.check_wallet("device value").await;
    // Late input cannot change the chosen wallet; a repeated Unlock is a no-op.
    assert_eq!(
        host.submit(Some("late")),
        super::super::error::make_error(Error::InvalidState)
    );
    assert_eq!(host.start(), done());
    host.check_wallet("device value").await;
    assert!(!host.screens().iter().any(
        |s| matches!(s, Screen::Confirm { body, .. } if body == "Enter passphrase\non host?")
    ));
}

#[async_test::test]
async fn test_process_completion_before_pending_ack() {
    let mut host = Host::new(true, None).await;
    host.send(Request::Unlock(pb::UnlockRequest {}));
    for _ in 0..100 {
        async_usb::spin();
    }
    host.input.borrow_mut().replace(String::new());
    for _ in 0..5 {
        async_usb::spin();
    }
    // Completing the UI does not overwrite the intermediate response or abandon next_request.
    assert_eq!(host.receive(), pending());
    assert_eq!(host.poll(false), entered());
    assert_eq!(host.poll(false), done());
    host.check_wallet("").await;
}

#[async_test::test]
async fn test_process_host_requires_actual_confirmation() {
    for value in ["host passphrase", "", "  spaces  "] {
        let mut host = Host::new(true, None).await;
        assert_eq!(host.start(), pending());
        assert_eq!(host.poll(true), ready());
        assert_eq!(host.active.get(), 0);
        for _ in 0..100 {
            async_usb::spin();
        }
        assert!(crate::keystore::is_locked());
        assert_eq!(host.submit(Some(value)), done());
        let screens = host.screens();
        assert!(screens.iter().any(
            |s| matches!(s, Screen::Confirm { body, .. } if body == "Enter passphrase\non host?")
        ));
        assert!(screens.iter().any(|s| matches!(s, Screen::PrintScreen { message, .. } if message == "Enter passphrase\non host")));
        let expected = if value.is_empty() {
            "Use empty passphrase?"
        } else {
            value
        };
        assert!(screens.iter().any(
            |s| matches!(s, Screen::Confirm { body, longtouch: true, .. } if body == expected)
        ));
        host.check_wallet(value).await;
    }
}

#[async_test::test]
async fn test_process_consent_rejection_and_host_cancellation_restart_entry() {
    // Screen 0 is the paused unlock animation; screen 1 is host-entry consent.
    for reject in [true, false] {
        let mut host = Host::new(true, if reject { Some(1) } else { None }).await;
        assert_eq!(host.start(), pending());
        if reject {
            assert_eq!(host.poll(true), pending());
        } else {
            assert_eq!(host.poll(true), ready());
            assert_eq!(host.submit(None), pending());
        }
        assert_eq!(host.entries.get(), 2);
        assert_eq!(host.active.get(), 1);
        host.input.borrow_mut().replace(String::new());
        assert_eq!(host.poll(false), entered());
        assert_eq!(host.poll(false), done());
        host.check_wallet("").await;
    }
}

#[async_test::test]
async fn test_process_actual_passphrase_rejection_restarts_entry() {
    for (value, abort_screen) in [("secret", 4), ("", 3)] {
        let mut host = Host::new(true, Some(abort_screen)).await;
        assert_eq!(host.start(), pending());
        assert_eq!(host.poll(true), ready());
        assert_eq!(host.submit(Some(value)), pending());
        assert_eq!(host.entries.get(), 2);
        host.input.borrow_mut().replace("device".into());
        assert_eq!(host.poll(false), entered());
        assert_eq!(host.poll(false), done());
        host.check_wallet("device").await;
    }
}

#[async_test::test]
async fn test_process_device_confirmation_rejection_restarts_entry() {
    // Screen 0 is the paused animation, followed by the introduction and actual value.
    for abort_screen in [1, 2] {
        let mut host = Host::new(true, Some(abort_screen)).await;
        assert_eq!(host.start(), pending());
        host.input.borrow_mut().replace("device".into());
        assert_eq!(host.poll(false), entered());
        assert_eq!(host.active.get(), 0);
        assert!(crate::keystore::is_locked());
        assert_eq!(host.poll(false), pending());
        assert_eq!(host.entries.get(), 2);
        assert_eq!(host.active.get(), 1);
        // Only the new entry phase allows the host to request consent again.
        assert_eq!(host.poll(true), ready());
        assert_eq!(host.submit(Some("host")), done());
        host.check_wallet("host").await;
    }
}

#[async_test::test]
async fn test_process_invalid_continuations_clear_partial_unlock() {
    for phase in 0..3 {
        for request in [
            Request::Unlock(pb::UnlockRequest {}),
            Request::UnlockContinue(pb::UnlockContinueRequest {
                request_host_entry: true,
            }),
            Request::UnlockHostInfo(pb::UnlockHostInfoRequest {
                passphrase: Some("early".into()),
            }),
        ] {
            if matches!(
                (&request, phase),
                (Request::UnlockContinue(_), 0 | 2) | (Request::UnlockHostInfo(_), 1)
            ) {
                continue;
            }
            let mut host = Host::new(true, None).await;
            assert_eq!(host.start(), pending());
            if phase == 1 {
                assert_eq!(host.poll(true), ready());
            } else if phase == 2 {
                host.input.borrow_mut().replace("device".into());
                assert_eq!(host.poll(false), entered());
            }
            assert_eq!(
                host.query(request),
                super::super::error::make_error(Error::InvalidState)
            );
            assert!(async_usb::is_idle());
            assert_eq!(host.active.get(), 0);
            let mut hal = HAL.0.borrow_mut().take().unwrap();
            assert!(crate::keystore::copy_seed(&mut hal).await.is_err());
            HAL.0.borrow_mut().replace(hal);
        }
    }
}

#[async_test::test]
async fn test_process_reset_clears_partial_unlock() {
    for phase in 0..5 {
        let mut host = Host::new(true, None).await;
        if phase == 0 {
            host.send(Request::Unlock(pb::UnlockRequest {}));
            for _ in 0..100 {
                async_usb::spin();
            }
        } else {
            assert_eq!(host.start(), pending());
            if phase == 2 {
                assert_eq!(host.poll(true), ready());
            } else if phase >= 3 {
                host.input.borrow_mut().replace("device".into());
                if phase == 3 {
                    assert_eq!(host.poll(false), entered());
                } else {
                    // Reset with PASSPHRASE_ENTERED still unread, before its continuation.
                    host.send(Request::UnlockContinue(pb::UnlockContinueRequest::default()));
                    for _ in 0..100 {
                        async_usb::spin();
                    }
                }
            }
        }
        let mut hal = TestingHal::new();
        crate::hww::reset_session(&mut hal);
        assert!(async_usb::is_idle());
        assert_eq!(host.active.get(), 0);
        assert!(crate::keystore::copy_seed(&mut hal).await.is_err());
        assert!(crate::hww::noise::encrypt(b"old session", &mut Vec::new()).is_err());
        // A fresh handshake after reset works without an old continuation.
        let _ = crate::hww::tests::init_noise_ciphers();
    }
}

#[async_test::test]
async fn test_process_passphrase_disabled() {
    let mut host = Host::new(false, None).await;
    assert_eq!(host.start(), done());
    assert_eq!(host.entries.get(), 0);
    host.check_wallet("").await;
}

#[async_test::test]
async fn test_process_invalid_host_input_restarts_entry() {
    for (value, expected_status) in [
        ("x".repeat(MAX_PASSPHRASE_LEN + 1), "Passphrase too\nlong"),
        ("\0".into(), "Unsupported\ncharacters"),
        ("é".into(), "Unsupported\ncharacters"),
        ("line\nbreak".into(), "Unsupported\ncharacters"),
        ("~".into(), "Unsupported\ncharacters"),
        ("`".into(), "Unsupported\ncharacters"),
    ] {
        let mut host = Host::new(true, None).await;
        assert_eq!(host.start(), pending());
        assert_eq!(host.poll(true), ready());
        assert_eq!(host.submit(Some(&value)), pending());
        assert!(crate::keystore::is_locked());
        assert_eq!(host.entries.get(), 2);
        assert_eq!(host.active.get(), 1);
        host.input.borrow_mut().replace("device".into());
        assert_eq!(host.poll(false), entered());
        assert_eq!(host.poll(false), done());
        host.check_wallet("device").await;
        let screens = host.screens();
        assert!(screens.iter().any(
            |s| matches!(s, Screen::Status { title, success: false } if title == expected_status)
        ));
        assert!(
            !screens
                .iter()
                .any(|s| matches!(s, Screen::Confirm { body, .. } if body == &value))
        );
    }
    assert!(validate_host_passphrase(&"x".repeat(MAX_PASSPHRASE_LEN)).is_ok());
    assert!(validate_host_passphrase("AZaz09 !\"#$%&'()*+,-./:;<=>?^[\\]@_{|}").is_ok());
}

#[async_test::test]
async fn test_process_invalid_host_input_can_retry_on_host() {
    let mut host = Host::new(true, None).await;
    assert_eq!(host.start(), pending());
    for value in ["x".repeat(MAX_PASSPHRASE_LEN + 1), "~".into()] {
        assert_eq!(host.poll(true), ready());
        assert_eq!(host.submit(Some(&value)), pending());
        assert!(crate::keystore::is_locked());
        assert_eq!(host.active.get(), 1);
    }
    assert_eq!(host.entries.get(), 3);
    assert_eq!(host.poll(true), ready());
    let valid = "x".repeat(MAX_PASSPHRASE_LEN);
    assert_eq!(host.submit(Some(&valid)), done());
    host.check_wallet(&valid).await;
}

#[async_test::test]
async fn test_process_uninitialized_returns_done() {
    crate::keystore::lock();
    let mut hal = TestingHal::new();
    for seeded in [false, true] {
        if seeded {
            crate::keystore::encrypt_and_store_seed(&mut hal, &SEED, "password")
                .await
                .unwrap();
        }
        // Repeated unlock requests must not change an uninitialized device or setup in progress.
        for _ in 0..2 {
            let request = pb::Request {
                request: Some(Request::Unlock(pb::UnlockRequest {})),
            }
            .encode_to_vec();
            let reply = super::super::process(&mut hal, request).await;
            assert_eq!(
                pb::Response::decode(reply.as_slice())
                    .unwrap()
                    .response
                    .unwrap(),
                done()
            );
            assert!(hal.ui.screens.is_empty());
            assert!(!hal.memory.is_initialized());
            assert_eq!(hal.memory.is_seeded(), seeded);
            assert!(crate::keystore::is_locked());
            if seeded {
                assert_eq!(
                    crate::keystore::copy_seed(&mut hal)
                        .await
                        .unwrap()
                        .as_slice(),
                    SEED.as_slice()
                );
            } else {
                assert!(crate::keystore::copy_seed(&mut hal).await.is_err());
            }
        }
    }
    crate::keystore::lock();
}

#[test]
fn test_decode_host_passphrase_fields() {
    use super::super::decode;
    // Length and character validation belongs to the unlock workflow, so it can show a
    // status and restart entry. Decoding only checks the protobuf encoding.
    let too_long = "x".repeat(MAX_PASSPHRASE_LEN + 1);
    for value in [
        None,
        Some(""),
        Some("value"),
        Some(too_long.as_str()),
        Some("~"),
        Some("é"),
        Some("\0"),
    ] {
        let encoded = pb::Request {
            request: Some(Request::UnlockHostInfo(pb::UnlockHostInfoRequest {
                passphrase: value.map(Into::into),
            })),
        }
        .encode_to_vec();
        let Request::UnlockHostInfo(request) = decode(&encoded).unwrap() else {
            panic!("expected host passphrase request");
        };
        assert_eq!(request.passphrase.as_deref(), value);
    }
    // Invalid UTF-8 and truncated strings are rejected as malformed protobuf.
    for bytes in [
        hex!("9202030a01ff").as_slice(),
        hex!("9202030a0561").as_slice(),
    ] {
        assert!(matches!(decode(bytes), Err(Error::InvalidInput)));
    }
}
