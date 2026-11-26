// SPDX-License-Identifier: Apache-2.0

use super::*;
use bitbox_u2fhid::VendorCommandHandler;
use bitbox02_rust::pb;
use prost::Message;

#[test]
fn test_args() {
    let args = Args::try_parse_from(["simulator"]).unwrap();
    assert!(!args.preseed && !args.unlock && !args.passphrase);
    assert_eq!(args.port, 15423);
    assert!(Args::try_parse_from(["simulator", "--unlock"]).is_err());
    assert!(Args::try_parse_from(["simulator", "--port", "65536"]).is_err());
    let args = Args::try_parse_from([
        "simulator",
        "--preseed",
        "--unlock",
        "--passphrase",
        "--port",
        "15424",
    ])
    .unwrap();
    assert!(args.preseed && args.unlock && args.passphrase);
    assert_eq!(args.port, 15424);
}

#[async_test::test]
async fn test_init_hww() {
    let mut hal = Backend::init_hal().unwrap();
    let platform = if cfg!(feature = "product-bitbox03") {
        3
    } else if cfg!(feature = "product-bitbox02nova") {
        2
    } else {
        0
    };
    for (preseed, unlock, passphrase) in [
        (false, false, false),
        (false, false, true),
        (true, false, false),
        (true, false, true),
        (true, true, false),
        (true, true, true),
    ] {
        hal.memory().reset_hww().unwrap();
        let args = Args {
            preseed,
            unlock,
            passphrase,
            port: 15423,
        };
        let _transport = init_hww(&mut hal, &args);
        assert_eq!(hal.memory().is_initialized(), preseed);
        assert_eq!(hal.memory().is_seeded(), preseed);
        assert_eq!(hal.memory().is_mnemonic_passphrase_enabled(), passphrase);
        assert_eq!(bitbox02_rust::keystore::is_locked(), !unlock);

        let mut handler =
            bitbox02_rust::hww::transport::HwwVendorHandler::new(SimulatorHal::default());
        let info = handler.handle_vendor_command(1, 0xc1, b"i", 0).unwrap();
        assert_eq!(
            &info[usize::from(info[0]) + 1..],
            &[
                platform,
                u8::from(cfg!(feature = "edition-btc-only")),
                u8::from(unlock),
                u8::from(preseed),
            ]
        );
    }

    for request in [
        pb::request::Request::BtcPub(pb::BtcPubRequest {
            coin: pb::BtcCoin::Ltc as i32,
            ..Default::default()
        }),
        pb::request::Request::Eth(pb::EthRequest::default()),
        pb::request::Request::Cardano(pb::CardanoRequest::default()),
    ] {
        let response = bitbox02_rust::hww::api::process(
            &mut hal,
            pb::Request {
                request: Some(request),
            }
            .encode_to_vec(),
        )
        .await;
        let Some(pb::response::Response::Error(error)) =
            pb::Response::decode(response.as_slice()).unwrap().response
        else {
            panic!("expected an API error");
        };
        assert_eq!(
            error.code,
            if cfg!(feature = "edition-btc-only") {
                106
            } else {
                101
            }
        );
    }
    bitbox02_rust::keystore::lock();
}

#[test]
fn test_handle_stream_reader_fragmented_reports() {
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let mut client = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
    let (server, _) = listener.accept().unwrap();
    let (sender, receiver) = mpsc::channel();
    let counter = Arc::new(AtomicU32::new(1));
    let reader = thread::spawn({
        let counter = counter.clone();
        move || handle_stream_reader(server, sender, counter)
    });
    client.write_all(&[1; 13]).unwrap();
    assert!(receiver.recv_timeout(Duration::from_millis(20)).is_err());
    client.write_all(&[1; 51]).unwrap();
    client.write_all(&[2; 64]).unwrap();
    assert_eq!(
        receiver.recv_timeout(Duration::from_secs(1)).unwrap(),
        [1; 64]
    );
    assert_eq!(
        receiver.recv_timeout(Duration::from_secs(1)).unwrap(),
        [2; 64]
    );
    drop(client);
    reader.join().unwrap();
    assert_eq!(counter.load(Ordering::SeqCst), 0);
    assert!(receiver.recv().is_err());
}
