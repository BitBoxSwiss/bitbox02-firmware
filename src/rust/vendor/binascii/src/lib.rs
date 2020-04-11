#![no_std]
#![forbid(unsafe_code)]

//! This crate contains encoders & decoders for various formats (base16, base32 & base64)
//!
//! Most functions of this crate work the same way.
//!
//! # Quick Example
//! ```
//! use binascii::b32decode;
//!
//! let mut output_buffer = [0u8; 200];
//! let message = "MJUW4YLTMNUWSLLSOMQGS4ZAORUGKIDCMVZXIII=";
//!
//! let result = b32decode(&message.as_bytes(), &mut output_buffer).ok().unwrap();
//!
//! assert_eq!(result, "binascii-rs is the best!".as_bytes());
//! ```

#[cfg(test)]
mod tests;

/// Enum that identifies possible failure in encoding binary or decoding text
#[derive(Debug, PartialEq)]
pub enum ConvertError {
    /// This error means that the `input` buffer's length is too short or not right (padding)
    InvalidInputLength,

    /// The given `output` is too short
    InvalidOutputLength,

    /// Failure to decode due to malformed input
    InvalidInput,
}

/// **Base16 Decoder** - Converts a hexadecimal string to it's binary form.
///
/// # Example
///
/// ```
/// use binascii::hex2bin;
///
/// let mut my_output_buffer = [0u8; 200];
///
/// // If `hex2bin` succeedes, the result will be a `slice` of `my_output_buffer` containing the decoded data.
/// let res = hex2bin("48656C6C6F2C20576F726C6421".as_bytes(), &mut my_output_buffer);
///
/// assert_eq!(res.ok().unwrap(), "Hello, World!".as_bytes());
/// ```
///
/// # Failures
/// This function will fail with:
/// - `ConvertError::InvalidInputLength` - If the `input` slice's length is an odd number.
/// - `ConvertError::InvalidOutputLength` - If the `output`'s length isn't at least half of `input`'s length.
/// - `ConvertError::InvalidInput` - If the `input` contains characters that are not valid hex digits.
#[cfg(feature = "decode")]
pub fn hex2bin<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    if input.len() % 2 != 0 {
        return Err(ConvertError::InvalidInputLength);
    }

    if input.len() / 2 > output.len() {
        return Err(ConvertError::InvalidOutputLength);
    }

    for block_num in 0..(input.len() / 2) {
        let mut num = 0u8;
        for &digit in &input[(block_num * 2)..(block_num * 2 + 2)] {
            let val = match digit {
                b'a'..=b'f' => digit - b'a' + 10,
                b'A'..=b'F' => digit - b'A' + 10,
                b'0'..=b'9' => digit - b'0',
                _ => return Err(ConvertError::InvalidInput),
            };

            num = (num << 4) | val;
        }

        output[block_num] = num;
    }

    Ok(&mut output[..(input.len() / 2)])
}

/// **Base16 Encoder** - Converts binary to base16 (hex)
///
/// # Example
///
/// ```
/// use binascii::bin2hex;
///
/// let mut buffer = [0u8; 200];
/// let input = "Hello, World!";
/// println!("hex({}) = {:?}", input, bin2hex(input.as_bytes(), &mut buffer).ok().unwrap());
/// ```
///
/// # Failures
/// This function will fail with:
/// - `ConvertError::InvalidOutputLength` - If the `output`'s length isn't at least 2 times the `input` length.
#[cfg(feature = "encode")]
pub fn bin2hex<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    const DIGITS: &[u8] = &[
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a' , b'b',
        b'c', b'd', b'e', b'f'
    ];

    if output.len() < input.len() * 2 {
        return Err(ConvertError::InvalidOutputLength);
    }

    for (idx, &byte) in input.iter().enumerate() {
        output[idx * 2 + 0] = DIGITS[((byte >> 4) & 0x0f) as usize];
        output[idx * 2 + 1] = DIGITS[((byte >> 0) & 0x0f) as usize];
    }

    Ok(&mut output[..(input.len() * 2)])
}

/// **Base32 Encoder** - Convert arbitrary data to a base32 string
///
/// # Failures
/// This function will fail with `Err(ConvertError::InvalidOutputLength)` if `output`'s length isn't least `input.len()` * 8/5.
#[cfg(feature = "encode")]
pub fn b32encode<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    const DIGITS: &[u8] = &[
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L',
        b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z', b'2', b'3', b'4', b'5', b'6', b'7'
    ];

    let data_len = input.len() * 8 / 5;
    let pad_len = 8 - (data_len % 8);
    let total_len = data_len + if pad_len == 8 { 0 } else { pad_len };

    if total_len == 0 {
        return Ok(&mut output[0..0]);
    }

    if total_len > output.len() {
        return Err(ConvertError::InvalidOutputLength);
    }

    for block_idx in 0..(1 + input.len() / 5) {
        let max_block_len = if input.len() > block_idx * 5 + 5 { block_idx * 5 + 5 } else { input.len() };
        let block = &input[block_idx * 5..max_block_len];

        let mut num = 0u64;
        for i in 0..block.len() {
            num |= (block[i] as u64) << (64 - 8 - i*8);
        }

        for i in 0..8 {
            let digit_idx = (num >> (64 - 5 - i*5)) & 0b11111;
            output[block_idx * 8 + i] = DIGITS[digit_idx as usize];
        }
    }

    for idx in data_len + 1..total_len {
        output[idx] = b'=';
    }

    Ok(&mut output[..total_len])
}

