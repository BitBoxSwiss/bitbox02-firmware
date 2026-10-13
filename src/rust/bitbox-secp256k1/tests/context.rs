// SPDX-License-Identifier: Apache-2.0

use bitbox_secp256k1::SECP256K1;
use bitcoin::secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use hex_lit::hex;

#[test]
fn test_init() {
    #[cfg(not(any(feature = "testing", feature = "c-unit-testing")))]
    assert!(std::panic::catch_unwind(|| SECP256K1.ctx()).is_err());

    let seed = hex!("d863bf4c526a840b541c4bc1d7335cf5e4fab8f07a2b2b0e3cc036952528641a");
    SECP256K1.init(&seed);

    let reference = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&hex!(
        "e90610a2b3bd72f3b4691f0529b0541702272b07f60d8ad5e781adf77474845b"
    ))
    .unwrap();
    let message = Message::from_digest(hex!(
        "ff3a9828e6127d5cc71e6898f6562538abf97f4de99544e1e64b517773612694"
    ));
    assert_eq!(
        PublicKey::from_secret_key(SECP256K1, &secret_key),
        PublicKey::from_secret_key(&reference, &secret_key),
    );
    assert_eq!(
        SECP256K1.sign_ecdsa_recoverable(&message, &secret_key),
        reference.sign_ecdsa_recoverable(&message, &secret_key),
    );

    let context = SECP256K1.ctx();
    SECP256K1.init(&hex!(
        "61d55a53c6728a04d62e7d37e9adab07102f73dc206b4f6087652e608eb8778a"
    ));
    assert_eq!(SECP256K1.ctx(), context);
}
