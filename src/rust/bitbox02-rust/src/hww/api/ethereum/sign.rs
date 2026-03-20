// SPDX-License-Identifier: Apache-2.0

use super::Error;
use super::amount::{Amount, calculate_percentage};
use super::params::Params;
use super::pb;
use crate::hal::ui::ConfirmParams;

use crate::keystore;

use crate::hal::Ui;
use crate::workflow::transaction;

use alloc::string::String;
use alloc::vec::Vec;
use hex_lit::hex;
use pb::eth_response::Response;

use core::ops::{Add, Mul};
use num_bigint::BigUint;
use num_traits::Zero;

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

#[derive(Clone, Debug, PartialEq, Eq)]
enum SwapAsset {
    Native,
    Erc20([u8; 20]),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SwapAmountBound {
    Exact,
    Minimum,
    Maximum,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SwapAmount {
    bound: SwapAmountBound,
    value: BigUint,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct UniswapSwap {
    from_asset: SwapAsset,
    from_amount: SwapAmount,
    to_asset: SwapAsset,
    to_amount: SwapAmount,
}

const SELECTOR_SWAP_EXACT_ETH_FOR_TOKENS: [u8; 4] = hex!("7ff36ab5");
const SELECTOR_SWAP_ETH_FOR_EXACT_TOKENS: [u8; 4] = hex!("fb3bdb41");
const SELECTOR_SWAP_EXACT_TOKENS_FOR_ETH: [u8; 4] = hex!("18cbafe5");
const SELECTOR_SWAP_TOKENS_FOR_EXACT_ETH: [u8; 4] = hex!("4a25d94a");
const SELECTOR_SWAP_EXACT_TOKENS_FOR_TOKENS: [u8; 4] = hex!("38ed1739");
const SELECTOR_SWAP_TOKENS_FOR_EXACT_TOKENS: [u8; 4] = hex!("8803dbee");
const SELECTOR_SWAP_EXACT_ETH_FOR_TOKENS_FEE: [u8; 4] = hex!("b6f9de95");
const SELECTOR_SWAP_EXACT_TOKENS_FOR_ETH_FEE: [u8; 4] = hex!("791ac947");
const SELECTOR_SWAP_EXACT_TOKENS_FOR_TOKENS_FEE: [u8; 4] = hex!("5c11d795");
const SELECTOR_EXACT_INPUT_SINGLE: [u8; 4] = hex!("04e45aaf");
const SELECTOR_EXACT_OUTPUT_SINGLE: [u8; 4] = hex!("5023b4df");
const SELECTOR_EXACT_INPUT: [u8; 4] = hex!("c04b8d59");
const SELECTOR_EXACT_OUTPUT: [u8; 4] = hex!("f28c0498");
const SELECTOR_MULTICALL_WITH_DEADLINE: [u8; 4] = hex!("5ae401dc");
const SELECTOR_MULTICALL: [u8; 4] = hex!("ac9650d8");
const SELECTOR_UNIVERSAL_ROUTER_EXECUTE_WITH_DEADLINE: [u8; 4] = hex!("3593564c");
const SELECTOR_UNIVERSAL_ROUTER_EXECUTE: [u8; 4] = hex!("24856bc3");

const UNIVERSAL_ROUTER_COMMAND_MASK: u8 = 0x3f;
const UNIVERSAL_ROUTER_COMMAND_V3_SWAP_EXACT_IN: u8 = 0x00;
const UNIVERSAL_ROUTER_COMMAND_V3_SWAP_EXACT_OUT: u8 = 0x01;
const UNIVERSAL_ROUTER_COMMAND_V2_SWAP_EXACT_IN: u8 = 0x08;
const UNIVERSAL_ROUTER_COMMAND_V2_SWAP_EXACT_OUT: u8 = 0x09;
const UNIVERSAL_ROUTER_COMMAND_UNWRAP_WETH: u8 = 0x0c;

fn wrapped_native(chain_id: u64) -> Option<[u8; 20]> {
    match chain_id {
        1 => Some(hex!("c02aaA39b223Fe8D0A0E5C4F27eAD9083C756Cc2")),
        10 => Some(hex!("4200000000000000000000000000000000000006")),
        56 => Some(hex!("bb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c")),
        100 => Some(hex!("6A023CCD1ff6F2045C3309768eAd9E68F978f6e1")),
        137 => Some(hex!("0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270")),
        250 => Some(hex!("21be370D5312f44cB42ce377BC9b8a0CeF1A4C83")),
        8453 => Some(hex!("4200000000000000000000000000000000000006")),
        42161 => Some(hex!("82aF49447D8a07e3bd95BD0d56f35241523fBab1")),
        11155111 => Some(hex!("fFf9976782d46CC05630D1f6EBAb18b2324d6B14")),
        _ => None,
    }
}

fn parse_word(data: &[u8], word_index: usize) -> Option<&[u8]> {
    let start = word_index.checked_mul(32)?;
    let end = start.checked_add(32)?;
    data.get(start..end)
}

fn parse_usize_word(word: &[u8]) -> Option<usize> {
    let word: &[u8; 32] = word.try_into().ok()?;
    if word[..24] != [0; 24] {
        return None;
    }
    usize::try_from(u64::from_be_bytes(word[24..32].try_into().unwrap())).ok()
}

fn parse_u256(data: &[u8], word_index: usize) -> Option<BigUint> {
    Some(BigUint::from_bytes_be(parse_word(data, word_index)?))
}

fn parse_u256_at(data: &[u8], start: usize) -> Option<BigUint> {
    Some(BigUint::from_bytes_be(data.get(start..start.checked_add(32)?)?))
}

fn parse_usize_at(data: &[u8], start: usize) -> Option<usize> {
    parse_usize_word(data.get(start..start.checked_add(32)?)?)
}

fn parse_address_word(data: &[u8], word_index: usize) -> Option<[u8; 20]> {
    parse_address_at(data, word_index.checked_mul(32)?)
}

fn parse_address_at(data: &[u8], start: usize) -> Option<[u8; 20]> {
    let word = data.get(start..start.checked_add(32)?)?;
    if word[..12] != [0; 12] {
        return None;
    }
    Some(word[12..].try_into().unwrap())
}

fn parse_abi_bytes(data: &[u8], offset: usize) -> Option<&[u8]> {
    if offset % 32 != 0 {
        return None;
    }
    let len = parse_usize_at(data, offset)?;
    let start = offset.checked_add(32)?;
    let padded_len = len.checked_add(31)?.checked_div(32)?.checked_mul(32)?;
    if start.checked_add(padded_len)? > data.len() {
        return None;
    }
    data.get(start..start.checked_add(len)?)
}

fn parse_abi_bytes_array_element(data: &[u8], array_offset: usize, index: usize) -> Option<&[u8]> {
    if array_offset % 32 != 0 {
        return None;
    }
    let num_elements = parse_usize_at(data, array_offset)?;
    if index >= num_elements {
        return None;
    }
    let head_start = array_offset.checked_add(32)?;
    let rel_offset = parse_usize_at(data, head_start.checked_add(index.checked_mul(32)?)?)?;
    let element_offset = head_start.checked_add(rel_offset)?;
    parse_abi_bytes(data, element_offset)
}

fn parse_address_array_first_last(data: &[u8], arg_index: usize) -> Option<([u8; 20], [u8; 20])> {
    let offset = parse_usize_word(parse_word(data, arg_index)?)?;
    if offset % 32 != 0 {
        return None;
    }
    let len = parse_usize_at(data, offset)?;
    if len < 2 {
        return None;
    }
    let first = parse_address_at(data, offset.checked_add(32)?)?;
    let last = parse_address_at(
        data,
        offset
            .checked_add(32)?
            .checked_add((len - 1).checked_mul(32)?)?,
    )?;
    Some((first, last))
}

fn parse_v3_path_endpoints(path: &[u8]) -> Option<([u8; 20], [u8; 20])> {
    if path.len() < 43 || (path.len() - 20) % 23 != 0 {
        return None;
    }
    Some((
        path[..20].try_into().unwrap(),
        path[path.len() - 20..].try_into().unwrap(),
    ))
}

fn tx_value_nonzero(request: &Transaction<'_>) -> Option<BigUint> {
    if request.value().is_empty() {
        None
    } else {
        let value = BigUint::from_bytes_be(request.value());
        if value.is_zero() {
            None
        } else {
            Some(value)
        }
    }
}

fn parse_uniswap_swap_v2(data: &[u8], request: &Transaction<'_>) -> Option<UniswapSwap> {
    let selector: [u8; 4] = data.get(..4)?.try_into().ok()?;
    let args = data.get(4..)?;

    match selector {
        SELECTOR_SWAP_EXACT_ETH_FOR_TOKENS | SELECTOR_SWAP_EXACT_ETH_FOR_TOKENS_FEE => {
            let amount_out_min = parse_u256(args, 0)?;
            let (path_start, path_end) = parse_address_array_first_last(args, 1)?;
            if let Some(expected_wrapped) = wrapped_native(request.chain_id()) {
                if path_start != expected_wrapped {
                    return None;
                }
            }
            Some(UniswapSwap {
                from_asset: SwapAsset::Native,
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: tx_value_nonzero(request)?,
                },
                to_asset: SwapAsset::Erc20(path_end),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Minimum,
                    value: amount_out_min,
                },
            })
        }
        SELECTOR_SWAP_ETH_FOR_EXACT_TOKENS => {
            let amount_out = parse_u256(args, 0)?;
            let (path_start, path_end) = parse_address_array_first_last(args, 1)?;
            if let Some(expected_wrapped) = wrapped_native(request.chain_id()) {
                if path_start != expected_wrapped {
                    return None;
                }
            }
            Some(UniswapSwap {
                from_asset: SwapAsset::Native,
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Maximum,
                    value: tx_value_nonzero(request)?,
                },
                to_asset: SwapAsset::Erc20(path_end),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: amount_out,
                },
            })
        }
        SELECTOR_SWAP_EXACT_TOKENS_FOR_ETH | SELECTOR_SWAP_EXACT_TOKENS_FOR_ETH_FEE => {
            if !request.value().is_empty() {
                return None;
            }
            let amount_in = parse_u256(args, 0)?;
            let amount_out_min = parse_u256(args, 1)?;
            let (path_start, path_end) = parse_address_array_first_last(args, 2)?;
            if let Some(expected_wrapped) = wrapped_native(request.chain_id()) {
                if path_end != expected_wrapped {
                    return None;
                }
            }
            Some(UniswapSwap {
                from_asset: SwapAsset::Erc20(path_start),
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: amount_in,
                },
                to_asset: SwapAsset::Native,
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Minimum,
                    value: amount_out_min,
                },
            })
        }
        SELECTOR_SWAP_TOKENS_FOR_EXACT_ETH => {
            if !request.value().is_empty() {
                return None;
            }
            let amount_out = parse_u256(args, 0)?;
            let amount_in_max = parse_u256(args, 1)?;
            let (path_start, path_end) = parse_address_array_first_last(args, 2)?;
            if let Some(expected_wrapped) = wrapped_native(request.chain_id()) {
                if path_end != expected_wrapped {
                    return None;
                }
            }
            Some(UniswapSwap {
                from_asset: SwapAsset::Erc20(path_start),
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Maximum,
                    value: amount_in_max,
                },
                to_asset: SwapAsset::Native,
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: amount_out,
                },
            })
        }
        SELECTOR_SWAP_EXACT_TOKENS_FOR_TOKENS | SELECTOR_SWAP_EXACT_TOKENS_FOR_TOKENS_FEE => {
            if !request.value().is_empty() {
                return None;
            }
            let amount_in = parse_u256(args, 0)?;
            let amount_out_min = parse_u256(args, 1)?;
            let (path_start, path_end) = parse_address_array_first_last(args, 2)?;
            Some(UniswapSwap {
                from_asset: SwapAsset::Erc20(path_start),
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: amount_in,
                },
                to_asset: SwapAsset::Erc20(path_end),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Minimum,
                    value: amount_out_min,
                },
            })
        }
        SELECTOR_SWAP_TOKENS_FOR_EXACT_TOKENS => {
            if !request.value().is_empty() {
                return None;
            }
            let amount_out = parse_u256(args, 0)?;
            let amount_in_max = parse_u256(args, 1)?;
            let (path_start, path_end) = parse_address_array_first_last(args, 2)?;
            Some(UniswapSwap {
                from_asset: SwapAsset::Erc20(path_start),
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Maximum,
                    value: amount_in_max,
                },
                to_asset: SwapAsset::Erc20(path_end),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: amount_out,
                },
            })
        }
        _ => None,
    }
}