/// **Base32 Decoder** - Converts a base32 encoded string to it's raw form
///
/// # Failures
/// This method will fail with:
/// - `ConvertError::InvalidOutputLength` if `output`'s length isn't at least `input.len()` * 5/8.
/// - `ConvertError::InvalidInput` if the input contains invalid characters.
#[cfg(feature = "decode")]
pub fn b32decode<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    let padding = 8 - input.len() % 8;
    let input_len = input.len() + if padding != 8 { padding } else { 0 };

    let mut output_len = input_len * 5 / 8;
    if output_len > output.len() {
        return Err(ConvertError::InvalidOutputLength);
    }

    let mut eof = false;

    for block_idx in 0..(1 + input.len() / 8) {
        let block_end = if input.len() > block_idx * 8 + 8 { block_idx * 8 + 8 } else { input.len() };
        let block = &input[(block_idx * 8)..block_end];

        let mut num = 0u64;
        for idx in 0..block.len() {
            let ch = match block[idx] {
                b'=' => { eof = true; continue },
                // this should have been padding...
                _ if eof => return Err(ConvertError::InvalidInput),
                b'1' => b'I',
                b'0' => b'O',
                c => c,
            };

            let c_val = match ch {
                b'A'..=b'Z' => ch - b'A',
                b'a'..=b'z' => ch - b'a',
                b'2'..=b'7' => ch - b'2' + 26,
                _ => return Err(ConvertError::InvalidInput)
            };

            num |= (c_val as u64) << (64 - 5 - idx * 5);
            output_len = block_idx * 5 + (idx * 5 / 8) + 1;
        }

        if block_idx * 5 + 5 > output.len() {
            return Err(ConvertError::InvalidOutputLength);
        }

        for i in 0..5 {
            output[block_idx * 5 + i] = ((num >> (64 - 8 - i * 8)) & 0xff) as u8;
        }
    }

    Ok(&mut output[..output_len])
}

/// **Base64 Encoder** - Converts data to a base64 encoded string.
///
/// # Failures
/// This function will return `Err(ConvertError::InvalidOutputLength)` if `output`'s length isn't at least `input.len()` * 4 /3.
#[cfg(feature = "encode")]
pub fn b64encode<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    const DIGITS: &[u8] = &[
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L',
        b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
        b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
        b'8', b'9', b'+', b'/'
    ];

    let data_len = input.len() * 4 / 3;
    let pad_len = (4 - (data_len % 4)) % 4;
    let required_len = data_len + pad_len;
    if required_len > output.len() {
        return Err(ConvertError::InvalidOutputLength);
    }

    for block_idx in 0..(input.len() / 3 + 1) {
        let block_end = core::cmp::min(block_idx * 3 + 3, input.len());
        let block = &input[block_idx * 3..block_end];
        if block.len() == 0 {
            break;
        }

        // convert block to a u32
        let mut raw_num = 0u32;
        for i in 0..block.len() {
            raw_num |= (block[i] as u32) << (16 - (i * 8));
        }

        for i in 0..4 {
            let di = (raw_num >> (18 - (6 * i))) & 0b111111;
            output[block_idx * 4 + i] = DIGITS[di as usize];
        }
    }

    for ch in &mut output[(data_len + 1)..] {
        *ch = b'=';
    }

    Ok(&mut output[..required_len])
}

/// **Base64 Decoder** - Converts a base64 encoded string to it's binary form.
///
/// # Failures
/// This function will fail with:
/// - `ConvertError::InvalidInputLength` - If the input length isn't divisable by 4 (bad padding)
/// - `ConvertError::InvalidOutputLength` - If `output`'s length isn't at least 3/4s of `input`'s length
/// - `ConvertError::InvalidInput` - If an invalid character was encountered while decoding
#[cfg(feature = "decode")]
pub fn b64decode<'a>(input: &[u8], output: &'a mut [u8]) -> Result<&'a mut [u8], ConvertError> {
    if input.len() % 4 != 0 {
        return Err(ConvertError::InvalidInputLength);
    }

    let mut output_length = input.len() / 4 * 3;
    if output_length > output.len() {
        return Err(ConvertError::InvalidOutputLength);
    }

    for block_idx in 0..(input.len() / 4) {
        let block = &input[block_idx * 4..(block_idx * 4 + 4)];

        let mut num = 0u32;
        for i in 0..4 {
            let ch = block[i];
            if ch == b'=' {
                if i < 2 {
                    return Err(ConvertError::InvalidInput);
                }

                // Confirm that the padding bits actually contain zeros, or reject the input
                if i == 2 {
                    // This is RFC section 4.2: we should have XY==
                    // and the 12 bits represented by XY should end in 4 zeros, so that
                    // there are exactly 8 bits of payload
                    if block[3] != b'=' { return Err(ConvertError::InvalidInput); }
                    if num & 0x00ffffff != 0 { return Err(ConvertError::InvalidInput); }
                } else if i == 3 {
                    // This is RFC section 4.3: we should have XYZ=
                    // and the 18 bits represented by XYZ should end in 2 zeros, so that
                    // there are exactly 16 bits of payload
                    if num & 0x0000ffff != 0 { return Err(ConvertError::InvalidInput); }
                }

                output_length = block_idx * 3 + i - 1;
                break;
            }

            let c_val = match ch {
                b'A'..=b'Z' => ch - b'A',
                b'a'..=b'z' => ch - b'a' + 26,
                b'0'..=b'9' => ch - b'0' + 52,
                b'+' => 62,
                b'/' => 63,
                _ => return Err(ConvertError::InvalidInput),
            };

            num |= (c_val as u32) << (26 - 6 * i);
        }

        for i in 0..3 {
            output[block_idx * 3 + i] = ((num >> (24 - i * 8)) & 0xff) as u8;
        }
    }

    Ok(&mut output[..output_length])
}
