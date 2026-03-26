// SPDX-License-Identifier: Apache-2.0

use super::Error;
use super::params;
use super::pb;
use crate::hal::Ui;
use crate::hal::ui::ConfirmParams;
use crate::workflow::transaction;

use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryInto;

use pb::solana_response::Response;

const LAMPORT_DECIMALS: usize = 9;
const BASE_FEE_LAMPORTS_PER_SIGNATURE: u64 = 5000;
const DEFAULT_COMPUTE_UNIT_LIMIT_PER_INSTRUCTION: u32 = 200_000;
const MAX_COMPUTE_UNIT_LIMIT: u32 = 1_400_000;

const SYSTEM_PROGRAM_ID: [u8; 32] = [0u8; 32];
const SPL_TOKEN_PROGRAM_ID: [u8; 32] = [
    0x06, 0xdd, 0xf6, 0xe1, 0xd7, 0x65, 0xa1, 0x93, 0xd9, 0xcb, 0xe1, 0x46, 0xce, 0xeb, 0x79, 0xac,
    0x1c, 0xb4, 0x85, 0xed, 0x5f, 0x5b, 0x37, 0x91, 0x3a, 0x8c, 0xf5, 0x85, 0x7e, 0xff, 0x00, 0xa9,
];
const COMPUTE_BUDGET_PROGRAM_ID: [u8; 32] = [
    0x03, 0x06, 0x46, 0x6f, 0xe5, 0x21, 0x17, 0x32, 0xff, 0xec, 0xad, 0xba, 0x72, 0xc3, 0x9b, 0xe7,
    0xbc, 0x8c, 0xe5, 0xbb, 0xc5, 0xf7, 0x12, 0x6b, 0x2c, 0x43, 0x9b, 0x3a, 0x40, 0x00, 0x00, 0x00,
];

struct Reader<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, pos: 0 }
    }

    fn read_u8(&mut self) -> Result<u8, Error> {
        let byte = *self.input.get(self.pos).ok_or(Error::InvalidInput)?;
        self.pos += 1;
        Ok(byte)
    }

    fn read_exact(&mut self, len: usize) -> Result<&'a [u8], Error> {
        let end = self.pos.checked_add(len).ok_or(Error::InvalidInput)?;
        let out = self.input.get(self.pos..end).ok_or(Error::InvalidInput)?;
        self.pos = end;
        Ok(out)
    }

    fn read_compact_u16(&mut self) -> Result<u16, Error> {
        let mut result: u16 = 0;
        let mut shift = 0;
        for _ in 0..3 {
            let byte = self.read_u8()?;
            result |= u16::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(result);
            }
            shift += 7;
        }
        Err(Error::InvalidInput)
    }

    fn finish(&self) -> Result<(), Error> {
        if self.pos == self.input.len() {
            Ok(())
        } else {
            Err(Error::InvalidInput)
        }
    }
}

#[derive(Clone)]
struct Instruction {
    program_id_index: u8,
    account_indices: Vec<u8>,
    data: Vec<u8>,
}

enum Transfer {
    System {
        recipient: [u8; 32],
        lamports: u64,
    },
    Spl {
        recipient: [u8; 32],
        mint: [u8; 32],
        amount: u64,
        decimals: u8,
    },
}

struct ParsedMessage {
    transfer: Transfer,
    fee_lamports: u64,
}

fn read_pubkey(reader: &mut Reader<'_>) -> Result<[u8; 32], Error> {
    reader
        .read_exact(32)?
        .try_into()
        .or(Err(Error::InvalidInput))
}

fn parse_instruction(reader: &mut Reader<'_>) -> Result<Instruction, Error> {
    let program_id_index = reader.read_u8()?;
    let account_count = usize::from(reader.read_compact_u16()?);
    let mut account_indices = Vec::with_capacity(account_count);
    for _ in 0..account_count {
        account_indices.push(reader.read_u8()?);
    }
    let data_len = usize::from(reader.read_compact_u16()?);
    let data = reader.read_exact(data_len)?.to_vec();
    Ok(Instruction {
        program_id_index,
        account_indices,
        data,
    })
}

