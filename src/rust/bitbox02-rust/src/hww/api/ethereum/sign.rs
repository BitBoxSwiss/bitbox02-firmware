// SPDX-License-Identifier: Apache-2.0

use super::Error;
use super::amount::{Amount, calculate_percentage};
use super::params::Params;
use super::pb;
use crate::hal::ui::ConfirmParams;

use crate::keystore;

use crate::hal::Ui;
use crate::workflow::transaction;

use alloc::vec::Vec;
use hex_lit::hex;
use pb::eth_response::Response;

use core::ops::{Add, Mul};
use num_bigint::BigUint;

// 1 ETH = 1e18 wei.
const WEI_DECIMALS: usize = 18;

pub enum Transaction<'a> {
    Legacy(&'a pb::EthSignRequest),
    Eip1559(&'a pb::EthSignEip1559Request),
}

impl Transaction<'_> {
    fn nonce(&self) -> &[u8] {
        match self {
            Transaction::Legacy(legacy) => &legacy.nonce,
            Transaction::Eip1559(eip1559) => &eip1559.nonce,
        }
    }
    fn gas_limit(&self) -> &[u8] {
        match self {
            Transaction::Legacy(legacy) => &legacy.gas_limit,
            Transaction::Eip1559(eip1559) => &eip1559.gas_limit,
        }
    }
    fn recipient(&self) -> &[u8] {
        match self {
            Transaction::Legacy(legacy) => &legacy.recipient,
            Transaction::Eip1559(eip1559) => &eip1559.recipient,
        }
    }
    fn value(&self) -> &[u8] {
        match self {
            Transaction::Legacy(legacy) => &legacy.value,
            Transaction::Eip1559(eip1559) => &eip1559.value,
        }
    }
    fn data(&self) -> &[u8] {
        match self {
            Transaction::Legacy(legacy) => &legacy.data,
            Transaction::Eip1559(eip1559) => &eip1559.data,
        }
    }
    fn chain_id(&self) -> u64 {
        match self {
            Transaction::Legacy(legacy) => legacy.chain_id,
            Transaction::Eip1559(eip1559) => eip1559.chain_id,
        }
    }
    fn keypath(&self) -> &[u32] {
        match self {
            Transaction::Legacy(legacy) => &legacy.keypath,
            Transaction::Eip1559(eip1559) => &eip1559.keypath,
        }
    }
    fn host_nonce_commitment(&self) -> Option<&pb::AntiKleptoHostNonceCommitment> {
        match self {
            Transaction::Legacy(legacy) => legacy.host_nonce_commitment.as_ref(),
            Transaction::Eip1559(eip1559) => eip1559.host_nonce_commitment.as_ref(),
        }
    }
    fn coin(&self) -> Result<Option<pb::EthCoin>, Error> {
        match self {
            Transaction::Legacy(legacy) => Ok(Some(pb::EthCoin::try_from(legacy.coin)?)),
            Transaction::Eip1559(_) => Ok(None),
        }
    }
    fn case(&self) -> Result<pb::EthAddressCase, Error> {
        match self {
            Transaction::Legacy(legacy) => Ok(pb::EthAddressCase::try_from(legacy.address_case)?),
            Transaction::Eip1559(eip1559) => {
                Ok(pb::EthAddressCase::try_from(eip1559.address_case)?)
            }
        }
    }
    fn data_length(&self) -> u32 {
        match self {
            Transaction::Legacy(legacy) => legacy.data_length,
            Transaction::Eip1559(eip1559) => eip1559.data_length,
        }
    }
}

/// Converts `recipient` to an array of 20 chars. If `recipient` is
/// not exactly 20 elements, `InvalidInput` is returned.
fn parse_recipient(recipient: &[u8]) -> Result<[u8; 20], Error> {
    recipient.try_into().or(Err(Error::InvalidInput))
}

/// Checks if the transaction is an ERC20 transaction.
/// An ERC20 transaction transacts 0 ETH, but contains an ERC20 transfer method call in the data.
/// The data field must look like:
/// `<0xa9059cbb><32 bytes recipient><32 bytes value>`
/// where recipient 20 bytes (zero padded to 32 bytes), and value is zero padded big endian number.
/// On success, the 20 byte recipient and transaction value are returned.
fn parse_erc20(request: &Transaction<'_>) -> Option<([u8; 20], BigUint)> {
    if !request.value().is_empty() || request.data().len() != 68 {
        return None;
    }
    let (method, recipient, value) = (
        &request.data()[..4],
        &request.data()[4..36],
        &request.data()[36..68],
    );
    if method != hex!("a9059cbb") {
        return None;
    }
    // Recipient must be zero padded.
    if recipient[..12] != [0u8; 12] {
        return None;
    }
    // Transacted value can't be zero.
    if value == [0u8; 32] {
        return None;
    }
    Some((
        recipient[12..].try_into().unwrap(),
        BigUint::from_bytes_be(value),
    ))
}

// For legacy transactions: `fee = gas limit * gas price`
// For 1559 transactions: `fee = gas limit * max fee per gas` where max fee per gas is composed of the base fee + priority fee
// In both instances we show the user the max possible fee, but the actual fee paid at execution might be lower
// That is because:
// 1) actual gas used will often be lower than gas limit (in the case of contract interactions, not simple ETH transfers)
// 2) in the case of 1559 base fee at execution time might also be lower so that `base fee + priority fee < max fee per gas`
fn parse_fee<'a>(request: &Transaction<'_>, params: &'a Params) -> Amount<'a> {
    let gas_limit = BigUint::from_bytes_be(request.gas_limit());
    match request {
        Transaction::Legacy(legacy) => {
            let gas_price = BigUint::from_bytes_be(&legacy.gas_price);
            Amount {
                unit: params.unit,
                decimals: WEI_DECIMALS,
                value: gas_price.mul(gas_limit),
            }
        }
        Transaction::Eip1559(eip1559) => {
            let max_fee_per_gas = BigUint::from_bytes_be(&eip1559.max_fee_per_gas);
            Amount {
                unit: params.unit,
                decimals: WEI_DECIMALS,
                value: max_fee_per_gas.mul(gas_limit),
            }
        }
    }
}

async fn hash_legacy(chain_id: u64, request: &pb::EthSignRequest) -> Result<[u8; 32], Error> {
    let mut producer = if request.data_length > 0 {
        super::sighash::ChunkingProducer::from_host(request.data_length)
    } else {
        super::sighash::ChunkingProducer::from_data(&request.data)
    };
    let mut params = super::sighash::ParamsLegacy {
        nonce: &request.nonce,
        gas_price: &request.gas_price,
        gas_limit: &request.gas_limit,
        recipient: &request.recipient,
        value: &request.value,
        data: &mut producer,
        chain_id,
    };
    super::sighash::compute_legacy(&mut params)
        .await
        .map_err(|_| Error::InvalidInput)
}

