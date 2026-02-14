// SPDX-License-Identifier: Apache-2.0

use sha3::{Digest, Keccak256};

const RLP_SMALL_TAG: u8 = 0xc0;
const RLP_LARGE_TAG: u8 = 0xf7;

use core::future::Future;
use core::pin::Pin;

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::Error;

/// An async producer/generator of a bytes array. This is used to be able to accumulate the RLP hash
/// of the `data` field, which can be very large and has to be streamed in chunks in that case.
pub trait DataProducer {
    /// Returns the length of the data.
    fn len(&self) -> u32;
    /// Returns the first byte of the data.
    fn first_byte(&self) -> u8;
    /// Produces a chunk of the data. Returns `Ok(Some(data))` if data was available,
    /// `Ok(None)` when there are no more chunks, or `Err` on failure.
    fn next<'a>(&'a mut self)
    -> Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, Error>> + 'a>>;
}

pub enum ChunkingProducer<'a> {
    Inline {
        data: &'a [u8],
        consumed: bool,
    },
    Host {
        total_length: u32,
        offset: u32,
        first_byte_cached: Option<u8>,
    },
}

impl<'a> ChunkingProducer<'a> {
    pub fn from_data(data: &'a [u8]) -> Self {
        Self::Inline {
            data,
            consumed: false,
        }
    }

    pub fn from_host(total_length: u32) -> Self {
        Self::Host {
            total_length,
            offset: 0,
            first_byte_cached: None,
        }
    }
}

impl DataProducer for ChunkingProducer<'_> {
    fn len(&self) -> u32 {
        match self {
            Self::Inline { data, .. } => data.len() as u32,
            Self::Host { total_length, .. } => *total_length,
        }
    }

    fn first_byte(&self) -> u8 {
        match self {
            Self::Inline { data, .. } => data[0],
            Self::Host {
                first_byte_cached, ..
            } => first_byte_cached.unwrap(),
        }
    }

    fn next<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, Error>> + 'a>> {
        Box::pin(async move {
            match self {
                Self::Inline { data, consumed } => {
                    if !*consumed {
                        *consumed = true;
                        Ok(Some(data.to_vec()))
                    } else {
                        Ok(None)
                    }
                }
                Self::Host {
                    total_length,
                    offset,
                    first_byte_cached,
                } => {
                    if *offset >= *total_length {
                        return Ok(None);
                    }

                    const CHUNK_SIZE: u32 = 4096;
                    let remaining = *total_length - *offset;
                    let chunk_length = core::cmp::min(CHUNK_SIZE, remaining);

                    let response =
                        super::next_request(super::pb::eth_response::Response::DataRequestChunk(
                            super::pb::EthSignDataRequestChunkResponse {
                                offset: *offset,
                                length: chunk_length,
                            },
                        ))
                        .await?;

                    match response {
                        super::pb::eth_request::Request::DataResponseChunk(
                            super::pb::EthSignDataResponseChunkRequest { chunk },
                        ) => {
                            // Error: chunk size mismatch
                            if chunk.len() as u32 != chunk_length {
                                return Err(Error::InvalidInput);
                            }

                            if *offset == 0 && !chunk.is_empty() {
                                *first_byte_cached = Some(chunk[0]);
                            }

                            *offset += chunk.len() as u32;
                            Ok(Some(chunk))
                        }
                        // Error: wrong response type
                        _ => Err(Error::InvalidInput),
                    }
                }
            }
        })
    }
}

pub struct ParamsLegacy<'a> {
    pub nonce: &'a [u8],
    pub gas_price: &'a [u8],
    pub gas_limit: &'a [u8],
    pub recipient: &'a [u8],
    pub value: &'a [u8],
    pub data: &'a mut dyn DataProducer,
    pub chain_id: u64,
}