fn parse_uniswap_swap_v3(data: &[u8], request: &Transaction<'_>) -> Option<UniswapSwap> {
    let selector: [u8; 4] = data.get(..4)?.try_into().ok()?;
    let args = data.get(4..)?;
    match selector {
        SELECTOR_EXACT_INPUT_SINGLE => {
            let token_in = parse_address_word(args, 0)?;
            let token_out = parse_address_word(args, 1)?;
            let amount_in = parse_u256(args, 5)?;
            let amount_out_min = parse_u256(args, 6)?;
            Some(UniswapSwap {
                from_asset: classify_input_asset(token_in, &amount_in, request)?,
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: amount_in,
                },
                to_asset: SwapAsset::Erc20(token_out),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Minimum,
                    value: amount_out_min,
                },
            })
        }
        SELECTOR_EXACT_OUTPUT_SINGLE => {
            let token_in = parse_address_word(args, 0)?;
            let token_out = parse_address_word(args, 1)?;
            let amount_out = parse_u256(args, 5)?;
            let amount_in_max = parse_u256(args, 6)?;
            Some(UniswapSwap {
                from_asset: classify_input_asset(token_in, &amount_in_max, request)?,
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Maximum,
                    value: amount_in_max,
                },
                to_asset: SwapAsset::Erc20(token_out),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: amount_out,
                },
            })
        }
        SELECTOR_EXACT_INPUT | SELECTOR_EXACT_OUTPUT => {
            let tuple_offset = parse_usize_word(parse_word(args, 0)?)?;
            let tuple_start = tuple_offset;
            let path_offset = parse_usize_at(args, tuple_start)?;
            let path =
                parse_abi_bytes(args, tuple_start.checked_add(path_offset)?)?;
            let (path_start, path_end) = parse_v3_path_endpoints(path)?;
            let amount_a = parse_u256_at(args, tuple_start.checked_add(3 * 32)?)?;
            let amount_b = parse_u256_at(args, tuple_start.checked_add(4 * 32)?)?;

            if selector == SELECTOR_EXACT_INPUT {
                Some(UniswapSwap {
                    from_asset: classify_input_asset(path_start, &amount_a, request)?,
                    from_amount: SwapAmount {
                        bound: SwapAmountBound::Exact,
                        value: amount_a,
                    },
                    to_asset: SwapAsset::Erc20(path_end),
                    to_amount: SwapAmount {
                        bound: SwapAmountBound::Minimum,
                        value: amount_b,
                    },
                })
            } else {
                Some(UniswapSwap {
                    from_asset: classify_input_asset(path_end, &amount_b, request)?,
                    from_amount: SwapAmount {
                        bound: SwapAmountBound::Maximum,
                        value: amount_b,
                    },
                    to_asset: SwapAsset::Erc20(path_start),
                    to_amount: SwapAmount {
                        bound: SwapAmountBound::Exact,
                        value: amount_a,
                    },
                })
            }
        }
        _ => None,
    }
}