async fn hash_eip1559(request: &pb::EthSignEip1559Request) -> Result<[u8; 32], Error> {
    let mut producer = if request.data_length > 0 {
        super::sighash::ChunkingProducer::from_host(request.data_length)
    } else {
        super::sighash::ChunkingProducer::from_data(&request.data)
    };
    let mut params = super::sighash::ParamsEIP1559 {
        chain_id: request.chain_id,
        nonce: &request.nonce,
        max_priority_fee_per_gas: &request.max_priority_fee_per_gas,
        max_fee_per_gas: &request.max_fee_per_gas,
        gas_limit: &request.gas_limit,
        recipient: &request.recipient,
        value: &request.value,
        data: &mut producer,
    };
    super::sighash::compute_eip1559(&mut params)
        .await
        .map_err(|_| Error::InvalidInput)
}

/// Verifies an ERC20 transfer.
///
/// If the ERC20 contract is known (stored in our list of supported ERC20 tokens), the token name,
/// amount, recipient, total and fee are shown for confirmation.
///
/// If the ERC20 token is unknown, only the recipient and fee can be shown. The token name and
/// amount are displayed as "unknown". The amount is not known because we don't know the number of
/// decimal places (specified in the ERC20 contract).
async fn verify_erc20_transaction(
    hal: &mut impl crate::hal::Hal,
    request: &Transaction<'_>,
    params: &Params,
    erc20_recipient: [u8; 20],
    erc20_value: BigUint,
) -> Result<(), Error> {
    let erc20_params = erc20_params::get(params.chain_id, parse_recipient(request.recipient())?);
    let formatted_fee = parse_fee(request, params).format();
    let recipient_address = super::address::from_pubkey_hash(&erc20_recipient, request.case()?);
    let (formatted_value, formatted_total) = match erc20_params {
        Some(erc20_params) => {
            let value = Amount {
                unit: erc20_params.unit,
                decimals: erc20_params.decimals as _,
                value: erc20_value,
            }
            .format();

            // ERC20 token: fee has a different unit (ETH), so the total is just the value again.
            (value.clone(), value.clone())
        }
        None => ("Unknown token".into(), "Unknown amount".into()),
    };
    hal.ui()
        .verify_recipient(&recipient_address, &formatted_value)
        .await?;
    transaction::verify_total_fee_maybe_warn(hal, &formatted_total, &formatted_fee, None).await?;
    Ok(())
}

/// Verifies a standard ETH transaction, meaning that the data field is empty or has unknown
/// contents.
///
/// If the data field is not empty, it will be shown for confirmation as a hex string. This is for
/// experts that know the expected encoding of a smart contract invocation.
///
/// The transacted value, recipient address, total and fee are confirmed.
async fn verify_standard_transaction(
    hal: &mut impl crate::hal::Hal,
    request: &Transaction<'_>,
    params: &Params,
) -> Result<(), Error> {
    let recipient = parse_recipient(request.recipient())?;

    let data_length = request.data_length();

    if !request.data().is_empty() || data_length > 0 {
        hal.ui()
            .confirm(&ConfirmParams {
                title: "Unknown\ncontract",
                body: if data_length > 0 {
                    "You are signing a\ncontract interaction\nwith large data."
                } else {
                    "You will be shown\nthe raw\ntransaction data."
                },
                accept_is_nextarrow: true,
                ..Default::default()
            })
            .await?;
        hal.ui()
            .confirm(&ConfirmParams {
                title: "Unknown\ncontract",
                body: if data_length > 0 {
                    "Only proceed if you\nfully understand\nthe risks involved."
                } else {
                    "Only proceed if you\nunderstand exactly\nwhat the data means."
                },
                accept_is_nextarrow: true,
                ..Default::default()
            })
            .await?;

        if data_length > 0 {
            // Streaming mode: data is too large to display, show size instead
            hal.ui()
                .confirm(&ConfirmParams {
                    title: "Transaction\ndata",
                    body: &alloc::format!("{} bytes\n(too large to\ndisplay)", data_length),
                    accept_is_nextarrow: true,
                    ..Default::default()
                })
                .await?;
        } else {
            // Nonstreaming mode: show hex data
            hal.ui()
                .confirm(&ConfirmParams {
                    title: "Transaction\ndata",
                    body: &hex::encode(request.data()),
                    scrollable: true,
                    display_size: request.data().len(),
                    accept_is_nextarrow: true,
                    ..Default::default()
                })
                .await?;
        }
    }

    let address = super::address::from_pubkey_hash(&recipient, request.case()?);
    let amount = Amount {
        unit: params.unit,
        decimals: WEI_DECIMALS,
        value: BigUint::from_bytes_be(request.value()),
    };
    hal.ui()
        .verify_recipient(&address, &amount.format())
        .await?;

    let fee = parse_fee(request, params);
    let total = Amount {
        unit: params.unit,
        decimals: WEI_DECIMALS,
        value: (&amount.value).add(&fee.value),
    };
    let percentage = calculate_percentage(&fee.value, &amount.value);
    transaction::verify_total_fee_maybe_warn(hal, &total.format(), &fee.format(), percentage)
        .await?;
    Ok(())
}