pub struct ParamsEIP1559<'a> {
    pub chain_id: u64,
    pub nonce: &'a [u8],
    pub max_priority_fee_per_gas: &'a [u8],
    pub max_fee_per_gas: &'a [u8],
    pub gas_limit: &'a [u8],
    pub recipient: &'a [u8],
    pub value: &'a [u8],
    pub data: &'a mut dyn DataProducer,
}

trait Write {
    // Writes the given data to the writer.
    fn write(&mut self, data: &[u8]);
    // Same as `write`, but it writes all the data produced by the async data producer.
    async fn write_producer(&mut self, producer: &mut dyn DataProducer) -> Result<(), Error>;
}

struct Hasher(Keccak256);

impl Write for Hasher {
    fn write(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    async fn write_producer(&mut self, producer: &mut dyn DataProducer) -> Result<(), Error> {
        while let Some(data) = producer.next().await? {
            self.0.update(&data);
        }
        Ok(())
    }
}

struct Counter(u32);

impl Write for Counter {
    fn write(&mut self, data: &[u8]) {
        self.0 += data.len() as u32;
    }

    async fn write_producer(&mut self, producer: &mut dyn DataProducer) -> Result<(), Error> {
        self.0 += producer.len();
        Ok(())
    }
}

fn hash_header<W: Write>(writer: &mut W, small_tag: u8, large_tag: u8, len: u16) {
    // According to the RLP spec., buffer headers are encoded differently for lengths below and
    // above 55 (for >55, length of length is encoded).
    if len <= 55 {
        writer.write(&[small_tag + len as u8]);
    } else if len <= 0xff {
        writer.write(&[large_tag + 1, len as u8]);
    } else {
        writer.write(&[large_tag + 2]);
        writer.write(&len.to_be_bytes());
    }
}

fn hash_element<W: Write>(writer: &mut W, bytes: &[u8]) {
    // hash header
    let len = bytes.len();
    if len != 1 || bytes[0] > 0x7f {
        hash_header(writer, 0x80, 0xb7, len as _);
    }
    writer.write(bytes);
}

// Async version of `hash_element()` that streams the data to be hashed from the producer.
async fn hash_producer<W: Write>(
    writer: &mut W,
    producer: &mut dyn DataProducer,
) -> Result<(), Error> {
    // hash header
    let len = producer.len();
    if len != 1 || producer.first_byte() > 0x7f {
        hash_header(writer, 0x80, 0xb7, len as _);
    }
    writer.write_producer(producer).await
}

fn hash_u64<W: Write>(writer: &mut W, value: u64) {
    let bigendian = value.to_be_bytes();
    let mut stripped: &[u8] = bigendian.as_slice();
    while let [0, rest @ ..] = stripped {
        stripped = rest;
    }
    hash_element(writer, stripped)
}

async fn hash_params_legacy<W: Write>(
    writer: &mut W,
    params: &mut ParamsLegacy<'_>,
) -> Result<(), Error> {
    hash_element(writer, params.nonce);
    hash_element(writer, params.gas_price);
    hash_element(writer, params.gas_limit);
    hash_element(writer, params.recipient);
    hash_element(writer, params.value);
    hash_producer(writer, &mut *params.data).await?;
    {
        // EIP155, encodes <chainID><0><0>
        hash_u64(writer, params.chain_id);
        hash_u64(writer, 0);
        hash_u64(writer, 0);
    }
    Ok(())
}

async fn hash_params_eip1559<W: Write>(
    writer: &mut W,
    params: &mut ParamsEIP1559<'_>,
) -> Result<(), Error> {
    hash_u64(writer, params.chain_id);
    hash_element(writer, params.nonce);
    hash_element(writer, params.max_priority_fee_per_gas);
    hash_element(writer, params.max_fee_per_gas);
    hash_element(writer, params.gas_limit);
    hash_element(writer, params.recipient);
    hash_element(writer, params.value);
    hash_producer(writer, &mut *params.data).await?;
    hash_header(writer, RLP_SMALL_TAG, RLP_LARGE_TAG, 0); // access list not currently supported and hashed as empty list
    Ok(())
}

/// Computes the sighash of an Ethereum transaction, using the chain_id as described in EIP155.
/// `params` are the transaction data. nonce, gas_price, gas_limit, and value are big endian and are
/// not allowed to have leading zeros (unchecked).
///
/// See https://github.com/ethereum/wiki/wiki/RLP
pub async fn compute_legacy(params: &mut ParamsLegacy<'_>) -> Result<[u8; 32], Error> {
    // We hash [nonce, gas price, gas limit, recipient, value, data], RLP encoded.
    // The list length prefix is (0xc0 + length of the encoding of all elements).

    // 1) calculate length
    let mut counter = Counter(0);
    hash_params_legacy(&mut counter, params).await?;

    if counter.0 > 0xffff {
        // Don't support bigger than this for now.
        return Err(Error::InvalidInput);
    }

    // 2) hash len and encoded tx elements
    let mut hasher = Hasher(Keccak256::new());
    hash_header(&mut hasher, RLP_SMALL_TAG, RLP_LARGE_TAG, counter.0 as u16);
    hash_params_legacy(&mut hasher, params).await?;
    Ok(hasher.0.finalize().into())
}

pub async fn compute_eip1559(params: &mut ParamsEIP1559<'_>) -> Result<[u8; 32], Error> {
    // https://eips.ethereum.org/EIPS/eip-1559
    // We hash [chain_id, nonce, max_priority_fee_per_gas, max_fee_per_gas, gas limit, recipient, value, data, access list]
    // RLP encoded. Prefixed with 0x02 for EIP1559 transaction type
    // The list length prefix is (0xc0 + length of the encoding of all elements).

    // 1) calculate length
    let mut counter = Counter(0);
    hash_params_eip1559(&mut counter, params).await?;

    if counter.0 > 0xffff {
        // Don't support bigger than this for now.
        return Err(Error::InvalidInput);
    }

    // 2) hash len and encoded tx elements
    let mut hasher = Hasher(Keccak256::new());
    hasher.write(&[0x02]); // prefix the rlp encoding with transaction type before hashing
    hash_header(&mut hasher, RLP_SMALL_TAG, RLP_LARGE_TAG, counter.0 as u16);
    hash_params_eip1559(&mut hasher, params).await?;
    Ok(hasher.0.finalize().into())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use alloc::boxed::Box;
    use alloc::string::String;
    use serde::Deserialize;
    use util::bb02_async::block_on;

    pub fn setup_chunk_responder(data: Vec<u8>) {
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(
            move |response: crate::pb::response::Response| match response {
                crate::pb::response::Response::Eth(crate::pb::EthResponse {
                    response: Some(super::super::pb::eth_response::Response::DataRequestChunk(req)),
                }) => {
                    let offset = req.offset as usize;
                    let length = req.length as usize;
                    let chunk = data[offset..offset + length].to_vec();
                    Ok(crate::pb::request::Request::Eth(crate::pb::EthRequest {
                        request: Some(super::super::pb::eth_request::Request::DataResponseChunk(
                            super::super::pb::EthSignDataResponseChunkRequest { chunk },
                        )),
                    }))
                }
                _ => panic!("unexpected response"),
            },
        ));
    }

