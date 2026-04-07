// SPDX-License-Identifier: Apache-2.0

//! This module implements signing of structured data according to EIP-712:
//! https://eips.ethereum.org/EIPS/eip-712#specification
//!
//! It implements the same functionality as
//! https://github.com/MetaMask/eth-sig-util/blob/v4.0.1/src/sign-typed-data.ts
//! using SignTypedDataVersion.V4.

use super::Error;
use super::MAX_CONFIRM_BODY_SIZE;
use super::pb;
use super::sighash::DataProducer;
use super::truncating_hex_preview_byte_cap;

use crate::hal::Ui;
use crate::hal::ui::ConfirmParams;
use crate::keystore;

use pb::eth_request::Request;
use pb::eth_response::Response;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use sha3::digest::Digest;

use num_bigint::{BigInt, BigUint};

use pb::eth_sign_typed_message_request::{DataType, MemberType, StructType};
use pb::eth_typed_message_value_response::RootObject;

const DOMAIN_TYPE_NAME: &str = "EIP712Domain";

const MAX_TYPED_MSG_STREAMING_DATA_LENGTH: u32 = 1024 * 1024;

fn get_type<'a>(types: &'a [StructType], name: &str) -> Option<&'a StructType> {
    types.iter().find(|t| t.name == name)
}

fn get_transitive_types<'a>(types: &'a [StructType], name: &'a str) -> Result<Vec<&'a str>, Error> {
    fn rec<'a>(
        types: &'a [StructType],
        name: &'a str,
        result: &mut Vec<&'a str>,
    ) -> Result<(), Error> {
        let typ = get_type(types, name).ok_or(Error::InvalidInput)?;
        if result.contains(&name) {
            return Ok(());
        }
        result.push(name);
        for member in typ.members.iter() {
            let mut member_type = member.r#type.as_ref().ok_or(Error::InvalidInput)?;
            while member_type.r#type == DataType::Array as i32 {
                member_type = member_type.array_type.as_ref().ok_or(Error::InvalidInput)?;
            }
            if member_type.r#type == DataType::Struct as i32 {
                rec(types, &member_type.struct_name, result)?;
            }
        }
        Ok(())
    }
    let mut result = Vec::new();
    rec(types, name, &mut result)?;
    Ok(result)
}

fn format_member_type(typ: &MemberType) -> Result<String, Error> {
    let formatted = match DataType::try_from(typ.r#type)? {
        DataType::Unknown => return Err(Error::InvalidInput),
        DataType::Bytes => {
            if typ.size == 0 {
                "bytes".into()
            } else if typ.size >= 1 && typ.size <= 32 {
                format!("bytes{}", typ.size)
            } else {
                return Err(Error::InvalidInput);
            }
        }
        DataType::Uint => {
            if typ.size < 1 || typ.size > 32 {
                return Err(Error::InvalidInput);
            }
            format!("uint{}", typ.size * 8)
        }
        DataType::Int => {
            if typ.size < 1 || typ.size > 32 {
                return Err(Error::InvalidInput);
            }
            format!("int{}", typ.size * 8)
        }
        DataType::Bool => "bool".into(),
        DataType::Address => "address".into(),
        DataType::String => "string".into(),
        DataType::Array => {
            let name = format_member_type(typ.array_type.as_ref().ok_or(Error::InvalidInput)?)?;
            if typ.size == 0 {
                format!("{}[]", name)
            } else {
                format!("{}[{}]", name, typ.size)
            }
        }
        DataType::Struct => {
            if typ.struct_name.is_empty() {
                return Err(Error::InvalidInput);
            }
            typ.struct_name.clone()
        }
    };
    Ok(formatted)
}

/// https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype
fn encode_type(types: &[StructType], name: &str) -> Result<String, Error> {
    let mut transitive_types = get_transitive_types(types, name)?;
    // First element contains the name of the type to encode - sort the rest of the types.
    transitive_types[1..].sort_unstable();
    Ok(transitive_types
        .iter()
        .map(|name| -> Result<String, Error> {
            let typ = get_type(types, name).ok_or(Error::InvalidInput)?;
            let params = typ
                .members
                .iter()
                .map(|member| -> Result<String, Error> {
                    Ok(format!(
                        "{} {}",
                        format_member_type(member.r#type.as_ref().ok_or(Error::InvalidInput)?)?,
                        member.name
                    ))
                })
                .collect::<Result<Vec<String>, Error>>()?
                .join(",");
            Ok(format!("{}({})", name, params))
        })
        .collect::<Result<Vec<String>, Error>>()?
        .join(""))
}

fn leftpad32(v: &[u8], signed: bool) -> Result<Vec<u8>, Error> {
    if v.len() > 32 {
        return Err(Error::InvalidInput);
    }
    let mut result = Vec::new();
    result.resize(
        32 - v.len(),
        if signed && !v.is_empty() && v[0] & 0x80u8 != 0 {
            0xff
        } else {
            0x00
        },
    );
    result.extend(v);
    Ok(result)
}

fn rightpad32(v: &[u8]) -> Result<Vec<u8>, Error> {
    if v.len() > 32 {
        return Err(Error::InvalidInput);
    }
    let mut result = v.to_vec();
    result.resize(32, 0);
    Ok(result)
}

fn type_hash(types: &[StructType], name: &str) -> Result<Vec<u8>, Error> {
    let encoded = encode_type(types, name)?;
    Ok(sha3::Keccak256::digest(encoded.as_bytes())
        .as_slice()
        .to_vec())
}

async fn get_value_from_host(
    root_object: RootObject,
    path: &[u32],
) -> Result<pb::EthTypedMessageValueRequest, Error> {
    let request = super::next_request(Response::TypedMsgValue(pb::EthTypedMessageValueResponse {
        root_object: root_object as _,
        path: path.to_vec(),
    }))
    .await?;
    match request {
        Request::TypedMsgValue(req) => Ok(req),
        _ => Err(Error::InvalidInput),
    }
}

/// https://eips.ethereum.org/EIPS/eip-712#definition-of-encodedata for all except structs and arrays.
///
/// The value is an encoding of the member value sent by the host.
///
/// Returns the 32 byte encoded value as well as a human readable representation that can be used
/// for user verification.
fn encode_value(typ: &MemberType, value: Vec<u8>) -> Result<(Vec<u8>, String), Error> {
    let result = match DataType::try_from(typ.r#type)? {
        DataType::Unknown => return Err(Error::InvalidInput),
        DataType::Bytes => {
            let encoded = if typ.size > 0 {
                if value.len() != typ.size as usize {
                    return Err(Error::InvalidInput);
                }
                rightpad32(&value)?
            } else {
                sha3::Keccak256::digest(&value).as_slice().to_vec()
            };
            (encoded, format!("0x{}", hex::encode(&value)))
        }
        DataType::Uint => {
            if value.len() > typ.size as usize {
                return Err(Error::InvalidInput);
            }
            (
                leftpad32(&value, false)?,
                BigUint::from_bytes_be(&value).to_string(),
            )
        }
        DataType::Int => {
            if value.len() > typ.size as usize {
                return Err(Error::InvalidInput);
            }
            (
                leftpad32(&value, true)?,
                BigInt::from_signed_bytes_be(&value).to_string(),
            )
        }
        DataType::Bool => {
            if value != b"\x00" && value != b"\x01" {
                return Err(Error::InvalidInput);
            }
            (
                leftpad32(&value, false)?,
                if value == b"\x00" {
                    "false".into()
                } else {
                    "true".into()
                },
            )
        }
        DataType::Address => {
            // The address is sent as a string by the host, so we can display it in the same way as
            // it is displayed on the host (mixed case vs lowercase, etc.).
            let encoded = if let [b'0', b'x' | b'X', rest @ ..] = value.as_slice() {
                leftpad32(&hex::decode(rest).or(Err(Error::InvalidInput))?, false)?
            } else {
                return Err(Error::InvalidInput);
            };
            let value_str = String::from_utf8(value).or(Err(Error::InvalidInput))?;
            (encoded, super::address::format_display_address(&value_str))
        }
        DataType::String => {
            if !util::ascii::is_printable_ascii(&value, util::ascii::Charset::AllNewline) {
                return Err(Error::InvalidInput);
            }
            (
                sha3::Keccak256::digest(&value).as_slice().to_vec(),
                String::from_utf8(value).or(Err(Error::InvalidInput))?,
            )
        }
        DataType::Array | DataType::Struct => panic!("encode_value"),
    };
    Ok(result)
}

fn confirm_title(root_object: RootObject) -> &'static str {
    match root_object {
        RootObject::Unknown => "Unknown",
        RootObject::Domain => "Domain",
        RootObject::Message => "Message",
    }
}

fn format_display_line_prefix(display_path: &str, line_num: usize, num_lines: usize) -> String {
    if num_lines > 1 {
        format!("{display_path}, line {}/{}", line_num + 1, num_lines)
    } else {
        display_path.to_string()
    }
}

fn format_display_line_body(
    display_path: &str,
    line_num: usize,
    num_lines: usize,
    line: &str,
) -> String {
    format!(
        "{}: {}",
        format_display_line_prefix(display_path, line_num, num_lines),
        line
    )
}