pub async fn _process(
    hal: &mut impl crate::hal::Hal,
    request: &Transaction<'_>,
) -> Result<Response, Error> {
    let params =
        super::params::get_and_warn_unknown(hal, request.coin()?, request.chain_id()).await?;

    if !super::keypath::is_valid_keypath_address(request.keypath()) {
        return Err(Error::InvalidInput);
    }
    super::keypath::warn_unusual_keypath(hal, &params, params.name, request.keypath()).await?;

    // Show chain confirmation only for known networks
    if super::params::is_known_network(request.coin()?, request.chain_id()) {
        hal.ui()
            .confirm(&ConfirmParams {
                body: &format!("Sign transaction on\n\n{}", params.name),
                accept_is_nextarrow: true,
                ..Default::default()
            })
            .await?;
    }

    // Size limits.
    const MAX_STREAMING_DATA_LENGTH: u32 = 1024 * 1024;
    const MAX_NONSTREAMING_DATA_LENGTH: usize = 6144;
    if request.nonce().len() > 16
        || request.gas_limit().len() > 16
        || request.value().len() > 32
        || request.data().len() > MAX_NONSTREAMING_DATA_LENGTH
        || request.data_length() > MAX_STREAMING_DATA_LENGTH
    {
        return Err(Error::InvalidInput);
    }

    // Can't use both inline data and streaming at the same time.
    if request.data_length() > 0 && !request.data().is_empty() {
        return Err(Error::InvalidInput);
    }

    // No zero prefix in the big endian numbers.
    if let [0, ..] = request.nonce()[..] {
        return Err(Error::InvalidInput);
    }
    if let [0, ..] = request.gas_limit()[..] {
        return Err(Error::InvalidInput);
    }
    if let [0, ..] = request.value()[..] {
        return Err(Error::InvalidInput);
    }

    // size and zero prefix checks for legacy and eip1559 transactions
    match request {
        Transaction::Legacy(legacy) => {
            if let [0, ..] = &legacy.gas_price[..] {
                return Err(Error::InvalidInput);
            }
            if legacy.gas_price.len() > 16 {
                return Err(Error::InvalidInput);
            }
        }
        Transaction::Eip1559(eip1559) => {
            if let [0, ..] = &eip1559.max_priority_fee_per_gas[..] {
                return Err(Error::InvalidInput);
            }
            if let [0, ..] = &eip1559.max_fee_per_gas[..] {
                return Err(Error::InvalidInput);
            }
            if eip1559.max_priority_fee_per_gas.len() > 16 || eip1559.max_fee_per_gas.len() > 16 {
                return Err(Error::InvalidInput);
            }
        }
    }

    let recipient = parse_recipient(request.recipient())?;
    if recipient == [0; 20] {
        // Reserved for contract creation.
        return Err(Error::InvalidInput);
    }

    if let Some((erc20_recipient, erc20_value)) = parse_erc20(request) {
        verify_erc20_transaction(hal, request, &params, erc20_recipient, erc20_value).await?;
    } else {
        verify_standard_transaction(hal, request, &params).await?;
    }
    hal.ui().status("Transaction\nconfirmed", true).await;

    let hash: [u8; 32] = match request {
        Transaction::Legacy(legacy) => hash_legacy(params.chain_id, legacy).await?,
        Transaction::Eip1559(eip1559) => hash_eip1559(eip1559).await?,
    };

    let host_nonce = match request.host_nonce_commitment() {
        // Engage in the anti-klepto protocol if the host sends a host nonce commitment.
        Some(pb::AntiKleptoHostNonceCommitment { commitment }) => {
            let signer_commitment = crate::secp256k1::secp256k1_nonce_commit(
                &keystore::secp256k1_get_private_key(hal, request.keypath())?
                    .as_slice()
                    .try_into()
                    .unwrap(),
                &hash,
                commitment
                    .as_slice()
                    .try_into()
                    .or(Err(Error::InvalidInput))?,
            )?;

            // Send signer commitment to host and wait for the host nonce from the host.
            super::antiklepto_get_host_nonce(signer_commitment).await?
        }

        // Return signature directly without the anti-klepto protocol, for backwards compatibility.
        None => [0; 32],
    };
    let sign_result = crate::secp256k1::secp256k1_sign(
        &keystore::secp256k1_get_private_key(hal, request.keypath())?
            .as_slice()
            .try_into()
            .unwrap(),
        &hash,
        &host_nonce,
    )?;
    let mut signature: Vec<u8> = sign_result.signature.to_vec();
    signature.push(sign_result.recid);

    Ok(Response::Sign(pb::EthSignResponse { signature }))
}