    pub fn clear_chunk_responder() {
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = None;
    }

    fn decode_hex(s: &str) -> Vec<u8> {
        hex::decode(s).unwrap()
    }

    #[derive(Debug, Deserialize)]
    struct Eip1559TestCase {
        chain_id: u64,
        nonce: String,
        max_priority_fee: String,
        max_fee_per_gas: String,
        gas_limit: String,
        recipient: String,
        value: String,
        data: String,
        expected_sighash: String,
    }

    #[derive(Debug, Deserialize)]
    struct LegacyTestCase {
        chain_id: u64,
        nonce: String,
        gas_price: String,
        gas_limit: String,
        recipient: String,
        value: String,
        data: String,
        expected_sighash: String,
    }

    const DATA_THRESHOLD: usize = 6144;

    #[test]
    fn test_compute_eip1559() {
        let json_data = include_str!("testdata/eip1559_tests.json");
        let tests: Vec<Eip1559TestCase> = serde_json::from_str(json_data).unwrap();

        for (i, test) in tests.iter().enumerate() {
            let nonce = decode_hex(&test.nonce);
            let max_priority_fee = decode_hex(&test.max_priority_fee);
            let max_fee_per_gas = decode_hex(&test.max_fee_per_gas);
            let gas_limit = decode_hex(&test.gas_limit);
            let recipient = decode_hex(&test.recipient);
            let value = decode_hex(&test.value);
            let data = decode_hex(&test.data);
            let expected_sighash: [u8; 32] = decode_hex(&test.expected_sighash).try_into().unwrap();

            if data.len() < DATA_THRESHOLD {
                let mut producer = ChunkingProducer::from_data(&data);
                let mut params = ParamsEIP1559 {
                    chain_id: test.chain_id,
                    nonce: &nonce,
                    max_priority_fee_per_gas: &max_priority_fee,
                    max_fee_per_gas: &max_fee_per_gas,
                    gas_limit: &gas_limit,
                    recipient: &recipient,
                    value: &value,
                    data: &mut producer,
                };
                let result = block_on(compute_eip1559(&mut params)).unwrap();
                assert_eq!(
                    result, expected_sighash,
                    "EIP1559 test {} failed (ChunkingProducer::from_data)",
                    i
                );
            } else {
                setup_chunk_responder(data.clone());
                let mut producer = ChunkingProducer::from_host(data.len() as u32);
                let mut params = ParamsEIP1559 {
                    chain_id: test.chain_id,
                    nonce: &nonce,
                    max_priority_fee_per_gas: &max_priority_fee,
                    max_fee_per_gas: &max_fee_per_gas,
                    gas_limit: &gas_limit,
                    recipient: &recipient,
                    value: &value,
                    data: &mut producer,
                };
                let result = block_on(compute_eip1559(&mut params)).unwrap();
                assert_eq!(
                    result, expected_sighash,
                    "EIP1559 test {} failed (ChunkingProducer::from_host)",
                    i
                );
                clear_chunk_responder();
            }
        }
    }

