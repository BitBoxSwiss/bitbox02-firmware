use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

// Our crate:
use bip32_ed25519::{Xprv, ED25519_EXPANDED_SECRET_KEY_SIZE};

// Another crate for comparison:
use ed25519_bip32;

fn mk_other_xprv(
    key: &[u8; ED25519_EXPANDED_SECRET_KEY_SIZE],
    chain_code: &[u8; 32],
) -> ed25519_bip32::XPrv {
    let mut k = [0u8; ed25519_bip32::XPRV_SIZE];
    k[..64].copy_from_slice(key);
    k[64..].copy_from_slice(chain_code);
    ed25519_bip32::XPrv::normalize_bytes_force3rd(k)
}

#[derive(Debug, Deserialize)]
struct PrivateDerivationTest {
    path: Vec<u32>,
    #[serde(with = "hex")]
    expected_kl: [u8; 32],
    #[serde(with = "hex")]
    expected_kr: [u8; 32],
    #[serde(with = "hex")]
    expected_chain_code: [u8; 32],
    #[serde(with = "hex")]
    expected_public_key: [u8; 32],
}
#[derive(Debug, Deserialize)]
struct PublicDerivationTest {
    path: Vec<u32>,
    #[serde(with = "hex")]
    expected_public_key: [u8; 32],
    #[serde(with = "hex")]
    expected_chain_code: [u8; 32],
}
#[derive(Debug, Deserialize)]
struct Test {
    #[serde(with = "hex")]
    kl: [u8; 32],
    #[serde(with = "hex")]
    kr: [u8; 32],
    #[serde(with = "hex")]
    chain_code: [u8; 32],

    private_derivations: Vec<PrivateDerivationTest>,
    public_derivations: Vec<PublicDerivationTest>,
}

/// Tests private and public key derivations using a fixture.
/// The fixture is generated using testdata/gen_table.py
#[test]
fn table_test() {
    println!("Loading table");
    let reader = BufReader::new(File::open("./tests/testdata/table.json").unwrap());
    let tests: Vec<Test> = serde_json::from_reader(reader).unwrap();
    println!("Running {} tests", tests.len());
    for (i, test) in tests.iter().enumerate() {
        println!("Running test #{}", i);
        let mut k = [0u8; ED25519_EXPANDED_SECRET_KEY_SIZE];
        k[..32].copy_from_slice(&test.kl);
        k[32..].copy_from_slice(&test.kr);
        for derivation in test.private_derivations.iter() {
            let mut xprv = Xprv::<sha2::Sha512>::from_normalize(&k, &test.chain_code);
            let mut xprv_other = mk_other_xprv(&k, &test.chain_code);
            for index in derivation.path.iter() {
                xprv = xprv.derive(*index);
                xprv_other = xprv_other.derive(ed25519_bip32::DerivationScheme::V2, *index);
            }
            // Check our implementation:
            assert_eq!(derivation.expected_kl, &xprv.expanded_secret_key()[..32]);
            assert_eq!(derivation.expected_kr, &xprv.expanded_secret_key()[32..]);
            assert_eq!(&derivation.expected_chain_code, xprv.chain_code());
            assert_eq!(
                &derivation.expected_public_key,
                xprv.public().pubkey_bytes(),
            );

            // Check other implementation:
            assert_eq!(
                derivation.expected_kl,
                xprv_other.extended_secret_key_slice()[..32],
            );
            assert_eq!(
                derivation.expected_kr,
                xprv_other.extended_secret_key_slice()[32..],
            );
            assert_eq!(
                &derivation.expected_chain_code,
                xprv_other.chain_code_slice(),
            );
            assert_eq!(
                &derivation.expected_public_key,
                xprv_other.public().public_key_slice(),
            );
        }
        for derivation in test.public_derivations.iter() {
            let mut xpub = Xprv::<sha2::Sha512>::from_normalize(&k, &test.chain_code).public();
            let mut xpub_other = mk_other_xprv(&k, &test.chain_code).public();
            for index in derivation.path.iter() {
                xpub = xpub.derive(*index).unwrap();
                xpub_other = xpub_other
                    .derive(ed25519_bip32::DerivationScheme::V2, *index)
                    .unwrap();
            }

            // Check our implementation:
            assert_eq!(&derivation.expected_chain_code, xpub.chain_code());
            assert_eq!(&derivation.expected_public_key, xpub.pubkey_bytes());

            // Check other implementation:
            assert_eq!(
                &derivation.expected_chain_code,
                xpub_other.chain_code_slice()
            );
            assert_eq!(
                &derivation.expected_public_key,
                xpub_other.public_key_slice()
            );
        }
    }
}