fn classify_input_asset(
    token_in: [u8; 20],
    amount_in: &BigUint,
    request: &Transaction<'_>,
) -> Option<SwapAsset> {
    if let Some(tx_value) = tx_value_nonzero(request) {
        if &tx_value < amount_in {
            return None;
        }
        if let Some(expected_wrapped) = wrapped_native(request.chain_id()) {
            if token_in != expected_wrapped {
                return None;
            }
        }
        Some(SwapAsset::Native)
    } else {
        Some(SwapAsset::Erc20(token_in))
    }
}

fn parse_uniswap_universal_router_swap_input_v2(
    command: u8,
    input: &[u8],
    request: &Transaction<'_>,
) -> Option<UniswapSwap> {
    let (path_start, path_end) = parse_address_array_first_last(input, 3)?;
    match command {
        UNIVERSAL_ROUTER_COMMAND_V2_SWAP_EXACT_IN => {
            let amount_in = parse_u256(input, 1)?;
            let amount_out_min = parse_u256(input, 2)?;
            Some(UniswapSwap {
                from_asset: classify_input_asset(path_start, &amount_in, request)?,
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: amount_in,
                },
                to_asset: SwapAsset::Erc20(path_end),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Minimum,
                    value: amount_out_min,
                },
            })
        }
        UNIVERSAL_ROUTER_COMMAND_V2_SWAP_EXACT_OUT => {
            let amount_out = parse_u256(input, 1)?;
            let amount_in_max = parse_u256(input, 2)?;
            Some(UniswapSwap {
                from_asset: classify_input_asset(path_start, &amount_in_max, request)?,
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Maximum,
                    value: amount_in_max,
                },
                to_asset: SwapAsset::Erc20(path_end),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: amount_out,
                },
            })
        }
        _ => None,
    }
}