    #[test]
    fn test_compute_legacy() {
        let json_data = include_str!("testdata/legacy_tests.json");
        let tests: Vec<LegacyTestCase> = serde_json::from_str(json_data).unwrap();

        for (i, test) in tests.iter().enumerate() {
            let nonce = decode_hex(&test.nonce);
            let gas_price = decode_hex(&test.gas_price);
            let gas_limit = decode_hex(&test.gas_limit);
            let recipient = decode_hex(&test.recipient);
            let value = decode_hex(&test.value);
            let data = decode_hex(&test.data);
            let expected_sighash: [u8; 32] = decode_hex(&test.expected_sighash).try_into().unwrap();

            if data.len() < DATA_THRESHOLD {
                let mut producer = ChunkingProducer::from_data(&data);
                let mut params = ParamsLegacy {
                    nonce: &nonce,
                    gas_price: &gas_price,
                    gas_limit: &gas_limit,
                    recipient: &recipient,
                    value: &value,
                    data: &mut producer,
                    chain_id: test.chain_id,
                };
                let result = block_on(compute_legacy(&mut params)).unwrap();
                assert_eq!(
                    result, expected_sighash,
                    "Legacy test {} failed (ChunkingProducer::from_data)",
                    i
                );
            } else {
                setup_chunk_responder(data.clone());
                let mut producer = ChunkingProducer::from_host(data.len() as u32);
                let mut params = ParamsLegacy {
                    nonce: &nonce,
                    gas_price: &gas_price,
                    gas_limit: &gas_limit,
                    recipient: &recipient,
                    value: &value,
                    data: &mut producer,
                    chain_id: test.chain_id,
                };
                let result = block_on(compute_legacy(&mut params)).unwrap();
                assert_eq!(
                    result, expected_sighash,
                    "Legacy test {} failed (ChunkingProducer::from_host)",
                    i
                );
                clear_chunk_responder();
            }
        }
    }