fn parse_system_transfer(
    instruction: &Instruction,
    account_keys: &[[u8; 32]],
) -> Result<Transfer, Error> {
    if instruction.account_indices.len() != 2 || instruction.data.len() != 12 {
        return Err(Error::InvalidInput);
    }
    let tag = u32::from_le_bytes(instruction.data[..4].try_into().unwrap());
    if tag != 2 || instruction.account_indices[0] != 0 {
        return Err(Error::InvalidInput);
    }
    let recipient = *account_keys
        .get(usize::from(instruction.account_indices[1]))
        .ok_or(Error::InvalidInput)?;
    Ok(Transfer::System {
        recipient,
        lamports: u64::from_le_bytes(instruction.data[4..12].try_into().unwrap()),
    })
}

fn parse_spl_transfer_checked(
    instruction: &Instruction,
    account_keys: &[[u8; 32]],
) -> Result<Transfer, Error> {
    if instruction.account_indices.len() != 4 || instruction.data.len() != 10 {
        return Err(Error::InvalidInput);
    }
    if instruction.data[0] != 12 || instruction.account_indices[3] != 0 {
        return Err(Error::InvalidInput);
    }
    let recipient = *account_keys
        .get(usize::from(instruction.account_indices[2]))
        .ok_or(Error::InvalidInput)?;
    let mint = *account_keys
        .get(usize::from(instruction.account_indices[1]))
        .ok_or(Error::InvalidInput)?;
    Ok(Transfer::Spl {
        recipient,
        mint,
        amount: u64::from_le_bytes(instruction.data[1..9].try_into().unwrap()),
        decimals: instruction.data[9],
    })
}

fn parse_compute_budget_instruction(
    instruction: &Instruction,
    compute_unit_limit: &mut Option<u32>,
    compute_unit_price: &mut Option<u64>,
) -> Result<(), Error> {
    match instruction.data.first().copied() {
        Some(2) if instruction.data.len() == 5 => {
            *compute_unit_limit = Some(u32::from_le_bytes(
                instruction.data[1..5].try_into().unwrap(),
            ));
            Ok(())
        }
        Some(3) if instruction.data.len() == 9 => {
            *compute_unit_price = Some(u64::from_le_bytes(
                instruction.data[1..9].try_into().unwrap(),
            ));
            Ok(())
        }
        _ => Err(Error::InvalidInput),
    }
}

fn priority_fee_lamports(
    compute_unit_limit: Option<u32>,
    compute_unit_price: Option<u64>,
    non_compute_budget_instruction_count: usize,
) -> Result<u64, Error> {
    let Some(compute_unit_price) = compute_unit_price else {
        return Ok(0);
    };
    let default_limit = core::cmp::min(
        DEFAULT_COMPUTE_UNIT_LIMIT_PER_INSTRUCTION
            .saturating_mul(non_compute_budget_instruction_count as u32),
        MAX_COMPUTE_UNIT_LIMIT,
    );
    let limit = u128::from(compute_unit_limit.unwrap_or(default_limit));
    let price = u128::from(compute_unit_price);
    let fee_micro_lamports = limit.checked_mul(price).ok_or(Error::InvalidInput)?;
    let lamports = fee_micro_lamports.div_ceil(1_000_000);
    u64::try_from(lamports).or(Err(Error::InvalidInput))
}