#[allow(clippy::too_many_arguments)]
async fn encode_member<U: sha3::digest::Update>(
    hal: &mut impl crate::hal::Hal,
    hasher: &mut U,
    types: &[StructType],
    member_type: &MemberType,
    root_object: RootObject,
    path: &[u32],
    formatted_path: &[String],
    title_suffix: Option<String>,
) -> Result<(), Error> {
    if member_type.r#type == DataType::Struct as i32 {
        let value_encoded = Box::pin(hash_struct(
            hal,
            types,
            root_object,
            &member_type.struct_name,
            path,
            formatted_path,
            title_suffix,
        ))
        .await?;
        hasher.update(&value_encoded);
    } else if member_type.r#type == DataType::Array as i32 {
        let encoded_value = Box::pin(hash_array(
            hal,
            types,
            member_type,
            root_object,
            path,
            formatted_path,
            title_suffix,
        ))
        .await?;
        hasher.update(&encoded_value);
    } else {
        let req = get_value_from_host(root_object, path).await?;
        let display_path = formatted_path.join(".");
        let data_type = DataType::try_from(member_type.r#type)?;
        let mut display_size = 0usize;

        let value_formatted = if req.data_length > 0 {
            // Streaming mode: only dynamic bytes may stream.
            if data_type != DataType::Bytes || member_type.size != 0 {
                return Err(Error::InvalidInput);
            }
            if !req.value.is_empty() {
                return Err(Error::InvalidInput);
            }
            if req.data_length > MAX_TYPED_MSG_STREAMING_DATA_LENGTH {
                return Err(Error::InvalidInput);
            }

            display_size = req.data_length as usize;
            let display_cap = truncating_hex_preview_byte_cap(
                format!("{}: 0x", format_display_line_prefix(&display_path, 0, 1)).len(),
                display_size,
            );
            let mut producer = super::sighash::ChunkingProducer::from_host(req.data_length)
                .with_preview(display_cap);
            let mut keccak = sha3::Keccak256::new();
            while let Some(chunk) = producer.next().await? {
                keccak.update(&chunk);
            }
            hasher.update(&keccak.finalize());

            format!("0x{}", hex::encode(producer.preview()))
        } else {
            let value_len = req.value.len();
            let (value_encoded, fmt) = encode_value(member_type, req.value)?;
            hasher.update(&value_encoded);
            if data_type == DataType::Bytes {
                display_size = value_len;
            }
            fmt
        };

        // Display value, splitting multi-line strings into separate screens.
        let lines: Vec<&str> = value_formatted.split('\n').collect();
        for (i, &line) in lines.iter().enumerate() {
            let body = format_display_line_body(&display_path, i, lines.len(), line);
            if body.len() > MAX_CONFIRM_BODY_SIZE {
                hal.ui()
                    .confirm(&ConfirmParams {
                        title: "Warning",
                        body: "The next value is\ntoo large to display\nin full",
                        accept_is_nextarrow: true,
                        ..Default::default()
                    })
                    .await?;
            }
            hal.ui()
                .confirm(&ConfirmParams {
                    title: &format!(
                        "{}{}",
                        confirm_title(root_object),
                        title_suffix.as_deref().unwrap_or("")
                    ),
                    body: &body,
                    scrollable: true,
                    display_size,
                    accept_is_nextarrow: true,
                    ..Default::default()
                })
                .await?;
        }
    }
    Ok(())
}