fn parse_uniswap_universal_router_swap_input_v3(
    command: u8,
    input: &[u8],
    request: &Transaction<'_>,
) -> Option<UniswapSwap> {
    let path_offset = parse_usize_word(parse_word(input, 3)?)?;
    let path = parse_abi_bytes(input, path_offset)?;
    let (path_start, path_end) = parse_v3_path_endpoints(path)?;
    match command {
        UNIVERSAL_ROUTER_COMMAND_V3_SWAP_EXACT_IN => {
            let amount_in = parse_u256(input, 1)?;
            let amount_out_min = parse_u256(input, 2)?;
            Some(UniswapSwap {
                from_asset: classify_input_asset(path_start, &amount_in, request)?,
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: amount_in,
                },
                to_asset: SwapAsset::Erc20(path_end),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Minimum,
                    value: amount_out_min,
                },
            })
        }
        UNIVERSAL_ROUTER_COMMAND_V3_SWAP_EXACT_OUT => {
            let amount_out = parse_u256(input, 1)?;
            let amount_in_max = parse_u256(input, 2)?;
            // v3 exact out path is encoded in reverse: out -> in.
            Some(UniswapSwap {
                from_asset: classify_input_asset(path_end, &amount_in_max, request)?,
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Maximum,
                    value: amount_in_max,
                },
                to_asset: SwapAsset::Erc20(path_start),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: amount_out,
                },
            })
        }
        _ => None,
    }
}

fn parse_uniswap_universal_router_swap(
    data: &[u8],
    request: &Transaction<'_>,
) -> Option<UniswapSwap> {
    let selector: [u8; 4] = data.get(..4)?.try_into().ok()?;
    if selector != SELECTOR_UNIVERSAL_ROUTER_EXECUTE
        && selector != SELECTOR_UNIVERSAL_ROUTER_EXECUTE_WITH_DEADLINE
    {
        return None;
    }
    let args = data.get(4..)?;
    let commands_offset = parse_usize_word(parse_word(args, 0)?)?;
    let inputs_offset = parse_usize_word(parse_word(args, 1)?)?;
    let commands = parse_abi_bytes(args, commands_offset)?;
    let num_inputs = parse_usize_at(args, inputs_offset)?;
    let max_entries = core::cmp::min(commands.len(), num_inputs);
    let wrapped_native_token = wrapped_native(request.chain_id());
    let has_tx_value = tx_value_nonzero(request).is_some();
    let has_unwrap_after = |commands: &[u8], start_index: usize| {
        commands[start_index..]
            .iter()
            .any(|command| (command & UNIVERSAL_ROUTER_COMMAND_MASK) == UNIVERSAL_ROUTER_COMMAND_UNWRAP_WETH)
    };

    for i in 0..max_entries {
        let command = commands[i] & UNIVERSAL_ROUTER_COMMAND_MASK;
        let input = parse_abi_bytes_array_element(args, inputs_offset, i)?;
        let parsed = match command {
            UNIVERSAL_ROUTER_COMMAND_V2_SWAP_EXACT_IN | UNIVERSAL_ROUTER_COMMAND_V2_SWAP_EXACT_OUT => {
                parse_uniswap_universal_router_swap_input_v2(command, input, request)
            }
            UNIVERSAL_ROUTER_COMMAND_V3_SWAP_EXACT_IN | UNIVERSAL_ROUTER_COMMAND_V3_SWAP_EXACT_OUT => {
                parse_uniswap_universal_router_swap_input_v3(command, input, request)
            }
            _ => None,
        };
        if let Some(mut parsed_swap) = parsed {
            if let Some(wrapped_native_token) = wrapped_native_token {
                if parsed_swap.from_asset == SwapAsset::Erc20(wrapped_native_token) && has_tx_value {
                    parsed_swap.from_asset = SwapAsset::Native;
                }
                if parsed_swap.to_asset == SwapAsset::Erc20(wrapped_native_token)
                    && has_unwrap_after(commands, i + 1)
                {
                    parsed_swap.to_asset = SwapAsset::Native;
                }
            }
            return Some(parsed_swap);
        }
    }
    None
}

fn parse_uniswap_swap_data(
    data: &[u8],
    request: &Transaction<'_>,
    depth: usize,
) -> Option<UniswapSwap> {
    if depth > 2 || data.len() < 4 {
        return None;
    }
    if let Some(parsed) = parse_uniswap_swap_v2(data, request) {
        return Some(parsed);
    }
    if let Some(parsed) = parse_uniswap_swap_v3(data, request) {
        return Some(parsed);
    }
    if let Some(parsed) = parse_uniswap_universal_router_swap(data, request) {
        return Some(parsed);
    }

    let selector: [u8; 4] = data.get(..4)?.try_into().ok()?;
    let args = data.get(4..)?;
    let bytes_array_arg = match selector {
        SELECTOR_MULTICALL => 0,
        SELECTOR_MULTICALL_WITH_DEADLINE => 1,
        _ => return None,
    };
    let array_offset = parse_usize_word(parse_word(args, bytes_array_arg)?)?;
    let num_elements = parse_usize_at(args, array_offset)?;
    let head_start = array_offset.checked_add(32)?;
    for element_index in 0..num_elements {
        let rel_offset = parse_usize_at(args, head_start.checked_add(element_index.checked_mul(32)?)?)?;
        let element_offset = head_start.checked_add(rel_offset)?;
        let element_data = parse_abi_bytes(args, element_offset)?;
        if let Some(parsed) = parse_uniswap_swap_data(element_data, request, depth + 1) {
            return Some(parsed);
        }
    }
    None
}

