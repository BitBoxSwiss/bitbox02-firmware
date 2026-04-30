// SPDX-License-Identifier: Apache-2.0

#[cfg(not(feature = "app-ethereum"))]
compile_error!(
    "Ethereum code is being compiled even though the app-ethereum feature is not enabled"
);

mod address;
mod amount;
mod keypath;
pub mod params;
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

// Keep this in sync with src/ui/components/label.h:MAX_LABEL_SIZE. `MAX_CONFIRM_BODY_SIZE` is the
// effective confirmation body limit and intentionally matches that UI label size limit.
const MAX_CONFIRM_BODY_SIZE: usize = 640;

/// Returns how many bytes of hex-encoded data to include in a preview body.
///
/// The preview body is rendered as `<prefix><hex>`. If the full value is longer than what fits,
/// one additional byte is included so the body exceeds `MAX_CONFIRM_BODY_SIZE` and the UI appends
/// `...`.
fn truncating_hex_preview_byte_cap(prefix_len: usize, data_length: usize) -> usize {
    let hex_chars_budget = MAX_CONFIRM_BODY_SIZE.saturating_sub(prefix_len);
    let bytes_that_fit = hex_chars_budget / 2;
    let needs_ellipsis = data_length > bytes_that_fit;
    let preview_bytes = bytes_that_fit + usize::from(needs_ellipsis);

    preview_bytes.min(data_length)
}

pub(crate) async fn derive_address(
    hal: &mut impl crate::hal::Hal,
    keypath: &[u32],
) -> Result<alloc::string::String, Error> {
    if !keypath::is_valid_keypath_address(keypath) {
        return Err(Error::InvalidInput);
    }
    let pubkey = crate::keystore::get_xpub(hal, keypath, crate::keystore::Compute::Twice)
        .await
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

    #[async_test::test]
    async fn test_derive_address() {
        let mut hal = TestingHal::new();

        // Standard Ethereum keypath
        let keypath = vec![44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
        let address = derive_address(&mut hal, &keypath).await.unwrap();

        // This is the expected address for the mock keystore seed with this keypath
        assert_eq!(address, "0x773A77b9D32589be03f9132AF759e294f7851be9");
    }

    #[async_test::test]
    async fn test_derive_address_invalid_keypath() {
        let mut hal = TestingHal::new();

        // Invalid keypath (too short)
        let keypath = vec![44 + HARDENED, 60 + HARDENED];
        let result = derive_address(&mut hal, &keypath).await;

        assert!(matches!(result, Err(Error::InvalidInput)));
    }

    #[test]
    fn test_transaction_data_display_byte_cap() {
        assert_eq!(truncating_hex_preview_byte_cap(0, 320), 320);
        assert_eq!(truncating_hex_preview_byte_cap(0, 321), 321);
        assert_eq!(truncating_hex_preview_byte_cap(0, 10_000), 321);
    }
}
