// SPDX-License-Identifier: Apache-2.0

use alloc::{format, string::String, vec::Vec};

use super::Error;
use super::pb;
use crate::hal::Ui;
use crate::hal::ui::ConfirmParams;
use crate::keystore::ed25519;
use chacha20poly1305::aead::{AeadInPlace, KeyInit};
use ed25519_dalek::VerifyingKey;
use hkdf::Hkdf;
use pb::response::Response;
use sha2::{Digest, Sha256};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519StaticSecret};
use zeroize::{Zeroize, Zeroizing};

const AUTH_SEED_LABEL: &[u8] = b"bitboxsync-auth-ed25519-seed-v1";
const WRAP_SEED_LABEL: &[u8] = b"bitboxsync-wrap-x25519-seed-v1";

const INTENT_PREFIX: &[u8] = b"bitboxsync-intent";
const JOIN_REQUEST_PREFIX: &[u8] = b"bitboxsync-join-request";
const WRAP_DEK_INFO: &[u8] = b"bitboxsync-wrap-dek-v1";

const KIND_CODE_KEYSTORE: u8 = 0x01;
const INTENT_VERSION: u8 = 0x01;
const JOIN_REQUEST_VERSION: u8 = 0x01;
const WRAPPED_DEK_VERSION: u8 = 0x01;
const SENSITIVE_ACTION_REVOKE_ALL_TOKENS: u8 = 0x01;
const SENSITIVE_ACTION_CREATE_NAMESPACE_INVITE: u8 = 0x02;

const CHALLENGE_LEN: usize = 32;
const KEY_ID_LEN: usize = 32;
const NAMESPACE_ID_LEN: usize = 16;
const INVITE_ID_LEN: usize = 16;
const INVITE_SERVER_SECRET_HASH_LEN: usize = 32;
const NAMESPACE_DEK_LEN: usize = 32;
const SERVER_ORIGIN_HASH_LEN: usize = 32;
const MAX_SERVER_ORIGIN_LEN: usize = 128;
const WRAPPED_DEK_LEN_V1: usize = 1 + 32 + NAMESPACE_ID_LEN + NAMESPACE_DEK_LEN + 16;
const SIGNATURE_LEN: usize = 64;

struct IdentityKeys {
    auth_seed: Zeroizing<[u8; 32]>,
    auth_public_key: [u8; 32],
    wrap_secret_key: Zeroizing<[u8; 32]>,
    wrap_public_key: [u8; 32],
}

async fn identity_keys(hal: &mut impl crate::hal::Hal) -> Result<IdentityKeys, Error> {
    let identity_root_key = crate::keystore::bip85_bitboxsync(hal)
        .await
        .map_err(|_| Error::Generic)?;
    let auth_seed = derive_labeled_key(&identity_root_key, AUTH_SEED_LABEL)?;
    let auth_public_key =
        VerifyingKey::from(&ed25519::expanded_secret_key_from_seed(&auth_seed)).to_bytes();

    let wrap_secret_key = derive_labeled_key(&identity_root_key, WRAP_SEED_LABEL)?;
    let wrap_secret = X25519StaticSecret::from(*wrap_secret_key);
    let wrap_public_key = X25519PublicKey::from(&wrap_secret).to_bytes();

    Ok(IdentityKeys {
        auth_seed,
        auth_public_key,
        wrap_secret_key,
        wrap_public_key,
    })
}

fn derive_labeled_key(
    identity_root_key: &[u8],
    label: &[u8],
) -> Result<Zeroizing<[u8; 32]>, Error> {
    let hkdf = Hkdf::<Sha256>::new(None, identity_root_key);
    let mut out = Zeroizing::new([0u8; 32]);
    hkdf.expand(label, &mut *out).map_err(|_| Error::Generic)?;
    Ok(out)
}

fn key_id(auth_public_key: &[u8; 32]) -> [u8; KEY_ID_LEN] {
    Sha256::digest(auth_public_key).into()
}

fn login_intent(challenge: &[u8], keys: &IdentityKeys) -> Result<Vec<u8>, Error> {
    if challenge.len() != CHALLENGE_LEN {
        return Err(Error::InvalidInput);
    }
    let key_id = key_id(&keys.auth_public_key);
    let mut out = Vec::with_capacity(
        INTENT_PREFIX.len()
            + 1
            + 1
            + CHALLENGE_LEN
            + 1
            + KEY_ID_LEN
            + keys.auth_public_key.len()
            + keys.wrap_public_key.len(),
    );
    out.extend_from_slice(INTENT_PREFIX);
    out.extend_from_slice(&[INTENT_VERSION, 0x01]);
    out.extend_from_slice(challenge);
    out.push(KIND_CODE_KEYSTORE);
    out.extend_from_slice(&key_id);
    out.extend_from_slice(&keys.auth_public_key);
    out.extend_from_slice(&keys.wrap_public_key);
    Ok(out)
}

fn refresh_intent(challenge: &[u8], keys: &IdentityKeys) -> Result<Vec<u8>, Error> {
    if challenge.len() != CHALLENGE_LEN {
        return Err(Error::InvalidInput);
    }
    let key_id = key_id(&keys.auth_public_key);
    let mut out = Vec::with_capacity(INTENT_PREFIX.len() + 1 + 1 + CHALLENGE_LEN + 1 + KEY_ID_LEN);
    out.extend_from_slice(INTENT_PREFIX);
    out.extend_from_slice(&[INTENT_VERSION, 0x02]);
    out.extend_from_slice(challenge);
    out.push(KIND_CODE_KEYSTORE);
    out.extend_from_slice(&key_id);
    Ok(out)
}