fn parse_uniswap_swap(request: &Transaction<'_>) -> Option<UniswapSwap> {
    parse_uniswap_swap_data(request.data(), request, 0)
}

enum SwapAssetDisplay {
    Known {
        unit: &'static str,
        decimals: usize,
    },
    UnknownToken {
        address: String,
    },
}

fn resolve_swap_asset(
    asset: &SwapAsset,
    chain_params: &Params,
    case: pb::EthAddressCase,
) -> SwapAssetDisplay {
    match asset {
        SwapAsset::Native => SwapAssetDisplay::Known {
            unit: chain_params.unit,
            decimals: WEI_DECIMALS,
        },
        SwapAsset::Erc20(address) => {
            if let Some(token) = erc20_params::get(chain_params.chain_id, *address) {
                SwapAssetDisplay::Known {
                    unit: token.unit,
                    decimals: token.decimals as usize,
                }
            } else {
                SwapAssetDisplay::UnknownToken {
                    address: super::address::format_display_address(&super::address::from_pubkey_hash(
                        address,
                        case,
                    )),
                }
            }
        }
    }
}

fn format_swap_amount(amount: &SwapAmount, asset: &SwapAssetDisplay) -> String {
    let mut line = match amount.bound {
        SwapAmountBound::Exact => String::new(),
        SwapAmountBound::Minimum => "min. ".into(),
        SwapAmountBound::Maximum => "max. ".into(),
    };
    match asset {
        SwapAssetDisplay::Known { unit, decimals } => {
            line.push_str(
                &Amount {
                    unit,
                    decimals: *decimals,
                    value: amount.value.clone(),
                }
                .format(),
            );
            line
        }
        SwapAssetDisplay::UnknownToken { address } => {
            line.push_str(&amount.value.to_str_radix(10));
            line.push_str(" raw units\n");
            line.push_str(address);
            line
        }
    }
}