/// Verify and sign an Ethereum transaction.
pub async fn process(
    hal: &mut impl crate::hal::Hal,
    request: &Transaction<'_>,
) -> Result<Response, Error> {
    let result = _process(hal, request).await;
    if let Err(Error::UserAbort) = result {
        hal.ui().status("Transaction\ncanceled", false).await;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;
    use crate::keystore::testing::mock_unlocked;
    use alloc::boxed::Box;
    use util::bb02_async::block_on;
    use util::bip32::HARDENED;

    use super::super::sighash::tests::{clear_chunk_responder, setup_chunk_responder};

    #[test]
    pub fn test_parse_recipient() {
        assert_eq!(
            parse_recipient(b"01234567890123456789"),
            Ok(*b"01234567890123456789"),
        );

        assert_eq!(
            parse_recipient(b"0123456789012345678"),
            Err(Error::InvalidInput),
        );
        assert_eq!(
            parse_recipient(b"012345678901234567890"),
            Err(Error::InvalidInput),
        );
    }

    #[test]
    pub fn test_parse_erc20() {
        let valid_data = hex!(
            "a9059cbb0000000000000000000000006162636465666768696a6b6c6d6e6f707172737400000000000000000000000000000000000000000000000000000055000000ff"
        );
        assert_eq!(
            parse_erc20(&Transaction::Legacy(&pb::EthSignRequest {
                data: valid_data.to_vec(),
                ..Default::default()
            })),
            Some((*b"abcdefghijklmnopqrst", 365072220415u64.into()))
        );

        // ETH value must be 0 when transacting ERC20.
        assert!(
            parse_erc20(&Transaction::Legacy(&pb::EthSignRequest {
                value: vec![0],
                data: valid_data.to_vec(),
                ..Default::default()
            }))
            .is_none()
        );

        // Invalid method (first byte)
        let invalid_data = hex!(
            "a8059cbb0000000000000000000000006162636465666768696a6b6c6d6e6f707172737400000000000000000000000000000000000000000000000000000000000000ff"
        );
        assert!(
            parse_erc20(&Transaction::Legacy(&pb::EthSignRequest {
                data: invalid_data.to_vec(),
                ..Default::default()
            }))
            .is_none()
        );

        // Recipient too long (not zero padded)
        let invalid_data = hex!(
            "a9059cbb0000000000000000000000626162636465666768696a6b6c6d6e6f707172737400000000000000000000000000000000000000000000000000000000000000ff"
        );
        assert!(
            parse_erc20(&Transaction::Legacy(&pb::EthSignRequest {
                data: invalid_data.to_vec(),
                ..Default::default()
            }))
            .is_none()
        );

        // Value can't be zero
        let invalid_data = hex!(
            "a9059cbb0000000000000000000000006162636465666768696a6b6c6d6e6f70717273740000000000000000000000000000000000000000000000000000000000000000"
        );
        assert!(
            parse_erc20(&Transaction::Legacy(&pb::EthSignRequest {
                data: invalid_data.to_vec(),
                ..Default::default()
            }))
            .is_none()
        );
    }

    /// Standard ETH transaction with no data field.
    #[test]
    pub fn test_process_standard_transaction() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        mock_unlocked();
        let expected_screens = vec![
            Screen::Confirm {
                title: "".into(),
                body: "Sign transaction on\n\nEthereum".into(),
                longtouch: false,
            },
            Screen::Recipient {
                recipient: "0x04F264Cf34440313B4A0192A352814FBe927b885".into(),
                amount: "0.530564 ETH".into(),
            },
            Screen::TotalFee {
                total: "0.53069 ETH".into(),
                fee: "0.000126 ETH".into(),
                longtouch: true,
            },
            Screen::Status {
                title: "Transaction\nconfirmed".into(),
                success: true,
            },
        ];

        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &Transaction::Legacy(&pb::EthSignRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                nonce: hex!("1fdc").to_vec(),
                gas_price: hex!("0165a0bc00").to_vec(),
                gas_limit: hex!("5208").to_vec(),
                recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
                value: hex!("075cf1259e9c4000").to_vec(),
                data: b"".to_vec(),
                host_nonce_commitment: None,
                chain_id: 0,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }))),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!("c3ae24c167e216cfb75c72b5e03ef97acc2b607f3acf63865f80960f76f656470f8e23f1d2788fb0070e28c2a5c8aaf15b5dbf30b40907ff6c5068fdcbc11a2d00")
                    .to_vec()
            }))
        );
        assert_eq!(mock_hal.ui.screens, expected_screens);

        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &Transaction::Eip1559(&pb::EthSignEip1559Request {
                keypath: KEYPATH.to_vec(),
                nonce: hex!("1fdc").to_vec(),
                max_priority_fee_per_gas: b"".to_vec(),
                max_fee_per_gas: hex!("0165a0bc00").to_vec(),
                gas_limit: hex!("5208").to_vec(),
                recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
                value: hex!("075cf1259e9c4000").to_vec(),
                data: b"".to_vec(),
                host_nonce_commitment: None,
                chain_id: 1,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }))),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!("289111770dc067895780de3e9b30454e331ba6661f046e9e26431576d7f08a496ffe6deffb07dd8d4713d8c523b6c33b53dd6ef2dc9c394d6e21f64307d2bcf001")
                    .to_vec()
            }))
        );
        assert_eq!(mock_hal.ui.screens, expected_screens);
    }

    /// Test a transaction with an unusually high fee.
    #[test]
    fn test_high_fee_warning() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        mock_unlocked();

        let mut mock_hal = TestingHal::new();
        assert!(
            block_on(process(
                &mut mock_hal,
                &Transaction::Legacy(&pb::EthSignRequest {
                    coin: pb::EthCoin::Eth as _,
                    keypath: KEYPATH.to_vec(),
                    nonce: hex!("1fdc").to_vec(),
                    // fee=gas_price*gas_limit=63713280000000000
                    gas_price: hex!("0165a0bc0000").to_vec(),
                    gas_limit: hex!("a208").to_vec(),
                    recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
                    // 530564000000000000
                    value: hex!("075cf1259e9c4000").to_vec(),
                    data: b"".to_vec(),
                    host_nonce_commitment: None,
                    chain_id: 0,
                    address_case: pb::EthAddressCase::Mixed as _,
                    data_length: 0,
                })
            ))
            .is_ok()
        );

        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "".into(),
                    body: "Sign transaction on\n\nEthereum".into(),
                    longtouch: false,
                },
                Screen::Recipient {
                    recipient: "0x04F264Cf34440313B4A0192A352814FBe927b885".into(),
                    amount: "0.530564 ETH".into(),
                },
                Screen::TotalFee {
                    total: "0.59427728 ETH".into(),
                    fee: "0.06371328 ETH".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "High fee".into(),
                    body: "The fee is 12.0%\nthe send amount.\nProceed?".into(),
                    longtouch: true,
                },
                Screen::Status {
                    title: "Transaction\nconfirmed".into(),
                    success: true,
                },
            ]
        );
    }

    /// Test an EIP-1559 transaction with an unusually high fee.
    #[test]
    fn test_high_fee_warning_eip1559() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert!(
            block_on(process(
                &mut mock_hal,
                &Transaction::Eip1559(&pb::EthSignEip1559Request {
                    keypath: KEYPATH.to_vec(),
                    nonce: hex!("1fdc").to_vec(),
                    // fee=max_fee_per_gas*gas_limit=63713280000000000
                    max_priority_fee_per_gas: b"".to_vec(),
                    max_fee_per_gas: hex!("0165a0bc0000").to_vec(),
                    gas_limit: hex!("a208").to_vec(),
                    recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
                    // 530564000000000000
                    value: hex!("075cf1259e9c4000").to_vec(),
                    data: b"".to_vec(),
                    host_nonce_commitment: None,
                    chain_id: 1,
                    address_case: pb::EthAddressCase::Mixed as _,
                    data_length: 0,
                })
            ))
            .is_ok()
        );

        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "".into(),
                    body: "Sign transaction on\n\nEthereum".into(),
                    longtouch: false,
                },
                Screen::Recipient {
                    recipient: "0x04F264Cf34440313B4A0192A352814FBe927b885".into(),
                    amount: "0.530564 ETH".into(),
                },
                Screen::TotalFee {
                    total: "0.59427728 ETH".into(),
                    fee: "0.06371328 ETH".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "High fee".into(),
                    body: "The fee is 12.0%\nthe send amount.\nProceed?".into(),
                    longtouch: true,
                },
                Screen::Status {
                    title: "Transaction\nconfirmed".into(),
                    success: true,
                },
            ]
        );
    }

    /// Standard ETH transaction on an unusual keypath (Sepolia on mainnet keypath)
    #[test]
    pub fn test_process_warn_unusual_keypath() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        block_on(process(
            &mut mock_hal,
            &Transaction::Legacy(&pb::EthSignRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                nonce: hex!("1fdc").to_vec(),
                gas_price: hex!("0165a0bc00").to_vec(),
                gas_limit: hex!("5208").to_vec(),
                recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
                value: hex!("075cf1259e9c4000").to_vec(),
                data: b"".to_vec(),
                host_nonce_commitment: None,
                chain_id: 11155111,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }),
        ))
        .unwrap();

        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sepolia".into(),
                    body: "Warning: unusual keypath m/44'/60'/0'/0/0. Proceed only if you know what you are doing.".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "".into(),
                    body: "Sign transaction on\n\nSepolia".into(),
                    longtouch: false,
                },
                Screen::Recipient {
                    recipient: "0x04F264Cf34440313B4A0192A352814FBe927b885".into(),
                    amount: "0.530564 SEPETH".into(),
                },
                Screen::TotalFee {
                    total: "0.53069 SEPETH".into(),
                    fee: "0.000126 SEPETH".into(),
                    longtouch: true,
                },
                Screen::Status {
                    title: "Transaction\nconfirmed".into(),
                    success: true,
                },
            ],
        );
    }

    /// Standard ETH transaction with an unknown data field.
    #[test]
    pub fn test_process_standard_transaction_with_data() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &Transaction::Legacy(&pb::EthSignRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                nonce: hex!("1fdc").to_vec(),
                gas_price: hex!("0165a0bc00").to_vec(),
                gas_limit: hex!("5208").to_vec(),
                recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
                value: hex!("075cf1259e9c4000").to_vec(),
                data: b"foo bar".to_vec(),
                host_nonce_commitment: None,
                chain_id: 0,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }))),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!("7d3f3713e3cf1082791d5c0fc68ec29eaff5e1ee8467a8ec547dc796e85a79042b7c01692fb72f5576ab50dcaa621ad1eeabd9975973b86256f40c6f8550ef4400")
                    .to_vec()
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "".into(),
                    body: "Sign transaction on\n\nEthereum".into(),
                    longtouch: false
                },
                Screen::Confirm {
                    title: "Unknown\ncontract".into(),
                    body: "You will be shown\nthe raw\ntransaction data.".into(),
                    longtouch: false
                },
                Screen::Confirm {
                    title: "Unknown\ncontract".into(),
                    body: "Only proceed if you\nunderstand exactly\nwhat the data means.".into(),
                    longtouch: false
                },
                Screen::Confirm {
                    title: "Transaction\ndata".into(),
                    body: "666f6f20626172".into(),
                    longtouch: false
                },
                Screen::Recipient {
                    recipient: "0x04F264Cf34440313B4A0192A352814FBe927b885".into(),
                    amount: "0.530564 ETH".into()
                },
                Screen::TotalFee {
                    total: "0.53069 ETH".into(),
                    fee: "0.000126 ETH".into(),
                    longtouch: true
                },
                Screen::Status {
                    title: "Transaction\nconfirmed".into(),
                    success: true
                }
            ]
        );
    }

    /// EIP-1559 ETH transaction with an unknown data field.
    #[test]
    pub fn test_process_eip1559_transaction_with_data() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &Transaction::Eip1559(&pb::EthSignEip1559Request {
                keypath: KEYPATH.to_vec(),
                nonce: hex!("1fdc").to_vec(),
                max_priority_fee_per_gas: hex!("3b9aca00").to_vec(),
                max_fee_per_gas: hex!("0165a0bc00").to_vec(),
                gas_limit: hex!("5208").to_vec(),
                recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
                value: hex!("075cf1259e9c4000").to_vec(),
                data: b"foo bar".to_vec(),
                host_nonce_commitment: None,
                chain_id: 1,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }))),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!("c5d9639a778a3415f63a11c03a58bede6b3cafff4f2ce6ea16411e76fba946f72166f09e313c07e78b7b1fff87450c4321170c02df2d36c44c3a021abf20546001")
                    .to_vec()
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "".into(),
                    body: "Sign transaction on\n\nEthereum".into(),
                    longtouch: false
                },
                Screen::Confirm {
                    title: "Unknown\ncontract".into(),
                    body: "You will be shown\nthe raw\ntransaction data.".into(),
                    longtouch: false
                },
                Screen::Confirm {
                    title: "Unknown\ncontract".into(),
                    body: "Only proceed if you\nunderstand exactly\nwhat the data means.".into(),
                    longtouch: false
                },
                Screen::Confirm {
                    title: "Transaction\ndata".into(),
                    body: "666f6f20626172".into(),
                    longtouch: false
                },
                Screen::Recipient {
                    recipient: "0x04F264Cf34440313B4A0192A352814FBe927b885".into(),
                    amount: "0.530564 ETH".into()
                },
                Screen::TotalFee {
                    total: "0.53069 ETH".into(),
                    fee: "0.000126 ETH".into(),
                    longtouch: true
                },
                Screen::Status {
                    title: "Transaction\nconfirmed".into(),
                    success: true
                }
            ]
        );
    }

    /// ERC20 transaction: recipient is an ERC20 contract address, and
    /// the data field contains an ERC20 transfer method invocation.
    #[test]
    pub fn test_process_standard_erc20_transaction() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        let expected_screens = vec![
            Screen::Confirm {
                title: "".into(),
                body: "Sign transaction on\n\nEthereum".into(),
                longtouch: false,
            },
            Screen::Recipient {
                recipient: "0xE6CE0a092A99700CD4ccCcBb1fEDc39Cf53E6330".into(),
                amount: "57 USDT".into(),
            },
            Screen::TotalFee {
                total: "57 USDT".into(),
                fee: "0.0012658164 ETH".into(),
                longtouch: true,
            },
            Screen::Status {
                title: "Transaction\nconfirmed".into(),
                success: true,
            },
        ];

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &Transaction::Legacy(&pb::EthSignRequest {
                coin: pb::EthCoin::RopstenEth as _, // ignored because chain_id > 0
                keypath: KEYPATH.to_vec(),
                nonce: hex!("2367").to_vec(),
                gas_price: hex!("027aca1a80").to_vec(),
                gas_limit: hex!("01d048").to_vec(),
                recipient: hex!("dac17f958d2ee523a2206206994597c13d831ec7").to_vec(),
                value: b"".to_vec(),
                data: hex!("a9059cbb000000000000000000000000e6ce0a092a99700cd4ccccbb1fedc39cf53e6330000000000000000000000000000000000000000000000000000000000365c040").to_vec(),
                host_nonce_commitment: None,
                chain_id: 1,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }))),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!("674e9a0170eee0ca8c406ec9a7df2e3a6bdd179cf69385800e1fd378e7cfb19c4d55162c547b04d1818e43901691aec988ef75cd67d9bb301d14902fd6e6929201")
                    .to_vec()
            }))
        );
        assert_eq!(mock_hal.ui.screens, expected_screens);

        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &Transaction::Eip1559(&pb::EthSignEip1559Request {
                keypath: KEYPATH.to_vec(),
                nonce: hex!("2367").to_vec(),
                max_priority_fee_per_gas: hex!("3b9aca00").to_vec(),
                max_fee_per_gas: hex!("027aca1a80").to_vec(),
                gas_limit: hex!("01d048").to_vec(),
                recipient: hex!("dac17f958d2ee523a2206206994597c13d831ec7").to_vec(),
                value: b"".to_vec(),
                data: hex!("a9059cbb000000000000000000000000e6ce0a092a99700cd4ccccbb1fedc39cf53e6330000000000000000000000000000000000000000000000000000000000365c040").to_vec(),
                host_nonce_commitment: None,
                chain_id: 1,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }))),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!("3162487880abdea1f352d9a4e3d56066f122f04ff112117c8ca3cd220f1666302dacd5e5e8da4cd39704e33443a9a7f32602d332bb52567c2e34aafe9ed48feb01")
                    .to_vec()
            }))
        );
        assert_eq!(mock_hal.ui.screens, expected_screens);
    }

    /// An ERC20 transaction which is not in our list of supported ERC20 tokens.
    #[test]
    pub fn test_process_standard_unknown_erc20_transaction() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        let expected_screens = vec![
            Screen::Confirm {
                title: "".into(),
                body: "Sign transaction on\n\nEthereum".into(),
                longtouch: false,
            },
            Screen::Recipient {
                recipient: "0x857B3D969eAcB775a9f79cabc62Ec4bB1D1cd60e".into(),
                amount: "Unknown token".into(),
            },
            Screen::TotalFee {
                total: "Unknown amount".into(),
                fee: "0.000067973 ETH".into(),
                longtouch: true,
            },
            Screen::Status {
                title: "Transaction\nconfirmed".into(),
                success: true,
            },
        ];
        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &Transaction::Legacy(&pb::EthSignRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                nonce: hex!("b9").to_vec(),
                gas_price: hex!("3b9aca00").to_vec(),
                gas_limit: hex!("010985").to_vec(),
                recipient: hex!("9c23d67aea7b95d80942e3836bcdf7e708a747c1").to_vec(),
                value: b"".to_vec(),
                data: hex!("a9059cbb000000000000000000000000857b3d969eacb775a9f79cabc62ec4bb1d1cd60e000000000000000000000000000000000000000000000098a63cbeb859d027b0").to_vec(),
                host_nonce_commitment: None,
                chain_id: 0,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }))),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!("ec6e530c8ee25434fc440e9ac0f888e9c63cf07ebcf1c2f8a83e2e8c39832c551512716f6e1a8b66ce3811a726bcb244664ef26f98ee35c0c9db4caab073985600")
                    .to_vec()
            }))
        );
        assert_eq!(mock_hal.ui.screens, expected_screens);

        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &Transaction::Eip1559(&pb::EthSignEip1559Request {
                keypath: KEYPATH.to_vec(),
                nonce: hex!("b9").to_vec(),
                max_priority_fee_per_gas: b"".to_vec(),
                max_fee_per_gas: hex!("3b9aca00").to_vec(),
                gas_limit: hex!("010985").to_vec(),
                recipient: hex!("9c23d67aea7b95d80942e3836bcdf7e708a747c1").to_vec(),
                value: b"".to_vec(),
                data: hex!("a9059cbb000000000000000000000000857b3d969eacb775a9f79cabc62ec4bb1d1cd60e000000000000000000000000000000000000000000000098a63cbeb859d027b0").to_vec(),
                host_nonce_commitment: None,
                chain_id: 1,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }))),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!("8203d80b600dce8e77cdcb119d45db7f60d7ca34e7369140e92d93919221f85a0a119d2464dfab65833095c12763fed37c072feb29610e1437f388958d77562801")
                    .to_vec()
            }))
        );
        assert_eq!(mock_hal.ui.screens, expected_screens);
    }

    #[test]
    pub fn test_process_unhappy() {
        let valid_request = pb::EthSignRequest {
            coin: pb::EthCoin::Eth as _,
            keypath: vec![44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0],
            nonce: hex!("1fdc").to_vec(),
            gas_price: hex!("0165a0bc00").to_vec(),
            gas_limit: hex!("5208").to_vec(),
            recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
            value: hex!("075cf1259e9c4000").to_vec(),
            data: b"".to_vec(),
            host_nonce_commitment: None,
            chain_id: 0,
            address_case: pb::EthAddressCase::Mixed as _,
            data_length: 0,
        };

        {
            // Check that the above is valid before making invalid variants.
            mock_unlocked();
            assert!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Legacy(&valid_request)
                ))
                .is_ok()
            );
        }

        {
            // invalid coin
            let mut invalid_request = valid_request.clone();
            invalid_request.coin = 100;
            assert_eq!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Legacy(&invalid_request)
                )),
                Err(Error::InvalidInput)
            );
        }

        {
            // invalid keypath (wrong coin part).
            let mut invalid_request = valid_request.clone();
            invalid_request.keypath = vec![44 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
            assert_eq!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Legacy(&invalid_request)
                )),
                Err(Error::InvalidInput)
            );
        }

        {
            // invalid keypath (account too high).
            let mut invalid_request = valid_request.clone();
            invalid_request.keypath = vec![44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 100];
            assert_eq!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Legacy(&invalid_request)
                )),
                Err(Error::InvalidInput)
            );
        }

        {
            // data too long
            let mut invalid_request = valid_request.clone();
            invalid_request.data = vec![0; 6145];
            assert_eq!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Legacy(&invalid_request)
                )),
                Err(Error::InvalidInput)
            );
        }

        {
            // recipient too long
            let mut invalid_request = valid_request.clone();
            invalid_request.recipient = vec![b'a'; 21];
            assert_eq!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Legacy(&invalid_request)
                )),
                Err(Error::InvalidInput)
            );
        }

        {
            // recipient has the right size, but is all zeroes
            let mut invalid_request = valid_request.clone();
            invalid_request.recipient = vec![0; 20];
            assert_eq!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Legacy(&invalid_request)
                )),
                Err(Error::InvalidInput)
            );
        }

        {
            // User rejects first, second or third screen.
            for i in 0..3 {
                let mut mock_hal = TestingHal::new();
                mock_hal.ui.abort_nth(i);
                assert_eq!(
                    block_on(process(&mut mock_hal, &Transaction::Legacy(&valid_request))),
                    Err(Error::UserAbort)
                );
                let mut expected_screens = [
                    Screen::Confirm {
                        title: "".into(),
                        body: "Sign transaction on\n\nEthereum".into(),
                        longtouch: false,
                    },
                    Screen::Recipient {
                        recipient: "0x04F264Cf34440313B4A0192A352814FBe927b885".into(),
                        amount: "0.530564 ETH".into(),
                    },
                    Screen::TotalFee {
                        total: "0.53069 ETH".into(),
                        fee: "0.000126 ETH".into(),
                        longtouch: true,
                    },
                ][..i + 1]
                    .to_vec();
                expected_screens.push(Screen::Status {
                    title: "Transaction\ncanceled".into(),
                    success: false,
                });
                assert_eq!(mock_hal.ui.screens, expected_screens);
            }
        }

        {
            // Keystore locked.
            keystore::lock();
            assert_eq!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Legacy(&valid_request)
                )),
                Err(Error::Generic)
            );
        }
    }

    #[test]
    pub fn test_process_unhappy_eip1559() {
        let valid_request = pb::EthSignEip1559Request {
            keypath: vec![44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0],
            nonce: hex!("1fdc").to_vec(),
            max_priority_fee_per_gas: b"".to_vec(),
            max_fee_per_gas: hex!("0165a0bc00").to_vec(),
            gas_limit: hex!("5208").to_vec(),
            recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
            value: hex!("075cf1259e9c4000").to_vec(),
            data: b"".to_vec(),
            host_nonce_commitment: None,
            chain_id: 1,
            address_case: pb::EthAddressCase::Mixed as _,
            data_length: 0,
        };

        {
            // Check that the above is valid before making invalid variants.
            mock_unlocked();
            assert!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Eip1559(&valid_request)
                ))
                .is_ok()
            );
        }

        {
            // invalid chain ID
            let mut invalid_request = valid_request.clone();
            invalid_request.chain_id = 0;
            assert_eq!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Eip1559(&invalid_request)
                )),
                Err(Error::InvalidInput)
            );
        }

        {
            // max_fee_per_gas with leading zero byte
            let mut invalid_request = valid_request.clone();
            invalid_request.max_fee_per_gas = hex!("000165a0bc00").to_vec();
            assert_eq!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Eip1559(&invalid_request)
                )),
                Err(Error::InvalidInput)
            );
        }

        {
            // max_priority_fee_per_gas with leading zero byte
            let mut invalid_request = valid_request.clone();
            invalid_request.max_priority_fee_per_gas = hex!("003b9aca00").to_vec();
            assert_eq!(
                block_on(process(
                    &mut TestingHal::new(),
                    &Transaction::Eip1559(&invalid_request)
                )),
                Err(Error::InvalidInput)
            );
        }
    }

    /// Unknown chain ID (network params not hardcoded in in the firmware).
    #[test]
    pub fn test_process_unknown_network() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &Transaction::Legacy(&pb::EthSignRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                nonce: hex!("1fdc").to_vec(),
                gas_price: hex!("0165a0bc00").to_vec(),
                gas_limit: hex!("5208").to_vec(),
                recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
                value: hex!("075cf1259e9c4000").to_vec(),
                data: b"".to_vec(),
                host_nonce_commitment: None,
                chain_id: 12345,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }))),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!("b1b6b34e15a0309ddc2603df4c4038ea8665ed85d3f2c81e7f1aa0254b2138720d601f4219fb29ab3d5ff776eae1be1526b467e2b0e630e8e634a4da4a822e3900").to_vec()
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Warning".into(),
                    body: "Unknown network\nwith chain ID:\n12345".into(),
                    longtouch: false
                },
                Screen::Confirm {
                    title: "Warning".into(),
                    body: "Only proceed if\nyou recognize\nthis chain ID.".into(),
                    longtouch: false
                },
                Screen::Recipient {
                    recipient: "0x04F264Cf34440313B4A0192A352814FBe927b885".into(),
                    amount: "0.530564 ".into(),
                },
                Screen::TotalFee {
                    total: "0.53069 ".into(),
                    fee: "0.000126 ".into(),
                    longtouch: true
                },
                Screen::Status {
                    title: "Transaction\nconfirmed".into(),
                    success: true
                }
            ]
        );
    }

    /// Test that the chain confirmation screen appears for known non-mainnet networks.
    #[test]
    pub fn test_chain_confirmation() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        // Test with Arbitrum (chain_id 42161)
        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        block_on(process(
            &mut mock_hal,
            &Transaction::Legacy(&pb::EthSignRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                nonce: hex!("1fdc").to_vec(),
                gas_price: hex!("0165a0bc00").to_vec(),
                gas_limit: hex!("5208").to_vec(),
                recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
                value: hex!("075cf1259e9c4000").to_vec(),
                data: b"".to_vec(),
                host_nonce_commitment: None,
                chain_id: 42161,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }),
        ))
        .unwrap();

        assert_eq!(
            mock_hal.ui.screens[0],
            Screen::Confirm {
                title: "".into(),
                body: "Sign transaction on\n\nArbitrum One".into(),
                longtouch: false,
            }
        );
    }

    /// Test that EIP-1559 transactions also get the chain confirmation screen
    #[test]
    pub fn test_chain_confirmation_for_eip1559() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        // Test with Polygon network (chain_id 137)
        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        block_on(process(
            &mut mock_hal,
            &Transaction::Eip1559(&pb::EthSignEip1559Request {
                keypath: KEYPATH.to_vec(),
                nonce: hex!("1fdc").to_vec(),
                max_priority_fee_per_gas: hex!("3b9aca00").to_vec(),
                max_fee_per_gas: hex!("0165a0bc00").to_vec(),
                gas_limit: hex!("5208").to_vec(),
                recipient: hex!("04f264cf34440313b4a0192a352814fbe927b885").to_vec(),
                value: hex!("075cf1259e9c4000").to_vec(),
                data: b"".to_vec(),
                host_nonce_commitment: None,
                chain_id: 137,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }),
        ))
        .unwrap();
        assert_eq!(
            mock_hal.ui.screens[0],
            Screen::Confirm {
                title: "".into(),
                body: "Sign transaction on\n\nPolygon".into(),
                longtouch: false,
            }
        );
    }

    #[test]
    pub fn test_streaming_equivalence_legacy() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
        let test_data: Vec<u8> = (0..4000u32).map(|i| (i % 256) as u8).collect();

        mock_unlocked();
        let mut mock_hal_nonstreaming = TestingHal::new();
        let nonstreaming_result = block_on(process(
            &mut mock_hal_nonstreaming,
            &Transaction::Legacy(&pb::EthSignRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                nonce: hex!("01").to_vec(),
                gas_price: hex!("04a817c800").to_vec(),
                gas_limit: hex!("0f4240").to_vec(),
                recipient: hex!("112233445566778899aabbccddeeff0011223344").to_vec(),
                value: b"".to_vec(),
                data: test_data.clone(),
                host_nonce_commitment: None,
                chain_id: 1,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }),
        ));

        setup_chunk_responder(test_data.clone());
        mock_unlocked();
        let mut mock_hal_streaming = TestingHal::new();
        let streaming_result = block_on(process(
            &mut mock_hal_streaming,
            &Transaction::Legacy(&pb::EthSignRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                nonce: hex!("01").to_vec(),
                gas_price: hex!("04a817c800").to_vec(),
                gas_limit: hex!("0f4240").to_vec(),
                recipient: hex!("112233445566778899aabbccddeeff0011223344").to_vec(),
                value: b"".to_vec(),
                data: vec![],
                host_nonce_commitment: None,
                chain_id: 1,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 4000,
            }),
        ));
        clear_chunk_responder();

        assert!(nonstreaming_result.is_ok());
        assert!(streaming_result.is_ok());
        match (&nonstreaming_result, &streaming_result) {
            (Ok(Response::Sign(trad)), Ok(Response::Sign(stream))) => {
                assert_eq!(trad.signature, stream.signature);
            }
            _ => panic!("Expected Sign responses from both modes"),
        }
    }

    #[test]
    pub fn test_streaming_equivalence_eip1559() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
        let test_data: Vec<u8> = (0..4000u32).map(|i| (i % 256) as u8).collect();

        mock_unlocked();
        let mut mock_hal_nonstreaming = TestingHal::new();
        let nonstreaming_result = block_on(process(
            &mut mock_hal_nonstreaming,
            &Transaction::Eip1559(&pb::EthSignEip1559Request {
                keypath: KEYPATH.to_vec(),
                nonce: hex!("01").to_vec(),
                max_priority_fee_per_gas: hex!("3b9aca00").to_vec(),
                max_fee_per_gas: hex!("04a817c800").to_vec(),
                gas_limit: hex!("0f4240").to_vec(),
                recipient: hex!("112233445566778899aabbccddeeff0011223344").to_vec(),
                value: b"".to_vec(),
                data: test_data.clone(),
                host_nonce_commitment: None,
                chain_id: 1,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            }),
        ));

        setup_chunk_responder(test_data.clone());
        mock_unlocked();
        let mut mock_hal_streaming = TestingHal::new();
        let streaming_result = block_on(process(
            &mut mock_hal_streaming,
            &Transaction::Eip1559(&pb::EthSignEip1559Request {
                keypath: KEYPATH.to_vec(),
                nonce: hex!("01").to_vec(),
                max_priority_fee_per_gas: hex!("3b9aca00").to_vec(),
                max_fee_per_gas: hex!("04a817c800").to_vec(),
                gas_limit: hex!("0f4240").to_vec(),
                recipient: hex!("112233445566778899aabbccddeeff0011223344").to_vec(),
                value: b"".to_vec(),
                data: vec![],
                host_nonce_commitment: None,
                chain_id: 1,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 4000,
            }),
        ));
        clear_chunk_responder();

        assert!(nonstreaming_result.is_ok());
        assert!(streaming_result.is_ok());
        match (&nonstreaming_result, &streaming_result) {
            (Ok(Response::Sign(trad)), Ok(Response::Sign(stream))) => {
                assert_eq!(trad.signature, stream.signature);
            }
            _ => panic!("Expected Sign responses from both modes"),
        }
    }

    #[test]
    pub fn test_streaming_large_data_legacy() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
        let test_data: Vec<u8> = (0..10000u32).map(|i| (i % 256) as u8).collect();

        setup_chunk_responder(test_data);
        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &Transaction::Legacy(&pb::EthSignRequest {
                    coin: pb::EthCoin::Eth as _,
                    keypath: KEYPATH.to_vec(),
                    nonce: hex!("01").to_vec(),
                    gas_price: hex!("04a817c800").to_vec(),
                    gas_limit: hex!("0f4240").to_vec(),
                    recipient: hex!("112233445566778899aabbccddeeff0011223344")
                        .to_vec(),
                    value: b"".to_vec(),
                    data: vec![],
                    host_nonce_commitment: None,
                    chain_id: 1,
                    address_case: pb::EthAddressCase::Mixed as _,
                    data_length: 10000,
                }),
            )),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!("f00a05084c540bb69b9d0d1e7783a0fe315ffc3ffdc0edc32a3d0e9d00f9d8a86c7b5c36fc136062adc1857e2edcf73eb75138d5390ed807b2cb0b90652fef2201")
                    .to_vec()
            }))
        );
        clear_chunk_responder();
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "".into(),
                    body: "Sign transaction on\n\nEthereum".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Unknown\ncontract".into(),
                    body: "You are signing a\ncontract interaction\nwith large data.".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Unknown\ncontract".into(),
                    body: "Only proceed if you\nfully understand\nthe risks involved.".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Transaction\ndata".into(),
                    body: "10000 bytes\n(too large to\ndisplay)".into(),
                    longtouch: false,
                },
                Screen::Recipient {
                    recipient: "0x112233445566778899AabbcCDDeEFF0011223344".into(),
                    amount: "0 ETH".into(),
                },
                Screen::TotalFee {
                    total: "0.02 ETH".into(),
                    fee: "0.02 ETH".into(),
                    longtouch: true,
                },
                Screen::Status {
                    title: "Transaction\nconfirmed".into(),
                    success: true,
                },
            ]
        );
    }

    #[test]
    pub fn test_streaming_large_data_eip1559() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
        let test_data: Vec<u8> = (0..12000u32).map(|i| (i % 256) as u8).collect();

        setup_chunk_responder(test_data);
        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &Transaction::Eip1559(&pb::EthSignEip1559Request {
                    keypath: KEYPATH.to_vec(),
                    nonce: hex!("01").to_vec(),
                    max_priority_fee_per_gas: hex!("3b9aca00").to_vec(),
                    max_fee_per_gas: hex!("04a817c800").to_vec(),
                    gas_limit: hex!("0f4240").to_vec(),
                    recipient: hex!("112233445566778899aabbccddeeff0011223344")
                        .to_vec(),
                    value: b"".to_vec(),
                    data: vec![],
                    host_nonce_commitment: None,
                    chain_id: 1,
                    address_case: pb::EthAddressCase::Mixed as _,
                    data_length: 12000,
                }),
            )),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!("dc853640b4a75390b5b59478c18b1fba135025bf40bb41d54f95d35628443e1900376e1be2916829be4cbb0d897cc69ad80987a57a489254d561dfd3071a0db101")
                    .to_vec()
            }))
        );
        clear_chunk_responder();
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "".into(),
                    body: "Sign transaction on\n\nEthereum".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Unknown\ncontract".into(),
                    body: "You are signing a\ncontract interaction\nwith large data.".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Unknown\ncontract".into(),
                    body: "Only proceed if you\nfully understand\nthe risks involved.".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Transaction\ndata".into(),
                    body: "12000 bytes\n(too large to\ndisplay)".into(),
                    longtouch: false,
                },
                Screen::Recipient {
                    recipient: "0x112233445566778899AabbcCDDeEFF0011223344".into(),
                    amount: "0 ETH".into(),
                },
                Screen::TotalFee {
                    total: "0.02 ETH".into(),
                    fee: "0.02 ETH".into(),
                    longtouch: true,
                },
                Screen::Status {
                    title: "Transaction\nconfirmed".into(),
                    success: true,
                },
            ]
        );
    }
}
