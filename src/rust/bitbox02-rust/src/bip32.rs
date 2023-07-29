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

#[derive(Clone)]
pub struct Xpub {
    xpub: pb::XPub,
}

impl core::convert::From<pb::XPub> for Xpub {
    fn from(xpub: pb::XPub) -> Self {
        Xpub { xpub }
    }
}

impl core::convert::From<&pb::XPub> for Xpub {
    fn from(xpub: &pb::XPub) -> Self {
        Xpub { xpub: xpub.clone() }
    }
}

impl core::convert::From<Xpub> for pb::XPub {
    fn from(xpub: Xpub) -> Self {
        xpub.xpub
    }
}

impl Xpub {
    /// Parses an 78-ytes xpub bytestring, encoded according to BIP32. The 4 version bytes are not
    /// checked and discarded.
    pub fn from_bytes(xpub: &[u8]) -> Result<Self, ()> {
        if xpub.len() != 78 {
            return Err(());
        }
        Ok(Self::from(pb::XPub {
            depth: xpub[4..5].to_vec(),
            parent_fingerprint: xpub[5..9].to_vec(),
            child_num: u32::from_be_bytes(core::convert::TryInto::try_into(&xpub[9..13]).unwrap()),
            chain_code: xpub[13..45].to_vec(),
            public_key: xpub[45..78].to_vec(),
        }))
    }