fn sensitive_action_intent(
    challenge: &[u8],
    action_code: u8,
    action_fields: &[u8],
    keys: &IdentityKeys,
) -> Result<Vec<u8>, Error> {
    if challenge.len() != CHALLENGE_LEN {
        return Err(Error::InvalidInput);
    }
    let key_id = key_id(&keys.auth_public_key);
    let mut out = Vec::with_capacity(
        INTENT_PREFIX.len() + 1 + 1 + 1 + CHALLENGE_LEN + 1 + KEY_ID_LEN + action_fields.len(),
    );
    out.extend_from_slice(INTENT_PREFIX);
    out.extend_from_slice(&[INTENT_VERSION, 0x03, action_code]);
    out.extend_from_slice(challenge);
    out.push(KIND_CODE_KEYSTORE);
    out.extend_from_slice(&key_id);
    out.extend_from_slice(action_fields);
    Ok(out)
}

fn revoke_all_tokens_intent(challenge: &[u8], keys: &IdentityKeys) -> Result<Vec<u8>, Error> {
    sensitive_action_intent(challenge, SENSITIVE_ACTION_REVOKE_ALL_TOKENS, &[], keys)
}

fn create_namespace_invite_action_fields(
    namespace_id: &[u8],
    invite_id: &[u8],
    invite_server_secret_hash: &[u8],
    expires_at: u64,
    max_accepted: u32,
) -> Result<Vec<u8>, Error> {
    if namespace_id.len() != NAMESPACE_ID_LEN
        || invite_id.len() != INVITE_ID_LEN
        || invite_server_secret_hash.len() != INVITE_SERVER_SECRET_HASH_LEN
    {
        return Err(Error::InvalidInput);
    }

    let mut out = Vec::with_capacity(
        NAMESPACE_ID_LEN + INVITE_ID_LEN + INVITE_SERVER_SECRET_HASH_LEN + 8 + 4,
    );
    out.extend_from_slice(namespace_id);
    out.extend_from_slice(invite_id);
    out.extend_from_slice(invite_server_secret_hash);
    out.extend_from_slice(&expires_at.to_be_bytes());
    out.extend_from_slice(&max_accepted.to_be_bytes());
    Ok(out)
}

fn create_namespace_invite_intent(
    request: &pb::BitBoxSyncSignCreateNamespaceInviteIntentRequest,
    keys: &IdentityKeys,
) -> Result<Vec<u8>, Error> {
    let action_fields = create_namespace_invite_action_fields(
        &request.namespace_id,
        &request.invite_id,
        &request.invite_server_secret_hash,
        request.expires_at,
        request.max_accepted,
    )?;
    sensitive_action_intent(
        &request.challenge,
        SENSITIVE_ACTION_CREATE_NAMESPACE_INVITE,
        &action_fields,
        keys,
    )
}