fn parse_message(message: &[u8], expected_signer: &[u8; 32]) -> Result<ParsedMessage, Error> {
    let mut reader = Reader::new(message);
    let first = reader.read_u8()?;
    let versioned = first & 0x80 != 0;
    let num_required_signatures = if versioned {
        if first & 0x7f != 0 {
            return Err(Error::InvalidInput);
        }
        reader.read_u8()?
    } else {
        first
    };
    if num_required_signatures != 1 {
        return Err(Error::InvalidInput);
    }
    let _num_readonly_signed = reader.read_u8()?;
    let _num_readonly_unsigned = reader.read_u8()?;

    let account_count = usize::from(reader.read_compact_u16()?);
    if account_count == 0 {
        return Err(Error::InvalidInput);
    }
    let mut account_keys = Vec::with_capacity(account_count);
    for _ in 0..account_count {
        account_keys.push(read_pubkey(&mut reader)?);
    }
    if account_keys[0] != *expected_signer {
        return Err(Error::InvalidInput);
    }

    let _recent_blockhash = reader.read_exact(32)?;
    let instruction_count = usize::from(reader.read_compact_u16()?);
    if instruction_count == 0 {
        return Err(Error::InvalidInput);
    }

    let mut transfer: Option<Transfer> = None;
    let mut compute_unit_limit: Option<u32> = None;
    let mut compute_unit_price: Option<u64> = None;
    let mut non_compute_budget_instruction_count = 0usize;

    for _ in 0..instruction_count {
        let instruction = parse_instruction(&mut reader)?;
        let program_id = *account_keys
            .get(usize::from(instruction.program_id_index))
            .ok_or(Error::InvalidInput)?;
        if program_id == SYSTEM_PROGRAM_ID {
            if transfer.is_some() {
                return Err(Error::InvalidInput);
            }
            transfer = Some(parse_system_transfer(&instruction, &account_keys)?);
            non_compute_budget_instruction_count += 1;
        } else if program_id == SPL_TOKEN_PROGRAM_ID {
            if transfer.is_some() {
                return Err(Error::InvalidInput);
            }
            transfer = Some(parse_spl_transfer_checked(&instruction, &account_keys)?);
            non_compute_budget_instruction_count += 1;
        } else if program_id == COMPUTE_BUDGET_PROGRAM_ID {
            parse_compute_budget_instruction(
                &instruction,
                &mut compute_unit_limit,
                &mut compute_unit_price,
            )?;
        } else {
            return Err(Error::InvalidInput);
        }
    }

    if versioned {
        let lookup_count = usize::from(reader.read_compact_u16()?);
        if lookup_count != 0 {
            return Err(Error::InvalidInput);
        }
    }
    reader.finish()?;

    let base_fee = u64::from(num_required_signatures) * BASE_FEE_LAMPORTS_PER_SIGNATURE;
    let priority_fee = priority_fee_lamports(
        compute_unit_limit,
        compute_unit_price,
        non_compute_budget_instruction_count,
    )?;
    let fee_lamports = base_fee
        .checked_add(priority_fee)
        .ok_or(Error::InvalidInput)?;
    Ok(ParsedMessage {
        transfer: transfer.ok_or(Error::InvalidInput)?,
        fee_lamports,
    })
}

fn format_amount(value: u64, decimals: usize, unit: &str) -> String {
    format!("{} {}", util::decimal::format(value, decimals), unit)
}

fn format_address(pubkey: &[u8; 32]) -> String {
    bitcoin::base58::encode(pubkey)
}

async fn verify(
    hal: &mut impl crate::hal::Hal,
    params: &params::Params,
    parsed: &ParsedMessage,
) -> Result<(), Error> {
    let formatted_fee = format_amount(parsed.fee_lamports, LAMPORT_DECIMALS, params.unit);
    match &parsed.transfer {
        Transfer::System {
            recipient,
            lamports,
        } => {
            let address = format_address(recipient);
            hal.ui()
                .verify_recipient(
                    &util::strings::format_address(&address),
                    &format_amount(*lamports, LAMPORT_DECIMALS, params.unit),
                )
                .await?;

            let total = lamports
                .checked_add(parsed.fee_lamports)
                .ok_or(Error::InvalidInput)?;
            let fee_percentage = if *lamports == 0 {
                None
            } else {
                Some((parsed.fee_lamports as f64) * 100. / (*lamports as f64))
            };
            transaction::verify_total_fee_maybe_warn(
                hal,
                &format_amount(total, LAMPORT_DECIMALS, params.unit),
                &formatted_fee,
                fee_percentage,
            )
            .await?;
        }
        Transfer::Spl {
            recipient,
            mint,
            amount,
            decimals,
        } => {
            let mint_address = format_address(mint);
            hal.ui()
                .confirm(&ConfirmParams {
                    title: params.name,
                    body: &format!(
                        "Token mint\n{}",
                        util::strings::format_address(&mint_address)
                    ),
                    scrollable: true,
                    accept_is_nextarrow: true,
                    ..Default::default()
                })
                .await?;
            let recipient_address = format_address(recipient);
            hal.ui()
                .verify_recipient(
                    &util::strings::format_address(&recipient_address),
                    &format_amount(*amount, usize::from(*decimals), "tokens"),
                )
                .await?;
            transaction::verify_total_fee_maybe_warn(
                hal,
                &format_amount(*amount, usize::from(*decimals), "tokens"),
                &formatted_fee,
                None,
            )
            .await?;
        }
    }
    Ok(())
}