    #[test]
    fn test_chunking_producer_inline_empty() {
        let mut producer = ChunkingProducer::from_data(&[]);
        assert_eq!(producer.len(), 0);

        let chunk = block_on(producer.next());
        assert_eq!(chunk, Ok(Some(vec![])));

        let chunk2 = block_on(producer.next());
        assert_eq!(chunk2, Ok(None));
    }

    #[test]
    fn test_chunking_producer_inline_single_byte() {
        let mut producer = ChunkingProducer::from_data(&[0x42]);
        assert_eq!(producer.len(), 1);
        assert_eq!(producer.first_byte(), 0x42);

        let chunk = block_on(producer.next());
        assert_eq!(chunk, Ok(Some(vec![0x42])));

        let chunk2 = block_on(producer.next());
        assert_eq!(chunk2, Ok(None));
    }

    #[test]
    fn test_chunking_producer_inline_4096_bytes() {
        let data = vec![0xAB; 4096];
        let mut producer = ChunkingProducer::from_data(&data);
        assert_eq!(producer.len(), 4096);
        assert_eq!(producer.first_byte(), 0xAB);

        let chunk = block_on(producer.next());
        assert_eq!(chunk, Ok(Some(data.clone())));

        let chunk2 = block_on(producer.next());
        assert_eq!(chunk2, Ok(None));
    }

    #[test]
    fn test_chunking_producer_inline_10kb() {
        let data = vec![0xCD; 10000];
        let mut producer = ChunkingProducer::from_data(&data);
        assert_eq!(producer.len(), 10000);
        assert_eq!(producer.first_byte(), 0xCD);

        let chunk = block_on(producer.next());
        assert_eq!(chunk, Ok(Some(data)));
    }

    #[test]
    fn test_chunking_producer_host_len() {
        assert_eq!(ChunkingProducer::from_host(1).len(), 1);
        assert_eq!(ChunkingProducer::from_host(4096).len(), 4096);
        assert_eq!(ChunkingProducer::from_host(4097).len(), 4097);
        assert_eq!(ChunkingProducer::from_host(10000).len(), 10000);
    }

    #[test]
    fn test_chunking_producer_single_chunk() {
        let data = vec![0xAB; 100];
        setup_chunk_responder(data.clone());

        let mut producer = ChunkingProducer::from_host(100);
        assert_eq!(producer.len(), 100);

        let chunk = block_on(producer.next()).unwrap();
        assert_eq!(chunk, Some(data));
        assert_eq!(producer.first_byte(), 0xAB);

        let chunk2 = block_on(producer.next()).unwrap();
        assert_eq!(chunk2, None);

        clear_chunk_responder();
    }

    #[test]
    fn test_chunking_producer_multiple_chunks() {
        let data = vec![0xCD; 10000];
        setup_chunk_responder(data);

        let mut producer = ChunkingProducer::from_host(10000);
        assert_eq!(producer.len(), 10000);

        let chunk1 = block_on(producer.next()).unwrap().unwrap();
        assert_eq!(chunk1.len(), 4096);
        assert_eq!(producer.first_byte(), 0xCD);

        let chunk2 = block_on(producer.next()).unwrap().unwrap();
        assert_eq!(chunk2.len(), 4096);

        let chunk3 = block_on(producer.next()).unwrap().unwrap();
        assert_eq!(chunk3.len(), 1808);

        let chunk4 = block_on(producer.next()).unwrap();
        assert_eq!(chunk4, None);

        clear_chunk_responder();
    }
}
