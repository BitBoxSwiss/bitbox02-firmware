// SPDX-License-Identifier: Apache-2.0

use sha3::{Digest, Keccak256};

const RLP_SMALL_TAG: u8 = 0xc0;
const RLP_LARGE_TAG: u8 = 0xf7;

use core::cell::RefCell;
use core::ops::DerefMut;

use alloc::vec::Vec;

use super::Error;

/// An async producer/generator of a bytes array. This is used to be able to accumulate the RLP hash
/// of the `data` field, which can be very large and has to be streamed in chunks in that case.
#[allow(async_fn_in_trait)]
pub trait DataProducer {
    type Error;
    /// Returns the length of the data.
    fn len(&self) -> usize;
    /// Returns the first byte of the data.
    fn first_byte(&self) -> u8;
    /// Produces a chunk of the data. Returns `Ok(Some(data))` if data was available,
    /// `Ok(None)` when there are no more chunks, or `Err` on failure.
    async fn next(&mut self) -> Result<Option<Vec<u8>>, Self::Error>;
}

/// Produces a byte slice in one shot.
pub struct SimpleProducer<'a>(&'a [u8], bool);

impl<'a> SimpleProducer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        SimpleProducer(data, false)
    }
}

impl<'a> DataProducer for SimpleProducer<'a> {
    type Error = Error;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn first_byte(&self) -> u8 {
        self.0[0]
    }

    async fn next(&mut self) -> Result<Option<Vec<u8>>, Self::Error> {
        if !self.1 {
            self.1 = true;
            Ok(Some(self.0.to_vec()))
        } else {
            Ok(None)
        }
    }
}

pub struct ChunkingProducer {
    total_length: usize,
    offset: usize,
    first_byte_cached: Option<u8>,
}

impl ChunkingProducer {
    pub fn new(total_length: usize) -> Self {
        Self {
            total_length,
            offset: 0,
            first_byte_cached: None,
        }
    }
}

impl DataProducer for ChunkingProducer {
    type Error = Error;

    fn len(&self) -> usize {
        self.total_length
    }

    fn first_byte(&self) -> u8 {
        self.first_byte_cached.unwrap()
    }

    async fn next(&mut self) -> Result<Option<Vec<u8>>, Self::Error> {
        if self.offset >= self.total_length {
            return Ok(None);
        }

        const CHUNK_SIZE: u32 = 4096;
        let remaining = self.total_length - self.offset;
        let chunk_length = core::cmp::min(CHUNK_SIZE as usize, remaining) as u32;

        let response = super::next_request(super::pb::eth_response::Response::DataChunkRequest(
            super::pb::EthSignDataRequestChunkResponse {
                offset: self.offset as u32,
                length: chunk_length,
            },
        ))
        .await?;

        match response {
            super::pb::eth_request::Request::DataChunk(
                super::pb::EthSignDataResponseChunkRequest { chunk },
            ) => {
                // Error: chunk size mismatch
                if chunk.len() != chunk_length as usize {
                    return Err(Error::InvalidInput);
                }

                if self.offset == 0 && !chunk.is_empty() {
                    self.first_byte_cached = Some(chunk[0]);
                }

                self.offset += chunk.len();
                Ok(Some(chunk))
            }
            // Error: wrong response type
            _ => Err(Error::InvalidInput),
        }
    }
}

pub struct ParamsLegacy<'a, D: DataProducer> {
    pub nonce: &'a [u8],
    pub gas_price: &'a [u8],
    pub gas_limit: &'a [u8],
    pub recipient: &'a [u8],
    pub value: &'a [u8],
    pub data: RefCell<D>,
    pub chain_id: u64,
}

pub struct ParamsEIP1559<'a, D: DataProducer> {
    pub chain_id: u64,
    pub nonce: &'a [u8],
    pub max_priority_fee_per_gas: &'a [u8],
    pub max_fee_per_gas: &'a [u8],
    pub gas_limit: &'a [u8],
    pub recipient: &'a [u8],
    pub value: &'a [u8],
    pub data: RefCell<D>,
}

trait Write {
    // Writes the given data to the writer.
    fn write(&mut self, data: &[u8]);
    // Same as `write`, but it writes all the data produced by the async data producer.
    async fn write_producer<D: DataProducer, T: DerefMut<Target = D>>(
        &mut self,
        producer: T,
    ) -> Result<(), D::Error>;
}

struct Hasher(Keccak256);