    /// Serializes a protobuf XPub to bytes according to the BIP32 specification. If xpub_type is
    /// None, the four version bytes are skipped.
    pub fn serialize(&self, xpub_type: Option<XPubType>) -> Result<Vec<u8>, ()> {
        let xpub = &self.xpub;
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
    pub fn serialize_str(&self, xpub_type: XPubType) -> Result<String, ()> {
        Ok(bitcoin::base58::encode_check(
            &self.serialize(Some(xpub_type))?,
        ))
    }

    /// Derives child xpub at the keypath. All keypath elements must be unhardened.
    pub fn derive(&self, keypath: &[u32]) -> Result<Self, ()> {
        let xpub_ser = self.serialize(Some(XPubType::Xpub))?;
        Xpub::from_bytes(&bitbox02::bip32::derive_xpub(&xpub_ser, keypath)?)
    }

    /// Returns the 33 bytes secp256k1 compressed pubkey.
    pub fn public_key(&self) -> &[u8] {
        self.xpub.public_key.as_slice()
    }

    /// Return the hash160 of the secp256k1 public key.
    pub fn pubkey_hash160(&self) -> Vec<u8> {
        bitbox02::hash160(self.public_key()).to_vec()
    }

    /// Return the 65 byte secp256k1 compressed pubkey:
    ///
    /// (<0x04><64 bytes X><64 bytes Y>).
    pub fn pubkey_uncompressed(&self) -> Result<[u8; 65], ()> {
        bitbox02::keystore::secp256k1_pubkey_compressed_to_uncompressed(self.public_key())
    }

    /// Return the tweaked taproot pubkey.
    ///
    /// Instead of returning the original pubkey at the keypath directly, it is tweaked with the
    /// hash of the pubkey.
    ///
    /// See
    /// https://github.com/bitcoin/bips/blob/edffe529056f6dfd33d8f716fb871467c3c09263/bip-0086.mediawiki#address-derivation
    pub fn schnorr_bip86_pubkey(&self) -> Result<[u8; 32], ()> {
        bitbox02::keystore::secp256k1_schnorr_bip86_pubkey(self.public_key())
    }
}

/// Parses a Base58Check-encoded xpub string. The 4 version bytes are not checked and discarded.
#[cfg(test)]
pub fn parse_xpub(xpub: &str) -> Result<pb::XPub, ()> {
    let decoded = bitcoin::base58::decode_check(xpub).or(Err(()))?;
    Ok(Xpub::from_bytes(&decoded)?.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_serialize_xpub() {
        let xpub = Xpub::from(parse_xpub("xpub6Eu7xJRyXRCi4eLYhJPnfZVjgAQtM7qFaEZwUhvgxGf4enEZMxevGzWvZTawCj9USP2MFTEhKQAwnqHwoaPHetTLqGuvq5r5uaLKyGx5QDZ").unwrap());
        assert_eq!(
            xpub.serialize(None).unwrap(),
            hex::decode("04b9d184d180000002b5b571ead68edac616c38491d9fd78d4697077e7675333452b586e3282705a3a0281bec7de8d182945744445948b54800e95267a5ac039bab6218a03b8e6f4b38a").unwrap(),
        );
        assert_eq!(
            xpub.serialize_str(XPubType::Tpub).unwrap().as_str(),
            "tpubDFGkUYFfEhAALSXQ9VNssUq71HWYLWLK7sAEqFyqJBQxQ4uGSBW1RSBkoVfijE6iEHZFs2kZrVzzV1nZCSEXYKudtsfEWcWKVXvjjLeRyd8",
        );
        assert_eq!(
            xpub.serialize_str(XPubType::Xpub).unwrap().as_str(),
            "xpub6Eu7xJRyXRCi4eLYhJPnfZVjgAQtM7qFaEZwUhvgxGf4enEZMxevGzWvZTawCj9USP2MFTEhKQAwnqHwoaPHetTLqGuvq5r5uaLKyGx5QDZ",
        );
        assert_eq!(
            xpub.serialize_str(XPubType::Ypub).unwrap().as_str(),
            "ypub6ZjPFy6tg6kBuwXfXfBQsebEr8ZLHjpkVM6AG6paLH2wht3nccpUu4B4afYXCdoPr299zvqFn4XVg7uWXGoJT88whccMQzfaBJPyMrVtPLb",
        );
        assert_eq!(
            xpub.serialize_str(XPubType::Zpub).unwrap().as_str(),
            "zpub6tZeZdmopnHfmEinN1y35jgk26hnEMpFQTcP3ViTiHQpkys1sGz3X7qCbsW7CYTKFfFxkQRpEit3ZQX5EyDKFMpYZxJmzuV4T2TckTXCeKB",
        );
        assert_eq!(
            xpub.serialize_str(XPubType::Vpub).unwrap().as_str(),
            "vpub5bEbLy69E47kN3xK2apYFPJjLE7zTsrFk1XVuv8vCFuJYac6reKo2sCeX3fmCuqdd6njkW3aQ5Tr2G4pNBZG4R696bX5fGD7N8D3CBtsPoq",
        );
        assert_eq!(
            xpub.serialize_str(XPubType::Upub).unwrap().as_str(),
            "upub5GQL3JRE5NaGWkmCCE2v3JDEAFyYXFrkpu1H8XF2pFXRVUnsbzAEQoYWVqiBD1BiDTfw12T1wR7J8yTFeV9FGBQYEFpf5MPd6Q9PoXnSeMz",
        );
        assert_eq!(
            xpub.serialize_str(XPubType::CapitalVpub).unwrap().as_str(),
            "Vpub5n8gUCpao1g7nd7gyFHX5TeY42AFgEXr4HBAqBQTa2jiAmB1d3i57z4aKkdEmM4XrZrid63hHHrM9RgafQiDCuCow4dV4fg7FrUun5qYdcZ",
        );
        assert_eq!(
            xpub.serialize_str(XPubType::CapitalZpub).unwrap().as_str(),
            "Zpub75TjgsWFPjr3BotAJgS1up2Yjtk3SiVqijG3xkz164FEPARvdgNKcEh8QaTakygDV8KwczRw7wGYga8qYCNGPqwDQRRBQJx4LkjVLLY6Yta",
        );
        assert_eq!(
            xpub.serialize_str(XPubType::CapitalUpub).unwrap().as_str(),
            "Upub5TJRAY9feL8dwKva8tVtsNZ2t41ojcYM9Aex3nWaC2Mq7fMnNPYWVvQSJYfemSQcSvjuscT8pdVoG951wiJCQfXD4iw4Ukrcz8RGPZ1BoeD",
        );
        assert_eq!(
            xpub.serialize_str(XPubType::CapitalYpub).unwrap().as_str(),
            "Ypub6kdUPCqLF4JZLWh3UKePhiw3ZvbbW6WLocjqBN67i3sML4chP2CkzB2zPNVzm52J5VD8sWqNfGuzoHXGpVxFbcFcY5ikpQ8a52fqwpAmYX2",
        );
    }

    #[test]
    fn test_derive() {
        let xpub_str = "xpub661MyMwAqRbcGpuMRXa55WgyqinF4dpxvqQK63xBHtnH5yK4e3cTLqbX9CP4mEMHUbqsjSQ8y3hhbAzuMhpn8eEiLNVSWYaVSbKMAtUPyYH";
        let xpub = Xpub::from(parse_xpub(xpub_str).unwrap());

        assert_eq!(
            xpub.derive(&[])
                .unwrap()
                .serialize_str(XPubType::Xpub)
                .unwrap(),
            xpub_str,
        );
        let expected = "xpub6CYiDoWMtLVQNrc4tbAvuRk5wjsp6MFgtYEdBUV7TGLUjutavHdEKLu9KpTpRxEZULbSwM1UQPaQpqAhmWYvngXCGHGE7hSZFNofeSRzmk5";
        assert_eq!(
            xpub.derive(&[0, 1, 2])
                .unwrap()
                .serialize_str(XPubType::Xpub)
                .unwrap(),
            expected,
        );

        assert!(xpub.derive(&[0, 1, util::bip32::HARDENED]).is_err());
    }

    #[test]
    fn test_pubkey_hash160() {
        let xpub = Xpub::from(parse_xpub("xpub6GugPDcUhrSudznFss7wXvQV3gwFTEanxHdCyoNoHnZEr3PTbh2Fosg4JjfphaYAsqjBhmtTZ3Yo8tmGjSHtaPhExNiMCSvPzreqjrX4Wr7").unwrap());
        assert_eq!(
            xpub.pubkey_hash160(),
            *b"\xb5\x12\x5c\xec\xa0\xc1\xc8\x90\xda\x07\x9a\x12\x88\xdc\xf7\x7a\xa6\xac\xc4\x99"
        );

        let xpub = Xpub::from(parse_xpub("xpub6FiMwSqu98LjKsbGy1PfgGRQA9XH7k6dfsyPedsyrdBRJDPwc658JA3qGc7DV2dWUYVGEqzRicztwzCj1NprQSRbSubWcnkKxM3Gwnyh4xo").unwrap());
        assert_eq!(
            xpub.pubkey_hash160(),
            *b"\xe5\xf8\x9a\xb6\x54\x37\x44\xf7\x8f\x15\x86\x7c\x43\x06\xee\x86\x6b\xb1\x1d\xf9"
        );
    }

    #[test]
    fn test_secp256k1_pubkey_uncompressed() {
        let xpub = Xpub::from(parse_xpub("xpub6FiMwSqu98LjKsbGy1PfgGRQA9XH7k6dfsyPedsyrdBRJDPwc658JA3qGc7DV2dWUYVGEqzRicztwzCj1NprQSRbSubWcnkKxM3Gwnyh4xo").unwrap());
        assert_eq!(
            xpub.pubkey_uncompressed().unwrap(),
            *b"\x04\x77\xa4\x4a\xa9\xe8\xc8\xfb\x51\x05\xef\x5e\xe2\x39\x4e\x8a\xed\x89\xad\x73\xfc\x74\x36\x14\x25\xf0\x63\x47\xec\xfe\x32\x61\x31\xe1\x33\x93\x67\xee\x3c\xbe\x87\x71\x92\x85\xa0\x7f\x77\x4b\x17\xeb\x93\x3e\xcf\x0b\x9b\x82\xac\xeb\xc1\x95\x22\x6d\x63\x42\x44",
        );
    }

    #[test]
    fn test_schnorr_bip86_pubkey() {
        // Test vectors from:
        // https://github.com/bitcoin/bips/blob/edffe529056f6dfd33d8f716fb871467c3c09263/bip-0086.mediawiki#test-vectors
        // Here we only test the creation of the tweaked pubkkey. See `Payload::from_simple` for address generation.

        let xpub = Xpub::from(parse_xpub("xpub6BgBgsespWvERF3LHQu6CnqdvfEvtMcQjYrcRzx53QJjSxarj2afYWcLteoGVky7D3UKDP9QyrLprQ3VCECoY49yfdDEHGCtMMj92pReUsQ").unwrap());

        assert_eq!(
            xpub.derive(&[0, 0]).unwrap().schnorr_bip86_pubkey().unwrap(),
            *b"\xa6\x08\x69\xf0\xdb\xcf\x1d\xc6\x59\xc9\xce\xcb\xaf\x80\x50\x13\x5e\xa9\xe8\xcd\xc4\x87\x05\x3f\x1d\xc6\x88\x09\x49\xdc\x68\x4c",
        );

        assert_eq!(
            xpub.derive(&[0, 1]).unwrap().schnorr_bip86_pubkey().unwrap(),
            *b"\xa8\x2f\x29\x94\x4d\x65\xb8\x6a\xe6\xb5\xe5\xcc\x75\xe2\x94\xea\xd6\xc5\x93\x91\xa1\xed\xc5\xe0\x16\xe3\x49\x8c\x67\xfc\x7b\xbb",
        );

        assert_eq!(
            xpub.derive(&[1, 0]).unwrap().schnorr_bip86_pubkey().unwrap(),
            *b"\x88\x2d\x74\xe5\xd0\x57\x2d\x5a\x81\x6c\xef\x00\x41\xa9\x6b\x6c\x1d\xe8\x32\xf6\xf9\x67\x6d\x96\x05\xc4\x4d\x5e\x9a\x97\xd3\xdc",
        );
    }
}
