// SPDX-License-Identifier: Apache-2.0

#[cfg(not(feature = "app-ethereum"))]
compile_error!(
    "Ethereum code is being compiled even though the app-ethereum feature is not enabled"
);

mod address;
mod amount;
mod keypath;
mod params;
mod pubrequest;
mod sighash;
mod sign;
mod sign_typed_msg;
mod signmsg;

use super::Error;
use super::pb;

use pb::eth_request::Request;
use pb::eth_response::Response;

use core::convert::TryInto;

pub(crate) fn derive_address(
    hal: &mut impl crate::hal::Hal,
    keypath: &[u32],
) -> Result<alloc::string::String, Error> {
    if !keypath::is_valid_keypath_address(keypath) {
        return Err(Error::InvalidInput);
    }
    let pubkey = crate::keystore::get_xpub_twice(hal, keypath)
        .or(Err(Error::InvalidInput))?
        .pubkey_uncompressed()?;
    Ok(address::from_pubkey(&pubkey))
}

/// Like `hww::next_request`, but for Ethereum requests/responses.
pub async fn next_request(response: Response) -> Result<Request, Error> {
    let request = crate::hww::next_request(pb::response::Response::Eth(pb::EthResponse {
        response: Some(response),
    }))
    .await?;
    match request {
        pb::request::Request::Eth(pb::EthRequest {
            request: Some(request),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

/// Sends the `signer_nonce_commitment` to the host and waits for the next request, which has to be a
/// `AntiKleptoSignatureRequest` message containing the host nonce.
pub async fn antiklepto_get_host_nonce(
    signer_nonce_commitment: [u8; 33],
) -> Result<[u8; 32], Error> {
    let request = next_request(Response::AntikleptoSignerCommitment(
        pb::AntiKleptoSignerCommitment {
            commitment: signer_nonce_commitment.to_vec(),
        },
    ))
    .await?;
    match request {
        Request::AntikleptoSignature(pb::AntiKleptoSignatureRequest { host_nonce }) => {
            Ok(host_nonce
                .as_slice()
                .try_into()
                .or(Err(Error::InvalidInput))?)
        }
        _ => Err(Error::InvalidState),
    }
}

/// Handle a Ethereum protobuf api call.
///
/// Returns `None` if the call was not handled by Rust, in which case it should be handled by
/// the C commander.
pub async fn process_api(
    hal: &mut impl crate::hal::Hal,
    request: &Request,
) -> Result<Response, Error> {
    match request {
        Request::Pub(request) => pubrequest::process(hal, request).await,
        Request::SignMsg(request) => signmsg::process(hal, request).await,
        Request::Sign(request) => sign::process(hal, &sign::Transaction::Legacy(request)).await,
        Request::SignEip1559(request) => {
            sign::process(hal, &sign::Transaction::Eip1559(request)).await
        }
        Request::SignTypedMsg(request) => sign_typed_msg::process(hal, request).await,
        // These are streamed asynchronously using the `next_request()` primitive
        Request::AntikleptoSignature(_)
        | Request::TypedMsgValue(_)
        | Request::DataResponseChunk(_) => Err(Error::InvalidState),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::testing::TestingHal;
    use util::bip32::HARDENED;

    #[test]
    fn test_derive_address() {
        let mut hal = TestingHal::new();

        // Standard Ethereum keypath
        let keypath = vec![44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
        let address = derive_address(&mut hal, &keypath).unwrap();

        // This is the expected address for the mock keystore seed with this keypath
        assert_eq!(address, "0x773A77b9D32589be03f9132AF759e294f7851be9");
    }

    #[test]
    fn test_derive_address_invalid_keypath() {
        let mut hal = TestingHal::new();

        // Invalid keypath (too short)
        let keypath = vec![44 + HARDENED, 60 + HARDENED];
        let result = derive_address(&mut hal, &keypath);

        assert!(matches!(result, Err(Error::InvalidInput)));
    }
}