fn validate_server_origin(server_origin: &str) -> Result<(), Error> {
    if !server_origin.is_ascii()
        || server_origin.len() > MAX_SERVER_ORIGIN_LEN
        || server_origin
            .as_bytes()
            .iter()
            .any(|byte| byte.is_ascii_control())
        || !server_origin.starts_with("https://")
    {
        return Err(Error::InvalidInput);
    }
    let authority = &server_origin["https://".len()..];
    if authority.is_empty()
        || authority
            .as_bytes()
            .iter()
            .any(|byte| matches!(*byte, b'/' | b'?' | b'#' | b'@' | b' '))
        || authority
            .as_bytes()
            .iter()
            .any(|byte| byte.is_ascii_uppercase())
    {
        return Err(Error::InvalidInput);
    }

    let host = if let Some((host, port)) = authority.rsplit_once(':') {
        if host.contains(':') || port.is_empty() || port.len() > 1 && port.starts_with('0') {
            return Err(Error::InvalidInput);
        }
        let port = port.parse::<u16>().map_err(|_| Error::InvalidInput)?;
        if port == 0 || port == 443 {
            return Err(Error::InvalidInput);
        }
        host
    } else {
        authority
    };
    if host.is_empty()
        || host.starts_with('.')
        || host.ends_with('.')
        || host.split('.').any(str::is_empty)
        || host.split('.').any(|label| {
            label.starts_with('-')
                || label.ends_with('-')
                || !label
                    .as_bytes()
                    .iter()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
        })
    {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

fn join_request_payload(
    request: &pb::BitBoxSyncSignJoinRequestIntentRequest,
    keys: &IdentityKeys,
) -> Result<Vec<u8>, Error> {
    if request.namespace_id.len() != NAMESPACE_ID_LEN || request.invite_id.len() != INVITE_ID_LEN {
        return Err(Error::InvalidInput);
    }
    validate_server_origin(&request.server_origin)?;

    let server_origin_hash: [u8; SERVER_ORIGIN_HASH_LEN] =
        Sha256::digest(request.server_origin.as_bytes()).into();
    let key_id = key_id(&keys.auth_public_key);
    let mut out = Vec::with_capacity(
        JOIN_REQUEST_PREFIX.len()
            + 1
            + NAMESPACE_ID_LEN
            + INVITE_ID_LEN
            + SERVER_ORIGIN_HASH_LEN
            + 1
            + KEY_ID_LEN
            + keys.auth_public_key.len()
            + keys.wrap_public_key.len()
            + 8,
    );
    out.extend_from_slice(JOIN_REQUEST_PREFIX);
    out.push(JOIN_REQUEST_VERSION);
    out.extend_from_slice(&request.namespace_id);
    out.extend_from_slice(&request.invite_id);
    out.extend_from_slice(&server_origin_hash);
    out.push(KIND_CODE_KEYSTORE);
    out.extend_from_slice(&key_id);
    out.extend_from_slice(&keys.auth_public_key);
    out.extend_from_slice(&keys.wrap_public_key);
    out.extend_from_slice(&request.expires_at.to_be_bytes());
    Ok(out)
}

fn format_fingerprint(bytes: &[u8]) -> String {
    let encoded = hex::encode_upper(bytes);
    format!("{} {}", &encoded[..4], &encoded[4..])
}

fn namespace_fingerprint(namespace_id: &[u8]) -> Result<String, Error> {
    if namespace_id.len() != NAMESPACE_ID_LEN {
        return Err(Error::InvalidInput);
    }
    let mut hasher = Sha256::new();
    hasher.update(b"BitBoxSync namespace fingerprint v1");
    hasher.update(namespace_id);
    let hash: [u8; 32] = hasher.finalize().into();
    Ok(format_fingerprint(&hash[..4]))
}

fn invite_fingerprint(namespace_id: &[u8], invite_id: &[u8]) -> Result<String, Error> {
    if namespace_id.len() != NAMESPACE_ID_LEN || invite_id.len() != INVITE_ID_LEN {
        return Err(Error::InvalidInput);
    }
    let mut hasher = Sha256::new();
    hasher.update(b"BitBoxSync invite fingerprint v1");
    hasher.update(namespace_id);
    hasher.update(invite_id);
    let hash: [u8; 32] = hasher.finalize().into();
    Ok(format_fingerprint(&hash[..4]))
}

fn format_expiry(expires_at: u64) -> Result<String, Error> {
    if expires_at > u32::MAX as u64 {
        return Err(Error::InvalidInput);
    }
    let expiry = util::datetime::format_datetime(expires_at as u32, 0, false)
        .map_err(|_| Error::InvalidInput)?;
    Ok(format!("{} UTC", expiry))
}

async fn confirm_create_namespace_invite(
    hal: &mut impl crate::hal::Hal,
    invite: &str,
    namespace: &str,
    expires_at: u64,
    max_accepted: u32,
) -> Result<(), Error> {
    let expiry = format_expiry(expires_at)?;
    hal.ui()
        .confirm(&ConfirmParams {
            title: "BitBoxSync",
            body: &format!("Create invite\nwith code\n{}", invite),
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    hal.ui()
        .confirm(&ConfirmParams {
            title: "BitBoxSync",
            body: &format!("Namespace\n{}", namespace),
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    hal.ui()
        .confirm(&ConfirmParams {
            title: "BitBoxSync",
            body: &format!("Expires\n{}", expiry),
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    hal.ui()
        .confirm(&ConfirmParams {
            title: "BitBoxSync",
            body: &format!("Max members: {}", max_accepted),
            longtouch: true,
            ..Default::default()
        })
        .await?;
    Ok(())
}

async fn confirm_join_request(
    hal: &mut impl crate::hal::Hal,
    invite: &str,
    server_origin: &str,
    namespace: &str,
    expires_at: u64,
) -> Result<(), Error> {
    let expiry = format_expiry(expires_at)?;
    hal.ui()
        .confirm(&ConfirmParams {
            title: "BitBoxSync",
            body: &format!("Join namespace\nwith code\n{}", invite),
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    hal.ui()
        .confirm(&ConfirmParams {
            title: "BitBoxSync",
            body: &format!("Server: {}", server_origin),
            scrollable: true,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    hal.ui()
        .confirm(&ConfirmParams {
            title: "BitBoxSync",
            body: &format!("Namespace\n{}", namespace),
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    hal.ui()
        .confirm(&ConfirmParams {
            title: "BitBoxSync",
            body: &format!("Expires\n{}", expiry),
            longtouch: true,
            ..Default::default()
        })
        .await?;
    Ok(())
}

fn sign_payload(keys: &IdentityKeys, payload: &[u8]) -> [u8; SIGNATURE_LEN] {
    let expanded_secret_key = ed25519::expanded_secret_key_from_seed(&keys.auth_seed);
    ed25519::sign_with_expanded_secret_key(&expanded_secret_key, payload).signature
}

fn signature_response(signature: [u8; SIGNATURE_LEN]) -> Response {
    Response::BitboxSync(pb::BitBoxSyncResponse {
        response: Some(pb::bit_box_sync_response::Response::Signature(
            pb::BitBoxSyncSignatureResponse {
                signature: signature.to_vec(),
            },
        )),
    })
}

fn labeled_extract(
    suite_id: &[u8],
    salt: Option<&[u8]>,
    label: &[u8],
    ikm: &[u8],
) -> Result<Zeroizing<[u8; 32]>, Error> {
    let mut labeled_ikm = Zeroizing::new(Vec::with_capacity(
        b"HPKE-v1".len() + suite_id.len() + label.len() + ikm.len(),
    ));
    labeled_ikm.extend_from_slice(b"HPKE-v1");
    labeled_ikm.extend_from_slice(suite_id);
    labeled_ikm.extend_from_slice(label);
    labeled_ikm.extend_from_slice(ikm);
    let (mut prk, _) = Hkdf::<Sha256>::extract(salt, &labeled_ikm);
    let mut out = Zeroizing::new([0u8; 32]);
    out.copy_from_slice(&prk);
    prk.zeroize();
    Ok(out)
}

fn labeled_expand(
    suite_id: &[u8],
    prk: &[u8],
    label: &[u8],
    info: &[u8],
    out: &mut [u8],
) -> Result<(), Error> {
    let mut labeled_info =
        Vec::with_capacity(2 + b"HPKE-v1".len() + suite_id.len() + label.len() + info.len());
    labeled_info.extend_from_slice(&(out.len() as u16).to_be_bytes());
    labeled_info.extend_from_slice(b"HPKE-v1");
    labeled_info.extend_from_slice(suite_id);
    labeled_info.extend_from_slice(label);
    labeled_info.extend_from_slice(info);
    Hkdf::<Sha256>::from_prk(prk)
        .map_err(|_| Error::Generic)?
        .expand(&labeled_info, out)
        .map_err(|_| Error::Generic)
}

fn hpke_shared_secret(
    recipient_secret_key: &[u8; 32],
    recipient_public_key: &[u8; 32],
    enc: &[u8],
) -> Result<Zeroizing<[u8; 32]>, Error> {
    if enc.len() != 32 {
        return Err(Error::InvalidInput);
    }
    let mut enc_array = [0u8; 32];
    enc_array.copy_from_slice(enc);

    let recipient_secret = X25519StaticSecret::from(*recipient_secret_key);
    let enc_public = X25519PublicKey::from(enc_array);
    let dh = Zeroizing::new(recipient_secret.diffie_hellman(&enc_public).to_bytes());
    if dh.iter().all(|byte| *byte == 0) {
        return Err(Error::InvalidInput);
    }

    let eae_prk = labeled_extract(b"KEM\x00\x20", None, b"eae_prk", &dh[..])?;
    let mut kem_context = Vec::with_capacity(64);
    kem_context.extend_from_slice(enc);
    kem_context.extend_from_slice(recipient_public_key);

    let mut shared_secret = Zeroizing::new([0u8; 32]);
    labeled_expand(
        b"KEM\x00\x20",
        &eae_prk[..],
        b"shared_secret",
        &kem_context,
        &mut *shared_secret,
    )?;
    Ok(shared_secret)
}

fn hpke_open(
    recipient_secret_key: &[u8; 32],
    recipient_public_key: &[u8; 32],
    enc: &[u8],
    ciphertext: &[u8],
) -> Result<Zeroizing<[u8; NAMESPACE_ID_LEN + NAMESPACE_DEK_LEN]>, Error> {
    if ciphertext.len() != NAMESPACE_ID_LEN + NAMESPACE_DEK_LEN + 16 {
        return Err(Error::InvalidInput);
    }
    let shared_secret = hpke_shared_secret(recipient_secret_key, recipient_public_key, enc)?;

    let psk_id_hash = labeled_extract(b"HPKE\x00\x20\x00\x01\x00\x03", None, b"psk_id_hash", b"")?;
    let info_hash = labeled_extract(
        b"HPKE\x00\x20\x00\x01\x00\x03",
        None,
        b"info_hash",
        WRAP_DEK_INFO,
    )?;
    let mut key_schedule_context = Vec::with_capacity(1 + 32 + 32);
    key_schedule_context.push(0x00);
    key_schedule_context.extend_from_slice(&psk_id_hash[..]);
    key_schedule_context.extend_from_slice(&info_hash[..]);

    let secret = labeled_extract(
        b"HPKE\x00\x20\x00\x01\x00\x03",
        Some(&shared_secret[..]),
        b"secret",
        b"",
    )?;

    let mut key = Zeroizing::new([0u8; 32]);
    labeled_expand(
        b"HPKE\x00\x20\x00\x01\x00\x03",
        &secret[..],
        b"key",
        &key_schedule_context,
        &mut *key,
    )?;
    let mut nonce = Zeroizing::new([0u8; 12]);
    labeled_expand(
        b"HPKE\x00\x20\x00\x01\x00\x03",
        &secret[..],
        b"base_nonce",
        &key_schedule_context,
        &mut *nonce,
    )?;

    let mut plaintext = Zeroizing::new([0u8; NAMESPACE_ID_LEN + NAMESPACE_DEK_LEN]);
    let plaintext_len = plaintext.len();
    plaintext.copy_from_slice(&ciphertext[..plaintext_len]);
    let tag = &ciphertext[plaintext_len..];
    chacha20poly1305::ChaCha20Poly1305::new((&*key).into())
        .decrypt_in_place_detached((&*nonce).into(), b"", &mut *plaintext, tag.into())
        .map_err(|_| Error::InvalidInput)?;
    Ok(plaintext)
}

fn unwrap_namespace_dek(
    keys: &IdentityKeys,
    namespace_id: &[u8],
    wrapped_dek: &[u8],
) -> Result<Zeroizing<[u8; NAMESPACE_DEK_LEN]>, Error> {
    if namespace_id.len() != NAMESPACE_ID_LEN
        || wrapped_dek.len() != WRAPPED_DEK_LEN_V1
        || wrapped_dek[0] != WRAPPED_DEK_VERSION
    {
        return Err(Error::InvalidInput);
    }
    let enc = &wrapped_dek[1..33];
    let ciphertext = &wrapped_dek[33..];
    let plaintext = hpke_open(
        &keys.wrap_secret_key,
        &keys.wrap_public_key,
        enc,
        ciphertext,
    )?;
    if &plaintext[..NAMESPACE_ID_LEN] != namespace_id {
        return Err(Error::InvalidInput);
    }

    let mut namespace_dek = Zeroizing::new([0u8; NAMESPACE_DEK_LEN]);
    namespace_dek.copy_from_slice(&plaintext[NAMESPACE_ID_LEN..]);
    Ok(namespace_dek)
}

pub async fn process(
    hal: &mut impl crate::hal::Hal,
    request: &pb::BitBoxSyncRequest,
) -> Result<Response, Error> {
    let request = request.request.as_ref().ok_or(Error::InvalidInput)?;
    let keys = identity_keys(hal).await?;
    match request {
        pb::bit_box_sync_request::Request::Identity(_) => {
            Ok(Response::BitboxSync(pb::BitBoxSyncResponse {
                response: Some(pb::bit_box_sync_response::Response::Identity(
                    pb::BitBoxSyncIdentityResponse {
                        auth_public_key: keys.auth_public_key.to_vec(),
                        wrap_public_key: keys.wrap_public_key.to_vec(),
                    },
                )),
            }))
        }
        pb::bit_box_sync_request::Request::SignLoginIntent(request) => {
            let payload = login_intent(&request.challenge, &keys)?;
            hal.ui()
                .confirm(&ConfirmParams {
                    title: "BitBoxSync",
                    body: "Login",
                    longtouch: true,
                    ..Default::default()
                })
                .await?;
            Ok(signature_response(sign_payload(&keys, &payload)))
        }
        pb::bit_box_sync_request::Request::SignRefreshIntent(request) => {
            let payload = refresh_intent(&request.challenge, &keys)?;
            hal.ui()
                .confirm(&ConfirmParams {
                    title: "BitBoxSync",
                    body: "Refresh session",
                    longtouch: true,
                    ..Default::default()
                })
                .await?;
            Ok(signature_response(sign_payload(&keys, &payload)))
        }
        pb::bit_box_sync_request::Request::SignRevokeAllTokensIntent(request) => {
            let payload = revoke_all_tokens_intent(&request.challenge, &keys)?;
            hal.ui()
                .confirm(&ConfirmParams {
                    title: "BitBoxSync",
                    body: "Revoke all sessions",
                    longtouch: true,
                    ..Default::default()
                })
                .await?;
            Ok(signature_response(sign_payload(&keys, &payload)))
        }
        pb::bit_box_sync_request::Request::SignCreateNamespaceInviteIntent(request) => {
            let payload = create_namespace_invite_intent(request, &keys)?;
            let namespace = namespace_fingerprint(&request.namespace_id)?;
            let invite = invite_fingerprint(&request.namespace_id, &request.invite_id)?;
            confirm_create_namespace_invite(
                hal,
                &invite,
                &namespace,
                request.expires_at,
                request.max_accepted,
            )
            .await?;
            Ok(signature_response(sign_payload(&keys, &payload)))
        }
        pb::bit_box_sync_request::Request::SignJoinRequestIntent(request) => {
            let payload = join_request_payload(request, &keys)?;
            let namespace = namespace_fingerprint(&request.namespace_id)?;
            let invite = invite_fingerprint(&request.namespace_id, &request.invite_id)?;
            confirm_join_request(
                hal,
                &invite,
                &request.server_origin,
                &namespace,
                request.expires_at,
            )
            .await?;
            Ok(signature_response(sign_payload(&keys, &payload)))
        }
        pb::bit_box_sync_request::Request::UnwrapNamespaceDek(request) => {
            // Deliberately no confirmation prompt: an unlocked BitBox being present is sufficient
            // to recover a missing namespace DEK without interrupting normal sync flows.
            let namespace_dek =
                unwrap_namespace_dek(&keys, &request.namespace_id, &request.wrapped_dek)?;
            Ok(Response::BitboxSync(pb::BitBoxSyncResponse {
                response: Some(pb::bit_box_sync_response::Response::UnwrapNamespaceDek(
                    pb::BitBoxSyncUnwrapNamespaceDekResponse {
                        namespace_dek: namespace_dek.to_vec(),
                    },
                )),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;
    use crate::keystore::testing::mock_unlocked_using_mnemonic;
    use hex_lit::hex;

    const MNEMONIC: &str = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";

    async fn unlocked_hal() -> TestingHal<'static> {
        mock_unlocked_using_mnemonic(MNEMONIC, "");
        TestingHal::new()
    }

    fn fixed_identity_keys() -> IdentityKeys {
        let auth_seed = [0x01u8; 32];
        let auth_public_key = ed25519_dalek::SigningKey::from_bytes(&auth_seed)
            .verifying_key()
            .to_bytes();
        let wrap_secret_key = Zeroizing::new([0x02u8; 32]);
        let wrap_secret = X25519StaticSecret::from(*wrap_secret_key);
        let wrap_public_key = X25519PublicKey::from(&wrap_secret).to_bytes();
        IdentityKeys {
            auth_seed: Zeroizing::new(auth_seed),
            auth_public_key,
            wrap_secret_key,
            wrap_public_key,
        }
    }

    #[test]
    fn test_identity_seed_derivation_uses_hkdf_info_labels() {
        let mut root = [0u8; 32];
        for (idx, byte) in root.iter_mut().enumerate() {
            *byte = idx as u8;
        }

        assert_eq!(
            &derive_labeled_key(&root, AUTH_SEED_LABEL).unwrap()[..],
            &hex!("916d5481be358ccd57c8cde0184005c6a29a135eb40c74d4a2718890c66dc437")
        );
        assert_eq!(
            &derive_labeled_key(&root, WRAP_SEED_LABEL).unwrap()[..],
            &hex!("f36acf48115db6a49209bf8f9592a3f42d3c23aeb1072c2f7fc813df78c412e0")
        );
    }

    #[test]
    fn test_canonical_payload_vectors() {
        let keys = fixed_identity_keys();
        assert_eq!(
            login_intent(&[0x10u8; CHALLENGE_LEN], &keys).unwrap(),
            hex!(
                "626974626f7873796e632d696e74656e74010110101010101010101010101010101010101010101010101010101010101010100134750f98bd59fcfc946da45aaabe933be154a4b5094e1c4abf42866505f3c97e8a88e3dd7409f195fd52db2d3cba5d72ca6709bf1d94121bf3748801b40f6f5cce8d3ad1ccb633ec7b70c17814a5c76ecd029685050d344745ba05870e587d59"
            )
        );
        assert_eq!(
            refresh_intent(&[0x11u8; CHALLENGE_LEN], &keys).unwrap(),
            hex!(
                "626974626f7873796e632d696e74656e74010211111111111111111111111111111111111111111111111111111111111111110134750f98bd59fcfc946da45aaabe933be154a4b5094e1c4abf42866505f3c97e"
            )
        );
        assert_eq!(
            revoke_all_tokens_intent(&[0x12u8; CHALLENGE_LEN], &keys).unwrap(),
            hex!(
                "626974626f7873796e632d696e74656e7401030112121212121212121212121212121212121212121212121212121212121212120134750f98bd59fcfc946da45aaabe933be154a4b5094e1c4abf42866505f3c97e"
            )
        );

        let create_request = pb::BitBoxSyncSignCreateNamespaceInviteIntentRequest {
            challenge: vec![0x13u8; CHALLENGE_LEN],
            namespace_id: vec![0x20u8; NAMESPACE_ID_LEN],
            invite_id: vec![0x21u8; INVITE_ID_LEN],
            invite_server_secret_hash: vec![0x22u8; INVITE_SERVER_SECRET_HASH_LEN],
            expires_at: 0x0102_0304_0506_0708,
            max_accepted: 10,
        };
        assert_eq!(
            create_namespace_invite_intent(&create_request, &keys).unwrap(),
            hex!(
                "626974626f7873796e632d696e74656e7401030213131313131313131313131313131313131313131313131313131313131313130134750f98bd59fcfc946da45aaabe933be154a4b5094e1c4abf42866505f3c97e2020202020202020202020202020202021212121212121212121212121212121222222222222222222222222222222222222222222222222222222222222222201020304050607080000000a"
            )
        );

        let join_request = pb::BitBoxSyncSignJoinRequestIntentRequest {
            namespace_id: vec![0x20u8; NAMESPACE_ID_LEN],
            invite_id: vec![0x21u8; INVITE_ID_LEN],
            server_origin: "https://sync.example".into(),
            expires_at: 0x0102_0304_0506_0708,
        };
        assert_eq!(
            join_request_payload(&join_request, &keys).unwrap(),
            hex!(
                "626974626f7873796e632d6a6f696e2d72657175657374012020202020202020202020202020202021212121212121212121212121212121ff93ec0a47d8af4a6dc161a681c24d41e1a2ddb9ea6d5c9dc55b106f4c1b6c150134750f98bd59fcfc946da45aaabe933be154a4b5094e1c4abf42866505f3c97e8a88e3dd7409f195fd52db2d3cba5d72ca6709bf1d94121bf3748801b40f6f5cce8d3ad1ccb633ec7b70c17814a5c76ecd029685050d344745ba05870e587d590102030405060708"
            )
        );
    }

    #[test]
    fn test_hpke_shared_secret_rejects_low_order_enc() {
        let recipient_secret_key = [7u8; 32];
        let recipient_secret = X25519StaticSecret::from(recipient_secret_key);
        let recipient_public_key = X25519PublicKey::from(&recipient_secret).to_bytes();

        assert_eq!(
            hpke_shared_secret(&recipient_secret_key, &recipient_public_key, &[0u8; 32]),
            Err(Error::InvalidInput)
        );
    }

    #[test]
    fn test_unwrap_namespace_dek_vector() {
        let keys = fixed_identity_keys();
        let namespace_id = [0x20u8; NAMESPACE_ID_LEN];
        let wrapped_dek = hex!(
            "015dfedd3b6bd47f6fa28ee15d969d5bb0ea53774d488bdaf9df1c6e0124b3ef227e6fcdf85c2247224b9d8abf9de548438723773828c60f0e6e70428f1433253430bf700131cf4a5209108789961845a0c57925575dc6cd8d1cdf167d2e9de498"
        );

        assert_eq!(
            &unwrap_namespace_dek(&keys, &namespace_id, &wrapped_dek).unwrap()[..],
            &[0x24u8; NAMESPACE_DEK_LEN]
        );

        assert_eq!(
            unwrap_namespace_dek(&keys, &[0x25u8; NAMESPACE_ID_LEN], &wrapped_dek),
            Err(Error::InvalidInput)
        );
        let mut tampered = wrapped_dek;
        tampered[WRAPPED_DEK_LEN_V1 - 1] ^= 0x01;
        assert_eq!(
            unwrap_namespace_dek(&keys, &namespace_id, &tampered),
            Err(Error::InvalidInput)
        );
    }

    #[test]
    fn test_server_origin_validation_rejects_noncanonical_ports() {
        assert!(validate_server_origin("https://sync.example").is_ok());
        assert!(validate_server_origin("https://sync.example:8443").is_ok());
        assert!(
            validate_server_origin(&format!(
                "https://{}",
                "a".repeat(MAX_SERVER_ORIGIN_LEN - "https://".len())
            ))
            .is_ok()
        );
        assert_eq!(
            validate_server_origin(&format!(
                "https://{}",
                "a".repeat(MAX_SERVER_ORIGIN_LEN - "https://".len() + 1)
            )),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            validate_server_origin("https://sync.example:443"),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            validate_server_origin("https://sync.example:0443"),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            validate_server_origin("https://sync.example:0001"),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            validate_server_origin("https://bad-.example"),
            Err(Error::InvalidInput)
        );
    }

    fn bitboxsync_response(response: Response) -> pb::bit_box_sync_response::Response {
        match response {
            Response::BitboxSync(response) => response.response.unwrap(),
            _ => panic!("unexpected response"),
        }
    }

    fn signature(response: Response) -> Vec<u8> {
        match bitboxsync_response(response) {
            pb::bit_box_sync_response::Response::Signature(response) => response.signature,
            _ => panic!("unexpected response"),
        }
    }

    fn identity(response: Response) -> pb::BitBoxSyncIdentityResponse {
        match bitboxsync_response(response) {
            pb::bit_box_sync_response::Response::Identity(response) => response,
            _ => panic!("unexpected response"),
        }
    }

    #[async_test::test]
    async fn test_identity_is_deterministic() {
        let mut hal = unlocked_hal().await;
        let response = identity(
            process(
                &mut hal,
                &pb::BitBoxSyncRequest {
                    request: Some(pb::bit_box_sync_request::Request::Identity(
                        pb::BitBoxSyncIdentityRequest {},
                    )),
                },
            )
            .await
            .unwrap(),
        );
        assert_eq!(response.auth_public_key.len(), 32);
        assert_eq!(response.wrap_public_key.len(), 32);

        let mut hal = unlocked_hal().await;
        let response_again = identity(
            process(
                &mut hal,
                &pb::BitBoxSyncRequest {
                    request: Some(pb::bit_box_sync_request::Request::Identity(
                        pb::BitBoxSyncIdentityRequest {},
                    )),
                },
            )
            .await
            .unwrap(),
        );
        assert_eq!(response, response_again);
        assert!(hal.ui.screens.is_empty());
    }

    #[async_test::test]
    async fn test_sign_login_confirms_and_signs_canonical_payload() {
        let mut hal = unlocked_hal().await;
        let keys = identity_keys(&mut hal).await.unwrap();
        let challenge = [42u8; CHALLENGE_LEN];
        let expected_payload = login_intent(&challenge, &keys).unwrap();

        let sig = signature(
            process(
                &mut hal,
                &pb::BitBoxSyncRequest {
                    request: Some(pb::bit_box_sync_request::Request::SignLoginIntent(
                        pb::BitBoxSyncSignLoginIntentRequest {
                            challenge: challenge.to_vec(),
                        },
                    )),
                },
            )
            .await
            .unwrap(),
        );
        assert_eq!(
            hal.ui.screens,
            vec![Screen::Confirm {
                title: "BitBoxSync".into(),
                body: "Login".into(),
                longtouch: true,
            }]
        );
        let public_key = ed25519_dalek::VerifyingKey::from_bytes(&keys.auth_public_key).unwrap();
        public_key
            .verify_strict(
                &expected_payload,
                &ed25519_dalek::Signature::from_slice(&sig).unwrap(),
            )
            .unwrap();
    }

    #[async_test::test]
    async fn test_signing_validates_lengths_before_confirming() {
        let mut hal = unlocked_hal().await;
        assert_eq!(
            process(
                &mut hal,
                &pb::BitBoxSyncRequest {
                    request: Some(pb::bit_box_sync_request::Request::SignLoginIntent(
                        pb::BitBoxSyncSignLoginIntentRequest {
                            challenge: vec![0u8; CHALLENGE_LEN - 1],
                        },
                    )),
                },
            )
            .await,
            Err(Error::InvalidInput)
        );
        assert_eq!(
            process(
                &mut hal,
                &pb::BitBoxSyncRequest {
                    request: Some(
                        pb::bit_box_sync_request::Request::SignCreateNamespaceInviteIntent(
                            pb::BitBoxSyncSignCreateNamespaceInviteIntentRequest {
                                challenge: vec![0u8; CHALLENGE_LEN],
                                namespace_id: vec![0u8; NAMESPACE_ID_LEN - 1],
                                invite_id: vec![0u8; INVITE_ID_LEN],
                                invite_server_secret_hash: vec![0u8; INVITE_SERVER_SECRET_HASH_LEN],
                                expires_at: 123,
                                max_accepted: 1,
                            },
                        ),
                    ),
                },
            )
            .await,
            Err(Error::InvalidInput)
        );
        assert!(hal.ui.screens.is_empty());
    }

    #[async_test::test]
    async fn test_create_namespace_invite_confirms_multiple_screens() {
        let mut hal = unlocked_hal().await;
        let keys = identity_keys(&mut hal).await.unwrap();
        let request = pb::BitBoxSyncSignCreateNamespaceInviteIntentRequest {
            challenge: vec![7u8; CHALLENGE_LEN],
            namespace_id: vec![1u8; NAMESPACE_ID_LEN],
            invite_id: vec![2u8; INVITE_ID_LEN],
            invite_server_secret_hash: vec![3u8; INVITE_SERVER_SECRET_HASH_LEN],
            expires_at: 123,
            max_accepted: 5,
        };
        let expected_payload = create_namespace_invite_intent(&request, &keys).unwrap();

        let sig = signature(
            process(
                &mut hal,
                &pb::BitBoxSyncRequest {
                    request: Some(
                        pb::bit_box_sync_request::Request::SignCreateNamespaceInviteIntent(request),
                    ),
                },
            )
            .await
            .unwrap(),
        );
        assert_eq!(
            hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "BitBoxSync".into(),
                    body: "Create invite\nwith code\n6242 1288".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "BitBoxSync".into(),
                    body: "Namespace\n3208 8CFF".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "BitBoxSync".into(),
                    body: "Expires\nThu 1970-01-01\n00:02 UTC".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "BitBoxSync".into(),
                    body: "Max members: 5".into(),
                    longtouch: true,
                },
            ]
        );
        let public_key = ed25519_dalek::VerifyingKey::from_bytes(&keys.auth_public_key).unwrap();
        public_key
            .verify_strict(
                &expected_payload,
                &ed25519_dalek::Signature::from_slice(&sig).unwrap(),
            )
            .unwrap();
    }

    #[async_test::test]
    async fn test_join_request_confirms_multiple_screens() {
        let mut hal = unlocked_hal().await;
        let keys = identity_keys(&mut hal).await.unwrap();
        let request = pb::BitBoxSyncSignJoinRequestIntentRequest {
            namespace_id: vec![1u8; NAMESPACE_ID_LEN],
            invite_id: vec![2u8; INVITE_ID_LEN],
            server_origin: "https://sync.example".into(),
            expires_at: 123,
        };
        let expected_payload = join_request_payload(&request, &keys).unwrap();

        let sig = signature(
            process(
                &mut hal,
                &pb::BitBoxSyncRequest {
                    request: Some(pb::bit_box_sync_request::Request::SignJoinRequestIntent(
                        request,
                    )),
                },
            )
            .await
            .unwrap(),
        );
        assert_eq!(
            hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "BitBoxSync".into(),
                    body: "Join namespace\nwith code\n6242 1288".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "BitBoxSync".into(),
                    body: "Server: https://sync.example".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "BitBoxSync".into(),
                    body: "Namespace\n3208 8CFF".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "BitBoxSync".into(),
                    body: "Expires\nThu 1970-01-01\n00:02 UTC".into(),
                    longtouch: true,
                },
            ]
        );
        let public_key = ed25519_dalek::VerifyingKey::from_bytes(&keys.auth_public_key).unwrap();
        public_key
            .verify_strict(
                &expected_payload,
                &ed25519_dalek::Signature::from_slice(&sig).unwrap(),
            )
            .unwrap();
    }

    #[async_test::test]
    async fn test_signing_rejects_expiry_that_cannot_be_displayed() {
        let mut hal = unlocked_hal().await;
        assert_eq!(
            process(
                &mut hal,
                &pb::BitBoxSyncRequest {
                    request: Some(
                        pb::bit_box_sync_request::Request::SignCreateNamespaceInviteIntent(
                            pb::BitBoxSyncSignCreateNamespaceInviteIntentRequest {
                                challenge: vec![0u8; CHALLENGE_LEN],
                                namespace_id: vec![0u8; NAMESPACE_ID_LEN],
                                invite_id: vec![0u8; INVITE_ID_LEN],
                                invite_server_secret_hash: vec![0u8; INVITE_SERVER_SECRET_HASH_LEN],
                                expires_at: u64::from(u32::MAX) + 1,
                                max_accepted: 1,
                            },
                        ),
                    ),
                },
            )
            .await,
            Err(Error::InvalidInput)
        );
        assert_eq!(
            process(
                &mut hal,
                &pb::BitBoxSyncRequest {
                    request: Some(pb::bit_box_sync_request::Request::SignJoinRequestIntent(
                        pb::BitBoxSyncSignJoinRequestIntentRequest {
                            namespace_id: vec![0u8; NAMESPACE_ID_LEN],
                            invite_id: vec![0u8; INVITE_ID_LEN],
                            server_origin: "https://sync.example".into(),
                            expires_at: u64::from(u32::MAX) + 1,
                        },
                    )),
                },
            )
            .await,
            Err(Error::InvalidInput)
        );
        assert!(hal.ui.screens.is_empty());
    }
}