async fn verify_uniswap_swap_transaction(
    hal: &mut impl crate::hal::Hal,
    request: &Transaction<'_>,
    chain_params: &Params,
    swap: &UniswapSwap,
) -> Result<(), Error> {
    let case = request.case()?;
    let from_asset = resolve_swap_asset(&swap.from_asset, chain_params, case);
    let to_asset = resolve_swap_asset(&swap.to_asset, chain_params, case);
    let from_display = format_swap_amount(&swap.from_amount, &from_asset);
    let to_display = format_swap_amount(&swap.to_amount, &to_asset);

    hal.ui()
        .confirm(&ConfirmParams {
            title: "Swap\nfrom",
            body: &from_display,
            scrollable: true,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    hal.ui()
        .confirm(&ConfirmParams {
            title: "Swap\nto",
            body: &to_display,
            scrollable: true,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

    let fee = parse_fee(request, chain_params);
    let (formatted_total, fee_percentage): (String, Option<f64>) = match swap.from_asset {
        SwapAsset::Native => {
            let total_value = (&swap.from_amount.value).add(&fee.value);
            let mut total = Amount {
                unit: chain_params.unit,
                decimals: WEI_DECIMALS,
                value: total_value,
            }
            .format();
            if swap.from_amount.bound == SwapAmountBound::Maximum {
                total = format!("max. {}", total);
            }
            (
                total,
                calculate_percentage(&fee.value, &swap.from_amount.value),
            )
        }
        _ => (from_display, None),
    };
    transaction::verify_total_fee_maybe_warn(
        hal,
        &formatted_total,
        &fee.format(),
        fee_percentage,
    )
    .await?;
    Ok(())
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
    let recipient_address_display = super::address::format_display_address(&recipient_address);
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
        .verify_recipient(&recipient_address_display, &formatted_value)
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
    let address_display = super::address::format_display_address(&address);
    let amount = Amount {
        unit: params.unit,
        decimals: WEI_DECIMALS,
        value: BigUint::from_bytes_be(request.value()),
    };
    hal.ui()
        .verify_recipient(&address_display, &amount.format())
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
    } else if let Some(uniswap_swap) = parse_uniswap_swap(request) {
        verify_uniswap_swap_transaction(hal, request, &params, &uniswap_swap).await?;
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
        Some(&host_nonce),
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

    fn abi_word_u64(value: u64) -> [u8; 32] {
        let mut word = [0u8; 32];
        word[24..].copy_from_slice(&value.to_be_bytes());
        word
    }

    fn abi_word_usize(value: usize) -> [u8; 32] {
        abi_word_u64(value as u64)
    }

    fn abi_word_address(address: [u8; 20]) -> [u8; 32] {
        let mut word = [0u8; 32];
        word[12..].copy_from_slice(&address);
        word
    }

    fn encode_multicall(calls: &[Vec<u8>]) -> Vec<u8> {
        let mut out = Vec::<u8>::new();
        out.extend_from_slice(&SELECTOR_MULTICALL);
        out.extend_from_slice(&abi_word_usize(32));

        out.extend_from_slice(&abi_word_usize(calls.len()));
        let offsets_start = out.len();
        for _ in calls {
            out.extend_from_slice(&[0u8; 32]);
        }

        let mut current_relative_offset = calls.len() * 32;
        for (index, call) in calls.iter().enumerate() {
            let start = offsets_start + index * 32;
            out[start..start + 32].copy_from_slice(&abi_word_usize(current_relative_offset));
            out.extend_from_slice(&abi_word_usize(call.len()));
            out.extend_from_slice(call);

            let padding = (32 - (call.len() % 32)) % 32;
            out.resize(out.len() + padding, 0);

            current_relative_offset += 32 + call.len() + padding;
        }
        out
    }

    fn make_v2_swap_exact_eth_for_tokens_data() -> Vec<u8> {
        const WETH: [u8; 20] = hex!("c02aaA39b223Fe8D0A0E5C4F27eAD9083C756Cc2");
        const USDT: [u8; 20] = hex!("dac17f958d2ee523a2206206994597c13d831ec7");
        const RECIPIENT: [u8; 20] = hex!("e6ce0a092a99700cd4ccccbb1fedc39cf53e6330");
        let mut data = Vec::<u8>::new();
        data.extend_from_slice(&SELECTOR_SWAP_EXACT_ETH_FOR_TOKENS);
        data.extend_from_slice(&abi_word_u64(57_000_000)); // amountOutMin
        data.extend_from_slice(&abi_word_u64(0x80)); // path offset
        data.extend_from_slice(&abi_word_address(RECIPIENT)); // to
        data.extend_from_slice(&abi_word_u64(1)); // deadline
        data.extend_from_slice(&abi_word_u64(2)); // path length
        data.extend_from_slice(&abi_word_address(WETH));
        data.extend_from_slice(&abi_word_address(USDT));
        data
    }

    fn make_v3_exact_input_single_data() -> Vec<u8> {
        const WETH: [u8; 20] = hex!("c02aaA39b223Fe8D0A0E5C4F27eAD9083C756Cc2");
        const USDT: [u8; 20] = hex!("dac17f958d2ee523a2206206994597c13d831ec7");
        const RECIPIENT: [u8; 20] = hex!("e6ce0a092a99700cd4ccccbb1fedc39cf53e6330");
        let mut data = Vec::<u8>::new();
        data.extend_from_slice(&SELECTOR_EXACT_INPUT_SINGLE);
        data.extend_from_slice(&abi_word_address(WETH));
        data.extend_from_slice(&abi_word_address(USDT));
        data.extend_from_slice(&abi_word_u64(3000)); // fee
        data.extend_from_slice(&abi_word_address(RECIPIENT));
        data.extend_from_slice(&abi_word_u64(1)); // deadline
        data.extend_from_slice(&abi_word_u64(1_000_000_000_000_000_000)); // amountIn
        data.extend_from_slice(&abi_word_u64(57_000_000)); // amountOutMinimum
        data.extend_from_slice(&abi_word_u64(0)); // sqrtPriceLimitX96
        data
    }

    fn encode_abi_bytes(data: &[u8]) -> Vec<u8> {
        let mut out = Vec::<u8>::new();
        out.extend_from_slice(&abi_word_usize(data.len()));
        out.extend_from_slice(data);
        let padding = (32 - (data.len() % 32)) % 32;
        out.resize(out.len() + padding, 0);
        out
    }

    fn encode_abi_bytes_array(elements: &[Vec<u8>]) -> Vec<u8> {
        let mut out = Vec::<u8>::new();
        out.extend_from_slice(&abi_word_usize(elements.len()));
        let offsets_start = out.len();
        for _ in elements {
            out.extend_from_slice(&[0u8; 32]);
        }

        let mut current_relative_offset = elements.len() * 32;
        for (index, element) in elements.iter().enumerate() {
            let start = offsets_start + index * 32;
            out[start..start + 32].copy_from_slice(&abi_word_usize(current_relative_offset));
            let encoded = encode_abi_bytes(element);
            out.extend_from_slice(&encoded);
            current_relative_offset += encoded.len();
        }
        out
    }

    fn encode_universal_router_execute(commands: &[u8], inputs: &[Vec<u8>]) -> Vec<u8> {
        let commands_blob = encode_abi_bytes(commands);
        let inputs_blob = encode_abi_bytes_array(inputs);
        let head_size = 64usize;
        let commands_offset = head_size;
        let inputs_offset = head_size + commands_blob.len();

        let mut out = Vec::<u8>::new();
        out.extend_from_slice(&SELECTOR_UNIVERSAL_ROUTER_EXECUTE);
        out.extend_from_slice(&abi_word_usize(commands_offset));
        out.extend_from_slice(&abi_word_usize(inputs_offset));
        out.extend_from_slice(&commands_blob);
        out.extend_from_slice(&inputs_blob);
        out
    }

    fn make_universal_router_v3_exact_in_input() -> Vec<u8> {
        const WETH: [u8; 20] = hex!("c02aaA39b223Fe8D0A0E5C4F27eAD9083C756Cc2");
        const USDT: [u8; 20] = hex!("dac17f958d2ee523a2206206994597c13d831ec7");
        const RECIPIENT: [u8; 20] = hex!("e6ce0a092a99700cd4ccccbb1fedc39cf53e6330");
        let mut path = Vec::<u8>::new();
        path.extend_from_slice(&WETH);
        path.extend_from_slice(&[0x00, 0x0b, 0xb8]); // 3000
        path.extend_from_slice(&USDT);

        let mut data = Vec::<u8>::new();
        data.extend_from_slice(&abi_word_address(RECIPIENT));
        data.extend_from_slice(&abi_word_u64(1_000_000_000_000_000_000)); // amountIn
        data.extend_from_slice(&abi_word_u64(57_000_000)); // amountOutMinimum
        data.extend_from_slice(&abi_word_u64(0xa0)); // path offset
        data.extend_from_slice(&abi_word_u64(0)); // payerIsUser=false
        data.extend_from_slice(&encode_abi_bytes(&path));
        data
    }

    fn make_universal_router_execute_v3_exact_in_data() -> Vec<u8> {
        encode_universal_router_execute(
            &[UNIVERSAL_ROUTER_COMMAND_V3_SWAP_EXACT_IN],
            &[make_universal_router_v3_exact_in_input()],
        )
    }

    fn make_universal_router_v3_exact_in_usdt_to_weth_input() -> Vec<u8> {
        const WETH: [u8; 20] = hex!("c02aaA39b223Fe8D0A0E5C4F27eAD9083C756Cc2");
        const USDT: [u8; 20] = hex!("dac17f958d2ee523a2206206994597c13d831ec7");
        const RECIPIENT: [u8; 20] = hex!("0000000000000000000000000000000000000002"); // router
        let mut path = Vec::<u8>::new();
        path.extend_from_slice(&USDT);
        path.extend_from_slice(&[0x00, 0x0b, 0xb8]); // 3000
        path.extend_from_slice(&WETH);

        let mut data = Vec::<u8>::new();
        data.extend_from_slice(&abi_word_address(RECIPIENT));
        data.extend_from_slice(&abi_word_u64(57_000_000)); // amountIn
        data.extend_from_slice(&abi_word_u64(10_000_000_000_000_000)); // amountOutMinimum
        data.extend_from_slice(&abi_word_u64(0xa0)); // path offset
        data.extend_from_slice(&abi_word_u64(0)); // payerIsUser=false
        data.extend_from_slice(&encode_abi_bytes(&path));
        data
    }

    fn make_universal_router_unwrap_weth_input() -> Vec<u8> {
        const RECIPIENT: [u8; 20] = hex!("e6ce0a092a99700cd4ccccbb1fedc39cf53e6330");
        let mut data = Vec::<u8>::new();
        data.extend_from_slice(&abi_word_address(RECIPIENT));
        data.extend_from_slice(&abi_word_u64(10_000_000_000_000_000)); // amountMin
        data
    }

    fn make_universal_router_execute_v3_exact_in_usdt_to_eth_data() -> Vec<u8> {
        encode_universal_router_execute(
            &[
                UNIVERSAL_ROUTER_COMMAND_V3_SWAP_EXACT_IN,
                UNIVERSAL_ROUTER_COMMAND_UNWRAP_WETH,
            ],
            &[
                make_universal_router_v3_exact_in_usdt_to_weth_input(),
                make_universal_router_unwrap_weth_input(),
            ],
        )
    }

    #[test]
    fn test_parse_uniswap_v2_swap_exact_eth_for_tokens() {
        let request = pb::EthSignRequest {
            value: hex!("0de0b6b3a7640000").to_vec(), // 1 ETH
            data: make_v2_swap_exact_eth_for_tokens_data(),
            chain_id: 1,
            ..Default::default()
        };
        assert_eq!(
            parse_uniswap_swap(&Transaction::Legacy(&request)),
            Some(UniswapSwap {
                from_asset: SwapAsset::Native,
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: BigUint::from(1_000_000_000_000_000_000u64),
                },
                to_asset: SwapAsset::Erc20(hex!("dac17f958d2ee523a2206206994597c13d831ec7")),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Minimum,
                    value: BigUint::from(57_000_000u64),
                },
            })
        );
    }

    #[test]
    fn test_parse_uniswap_v3_exact_input_single_in_multicall() {
        let inner_call = make_v3_exact_input_single_data();
        let request = pb::EthSignRequest {
            value: hex!("0de0b6b3a7640000").to_vec(), // 1 ETH
            data: encode_multicall(&[inner_call]),
            chain_id: 1,
            ..Default::default()
        };
        assert_eq!(
            parse_uniswap_swap(&Transaction::Legacy(&request)),
            Some(UniswapSwap {
                from_asset: SwapAsset::Native,
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: BigUint::from(1_000_000_000_000_000_000u64),
                },
                to_asset: SwapAsset::Erc20(hex!("dac17f958d2ee523a2206206994597c13d831ec7")),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Minimum,
                    value: BigUint::from(57_000_000u64),
                },
            })
        );
    }

    #[test]
    fn test_parse_uniswap_universal_router_execute_v3_exact_in() {
        let request = pb::EthSignRequest {
            value: hex!("0de0b6b3a7640000").to_vec(), // 1 ETH
            data: make_universal_router_execute_v3_exact_in_data(),
            chain_id: 1,
            ..Default::default()
        };
        assert_eq!(
            parse_uniswap_swap(&Transaction::Legacy(&request)),
            Some(UniswapSwap {
                from_asset: SwapAsset::Native,
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: BigUint::from(1_000_000_000_000_000_000u64),
                },
                to_asset: SwapAsset::Erc20(hex!("dac17f958d2ee523a2206206994597c13d831ec7")),
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Minimum,
                    value: BigUint::from(57_000_000u64),
                },
            })
        );
    }

    #[test]
    fn test_parse_uniswap_universal_router_execute_v3_exact_in_with_unwrap() {
        let request = pb::EthSignRequest {
            value: b"".to_vec(),
            data: make_universal_router_execute_v3_exact_in_usdt_to_eth_data(),
            chain_id: 1,
            ..Default::default()
        };
        assert_eq!(
            parse_uniswap_swap(&Transaction::Legacy(&request)),
            Some(UniswapSwap {
                from_asset: SwapAsset::Erc20(hex!("dac17f958d2ee523a2206206994597c13d831ec7")),
                from_amount: SwapAmount {
                    bound: SwapAmountBound::Exact,
                    value: BigUint::from(57_000_000u64),
                },
                to_asset: SwapAsset::Native,
                to_amount: SwapAmount {
                    bound: SwapAmountBound::Minimum,
                    value: BigUint::from(10_000_000_000_000_000u64),
                },
            })
        );
    }

    #[test]
    pub fn test_process_uniswap_swap_exact_eth_for_tokens() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
        let expected_screens = vec![
            Screen::Confirm {
                title: "".into(),
                body: "Sign transaction on\n\nEthereum".into(),
                longtouch: false,
            },
            Screen::Confirm {
                title: "Swap\nfrom".into(),
                body: "1 ETH".into(),
                longtouch: false,
            },
            Screen::Confirm {
                title: "Swap\nto".into(),
                body: "min. 57 USDT".into(),
                longtouch: false,
            },
            Screen::TotalFee {
                total: "1.0012658164 ETH".into(),
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
        assert!(
            block_on(process(&mut mock_hal, &Transaction::Legacy(&pb::EthSignRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                nonce: hex!("2367").to_vec(),
                gas_price: hex!("027aca1a80").to_vec(),
                gas_limit: hex!("01d048").to_vec(),
                recipient: hex!("7a250d5630b4cf539739df2c5dacab4c659f2488").to_vec(),
                value: hex!("0de0b6b3a7640000").to_vec(), // 1 ETH
                data: make_v2_swap_exact_eth_for_tokens_data(),
                host_nonce_commitment: None,
                chain_id: 1,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            })))
            .is_ok()
        );
        assert_eq!(mock_hal.ui.screens, expected_screens);

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert!(
            block_on(process(
                &mut mock_hal,
                &Transaction::Eip1559(&pb::EthSignEip1559Request {
                    keypath: KEYPATH.to_vec(),
                    nonce: hex!("2367").to_vec(),
                    max_priority_fee_per_gas: b"".to_vec(),
                    max_fee_per_gas: hex!("027aca1a80").to_vec(),
                    gas_limit: hex!("01d048").to_vec(),
                    recipient: hex!("7a250d5630b4cf539739df2c5dacab4c659f2488").to_vec(),
                    value: hex!("0de0b6b3a7640000").to_vec(), // 1 ETH
                    data: make_v2_swap_exact_eth_for_tokens_data(),
                    host_nonce_commitment: None,
                    chain_id: 1,
                    address_case: pb::EthAddressCase::Mixed as _,
                    data_length: 0,
                })
            ))
            .is_ok()
        );
        assert_eq!(mock_hal.ui.screens, expected_screens);
    }

    #[test]
    pub fn test_process_uniswap_universal_router_v3_exact_in() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
        let expected_screens = vec![
            Screen::Confirm {
                title: "".into(),
                body: "Sign transaction on\n\nEthereum".into(),
                longtouch: false,
            },
            Screen::Confirm {
                title: "Swap\nfrom".into(),
                body: "1 ETH".into(),
                longtouch: false,
            },
            Screen::Confirm {
                title: "Swap\nto".into(),
                body: "min. 57 USDT".into(),
                longtouch: false,
            },
            Screen::TotalFee {
                total: "1.0012658164 ETH".into(),
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
        assert!(
            block_on(process(&mut mock_hal, &Transaction::Legacy(&pb::EthSignRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                nonce: hex!("2367").to_vec(),
                gas_price: hex!("027aca1a80").to_vec(),
                gas_limit: hex!("01d048").to_vec(),
                recipient: hex!("ef1c6e67703c7bd7107eed8303fbe6ec2554bf6b").to_vec(),
                value: hex!("0de0b6b3a7640000").to_vec(), // 1 ETH
                data: make_universal_router_execute_v3_exact_in_data(),
                host_nonce_commitment: None,
                chain_id: 1,
                address_case: pb::EthAddressCase::Mixed as _,
                data_length: 0,
            })))
            .is_ok()
        );
        assert_eq!(mock_hal.ui.screens, expected_screens);
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
                recipient: "0x 04F2 64Cf 3444 0313 B4A0 192A 3528 14FB e927 b885".into(),
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
                    recipient: "0x 04F2 64Cf 3444 0313 B4A0 192A 3528 14FB e927 b885".into(),
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
                    recipient: "0x 04F2 64Cf 3444 0313 B4A0 192A 3528 14FB e927 b885".into(),
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
                    recipient: "0x 04F2 64Cf 3444 0313 B4A0 192A 3528 14FB e927 b885".into(),
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
                    recipient: "0x 04F2 64Cf 3444 0313 B4A0 192A 3528 14FB e927 b885".into(),
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
                    recipient: "0x 04F2 64Cf 3444 0313 B4A0 192A 3528 14FB e927 b885".into(),
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
                recipient: "0x E6CE 0a09 2A99 700C D4cc CcBb 1fED c39C f53E 6330".into(),
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
                recipient: "0x 857B 3D96 9eAc B775 a9f7 9cab c62E c4bB 1D1c d60e".into(),
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
                        recipient: "0x 04F2 64Cf 3444 0313 B4A0 192A 3528 14FB e927 b885".into(),
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
                    recipient: "0x 04F2 64Cf 3444 0313 B4A0 192A 3528 14FB e927 b885".into(),
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
                    recipient: "0x 1122 3344 5566 7788 99Aa bbcC DDeE FF00 1122 3344".into(),
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
    pub fn test_streaming_1_byte_legacy() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
        let test_data: Vec<u8> = vec![0x42];

        setup_chunk_responder(test_data);
        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        let result = block_on(process(
            &mut mock_hal,
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
                data_length: 1,
            }),
        ));
        clear_chunk_responder();
        match result {
            Ok(Response::Sign(ref sig)) => {
                assert_eq!(sig.signature.len(), 65);
            }
            other => panic!("expected Ok(Sign), got {:?}", other),
        }
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
                    recipient: "0x 1122 3344 5566 7788 99Aa bbcC DDeE FF00 1122 3344".into(),
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
