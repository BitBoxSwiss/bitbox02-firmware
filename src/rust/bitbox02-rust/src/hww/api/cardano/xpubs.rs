// SPDX-License-Identifier: Apache-2.0

use super::Error;
use super::pb;

use alloc::vec::Vec;

use pb::cardano_response::Response;

use super::keypath::validate_account_shelley;

/// Return the xpub at the request keypath.
///
/// 64 bytes: 32 bytes public key + 32 bytes chain code.
pub async fn process(
    hal: &mut impl crate::hal::Hal,
    request: &pb::CardanoXpubsRequest,
) -> Result<Response, Error> {
    let mut xpubs: Vec<Vec<u8>> = Vec::with_capacity(request.keypaths.len());
    for pb::Keypath { keypath } in &request.keypaths {
        validate_account_shelley(keypath)?;

        let xpub = crate::keystore::ed25519::get_xpub_twice(hal, keypath).await?;
        let mut xpub_bytes = Vec::with_capacity(64);
        xpub_bytes.extend_from_slice(xpub.pubkey_bytes());
        xpub_bytes.extend_from_slice(xpub.chain_code());
        xpubs.push(xpub_bytes);
    }
    Ok(Response::Xpubs(pb::CardanoXpubsResponse { xpubs }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::keystore::testing::mock_unlocked;
    use hex_lit::hex;
    use util::bip32::HARDENED;

    #[async_test::test]
    async fn test_process() {
        crate::keystore::lock();
        assert_eq!(
            process(
                &mut crate::hal::testing::TestingHal::new(),
                &pb::CardanoXpubsRequest { keypaths: vec![] }
            )
            .await,
            Ok(Response::Xpubs(pb::CardanoXpubsResponse { xpubs: vec![] })),
        );

        // Locked.
        assert_eq!(
            process(
                &mut crate::hal::testing::TestingHal::new(),
                &pb::CardanoXpubsRequest {
                    keypaths: vec![pb::Keypath {
                        keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED]
                    }],
                }
            )
            .await,
            Err(Error::Generic),
        );

        mock_unlocked();
        assert_eq!(
            process(
                &mut crate::hal::testing::TestingHal::new(),
                &pb::CardanoXpubsRequest {
                    keypaths: vec![
                        pb::Keypath {
                            keypath: vec![1852 + HARDENED, 1815 + HARDENED, HARDENED]
                        },
                        pb::Keypath {
                            keypath: vec![1852 + HARDENED, 1815 + HARDENED, 1 + HARDENED]
                        }
                    ],
                }
            )
            .await,
            Ok(Response::Xpubs(pb::CardanoXpubsResponse {
                xpubs: vec![
                    hex!(
                        "875d15a5b1eaeb724bd93d6d36cb4b61bc45dbba78db9cb08b93e7289259d3d8aedf6401c51f2d981b017fd70435e2d9dfa0d74e7cce4b92061dfb088b5f08ce"
                    )
                    .to_vec(),
                    hex!(
                        "cdd998bb3f95231a7348eadfc0f8974d14ddd39e47bd3c281ad996967a31817e5dc7f05be2d4da6a1d1924b2819200b8710416e12efa01c04d15dca7ead7bfe9"
                    )
                    .to_vec(),
                ],
            })),
        );

        // Invalid keypaths
        let invalid_keypaths: &[&[u32]] = &[
            // Invalid purpose
            &[1851 + HARDENED, 1815 + HARDENED, HARDENED],
            // Invalid coin
            &[1852 + HARDENED, 1814 + HARDENED, HARDENED],
            // Account too low
            &[1852 + HARDENED, 1815 + HARDENED, 0],
            &[1852 + HARDENED, 1815 + HARDENED, HARDENED - 1],
            // Account too high
            &[1852 + HARDENED, 1815 + HARDENED, 100 + HARDENED],
            // Wrong number of elements (too short)
            &[1852 + HARDENED, 1815 + HARDENED],
            // Wrong number of elements (too long)
            &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0],
        ];
        for invalid_keypath in invalid_keypaths {
            assert_eq!(
                process(
                    &mut crate::hal::testing::TestingHal::new(),
                    &pb::CardanoXpubsRequest {
                        keypaths: vec![pb::Keypath {
                            keypath: invalid_keypath.to_vec(),
                        },],
                    }
                )
                .await,
                Err(Error::InvalidInput),
            );
        }
    }
}
