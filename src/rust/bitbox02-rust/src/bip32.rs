// Copyright 2022 Shift Crypto AG
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

use super::pb;
use alloc::string::String;
use alloc::vec::Vec;

pub use pb::btc_pub_request::XPubType;

/// Serializes a protobuf XPub to bytes according to the BIP32 specification. If xpub_type is None,
/// the four version bytes are skipped.
pub fn serialize_xpub(xpub: &pb::XPub, xpub_type: Option<XPubType>) -> Result<Vec<u8>, ()> {
    if xpub.depth.len() != 1
        || xpub.parent_fingerprint.len() != 4
        || xpub.chain_code.len() != 32
        || xpub.public_key.len() != 33
    {
        return Err(());
    }

    // Version bytes for mainnet public, see BIP32.
    let mut result: Vec<u8> = Vec::new();
    if let Some(xpub_type) = xpub_type {
        let version = match xpub_type {
            XPubType::Tpub => b"\x04\x35\x87\xcf",
            XPubType::Xpub => b"\x04\x88\xb2\x1e",
            XPubType::Ypub => b"\x04\x9d\x7c\xb2",
            XPubType::Zpub => b"\x04\xb2\x47\x46",
            XPubType::Vpub => b"\x04\x5f\x1c\xf6",
            XPubType::Upub => b"\x04\x4a\x52\x62",
            XPubType::CapitalVpub => b"\x02\x57\x54\x83",
            XPubType::CapitalZpub => b"\x02\xaa\x7e\xd3",
            XPubType::CapitalUpub => b"\x02\x42\x89\xef",
            XPubType::CapitalYpub => b"\x02\x95\xb4\x3f",
        };
        result.extend_from_slice(version);
    }
    result.extend_from_slice(&xpub.depth);
    result.extend_from_slice(&xpub.parent_fingerprint);
    result.extend_from_slice(&xpub.child_num.to_be_bytes());
    result.extend_from_slice(&xpub.chain_code);
    result.extend_from_slice(&xpub.public_key);
    Ok(result)
}

/// Serialize an xpub as a Base58Check encoded string according to BIP32.
pub fn serialize_xpub_str(xpub: &pb::XPub, xpub_type: XPubType) -> Result<String, ()> {
    Ok(bs58::encode(serialize_xpub(xpub, Some(xpub_type))?)
        .with_check()
        .into_string())
}

#[cfg(test)]
pub fn parse_xpub(xpub: &str) -> Result<pb::XPub, ()> {
    let decoded = bs58::decode(xpub).into_vec().or(Err(()))?;
    Ok(pb::XPub {
        depth: decoded[4..5].to_vec(),
        parent_fingerprint: decoded[5..9].to_vec(),
        child_num: u32::from_be_bytes(core::convert::TryInto::try_into(&decoded[9..13]).unwrap()),
        chain_code: decoded[13..45].to_vec(),
        public_key: decoded[45..78].to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_serialize_xpub() {
        let xpub = parse_xpub("xpub6Eu7xJRyXRCi4eLYhJPnfZVjgAQtM7qFaEZwUhvgxGf4enEZMxevGzWvZTawCj9USP2MFTEhKQAwnqHwoaPHetTLqGuvq5r5uaLKyGx5QDZ").unwrap();
        assert_eq!(
            serialize_xpub(&xpub, None).unwrap(),
            hex::decode("04b9d184d180000002b5b571ead68edac616c38491d9fd78d4697077e7675333452b586e3282705a3a0281bec7de8d182945744445948b54800e95267a5ac039bab6218a03b8e6f4b38a").unwrap(),
        );
        assert_eq!(
            serialize_xpub_str(&xpub, XPubType::Tpub).unwrap().as_str(),
            "tpubDFGkUYFfEhAALSXQ9VNssUq71HWYLWLK7sAEqFyqJBQxQ4uGSBW1RSBkoVfijE6iEHZFs2kZrVzzV1nZCSEXYKudtsfEWcWKVXvjjLeRyd8",
        );
        assert_eq!(
            serialize_xpub_str(&xpub, XPubType::Xpub).unwrap().as_str(),
            "xpub6Eu7xJRyXRCi4eLYhJPnfZVjgAQtM7qFaEZwUhvgxGf4enEZMxevGzWvZTawCj9USP2MFTEhKQAwnqHwoaPHetTLqGuvq5r5uaLKyGx5QDZ",
        );
        assert_eq!(
            serialize_xpub_str(&xpub, XPubType::Ypub).unwrap().as_str(),
            "ypub6ZjPFy6tg6kBuwXfXfBQsebEr8ZLHjpkVM6AG6paLH2wht3nccpUu4B4afYXCdoPr299zvqFn4XVg7uWXGoJT88whccMQzfaBJPyMrVtPLb",
        );
        assert_eq!(
            serialize_xpub_str(&xpub, XPubType::Zpub).unwrap().as_str(),
            "zpub6tZeZdmopnHfmEinN1y35jgk26hnEMpFQTcP3ViTiHQpkys1sGz3X7qCbsW7CYTKFfFxkQRpEit3ZQX5EyDKFMpYZxJmzuV4T2TckTXCeKB",
        );
        assert_eq!(
            serialize_xpub_str(&xpub, XPubType::Vpub).unwrap().as_str(),
            "vpub5bEbLy69E47kN3xK2apYFPJjLE7zTsrFk1XVuv8vCFuJYac6reKo2sCeX3fmCuqdd6njkW3aQ5Tr2G4pNBZG4R696bX5fGD7N8D3CBtsPoq",
        );
        assert_eq!(
            serialize_xpub_str(&xpub, XPubType::Upub).unwrap().as_str(),
            "upub5GQL3JRE5NaGWkmCCE2v3JDEAFyYXFrkpu1H8XF2pFXRVUnsbzAEQoYWVqiBD1BiDTfw12T1wR7J8yTFeV9FGBQYEFpf5MPd6Q9PoXnSeMz",
        );
        assert_eq!(
            serialize_xpub_str(&xpub, XPubType::CapitalVpub).unwrap().as_str(),
            "Vpub5n8gUCpao1g7nd7gyFHX5TeY42AFgEXr4HBAqBQTa2jiAmB1d3i57z4aKkdEmM4XrZrid63hHHrM9RgafQiDCuCow4dV4fg7FrUun5qYdcZ",
        );
        assert_eq!(
            serialize_xpub_str(&xpub, XPubType::CapitalZpub).unwrap().as_str(),
            "Zpub75TjgsWFPjr3BotAJgS1up2Yjtk3SiVqijG3xkz164FEPARvdgNKcEh8QaTakygDV8KwczRw7wGYga8qYCNGPqwDQRRBQJx4LkjVLLY6Yta",
        );
        assert_eq!(
            serialize_xpub_str(&xpub, XPubType::CapitalUpub).unwrap().as_str(),
            "Upub5TJRAY9feL8dwKva8tVtsNZ2t41ojcYM9Aex3nWaC2Mq7fMnNPYWVvQSJYfemSQcSvjuscT8pdVoG951wiJCQfXD4iw4Ukrcz8RGPZ1BoeD",
        );
        assert_eq!(
            serialize_xpub_str(&xpub, XPubType::CapitalYpub).unwrap().as_str(),
            "Ypub6kdUPCqLF4JZLWh3UKePhiw3ZvbbW6WLocjqBN67i3sML4chP2CkzB2zPNVzm52J5VD8sWqNfGuzoHXGpVxFbcFcY5ikpQ8a52fqwpAmYX2",
        );
    }
}