impl Write for Hasher {
    fn write(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    async fn write_producer<D: DataProducer, T: DerefMut<Target = D>>(
        &mut self,
        mut producer: T,
    ) -> Result<(), D::Error> {
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

    async fn write_producer<D: DataProducer, T: DerefMut<Target = D>>(
        &mut self,
        producer: T,
    ) -> Result<(), D::Error> {
        self.0 += producer.len() as u32;
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
async fn hash_producer<W: Write, D: DataProducer, T: DerefMut<Target = D>>(
    writer: &mut W,
    producer: T,
) -> Result<(), D::Error> {
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

async fn hash_params_legacy<W: Write, D: DataProducer>(
    writer: &mut W,
    params: &ParamsLegacy<'_, D>,
) -> Result<(), D::Error> {
    hash_element(writer, params.nonce);
    hash_element(writer, params.gas_price);
    hash_element(writer, params.gas_limit);
    hash_element(writer, params.recipient);
    hash_element(writer, params.value);
    hash_producer(writer, params.data.borrow_mut()).await?;
    {
        // EIP155, encodes <chainID><0><0>
        hash_u64(writer, params.chain_id);
        hash_u64(writer, 0);
        hash_u64(writer, 0);
    }
    Ok(())
}

async fn hash_params_eip1559<W: Write, D: DataProducer>(
    writer: &mut W,
    params: &ParamsEIP1559<'_, D>,
) -> Result<(), D::Error> {
    hash_u64(writer, params.chain_id);
    hash_element(writer, params.nonce);
    hash_element(writer, params.max_priority_fee_per_gas);
    hash_element(writer, params.max_fee_per_gas);
    hash_element(writer, params.gas_limit);
    hash_element(writer, params.recipient);
    hash_element(writer, params.value);
    hash_producer(writer, params.data.borrow_mut()).await?;
    hash_header(writer, RLP_SMALL_TAG, RLP_LARGE_TAG, 0); // access list not currently supported and hashed as empty list
    Ok(())
}

/// Computes the sighash of an Ethereum transaction, using the chain_id as described in EIP155.
/// `params` are the transaction data. nonce, gas_price, gas_limit, and value are big endian and are
/// not allowed to have leading zeros (unchecked).
///
/// See https://github.com/ethereum/wiki/wiki/RLP
pub async fn compute_legacy<D: DataProducer<Error = Error>>(
    params: &ParamsLegacy<'_, D>,
) -> Result<[u8; 32], Error> {
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

pub async fn compute_eip1559<D: DataProducer<Error = Error>>(
    params: &ParamsEIP1559<'_, D>,
) -> Result<[u8; 32], Error> {
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

#[cfg(any(test, feature = "testing"))]
pub mod tests {
    use super::*;

    use alloc::boxed::Box;
    #[cfg(test)]
    use util::bb02_async::block_on;

    pub fn setup_chunk_responder(data: Vec<u8>) {
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() = Some(Box::new(
            move |response: crate::pb::response::Response| match response {
                crate::pb::response::Response::Eth(crate::pb::EthResponse {
                    response: Some(super::super::pb::eth_response::Response::DataChunkRequest(req)),
                }) => {
                    let offset = req.offset as usize;
                    let length = req.length as usize;
                    let chunk = data[offset..offset + length].to_vec();
                    Ok(crate::pb::request::Request::Eth(crate::pb::EthRequest {
                        request: Some(super::super::pb::eth_request::Request::DataChunk(
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

    pub fn decode_hex(s: &str) -> Vec<u8> {
        hex::decode(s).unwrap()
    }

    #[test]
    fn test_simple_producer_empty() {
        let mut producer = SimpleProducer::new(&[]);
        assert_eq!(producer.len(), 0);

        let chunk = block_on(producer.next());
        assert_eq!(chunk, Ok(Some(vec![])));

        let chunk2 = block_on(producer.next());
        assert_eq!(chunk2, Ok(None));
    }

    #[test]
    fn test_simple_producer_single_byte() {
        let mut producer = SimpleProducer::new(&[0x42]);
        assert_eq!(producer.len(), 1);
        assert_eq!(producer.first_byte(), 0x42);

        let chunk = block_on(producer.next());
        assert_eq!(chunk, Ok(Some(vec![0x42])));

        let chunk2 = block_on(producer.next());
        assert_eq!(chunk2, Ok(None));
    }

    #[test]
    fn test_simple_producer_4096_bytes() {
        let data = vec![0xAB; 4096];
        let mut producer = SimpleProducer::new(&data);
        assert_eq!(producer.len(), 4096);
        assert_eq!(producer.first_byte(), 0xAB);

        let chunk = block_on(producer.next());
        assert_eq!(chunk, Ok(Some(data.clone())));

        let chunk2 = block_on(producer.next());
        assert_eq!(chunk2, Ok(None));
    }

    #[test]
    fn test_simple_producer_10kb() {
        let data = vec![0xCD; 10000];
        let mut producer = SimpleProducer::new(&data);
        assert_eq!(producer.len(), 10000);
        assert_eq!(producer.first_byte(), 0xCD);

        let chunk = block_on(producer.next());
        assert_eq!(chunk, Ok(Some(data)));
    }

    #[test]
    fn test_chunking_producer_len() {
        assert_eq!(ChunkingProducer::new(1).len(), 1);
        assert_eq!(ChunkingProducer::new(4096).len(), 4096);
        assert_eq!(ChunkingProducer::new(4097).len(), 4097);
        assert_eq!(ChunkingProducer::new(10000).len(), 10000);
    }

    #[test]
    fn test_chunking_producer_single_chunk() {
        let data = vec![0xAB; 100];
        setup_chunk_responder(data.clone());

        let mut producer = ChunkingProducer::new(100);
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

        let mut producer = ChunkingProducer::new(10000);
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
