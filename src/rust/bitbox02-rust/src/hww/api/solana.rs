// SPDX-License-Identifier: Apache-2.0

#[cfg(not(feature = "app-solana"))]
compile_error!("Solana code is being compiled even though the app-solana feature is not enabled");

mod address;
pub mod keypath;
mod params;
mod sign_transaction;

use super::Error;
use super::pb;

use pb::solana_request::Request;
use pb::solana_response::Response;

pub(crate) fn derive_pubkey(
    hal: &mut impl crate::hal::Hal,
    keypath: &[u32],
) -> Result<[u8; 32], Error> {
    if !keypath::is_valid_keypath(keypath) {
        return Err(Error::InvalidInput);
    }
    Ok(*crate::keystore::ed25519::get_xpub_twice(hal, keypath)
        .or(Err(Error::InvalidInput))?
        .pubkey_bytes())
}

pub async fn process_api(
    hal: &mut impl crate::hal::Hal,
    request: &Request,
) -> Result<Response, Error> {
    match request {
        Request::Pub(request) => address::process(hal, request).await,
        Request::SignTransaction(request) => sign_transaction::process(hal, request).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::testing::TestingHal;
    use crate::keystore::testing::mock_unlocked;
    use util::bip32::HARDENED;

    #[test]
    fn test_derive_pubkey() {
        mock_unlocked();
        let pubkey = derive_pubkey(
            &mut TestingHal::new(),
            &[44 + HARDENED, 501 + HARDENED, HARDENED, HARDENED],
        )
        .unwrap();
        assert!(!address::from_pubkey(&pubkey).is_empty());
    }
}