pub async fn process(
    hal: &mut impl crate::hal::Hal,
    request: &pb::SolanaSignTransactionRequest,
) -> Result<Response, Error> {
    super::keypath::validate(&request.keypath)?;
    let network = pb::SolanaNetwork::try_from(request.network)?;
    let params = params::get(network);
    let expected_signer = super::derive_pubkey(hal, &request.keypath)?;
    let parsed = parse_message(&request.message, &expected_signer)?;
    verify(hal, params, &parsed).await?;

    let signature = crate::keystore::ed25519::sign_message(hal, &request.keypath, &request.message)
        .or(Err(Error::Generic))?
        .signature;
    Ok(Response::SignTransaction(
        pb::SolanaSignTransactionResponse {
            signature: signature.to_vec(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;
    use crate::keystore::testing::mock_unlocked;
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};
    use util::bb02_async::block_on;
    use util::bip32::HARDENED;

    fn encode_compact_u16(mut value: u16) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            out.push(byte);
            if value == 0 {
                break;
            }
        }
        out
    }

    fn push_instruction(
        out: &mut Vec<u8>,
        program_id_index: u8,
        account_indices: &[u8],
        data: &[u8],
    ) {
        out.push(program_id_index);
        out.extend_from_slice(&encode_compact_u16(account_indices.len() as u16));
        out.extend_from_slice(account_indices);
        out.extend_from_slice(&encode_compact_u16(data.len() as u16));
        out.extend_from_slice(data);
    }

    fn make_legacy_message(account_keys: &[[u8; 32]], instructions: &[Instruction]) -> Vec<u8> {
        let mut out = vec![1, 0, 1];
        out.extend_from_slice(&encode_compact_u16(account_keys.len() as u16));
        for account_key in account_keys {
            out.extend_from_slice(account_key);
        }
        out.extend_from_slice(&[0x42; 32]);
        out.extend_from_slice(&encode_compact_u16(instructions.len() as u16));
        for instruction in instructions {
            push_instruction(
                &mut out,
                instruction.program_id_index,
                &instruction.account_indices,
                &instruction.data,
            );
        }
        out
    }

    fn make_v0_message(account_keys: &[[u8; 32]], instructions: &[Instruction]) -> Vec<u8> {
        let mut out = vec![0x80, 1, 0, 1];
        out.extend_from_slice(&encode_compact_u16(account_keys.len() as u16));
        for account_key in account_keys {
            out.extend_from_slice(account_key);
        }
        out.extend_from_slice(&[0x24; 32]);
        out.extend_from_slice(&encode_compact_u16(instructions.len() as u16));
        for instruction in instructions {
            push_instruction(
                &mut out,
                instruction.program_id_index,
                &instruction.account_indices,
                &instruction.data,
            );
        }
        out.push(0);
        out
    }

    fn system_transfer_instruction(lamports: u64) -> Instruction {
        let mut data = Vec::new();
        data.extend_from_slice(&2u32.to_le_bytes());
        data.extend_from_slice(&lamports.to_le_bytes());
        Instruction {
            program_id_index: 2,
            account_indices: vec![0, 1],
            data,
        }
    }

    fn compute_budget_limit_instruction(program_id_index: u8, limit: u32) -> Instruction {
        let mut data = vec![2];
        data.extend_from_slice(&limit.to_le_bytes());
        Instruction {
            program_id_index,
            account_indices: vec![],
            data,
        }
    }

    fn compute_budget_price_instruction(program_id_index: u8, price: u64) -> Instruction {
        let mut data = vec![3];
        data.extend_from_slice(&price.to_le_bytes());
        Instruction {
            program_id_index,
            account_indices: vec![],
            data,
        }
    }

    fn spl_transfer_checked_instruction(amount: u64, decimals: u8) -> Instruction {
        let mut data = vec![12];
        data.extend_from_slice(&amount.to_le_bytes());
        data.push(decimals);
        Instruction {
            program_id_index: 4,
            account_indices: vec![1, 2, 3, 0],
            data,
        }
    }

    #[test]
    fn test_parse_message_rejects_address_lookup_tables() {
        let mut message = vec![0x80, 1, 0, 1, 1];
        message.extend_from_slice(&[0u8; 32]);
        message.extend_from_slice(&[0u8; 32]);
        message.push(0);
        message.push(1);
        message.extend_from_slice(&[0u8; 32]);
        message.push(0);
        message.push(0);
        assert!(matches!(
            parse_message(&message, &[0u8; 32]),
            Err(Error::InvalidInput)
        ));
    }

    #[test]
    fn test_process_system_transfer_legacy() {
        mock_unlocked();
        let keypath = vec![44 + HARDENED, 501 + HARDENED, HARDENED, HARDENED];
        let signer = super::super::derive_pubkey(&mut TestingHal::new(), &keypath).unwrap();
        let recipient = [0x11; 32];
        let account_keys = vec![
            signer,
            recipient,
            SYSTEM_PROGRAM_ID,
            COMPUTE_BUDGET_PROGRAM_ID,
        ];
        let instructions = vec![
            compute_budget_limit_instruction(3, 10_000),
            compute_budget_price_instruction(3, 1_000_000),
            system_transfer_instruction(1_500_000_000),
        ];
        let message = make_legacy_message(&account_keys, &instructions);

        let mut hal = TestingHal::new();
        let response = block_on(process(
            &mut hal,
            &pb::SolanaSignTransactionRequest {
                network: pb::SolanaNetwork::SolanaMainnet as _,
                keypath: keypath.clone(),
                message: message.clone(),
            },
        ))
        .unwrap();
        let Response::SignTransaction(pb::SolanaSignTransactionResponse { signature }) = response
        else {
            panic!("unexpected response");
        };
        let signature = Signature::from_slice(&signature).unwrap();
        VerifyingKey::from_bytes(&signer)
            .unwrap()
            .verify(&message, &signature)
            .unwrap();
        assert!(hal.ui.screens.contains(&Screen::Recipient {
            recipient: util::strings::format_address(&format_address(&recipient)),
            amount: "1.5 SOL".into(),
        }));
        assert!(hal.ui.screens.contains(&Screen::TotalFee {
            total: "1.500015 SOL".into(),
            fee: "0.000015 SOL".into(),
            longtouch: true,
        }));
    }

    #[test]
    fn test_process_spl_transfer_v0() {
        mock_unlocked();
        let keypath = vec![44 + HARDENED, 501 + HARDENED, HARDENED, HARDENED];
        let signer = super::super::derive_pubkey(&mut TestingHal::new(), &keypath).unwrap();
        let source = [0x21; 32];
        let mint = [0x22; 32];
        let recipient = [0x23; 32];
        let account_keys = vec![
            signer,
            source,
            mint,
            recipient,
            SPL_TOKEN_PROGRAM_ID,
            COMPUTE_BUDGET_PROGRAM_ID,
        ];
        let instructions = vec![
            compute_budget_limit_instruction(5, 20_000),
            spl_transfer_checked_instruction(123_450_000, 6),
        ];
        let message = make_v0_message(&account_keys, &instructions);

        let mut hal = TestingHal::new();
        let response = block_on(process(
            &mut hal,
            &pb::SolanaSignTransactionRequest {
                network: pb::SolanaNetwork::SolanaDevnet as _,
                keypath: keypath.clone(),
                message: message.clone(),
            },
        ))
        .unwrap();
        let Response::SignTransaction(pb::SolanaSignTransactionResponse { signature }) = response
        else {
            panic!("unexpected response");
        };
        let signature = Signature::from_slice(&signature).unwrap();
        VerifyingKey::from_bytes(&signer)
            .unwrap()
            .verify(&message, &signature)
            .unwrap();
        assert!(hal.ui.contains_confirm(
            "Solana Devnet",
            &format!(
                "Token mint\n{}",
                util::strings::format_address(&format_address(&mint))
            ),
        ));
        assert!(hal.ui.screens.contains(&Screen::Recipient {
            recipient: util::strings::format_address(&format_address(&recipient)),
            amount: "123.45 tokens".into(),
        }));
    }
}