async fn hash_array(
    hal: &mut impl crate::hal::Hal,
    types: &[StructType],
    member_type: &MemberType,
    root_object: RootObject,
    path: &[u32],
    formatted_path: &[String],
    title_suffix: Option<String>,
) -> Result<Vec<u8>, Error> {
    let array_size = if member_type.size > 0 {
        member_type.size
    } else {
        let req = get_value_from_host(root_object, path).await?;
        if req.data_length > 0 {
            return Err(Error::InvalidInput);
        }
        u32::from_be_bytes(req.value.try_into().or(Err(Error::InvalidInput))?)
    };

    let array_type = member_type.array_type.as_ref().ok_or(Error::InvalidInput)?;

    hal.ui()
        .confirm(&ConfirmParams {
            title: &format!(
                "{}{}",
                confirm_title(root_object),
                title_suffix.as_deref().unwrap_or("")
            ),
            body: &format!(
                "{}: {}",
                formatted_path.join("."),
                if array_size == 0 {
                    "(empty list)".into()
                } else {
                    format!("list with {} elements", array_size)
                }
            ),
            scrollable: true,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

    let mut hasher = sha3::Keccak256::new();
    let mut child_path = path.to_vec();
    let mut child_formatted_path = formatted_path.to_vec();
    child_path.push(0);
    let member_name = child_formatted_path.last().unwrap().clone();
    for index in 0..array_size {
        *child_path.last_mut().unwrap() = index;
        *child_formatted_path.last_mut().unwrap() =
            format!("{}[{}/{}]", member_name, index + 1, array_size);

        encode_member(
            hal,
            &mut hasher,
            types,
            array_type,
            root_object,
            &child_path,
            &child_formatted_path,
            title_suffix.clone(),
        )
        .await?;
    }
    Ok(hasher.finalize().to_vec())
}

async fn hash_struct(
    hal: &mut impl crate::hal::Hal,
    types: &[StructType],
    root_object: RootObject,
    struct_name: &str,
    path: &[u32],
    formatted_path: &[String],
    title_suffix: Option<String>,
) -> Result<Vec<u8>, Error> {
    let mut hasher = sha3::Keccak256::new();
    hasher.update(&type_hash(types, struct_name)?);

    let typ = get_type(types, struct_name).ok_or(Error::InvalidInput)?;
    let mut child_path = path.to_vec();
    child_path.push(0);
    let mut child_formatted_path = formatted_path.to_vec();
    child_formatted_path.push("".into());
    for (index, member) in typ.members.iter().enumerate() {
        *child_path.last_mut().unwrap() = index as u32;
        child_formatted_path
            .last_mut()
            .unwrap()
            .clone_from(&member.name);
        let member_type = member.r#type.as_ref().ok_or(Error::InvalidInput)?;
        encode_member(
            hal,
            &mut hasher,
            types,
            member_type,
            root_object,
            &child_path,
            &child_formatted_path,
            if title_suffix.is_some() {
                title_suffix.clone()
            } else {
                Some(format!(" ({}/{})", index + 1, typ.members.len()))
            },
        )
        .await?;
    }

    Ok(hasher.finalize().to_vec())
}

/// The chain ID can optionally be part of the "domain" object. If it is present, we validate that
/// it matches chain ID provided in the request. In theory, the chain ID can be known to the wallet
/// app without including it in the domain to be signed, which is why it is provided directly in the
/// request regardless of whether it is present in the domain.
async fn validate_chain_id(request: &pb::EthSignTypedMessageRequest) -> Result<(), Error> {
    let domain_type = get_type(&request.types, DOMAIN_TYPE_NAME).ok_or(Error::InvalidInput)?;
    let chain_id_index = match domain_type
        .members
        .iter()
        .position(|member| member.name == "chainId")
    {
        Some(i) => i,
        None => {
            // Chain ID is not part of the domain - skip validation
            return Ok(());
        }
    };
    let req = get_value_from_host(RootObject::Domain, &[chain_id_index as u32]).await?;
    if req.data_length > 0 {
        return Err(Error::InvalidInput);
    }
    let chain_id: u64 = BigUint::from_bytes_be(&req.value)
        .try_into()
        .or(Err(Error::InvalidInput))?;
    if chain_id != request.chain_id {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

async fn eip712_sighash(
    hal: &mut impl crate::hal::Hal,
    types: &[StructType],
    primary_type: &str,
) -> Result<[u8; 32], Error> {
    let mut hasher = sha3::Keccak256::new();
    hasher.update([0x19u8, 0x01]);
    let domain_separator = hash_struct(
        hal,
        types,
        RootObject::Domain,
        DOMAIN_TYPE_NAME,
        &[],
        &[],
        None,
    )
    .await?;
    hasher.update(&domain_separator);
    // If primaryType is the domain type, skip the message hashing. This does not seem to conform to
    // the spec, but eth-sig-util implements it like that:
    // https://github.com/MetaMask/eth-sig-util/pull/51#issuecomment-1135089739
    if primary_type != DOMAIN_TYPE_NAME {
        let message_struct_hash = hash_struct(
            hal,
            types,
            RootObject::Message,
            primary_type,
            &[],
            &[],
            None,
        )
        .await?;
        hasher.update(&message_struct_hash);
    }
    Ok(hasher.finalize().into())
}

/// Process a EIP-712 sign typed message request.
///
/// https://eips.ethereum.org/EIPS/eip-712
///
/// The result contains a 65 byte signature. The first 64 bytes are the secp256k1 signature in
/// compact format (R and S values), and the last byte is the recoverable id (recid).
pub async fn process(
    hal: &mut impl crate::hal::Hal,
    request: &pb::EthSignTypedMessageRequest,
) -> Result<Response, Error> {
    validate_chain_id(request).await?;

    // Base component on the screen stack during signing, which is shown while the device is waiting
    // for the next signing api call. Without this, the 'See the BitBoxApp' waiting screen would
    // flicker in between user confirmations.
    let _empty_component = hal.ui().empty_create();

    // Verify address. We don't need the actual result, but we have to propagate validation or user
    // abort errors.
    super::pubrequest::process(
        hal,
        &pb::EthPubRequest {
            output_type: pb::eth_pub_request::OutputType::Address as _,
            keypath: request.keypath.clone(),
            coin: 0,
            display: true,
            contract_address: Vec::new(),
            chain_id: request.chain_id,
        },
    )
    .await?;

    let sighash: [u8; 32] = eip712_sighash(hal, &request.types, &request.primary_type).await?;

    hal.ui()
        .confirm(&ConfirmParams {
            body: "Sign data?",
            longtouch: true,
            ..Default::default()
        })
        .await?;

    let host_nonce = match request.host_nonce_commitment {
        Some(pb::AntiKleptoHostNonceCommitment { ref commitment }) => {
            let signer_commitment = crate::secp256k1::secp256k1_nonce_commit(
                keystore::secp256k1_get_private_key(hal, &request.keypath)?
                    .as_slice()
                    .try_into()
                    .unwrap(),
                &sighash,
                commitment
                    .as_slice()
                    .try_into()
                    .or(Err(Error::InvalidInput))?,
            )?;

            // Send signer commitment to host and wait for the host nonce from the host.
            Some(super::antiklepto_get_host_nonce(signer_commitment).await?)
        }

        None => None,
    };

    let sign_result = crate::secp256k1::secp256k1_sign(
        keystore::secp256k1_get_private_key(hal, &request.keypath)?
            .as_slice()
            .try_into()
            .unwrap(),
        &sighash,
        host_nonce.as_ref(),
    )?;
    let mut signature: Vec<u8> = sign_result.signature.to_vec();
    signature.push(sign_result.recid);

    Ok(Response::Sign(pb::EthSignResponse { signature }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;
    use crate::keystore::testing::mock_unlocked;
    use util::bip32::HARDENED;

    use alloc::boxed::Box;

    use pb::eth_sign_typed_message_request::Member;

    fn mk_type(data_type: DataType) -> MemberType {
        MemberType {
            r#type: data_type as _,
            ..Default::default()
        }
    }
    fn mk_sized_type(data_type: DataType, size: u32) -> MemberType {
        MemberType {
            r#type: data_type as _,
            size,
            ..Default::default()
        }
    }
    fn mk_arr_type(array_type: MemberType) -> MemberType {
        MemberType {
            r#type: DataType::Array as _,
            array_type: Some(Box::new(array_type)),
            ..Default::default()
        }
    }
    fn mk_struct_type(struct_name: &str) -> MemberType {
        MemberType {
            r#type: DataType::Struct as _,
            struct_name: struct_name.into(),
            ..Default::default()
        }
    }
    fn mk_member(name: &str, typ: MemberType) -> Member {
        Member {
            name: name.into(),
            r#type: Some(typ),
        }
    }

    fn make_types() -> Vec<StructType> {
        vec![
            StructType {
                name: "EIP712Domain".into(),
                members: vec![
                    mk_member("name", mk_type(DataType::String)),
                    mk_member("version", mk_type(DataType::String)),
                    mk_member("chainId", mk_sized_type(DataType::Uint, 32)),
                    mk_member("verifyingContract", mk_type(DataType::Address)),
                ],
            },
            StructType {
                name: "Person".into(),
                members: vec![
                    mk_member("name", mk_type(DataType::String)),
                    mk_member("wallet", mk_type(DataType::Address)),
                ],
            },
            StructType {
                name: "Attachment".into(),
                members: vec![mk_member("contents", mk_type(DataType::String))],
            },
            StructType {
                name: "Mail".into(),
                members: vec![
                    mk_member("from", mk_struct_type("Person")),
                    mk_member("to", mk_struct_type("Person")),
                    mk_member("contents", mk_type(DataType::String)),
                    mk_member("attachments", mk_arr_type(mk_struct_type("Attachment"))),
                ],
            },
        ]
    }

    async fn run_single_message_typed_msg(
        member_type: MemberType,
        message_obj: Object<'static>,
    ) -> TestingHal<'static> {
        let typed_msg = alloc::rc::Rc::new(TypedMessage::new(
            vec![
                StructType {
                    name: "EIP712Domain".into(),
                    members: vec![mk_member("name", mk_type(DataType::String))],
                },
                StructType {
                    name: "Msg".into(),
                    members: vec![mk_member("data", member_type)],
                },
            ],
            "Msg",
            Object::Struct(vec![Object::String("test")]),
            Object::Struct(vec![message_obj]),
        ));
        {
            let typed_msg = typed_msg.clone();
            *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(move |response| {
                Ok(typed_msg.handle_host_response(&response).unwrap())
            }));
        }
        let mut mock_hal = TestingHal::new();
        eip712_sighash(&mut mock_hal, &typed_msg.types, typed_msg.primary_type)
            .await
            .unwrap();
        mock_hal
    }

    async fn run_single_string_message(message: String) -> TestingHal<'static> {
        let message = Box::leak(message.into_boxed_str());
        run_single_message_typed_msg(mk_type(DataType::String), Object::String(message)).await
    }

    async fn run_single_streaming_bytes_message(data: Vec<u8>) -> TestingHal<'static> {
        run_single_message_typed_msg(mk_type(DataType::Bytes), Object::StreamingBytes(data)).await
    }

    /// A utility structure to build domain/message objects for testing.
    enum Object<'a> {
        String(&'a str),
        Bytes(&'a [u8]),
        Bool(bool),
        BigInt(BigInt),
        BigUint(BigUint),
        List(Vec<Object<'a>>),
        Struct(Vec<Object<'a>>),
        StreamingBytes(Vec<u8>),
    }

    impl Object<'_> {
        fn encode(&self) -> Vec<u8> {
            match self {
                Object::String(s) => s.as_bytes().to_vec(),
                Object::Bytes(b) => b.to_vec(),
                Object::Bool(b) => [(*b).into()].to_vec(),
                Object::BigInt(i) => i.to_signed_bytes_be(),
                Object::BigUint(i) => i.to_bytes_be(),
                Object::List(l) => (l.len() as u32).to_be_bytes().to_vec(),
                Object::StreamingBytes(_) => panic!("streaming values use get_value_protobuf"),
                _ => panic!("unexpected"),
            }
        }

        fn get_value(&self, path: &[u32]) -> Vec<u8> {
            if path.is_empty() {
                return self.encode();
            }

            match self {
                Object::Struct(l) | Object::List(l) => l[path[0] as usize].get_value(&path[1..]),
                _ => panic!("unexpected"),
            }
        }

        fn get_object(&self, path: &[u32]) -> &Object<'_> {
            if path.is_empty() {
                return self;
            }
            match self {
                Object::Struct(l) | Object::List(l) => l[path[0] as usize].get_object(&path[1..]),
                _ => panic!("unexpected"),
            }
        }

        fn get_value_protobuf(&self, path: &[u32]) -> pb::request::Request {
            let obj = self.get_object(path);
            match obj {
                Object::StreamingBytes(data) => pb::request::Request::Eth(pb::EthRequest {
                    request: Some(Request::TypedMsgValue(pb::EthTypedMessageValueRequest {
                        value: vec![],
                        data_length: data.len() as u32,
                    })),
                }),
                _ => pb::request::Request::Eth(pb::EthRequest {
                    request: Some(Request::TypedMsgValue(pb::EthTypedMessageValueRequest {
                        value: self.get_value(path),
                        data_length: 0,
                    })),
                }),
            }
        }

        fn get_streaming_data(&self, path: &[u32]) -> Option<&[u8]> {
            let obj = self.get_object(path);
            match obj {
                Object::StreamingBytes(data) => Some(data),
                _ => None,
            }
        }
    }

    struct StreamingState {
        data: Vec<u8>,
    }

    /// Typed message to be signed, as it would exist on the host.
    struct TypedMessage<'a> {
        types: Vec<StructType>,
        primary_type: &'a str,
        domain: Object<'a>,
        message: Object<'a>,
        streaming: core::cell::RefCell<Option<StreamingState>>,
    }

    impl<'a> TypedMessage<'a> {
        fn new(
            types: Vec<StructType>,
            primary_type: &'a str,
            domain: Object<'a>,
            message: Object<'a>,
        ) -> Self {
            TypedMessage {
                types,
                primary_type,
                domain,
                message,
                streaming: core::cell::RefCell::new(None),
            }
        }
    }

    impl TypedMessage<'_> {
        /// The host is asked for a value at a member of an object. This handles this request and
        /// responds with value. Also handles DataRequestChunk for streaming.
        fn handle_host_response(
            &self,
            response: &pb::response::Response,
        ) -> Option<pb::request::Request> {
            match response {
                pb::response::Response::Eth(pb::EthResponse {
                    response:
                        Some(Response::TypedMsgValue(pb::EthTypedMessageValueResponse {
                            root_object,
                            path,
                        })),
                }) => {
                    let obj = match RootObject::try_from(*root_object).unwrap() {
                        RootObject::Domain => &self.domain,
                        RootObject::Message => &self.message,
                        _ => return None,
                    };
                    // If this value should stream, set up streaming state.
                    if let Some(data) = obj.get_streaming_data(path) {
                        *self.streaming.borrow_mut() = Some(StreamingState {
                            data: data.to_vec(),
                        });
                    }
                    return Some(obj.get_value_protobuf(path));
                }
                pb::response::Response::Eth(pb::EthResponse {
                    response: Some(Response::DataRequestChunk(req)),
                }) => {
                    let mut streaming = self.streaming.borrow_mut();
                    let state = streaming
                        .as_mut()
                        .expect("chunk request without streaming state");
                    let offset = req.offset as usize;
                    let length = req.length as usize;
                    let chunk = state.data[offset..offset + length].to_vec();
                    return Some(pb::request::Request::Eth(pb::EthRequest {
                        request: Some(Request::DataResponseChunk(
                            pb::EthSignDataResponseChunkRequest { chunk },
                        )),
                    }));
                }
                _ => {}
            }
            None
        }
    }

    #[test]
    fn test_leftpad32() {
        assert_eq!(leftpad32(&[] as &[u8], false), Ok(vec![0u8; 32]));
        assert_eq!(leftpad32(&[0], false), Ok(vec![0u8; 32]));
        assert_eq!(leftpad32(&[0, 0], false), Ok(vec![0u8; 32]));
        assert_eq!(
            leftpad32(&[1], false),
            Ok(vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ])
        );
        assert_eq!(
            leftpad32(&[1, 2, 3, 4], false),
            Ok(vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 2, 3, 4,
            ])
        );
        assert_eq!(
            leftpad32(
                &[
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                    23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
                ],
                false
            ),
            Ok(vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32,
            ])
        );
        assert_eq!(
            leftpad32(
                &[
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                    23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
                ],
                false
            ),
            Err(Error::InvalidInput),
        );

        assert_eq!(
            leftpad32(b"\x80", true),
            Ok(b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\x80".to_vec()),
        );
    }

    #[test]
    fn test_get_transitive_types() {
        assert!(get_transitive_types(&[], "type-doesnt-exist").is_err());

        let types = make_types();
        assert_eq!(
            get_transitive_types(&types, "Mail",).unwrap(),
            vec!["Mail", "Person", "Attachment"] as Vec<&str>,
        );
    }

    #[test]
    fn test_format_member_type() {
        assert_eq!(
            format_member_type(&mk_type(DataType::Bytes)).unwrap(),
            "bytes"
        );
        assert_eq!(
            format_member_type(&mk_sized_type(DataType::Bytes, 10)).unwrap(),
            "bytes10"
        );
        assert_eq!(
            format_member_type(&mk_sized_type(DataType::Bytes, 32)).unwrap(),
            "bytes32"
        );
        assert_eq!(
            format_member_type(&mk_sized_type(DataType::Bytes, 33)),
            Err(Error::InvalidInput),
        );
        // Uint must be sized.
        assert_eq!(
            format_member_type(&mk_type(DataType::Uint)),
            Err(Error::InvalidInput),
        );
        assert_eq!(
            format_member_type(&mk_sized_type(DataType::Uint, 1)).unwrap(),
            "uint8",
        );
        assert_eq!(
            format_member_type(&mk_sized_type(DataType::Uint, 4)).unwrap(),
            "uint32",
        );
        assert_eq!(
            format_member_type(&mk_sized_type(DataType::Uint, 32)).unwrap(),
            "uint256",
        );
        assert_eq!(
            format_member_type(&mk_sized_type(DataType::Uint, 33)),
            Err(Error::InvalidInput),
        );
        // Int must be sized.
        assert_eq!(
            format_member_type(&mk_type(DataType::Int)),
            Err(Error::InvalidInput),
        );
        assert_eq!(
            format_member_type(&mk_sized_type(DataType::Int, 1)).unwrap(),
            "int8",
        );
        assert_eq!(
            format_member_type(&mk_sized_type(DataType::Int, 4)).unwrap(),
            "int32",
        );
        assert_eq!(
            format_member_type(&mk_sized_type(DataType::Int, 32)).unwrap(),
            "int256",
        );
        assert_eq!(
            format_member_type(&mk_sized_type(DataType::Int, 33)),
            Err(Error::InvalidInput),
        );
        assert_eq!(
            format_member_type(&mk_type(DataType::Bool)).unwrap(),
            "bool",
        );
        assert_eq!(
            format_member_type(&mk_type(DataType::Address)).unwrap(),
            "address",
        );
        assert_eq!(
            format_member_type(&mk_type(DataType::String)).unwrap(),
            "string",
        );
        // Missing array_type
        assert_eq!(
            format_member_type(&MemberType {
                r#type: DataType::Array as _,
                ..Default::default()
            }),
            Err(Error::InvalidInput),
        );
        assert_eq!(
            format_member_type(&mk_arr_type(mk_type(DataType::Address))).unwrap(),
            "address[]",
        );
        assert_eq!(
            format_member_type(&MemberType {
                r#type: DataType::Array as _,
                size: 10,
                array_type: Some(Box::new(mk_type(DataType::Address))),
                ..Default::default()
            })
            .unwrap(),
            "address[10]",
        );
        assert_eq!(
            format_member_type(&MemberType {
                r#type: DataType::Array as _,
                size: 10,
                array_type: Some(Box::new(mk_sized_type(DataType::Uint, 4))),
                ..Default::default()
            })
            .unwrap(),
            "uint32[10]",
        );
        assert_eq!(
            format_member_type(&mk_arr_type(MemberType {
                r#type: DataType::Array as _,
                size: 10,
                array_type: Some(Box::new(mk_sized_type(DataType::Uint, 4))),
                ..Default::default()
            }))
            .unwrap(),
            "uint32[10][]",
        );
        assert_eq!(
            format_member_type(&mk_arr_type(mk_arr_type(mk_struct_type("Person")))).unwrap(),
            "Person[][]",
        );
        // Missing struct_name
        assert_eq!(
            format_member_type(&MemberType {
                r#type: DataType::Struct as _,
                ..Default::default()
            }),
            Err(Error::InvalidInput),
        );
        assert_eq!(
            format_member_type(&mk_struct_type("Person")).unwrap(),
            "Person",
        );
    }

    #[test]
    fn test_streaming_display_byte_cap() {
        let exact_fit = truncating_hex_preview_byte_cap("data: 0x".len(), 316);
        assert_eq!(exact_fit, 316);
        let exact_fit_body = format_display_line_body(
            "data",
            0,
            1,
            &format!("0x{}", hex::encode(vec![0u8; exact_fit])),
        );
        assert_eq!(exact_fit_body.len(), MAX_CONFIRM_BODY_SIZE);

        let truncated = truncating_hex_preview_byte_cap("data: 0x".len(), 10_000);
        assert_eq!(truncated, 317);
        let truncated_body = format_display_line_body(
            "data",
            0,
            1,
            &format!("0x{}", hex::encode(vec![0u8; truncated])),
        );
        assert!(truncated_body.len() > MAX_CONFIRM_BODY_SIZE);
    }

    #[async_test::test]
    async fn test_multiline_warning_not_shown_when_each_line_fits() {
        let line1 = "a".repeat(400);
        let line2 = "b".repeat(300);
        let mock_hal = run_single_string_message(format!("{line1}\n{line2}")).await;

        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Domain (1/1)".into(),
                    body: "name: test".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Message (1/1)".into(),
                    body: format!("data, line 1/2: {line1}"),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Message (1/1)".into(),
                    body: format!("data, line 2/2: {line2}"),
                    longtouch: false,
                },
            ]
        );
    }

    #[async_test::test]
    async fn test_multiline_warning_shown_only_for_overlong_line() {
        let line2 = "b".repeat(MAX_CONFIRM_BODY_SIZE);
        let mock_hal = run_single_string_message(format!("ok\n{line2}")).await;

        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Domain (1/1)".into(),
                    body: "name: test".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Message (1/1)".into(),
                    body: "data, line 1/2: ok".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Warning".into(),
                    body: "The next value is\ntoo large to display\nin full".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Message (1/1)".into(),
                    body: format!("data, line 2/2: {line2}"),
                    longtouch: false,
                },
            ]
        );
    }

    #[async_test::test]
    async fn test_streaming_bytes_show_display_size_and_truncated_body() {
        let data: Vec<u8> = (0u8..=255).cycle().take(10_000).collect();
        let mock_hal = run_single_streaming_bytes_message(data).await;

        assert_eq!(mock_hal.ui.confirm_display_sizes, vec![0, 0, 10_000]);
        assert_eq!(
            mock_hal.ui.screens[1],
            Screen::Confirm {
                title: "Warning".into(),
                body: "The next value is\ntoo large to display\nin full".into(),
                longtouch: false,
            }
        );
        match &mock_hal.ui.screens[2] {
            Screen::Confirm { title, body, .. } => {
                assert_eq!(title, "Message (1/1)");
                assert!(body.len() > MAX_CONFIRM_BODY_SIZE);
            }
            _ => panic!("unexpected screen"),
        }
    }

    #[test]
    fn test_encode_type() {
        assert_eq!(
            encode_type(&make_types(), "EIP712Domain").unwrap(),
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
        );
        assert_eq!(
            encode_type(&make_types(), "Mail").unwrap(),
            "Mail(Person from,Person to,string contents,Attachment[] attachments)Attachment(string contents)Person(string name,address wallet)",
        );
    }

    #[test]
    fn test_type_hash() {
        assert_eq!(
            type_hash(&make_types(), "EIP712Domain").unwrap(),
            b"\x8b\x73\xc3\xc6\x9b\xb8\xfe\x3d\x51\x2e\xcc\x4c\xf7\x59\xcc\x79\x23\x9f\x7b\x17\x9b\x0f\xfa\xca\xa9\xa7\x5d\x52\x2b\x39\x40\x0f".to_vec(),
        );
    }

    /// Test computation of the domain separator, which is `hashStruct(domain)`.
    #[async_test::test]
    async fn test_domain_separator() {
        let typed_msg = alloc::rc::Rc::new(TypedMessage::new(
            make_types(),
            "Mail",
            Object::Struct(vec![
                Object::String("Ether Mail"),
                Object::String("1"),
                Object::BigUint(BigUint::from(1u32)),
                Object::String("0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"),
            ]),
            Object::Struct(vec![]),
        ));
        {
            let typed_msg = typed_msg.clone();
            *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(move |response| {
                Ok(typed_msg.handle_host_response(&response).unwrap())
            }));
        }
        let mut mock_hal = TestingHal::new();
        let domain_separator = hash_struct(
            &mut mock_hal,
            &typed_msg.types,
            RootObject::Domain,
            "EIP712Domain",
            &[],
            &[],
            None,
        )
        .await
        .unwrap();
        assert_eq!(
            domain_separator,
            b"\xf2\xce\xe3\x75\xfa\x42\xb4\x21\x43\x80\x40\x25\xfc\x44\x9d\xea\xfd\x50\xcc\x03\x1c\xa2\x57\xe0\xb1\x94\xa6\x50\xa9\x12\x09\x0f".to_vec());
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Domain (1/4)".into(),
                    body: "name: Ether Mail".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Domain (2/4)".into(),
                    body: "version: 1".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Domain (3/4)".into(),
                    body: "chainId: 1".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Domain (4/4)".into(),
                    body: "verifyingContract: 0x CcCC cccc CCCC cCCC CCCc CcCc cCcC CCcC cccc cccC"
                        .into(),
                    longtouch: false,
                },
            ]
        );
    }

    /// A typed data object which contains almost every type possible.
    ///
    /// Reproduce the below sighash result by running the below with nodejs:
    ///
    /// ```
    /// `npm install  @metamask/eth-sig-util@v4.0.1`.
    /// Then put this into `test.js` and run `node test.js`.
    /// const util = require('@metamask/eth-sig-util');
    /// const msgParams = ({
    ///   types: {
    ///     EIP712Domain: [
    ///       { name: 'name', type: 'string' },
    ///       { name: 'version', type: 'string' },
    ///       { name: 'chainId', type: 'uint256' },
    ///       { name: 'verifyingContract', type: 'address' },
    ///     ],
    ///     SmallType: [
    ///       { name: 'name', type: 'string' },
    ///       { name: 'arr', type: 'bool[]' },
    ///     ],
    ///     AllTypes: [
    ///       { name: 'str', type: 'string' },
    ///       { name: 'emptyArray', type: 'string[]' },
    ///       { name: 'name_address', type: 'address' },
    ///       { name: 'name_string', type: 'string[]' },
    ///       { name: 'name_bytes', type: 'bytes[]' },
    ///       { name: 'name_bytes1', type: 'bytes1' },
    ///       { name: 'name_bytes10', type: 'bytes10' },
    ///       { name: 'name_bytes32', type: 'bytes32' },
    ///       { name: 'name_uint8', type: 'uint8[]' },
    ///       { name: 'name_uint32', type: 'uint32[]' },
    ///       { name: 'name_uint64', type: 'uint64' },
    ///       { name: 'name_uint128', type: 'uint128' },
    ///       { name: 'name_uint256', type: 'uint256' },
    ///       { name: 'name_int8', type: 'int8[]' },
    ///       { name: 'name_int32', type: 'int32[]' },
    ///       { name: 'name_int64', type: 'int64[]' },
    ///       { name: 'name_int128', type: 'int128[]' },
    ///       { name: 'name_int256', type: 'int256[]' },
    ///       { name: 'name_bool', type: 'bool[]' },
    ///       { name: 'name_struct', type: 'SmallType' },
    ///       { name: 'arrayOfStructs', type: 'SmallType[]' },
    ///       { name: 'fixedArrayOfStructs', type: 'SmallType[2]' },
    ///       { name: 'nestedArray', type: 'uint32[][2][]' },
    ///     ],
    ///   },
    ///   domain: {
    ///     chainId: 1,
    ///     name: 'Ether Mail',
    ///     verifyingContract: '0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC',
    ///     version: '1',
    ///   },
    ///   primaryType: 'AllTypes',
    ///   message: {
    ///     str: 'str',
    ///     emptyArray: [],
    ///     name_address: '0xa21A16EC22a940990922220E4ab5bF4C2310F556',
    ///     name_string: ['', 'a', 'aa', '|@#!$', 'long long long long long long long long', 'multi\n\nline'],
    ///     name_bytes: ['', '0xaabbcc'],
    ///     name_bytes1: '0xaa',
    ///     name_bytes10: '0x112233445566778899aa',
    ///     name_bytes32: '0xd0f02988fd881565e927c7473c287322db166901bac03bef55d7a52a5c750ab4',
    ///     name_uint8: [0, 1, 10, 255],
    ///     name_uint32: [0, 256, 65536, 4294967295],
    ///     name_uint64:  '18446744073709551615',
    ///     name_uint128: '340282366920938463463374607431768211455',
    ///     name_uint256: '115792089237316195423570985008687907853269984665640564039457584007913129639935',
    ///     name_int8: [0, 10, -10, 127, -128],
    ///     name_int32: [2147483647, -2147483648],
    ///     name_int64:  [500, -500, '9223372036854775807', '-9223372036854775808'],
    ///     name_int128: ['170141183460469231731687303715884105727', '-170141183460469231731687303715884105728'],
    ///     name_int256: ['57896044618658097711785492504343953926634992332820282019728792003956564819967', '-57896044618658097711785492504343953926634992332820282019728792003956564819968'],
    ///     name_bool: [false, true],
    ///     name_struct: { name: 'struct name', arr: [] },
    ///     arrayOfStructs: [{ name: 'name 1', arr: [] }, { name: 'name 2', arr: [false] }, { name: 'name 3', arr: [false, true] }],
    ///     fixedArrayOfStructs: [{ name: 'name 1', arr: [] }, {name: 'name 2', arr: [false, false, true] }],
    ///     nestedArray: [[[1, 2], [3, 4, 5]], [[6, 7], [8]], [[], [9]]],
    ///   },
    /// });
    ///
    /// console.log("sighash:", util.TypedDataUtils.eip712Hash(msgParams, 'V4').toString('hex'));
    /// ```
    #[async_test::test]
    async fn test_exhaustive_data() {
        const EXPECTED_DIALOGS: &[(&str, &str)] = &[
            ("Domain (1/4)", "name: Ether Mail"),
            ("Domain (2/4)", "version: 1"),
            ("Domain (3/4)", "chainId: 1"),
            (
                "Domain (4/4)",
                "verifyingContract: 0x CcCC cccc CCCC cCCC CCCc CcCc cCcC CCcC cccc cccC",
            ),
            ("Message (1/23)", "str: str"),
            ("Message (2/23)", "emptyArray: (empty list)"),
            (
                "Message (3/23)",
                "name_address: 0x a21A 16EC 22a9 4099 0922 220E 4ab5 bF4C 2310 F556",
            ),
            ("Message (4/23)", "name_string: list with 6 elements"),
            ("Message (4/23)", "name_string[1/6]: "),
            ("Message (4/23)", "name_string[2/6]: a"),
            ("Message (4/23)", "name_string[3/6]: aa"),
            ("Message (4/23)", "name_string[4/6]: |@#!$"),
            (
                "Message (4/23)",
                "name_string[5/6]: long long long long long long long long",
            ),
            ("Message (4/23)", "name_string[6/6], line 1/3: multi"),
            ("Message (4/23)", "name_string[6/6], line 2/3: "),
            ("Message (4/23)", "name_string[6/6], line 3/3: line"),
            ("Message (5/23)", "name_bytes: list with 2 elements"),
            ("Message (5/23)", "name_bytes[1/2]: 0x"),
            ("Message (5/23)", "name_bytes[2/2]: 0xaabbcc"),
            ("Message (6/23)", "name_bytes1: 0xaa"),
            ("Message (7/23)", "name_bytes10: 0x112233445566778899aa"),
            (
                "Message (8/23)",
                "name_bytes32: 0xd0f02988fd881565e927c7473c287322db166901bac03bef55d7a52a5c750ab4",
            ),
            ("Message (9/23)", "name_uint8: list with 4 elements"),
            ("Message (9/23)", "name_uint8[1/4]: 0"),
            ("Message (9/23)", "name_uint8[2/4]: 1"),
            ("Message (9/23)", "name_uint8[3/4]: 10"),
            ("Message (9/23)", "name_uint8[4/4]: 255"),
            ("Message (10/23)", "name_uint32: list with 4 elements"),
            ("Message (10/23)", "name_uint32[1/4]: 0"),
            ("Message (10/23)", "name_uint32[2/4]: 256"),
            ("Message (10/23)", "name_uint32[3/4]: 65536"),
            ("Message (10/23)", "name_uint32[4/4]: 4294967295"),
            ("Message (11/23)", "name_uint64: 18446744073709551615"),
            (
                "Message (12/23)",
                "name_uint128: 340282366920938463463374607431768211455",
            ),
            (
                "Message (13/23)",
                "name_uint256: 115792089237316195423570985008687907853269984665640564039457584007913129639935",
            ),
            ("Message (14/23)", "name_int8: list with 5 elements"),
            ("Message (14/23)", "name_int8[1/5]: 0"),
            ("Message (14/23)", "name_int8[2/5]: 10"),
            ("Message (14/23)", "name_int8[3/5]: -10"),
            ("Message (14/23)", "name_int8[4/5]: 127"),
            ("Message (14/23)", "name_int8[5/5]: -128"),
            ("Message (15/23)", "name_int32: list with 2 elements"),
            ("Message (15/23)", "name_int32[1/2]: 2147483647"),
            ("Message (15/23)", "name_int32[2/2]: -2147483648"),
            ("Message (16/23)", "name_int64: list with 4 elements"),
            ("Message (16/23)", "name_int64[1/4]: 500"),
            ("Message (16/23)", "name_int64[2/4]: -500"),
            ("Message (16/23)", "name_int64[3/4]: 9223372036854775807"),
            ("Message (16/23)", "name_int64[4/4]: -9223372036854775808"),
            ("Message (17/23)", "name_int128: list with 2 elements"),
            (
                "Message (17/23)",
                "name_int128[1/2]: 170141183460469231731687303715884105727",
            ),
            (
                "Message (17/23)",
                "name_int128[2/2]: -170141183460469231731687303715884105728",
            ),
            ("Message (18/23)", "name_int256: list with 2 elements"),
            (
                "Message (18/23)",
                "name_int256[1/2]: 57896044618658097711785492504343953926634992332820282019728792003956564819967",
            ),
            (
                "Message (18/23)",
                "name_int256[2/2]: -57896044618658097711785492504343953926634992332820282019728792003956564819968",
            ),
            ("Message (19/23)", "name_bool: list with 2 elements"),
            ("Message (19/23)", "name_bool[1/2]: false"),
            ("Message (19/23)", "name_bool[2/2]: true"),
            ("Message (20/23)", "name_struct.name: struct name"),
            ("Message (20/23)", "name_struct.arr: (empty list)"),
            ("Message (21/23)", "arrayOfStructs: list with 3 elements"),
            ("Message (21/23)", "arrayOfStructs[1/3].name: name 1"),
            ("Message (21/23)", "arrayOfStructs[1/3].arr: (empty list)"),
            ("Message (21/23)", "arrayOfStructs[2/3].name: name 2"),
            (
                "Message (21/23)",
                "arrayOfStructs[2/3].arr: list with 1 elements",
            ),
            ("Message (21/23)", "arrayOfStructs[2/3].arr[1/1]: false"),
            ("Message (21/23)", "arrayOfStructs[3/3].name: name 3"),
            (
                "Message (21/23)",
                "arrayOfStructs[3/3].arr: list with 2 elements",
            ),
            ("Message (21/23)", "arrayOfStructs[3/3].arr[1/2]: false"),
            ("Message (21/23)", "arrayOfStructs[3/3].arr[2/2]: true"),
            (
                "Message (22/23)",
                "fixedArrayOfStructs: list with 2 elements",
            ),
            ("Message (22/23)", "fixedArrayOfStructs[1/2].name: name 1"),
            (
                "Message (22/23)",
                "fixedArrayOfStructs[1/2].arr: (empty list)",
            ),
            ("Message (22/23)", "fixedArrayOfStructs[2/2].name: name 2"),
            (
                "Message (22/23)",
                "fixedArrayOfStructs[2/2].arr: list with 3 elements",
            ),
            (
                "Message (22/23)",
                "fixedArrayOfStructs[2/2].arr[1/3]: false",
            ),
            (
                "Message (22/23)",
                "fixedArrayOfStructs[2/2].arr[2/3]: false",
            ),
            ("Message (22/23)", "fixedArrayOfStructs[2/2].arr[3/3]: true"),
            ("Message (23/23)", "nestedArray: list with 3 elements"),
            ("Message (23/23)", "nestedArray[1/3]: list with 2 elements"),
            (
                "Message (23/23)",
                "nestedArray[1/3][1/2]: list with 2 elements",
            ),
            ("Message (23/23)", "nestedArray[1/3][1/2][1/2]: 1"),
            ("Message (23/23)", "nestedArray[1/3][1/2][2/2]: 2"),
            (
                "Message (23/23)",
                "nestedArray[1/3][2/2]: list with 3 elements",
            ),
            ("Message (23/23)", "nestedArray[1/3][2/2][1/3]: 3"),
            ("Message (23/23)", "nestedArray[1/3][2/2][2/3]: 4"),
            ("Message (23/23)", "nestedArray[1/3][2/2][3/3]: 5"),
            ("Message (23/23)", "nestedArray[2/3]: list with 2 elements"),
            (
                "Message (23/23)",
                "nestedArray[2/3][1/2]: list with 2 elements",
            ),
            ("Message (23/23)", "nestedArray[2/3][1/2][1/2]: 6"),
            ("Message (23/23)", "nestedArray[2/3][1/2][2/2]: 7"),
            (
                "Message (23/23)",
                "nestedArray[2/3][2/2]: list with 1 elements",
            ),
            ("Message (23/23)", "nestedArray[2/3][2/2][1/1]: 8"),
            ("Message (23/23)", "nestedArray[3/3]: list with 2 elements"),
            ("Message (23/23)", "nestedArray[3/3][1/2]: (empty list)"),
            (
                "Message (23/23)",
                "nestedArray[3/3][2/2]: list with 1 elements",
            ),
            ("Message (23/23)", "nestedArray[3/3][2/2][1/1]: 9"),
        ];

        let bytes32 = b"\xd0\xf0\x29\x88\xfd\x88\x15\x65\xe9\x27\xc7\x47\x3c\x28\x73\x22\xdb\x16\x69\x01\xba\xc0\x3b\xef\x55\xd7\xa5\x2a\x5c\x75\x0a\xb4";
        let bigint256_positive = b"\x7f\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff";
        let bigint256_negative = b"\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let typed_msg = alloc::rc::Rc::new(TypedMessage::new(
            vec![
                StructType {
                    name: "EIP712Domain".into(),
                    members: vec![
                        mk_member("name", mk_type(DataType::String)),
                        mk_member("version", mk_type(DataType::String)),
                        mk_member("chainId", mk_sized_type(DataType::Uint, 32)),
                        mk_member("verifyingContract", mk_type(DataType::Address)),
                    ],
                },
                StructType {
                    name: "SmallType".into(),
                    members: vec![
                        mk_member("name", mk_type(DataType::String)),
                        mk_member("arr", mk_arr_type(mk_type(DataType::Bool))),
                    ],
                },
                StructType {
                    name: "AllTypes".into(),
                    members: vec![
                        mk_member("str", mk_type(DataType::String)),
                        mk_member("emptyArray", mk_arr_type(mk_type(DataType::String))),
                        mk_member("name_address", mk_type(DataType::Address)),
                        mk_member("name_string", mk_arr_type(mk_type(DataType::String))),
                        mk_member("name_bytes", mk_arr_type(mk_type(DataType::Bytes))),
                        mk_member("name_bytes1", mk_sized_type(DataType::Bytes, 1)),
                        mk_member("name_bytes10", mk_sized_type(DataType::Bytes, 10)),
                        mk_member("name_bytes32", mk_sized_type(DataType::Bytes, 32)),
                        mk_member("name_uint8", mk_arr_type(mk_sized_type(DataType::Uint, 1))),
                        mk_member("name_uint32", mk_arr_type(mk_sized_type(DataType::Uint, 4))),
                        mk_member("name_uint64", mk_sized_type(DataType::Uint, 8)),
                        mk_member("name_uint128", mk_sized_type(DataType::Uint, 16)),
                        mk_member("name_uint256", mk_sized_type(DataType::Uint, 32)),
                        mk_member("name_int8", mk_arr_type(mk_sized_type(DataType::Int, 1))),
                        mk_member("name_int32", mk_arr_type(mk_sized_type(DataType::Int, 4))),
                        mk_member("name_int64", mk_arr_type(mk_sized_type(DataType::Int, 8))),
                        mk_member("name_int128", mk_arr_type(mk_sized_type(DataType::Int, 16))),
                        mk_member("name_int256", mk_arr_type(mk_sized_type(DataType::Int, 32))),
                        mk_member("name_bool", mk_arr_type(mk_type(DataType::Bool))),
                        mk_member("name_struct", mk_struct_type("SmallType")),
                        mk_member("arrayOfStructs", mk_arr_type(mk_struct_type("SmallType"))),
                        mk_member(
                            "fixedArrayOfStructs",
                            MemberType {
                                r#type: DataType::Array as _,
                                size: 2,
                                array_type: Some(Box::new(mk_struct_type("SmallType"))),
                                ..Default::default()
                            },
                        ),
                        mk_member(
                            "nestedArray",
                            mk_arr_type(MemberType {
                                r#type: DataType::Array as _,
                                size: 2,
                                array_type: Some(Box::new(mk_arr_type(mk_sized_type(
                                    DataType::Uint,
                                    4,
                                )))),
                                ..Default::default()
                            }),
                        ),
                    ],
                },
            ],
            "AllTypes",
            Object::Struct(vec![
                // name
                Object::String("Ether Mail"),
                // version
                Object::String("1"),
                // chainId
                Object::BigUint(BigUint::from(1u32)),
                // verifyingContract
                Object::String("0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"),
            ]),
            Object::Struct(vec![
                // str
                Object::String("str"),
                // emptyArray
                Object::List(vec![]),
                // name_address
                Object::String("0xa21A16EC22a940990922220E4ab5bF4C2310F556"),
                // name_string
                Object::List(vec![
                    Object::String(""),
                    Object::String("a"),
                    Object::String("aa"),
                    Object::String("|@#!$"),
                    Object::String("long long long long long long long long"),
                    Object::String("multi\n\nline"),
                ]),
                // name_bytes
                Object::List(vec![Object::Bytes(b""), Object::Bytes(b"\xaa\xbb\xcc")]),
                // name_bytes1
                Object::Bytes(b"\xaa"),
                // name_bytes10
                Object::Bytes(b"\x11\x22\x33\x44\x55\x66\x77\x88\x99\xaa"),
                // name_bytes32
                Object::Bytes(bytes32),
                // name_uint8
                Object::List(vec![
                    Object::BigUint(BigUint::from(0u8)),
                    Object::BigUint(BigUint::from(1u8)),
                    Object::BigUint(BigUint::from(10u8)),
                    Object::BigUint(BigUint::from(255u8)),
                ]),
                // name_uint32
                Object::List(vec![
                    Object::BigUint(BigUint::from(0u32)),
                    Object::BigUint(BigUint::from(256u32)),
                    Object::BigUint(BigUint::from(65536u32)),
                    Object::BigUint(BigUint::from(4294967295u32)),
                ]),
                // name_uint64
                Object::BigUint(BigUint::from_bytes_be(&[0xff; 8])),
                // name_uint128
                Object::BigUint(BigUint::from_bytes_be(&[0xff; 16])),
                // name_uint256
                Object::BigUint(BigUint::from_bytes_be(&[0xff; 32])),
                // name_int8
                Object::List(vec![
                    Object::BigInt(BigInt::from(0i8)),
                    Object::BigInt(BigInt::from(10i8)),
                    Object::BigInt(BigInt::from(-10i8)),
                    Object::BigInt(BigInt::from(127i8)),
                    Object::BigInt(BigInt::from(-128i8)),
                ]),
                // name_int32
                Object::List(vec![
                    Object::BigInt(BigInt::from(2147483647i32)),
                    Object::BigInt(BigInt::from(-2147483648i32)),
                ]),
                // name_int64
                Object::List(vec![
                    Object::BigInt(BigInt::from(500i64)),
                    Object::BigInt(BigInt::from(-500i64)),
                    Object::BigInt(BigInt::from(9223372036854775807i64)),
                    Object::BigInt(BigInt::from(-9223372036854775808i64)),
                ]),
                // name_int128
                Object::List(vec![
                    Object::BigInt(BigInt::from(170141183460469231731687303715884105727i128)),
                    Object::BigInt(BigInt::from(-170141183460469231731687303715884105728i128)),
                ]),
                // name_int256
                Object::List(vec![
                    Object::BigInt(BigInt::from_signed_bytes_be(bigint256_positive)),
                    Object::BigInt(BigInt::from_signed_bytes_be(bigint256_negative)),
                ]),
                // name_bool
                Object::List(vec![Object::Bool(false), Object::Bool(true)]),
                // name_struct
                Object::Struct(vec![Object::String("struct name"), Object::List(vec![])]),
                // arrayOfStructs
                Object::List(vec![
                    Object::Struct(vec![
                        // name
                        Object::String("name 1"),
                        // arr
                        Object::List(vec![]),
                    ]),
                    Object::Struct(vec![
                        // name
                        Object::String("name 2"),
                        // arr
                        Object::List(vec![Object::Bool(false)]),
                    ]),
                    Object::Struct(vec![
                        // name
                        Object::String("name 3"),
                        // arr
                        Object::List(vec![Object::Bool(false), Object::Bool(true)]),
                    ]),
                ]),
                // fixedArrayOfStructs
                Object::List(vec![
                    Object::Struct(vec![
                        // name
                        Object::String("name 1"),
                        // arr
                        Object::List(vec![]),
                    ]),
                    Object::Struct(vec![
                        // name
                        Object::String("name 2"),
                        // arr
                        Object::List(vec![
                            Object::Bool(false),
                            Object::Bool(false),
                            Object::Bool(true),
                        ]),
                    ]),
                ]),
                // nestedArray
                Object::List(vec![
                    Object::List(vec![
                        Object::List(vec![
                            Object::BigUint(BigUint::from(1u32)),
                            Object::BigUint(BigUint::from(2u32)),
                        ]),
                        Object::List(vec![
                            Object::BigUint(BigUint::from(3u32)),
                            Object::BigUint(BigUint::from(4u32)),
                            Object::BigUint(BigUint::from(5u32)),
                        ]),
                    ]),
                    Object::List(vec![
                        Object::List(vec![
                            Object::BigUint(BigUint::from(6u32)),
                            Object::BigUint(BigUint::from(7u32)),
                        ]),
                        Object::List(vec![Object::BigUint(BigUint::from(8u32))]),
                    ]),
                    Object::List(vec![
                        Object::List(vec![]),
                        Object::List(vec![Object::BigUint(BigUint::from(9u32))]),
                    ]),
                ]),
            ]),
        ));

        {
            let typed_msg = typed_msg.clone();
            *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(move |response| {
                Ok(typed_msg.handle_host_response(&response).unwrap())
            }));
        }
        let mut mock_hal = TestingHal::new();
        let sighash = eip712_sighash(&mut mock_hal, &typed_msg.types, typed_msg.primary_type)
            .await
            .unwrap();
        assert_eq!(
            sighash,
            *b"\x0e\xfe\x31\xa8\x81\x9b\x6c\x38\x1c\x9e\x97\xcf\xd2\x99\x5a\xa6\xf2\x1e\x4a\x72\x87\x9a\xc1\x31\xb2\xf6\x48\xd0\x83\x28\x1c\x83",
        );
        assert_eq!(
            mock_hal.ui.screens,
            EXPECTED_DIALOGS
                .iter()
                .map(|&(title, body)| Screen::Confirm {
                    title: title.into(),
                    body: body.into(),
                    longtouch: false
                })
                .collect::<Vec<_>>()
        );
    }

    /// Test case whree primaryType=='EIP712Domain'.
    /// See https://github.com/MetaMask/eth-sig-util/pull/51.
    ///
    /// A typed data object which contains almost every type possible.
    ///
    /// Reproduce the below sighash result by running the below with nodejs:
    ///
    /// ```
    /// `npm install  @metamask/eth-sig-util@v4.0.1`.
    /// Then put this into `test.js` and run `node test.js`.
    /// const util = require('@metamask/eth-sig-util');
    /// const msgParams = ({
    ///   types: {
    ///     EIP712Domain: [
    ///       { name: 'name', type: 'string' },
    ///       { name: 'version', type: 'string' },
    ///       { name: 'chainId', type: 'uint256' },
    ///       { name: 'verifyingContract', type: 'address' },
    ///     ],
    ///   domain: {
    ///     chainId: 1,
    ///     name: 'Ether Mail',
    ///     verifyingContract: '0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC',
    ///     version: '1',
    ///   },
    ///   primaryType: 'EIP712Domain',
    /// });
    ///
    /// console.log("sighash:", util.TypedDataUtils.eip712Hash(msgParams, 'V4').toString('hex'));
    /// ```
    #[async_test::test]
    async fn test_no_message() {
        let typed_msg = alloc::rc::Rc::new(TypedMessage::new(
            vec![StructType {
                name: "EIP712Domain".into(),
                members: vec![
                    mk_member("name", mk_type(DataType::String)),
                    mk_member("version", mk_type(DataType::String)),
                    mk_member("chainId", mk_sized_type(DataType::Uint, 32)),
                    mk_member("verifyingContract", mk_type(DataType::Address)),
                ],
            }],
            "EIP712Domain",
            Object::Struct(vec![
                // name
                Object::String("Ether Mail"),
                // version
                Object::String("1"),
                // chainId
                Object::BigUint(BigUint::from(1u32)),
                // verifyingContract
                Object::String("0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"),
            ]),
            Object::Struct(vec![]),
        ));

        {
            let typed_msg = typed_msg.clone();
            *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(move |response| {
                Ok(typed_msg.handle_host_response(&response).unwrap())
            }));
        }
        let sighash = eip712_sighash(
            &mut TestingHal::new(),
            &typed_msg.types,
            typed_msg.primary_type,
        )
        .await
        .unwrap();
        assert_eq!(
            sighash,
            *b"\xaa\x83\xc7\x03\x05\xec\x6c\x13\x1e\x7a\x88\xf2\x58\xc4\x08\x13\x44\x7b\xec\x8b\x9b\xce\xf9\x4e\x54\x79\x60\x3d\x99\x59\xda\x07",
        );
    }

    // Test vectors generated by testdata/gen_typed_msg_streaming_tests.js using
    // @metamask/eth-sig-util v4.0.1.
    #[derive(serde::Deserialize)]
    struct StreamingTestCase {
        description: String,
        types: serde_json::Value,
        primary_type: String,
        domain: serde_json::Value,
        field_type: String,
        message_data: String,
        expected_sighash: String,
        expected_signature: Option<String>,
        address: Option<String>,
        expected_screens: Vec<(String, String)>,
    }

    fn decode_hex(s: &str) -> Vec<u8> {
        hex::decode(s).unwrap()
    }

    fn load_streaming_test_cases() -> Vec<StreamingTestCase> {
        let json_data = include_str!("testdata/typed_msg_streaming_tests.json");
        serde_json::from_str(json_data).unwrap()
    }

    fn parse_json_types(json: &serde_json::Value) -> Vec<StructType> {
        json.as_object()
            .unwrap()
            .iter()
            .map(|(name, members_val)| StructType {
                name: name.clone(),
                members: members_val
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|m| {
                        let type_str = m["type"].as_str().unwrap();
                        let member_type = match type_str {
                            "string" => mk_type(DataType::String),
                            "bytes" => mk_type(DataType::Bytes),
                            _ => panic!("unhandled type in test data: {type_str}"),
                        };
                        mk_member(m["name"].as_str().unwrap(), member_type)
                    })
                    .collect(),
            })
            .collect()
    }

    fn build_domain_object(
        domain_json: &serde_json::Value,
        types: &[StructType],
    ) -> Object<'static> {
        let domain_type = get_type(types, "EIP712Domain").unwrap();
        Object::Struct(
            domain_type
                .members
                .iter()
                .map(|member| {
                    let s: &'static str = Box::leak(
                        domain_json[&member.name]
                            .as_str()
                            .unwrap()
                            .to_string()
                            .into_boxed_str(),
                    );
                    Object::String(s)
                })
                .collect(),
        )
    }

    /// Verify streaming and inline sighashes match expected values, verify display screens,
    /// and optionally verify signatures. Test vectors generated by
    /// testdata/gen_typed_msg_streaming_tests.js using @metamask/eth-sig-util v4.0.1.
    #[async_test::test]
    async fn test_streaming_equivalence() {
        let tests = load_streaming_test_cases();
        for tc in &tests {
            let expected: [u8; 32] = decode_hex(&tc.expected_sighash).try_into().unwrap();
            let types = parse_json_types(&tc.types);
            // Leak to get 'static refs needed by the mock closure -- test-only.
            let primary_type: &'static str = Box::leak(tc.primary_type.clone().into_boxed_str());

            // Inline sighash.
            {
                let inline_value = match tc.field_type.as_str() {
                    "bytes" => {
                        let data: Vec<u8> = decode_hex(&tc.message_data);
                        let data_static: &'static [u8] = Box::leak(data.into_boxed_slice());
                        Object::Bytes(data_static)
                    }
                    "string" => {
                        let data_static: &'static str =
                            Box::leak(tc.message_data.clone().into_boxed_str());
                        Object::String(data_static)
                    }
                    _ => panic!("unhandled field_type in test data: {}", tc.field_type),
                };
                let typed_msg = alloc::rc::Rc::new(TypedMessage::new(
                    types.clone(),
                    primary_type,
                    build_domain_object(&tc.domain, &types),
                    Object::Struct(vec![inline_value]),
                ));
                let typed_msg_clone = typed_msg.clone();
                *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(move |response| {
                    Ok(typed_msg_clone.handle_host_response(&response).unwrap())
                }));
                let inline_sighash = eip712_sighash(
                    &mut TestingHal::new(),
                    &typed_msg.types,
                    typed_msg.primary_type,
                )
                .await
                .unwrap();
                assert_eq!(
                    inline_sighash, expected,
                    "inline sighash mismatch for: {}",
                    tc.description
                );
            }

            // Streaming sighash + display verification (bytes only).
            if tc.field_type == "bytes" {
                let data: Vec<u8> = decode_hex(&tc.message_data);
                let typed_msg = alloc::rc::Rc::new(TypedMessage::new(
                    types.clone(),
                    primary_type,
                    build_domain_object(&tc.domain, &types),
                    Object::Struct(vec![Object::StreamingBytes(data.clone())]),
                ));
                let typed_msg_clone = typed_msg.clone();
                *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(move |response| {
                    Ok(typed_msg_clone.handle_host_response(&response).unwrap())
                }));
                let mut mock_hal = TestingHal::new();
                let streaming_sighash =
                    eip712_sighash(&mut mock_hal, &typed_msg.types, typed_msg.primary_type)
                        .await
                        .unwrap();
                assert_eq!(
                    streaming_sighash, expected,
                    "streaming sighash mismatch for: {}",
                    tc.description
                );
                let expected_screens: Vec<Screen> = tc
                    .expected_screens
                    .iter()
                    .map(|(title, body)| Screen::Confirm {
                        title: title.clone(),
                        body: body.clone(),
                        longtouch: false,
                    })
                    .collect();
                assert_eq!(
                    mock_hal.ui.screens, expected_screens,
                    "screens mismatch for: {}",
                    tc.description
                );
            } else if tc.field_type == "string" {
                let data_static: &'static str = Box::leak(tc.message_data.clone().into_boxed_str());
                let typed_msg = alloc::rc::Rc::new(TypedMessage::new(
                    types.clone(),
                    primary_type,
                    build_domain_object(&tc.domain, &types),
                    Object::Struct(vec![Object::String(data_static)]),
                ));
                let typed_msg_clone = typed_msg.clone();
                *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(move |response| {
                    Ok(typed_msg_clone.handle_host_response(&response).unwrap())
                }));
                let mut mock_hal = TestingHal::new();
                let sighash =
                    eip712_sighash(&mut mock_hal, &typed_msg.types, typed_msg.primary_type)
                        .await
                        .unwrap();
                assert_eq!(
                    sighash, expected,
                    "sighash mismatch for: {}",
                    tc.description
                );
                let expected_screens: Vec<Screen> = tc
                    .expected_screens
                    .iter()
                    .map(|(title, body)| Screen::Confirm {
                        title: title.clone(),
                        body: body.clone(),
                        longtouch: false,
                    })
                    .collect();
                assert_eq!(
                    mock_hal.ui.screens, expected_screens,
                    "screens mismatch for: {}",
                    tc.description
                );
            }

            // Verify signature if expected.
            if let (Some(expected_sig_hex), Some(address)) = (&tc.expected_signature, &tc.address) {
                assert_eq!(tc.field_type, "bytes");
                mock_unlocked();
                let data: Vec<u8> = decode_hex(&tc.message_data);
                let expected_sig = decode_hex(expected_sig_hex);
                let typed_msg = alloc::rc::Rc::new(TypedMessage::new(
                    types.clone(),
                    primary_type,
                    build_domain_object(&tc.domain, &types),
                    Object::Struct(vec![Object::StreamingBytes(data.clone())]),
                ));
                let typed_msg_clone = typed_msg.clone();
                *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(move |response| {
                    Ok(typed_msg_clone.handle_host_response(&response).unwrap())
                }));
                let mut mock_hal = TestingHal::new();
                assert_eq!(
                    process(
                        &mut mock_hal,
                        &pb::EthSignTypedMessageRequest {
                            chain_id: 1,
                            keypath: vec![44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0],
                            types: types.clone(),
                            primary_type: tc.primary_type.clone(),
                            host_nonce_commitment: None,
                        }
                    )
                    .await,
                    Ok(Response::Sign(pb::EthSignResponse {
                        signature: expected_sig,
                    })),
                    "signature mismatch for: {}",
                    tc.description
                );
                let mut expected_screens = vec![Screen::Confirm {
                    title: "Ethereum".into(),
                    body: address.clone(),
                    longtouch: false,
                }];
                expected_screens.extend(tc.expected_screens.iter().map(|(title, body)| {
                    Screen::Confirm {
                        title: title.clone(),
                        body: body.clone(),
                        longtouch: false,
                    }
                }));
                expected_screens.push(Screen::Confirm {
                    title: "".into(),
                    body: "Sign data?".into(),
                    longtouch: true,
                });
                assert_eq!(
                    mock_hal.ui.screens, expected_screens,
                    "signature flow screens mismatch for: {}",
                    tc.description
                );
            }
        }
    }

    /// Streaming is rejected for non-streamable types (e.g. uint).
    #[async_test::test]
    async fn test_streaming_rejected_for_uint() {
        let typed_msg = alloc::rc::Rc::new(TypedMessage::new(
            vec![
                StructType {
                    name: "EIP712Domain".into(),
                    members: vec![mk_member("name", mk_type(DataType::String))],
                },
                StructType {
                    name: "Msg".into(),
                    members: vec![mk_member("data", mk_sized_type(DataType::Uint, 32))],
                },
            ],
            "Msg",
            Object::Struct(vec![Object::String("test")]),
            Object::Struct(vec![Object::StreamingBytes(vec![0x01; 100])]),
        ));
        {
            let typed_msg = typed_msg.clone();
            *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(move |response| {
                Ok(typed_msg.handle_host_response(&response).unwrap())
            }));
        }
        let mut mock_hal = TestingHal::new();
        let result = eip712_sighash(&mut mock_hal, &typed_msg.types, typed_msg.primary_type).await;
        assert_eq!(result, Err(Error::InvalidInput));
    }

    /// Streaming is rejected for fixed-size bytes (e.g. bytes32).
    #[async_test::test]
    async fn test_streaming_rejected_for_fixed_bytes() {
        let typed_msg = alloc::rc::Rc::new(TypedMessage::new(
            vec![
                StructType {
                    name: "EIP712Domain".into(),
                    members: vec![mk_member("name", mk_type(DataType::String))],
                },
                StructType {
                    name: "Msg".into(),
                    members: vec![mk_member("data", mk_sized_type(DataType::Bytes, 32))],
                },
            ],
            "Msg",
            Object::Struct(vec![Object::String("test")]),
            Object::Struct(vec![Object::StreamingBytes(vec![0x01; 32])]),
        ));
        {
            let typed_msg = typed_msg.clone();
            *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(move |response| {
                Ok(typed_msg.handle_host_response(&response).unwrap())
            }));
        }
        let mut mock_hal = TestingHal::new();
        let result = eip712_sighash(&mut mock_hal, &typed_msg.types, typed_msg.primary_type).await;
        assert_eq!(result, Err(Error::InvalidInput));
    }

    /// data_length exceeding the max is rejected.
    #[async_test::test]
    async fn test_streaming_exceeding_max_rejected() {
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() =
            Some(Box::new(|response| match &response {
                pb::response::Response::Eth(pb::EthResponse {
                    response:
                        Some(Response::TypedMsgValue(pb::EthTypedMessageValueResponse {
                            root_object,
                            path: _,
                        })),
                }) => {
                    if *root_object == RootObject::Domain as i32 {
                        return Ok(pb::request::Request::Eth(pb::EthRequest {
                            request: Some(Request::TypedMsgValue(
                                pb::EthTypedMessageValueRequest {
                                    value: b"test".to_vec(),
                                    data_length: 0,
                                },
                            )),
                        }));
                    }
                    Ok(pb::request::Request::Eth(pb::EthRequest {
                        request: Some(Request::TypedMsgValue(pb::EthTypedMessageValueRequest {
                            value: vec![],
                            data_length: MAX_TYPED_MSG_STREAMING_DATA_LENGTH + 1,
                        })),
                    }))
                }
                _ => panic!("unexpected"),
            }));
        let types = vec![
            StructType {
                name: "EIP712Domain".into(),
                members: vec![mk_member("name", mk_type(DataType::String))],
            },
            StructType {
                name: "Msg".into(),
                members: vec![mk_member("data", mk_type(DataType::Bytes))],
            },
        ];
        let mut mock_hal = TestingHal::new();
        let result = eip712_sighash(&mut mock_hal, &types, "Msg").await;
        assert_eq!(result, Err(Error::InvalidInput));
    }

    /// Both value and data_length non-empty is rejected.
    #[async_test::test]
    async fn test_streaming_value_and_data_length_both_set_rejected() {
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(|response| {
            match &response {
                pb::response::Response::Eth(pb::EthResponse {
                    response:
                        Some(Response::TypedMsgValue(pb::EthTypedMessageValueResponse {
                            root_object,
                            ..
                        })),
                }) => {
                    if *root_object == RootObject::Domain as i32 {
                        return Ok(pb::request::Request::Eth(pb::EthRequest {
                            request: Some(Request::TypedMsgValue(
                                pb::EthTypedMessageValueRequest {
                                    value: b"test".to_vec(),
                                    data_length: 0,
                                },
                            )),
                        }));
                    }
                    // message.data: both value and data_length set
                    Ok(pb::request::Request::Eth(pb::EthRequest {
                        request: Some(Request::TypedMsgValue(pb::EthTypedMessageValueRequest {
                            value: vec![1, 2, 3],
                            data_length: 100,
                        })),
                    }))
                }
                _ => panic!("unexpected"),
            }
        }));
        let types = vec![
            StructType {
                name: "EIP712Domain".into(),
                members: vec![mk_member("name", mk_type(DataType::String))],
            },
            StructType {
                name: "Msg".into(),
                members: vec![mk_member("data", mk_type(DataType::Bytes))],
            },
        ];
        let mut mock_hal = TestingHal::new();
        let result = eip712_sighash(&mut mock_hal, &types, "Msg").await;
        assert_eq!(result, Err(Error::InvalidInput));
    }
}
