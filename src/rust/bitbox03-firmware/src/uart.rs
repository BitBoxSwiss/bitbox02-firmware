// SPDX-License-Identifier: Apache-2.0

use core::ffi::CStr;
use core::fmt;

const DEFAULT_TIMEOUT_MS: u32 = 1000;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    Peripheral,
    Busy,
    Timeout,
    Unknown(u32),
}

impl Error {
    fn from_hal_status(status: st_drivers_sys::HAL_StatusTypeDef) -> Result<(), Self> {
        match status as u32 {
            0 => Ok(()),
            1 => Err(Self::Peripheral),
            2 => Err(Self::Busy),
            3 => Err(Self::Timeout),
            code => Err(Self::Unknown(code)),
        }
    }
}

pub struct Uart {
    timeout_ms: u32,
    last_byte_was_cr: bool,
}

impl Default for Uart {
    fn default() -> Self {
        Self::new(DEFAULT_TIMEOUT_MS)
    }
}

impl Uart {
    pub const fn new(timeout_ms: u32) -> Self {
        Self {
            timeout_ms,
            last_byte_was_cr: false,
        }
    }

    //pub fn write_all(&mut self, bytes: &[u8]) -> Result<(), Error> {
    //    self.write_all_raw(bytes)?;
    //    self.last_byte_was_cr = bytes.last().copied() == Some(b'\r');
    //    Ok(())
    //}

    pub fn write_all_crlf(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let mut segment_start = 0usize;
        for (i, byte) in bytes.iter().copied().enumerate() {
            if byte != b'\n' {
                continue;
            }
            if segment_start < i {
                self.write_all_raw(&bytes[segment_start..i])?;
            }

            let previous_was_cr = if i == 0 {
                self.last_byte_was_cr
            } else {
                bytes[i - 1] == b'\r'
            };

            if previous_was_cr {
                self.write_all_raw(b"\n")?;
            } else {
                self.write_all_raw(b"\r\n")?;
            }
            segment_start = i + 1;
        }

        if segment_start < bytes.len() {
            self.write_all_raw(&bytes[segment_start..])?;
        }

        self.last_byte_was_cr = bytes.last().copied() == Some(b'\r');
        Ok(())
    }

    pub fn write_cstr_crlf(&mut self, cstr: &CStr) -> Result<(), Error> {
        self.write_all_crlf(cstr.to_bytes())
    }

    fn write_all_raw(&mut self, mut bytes: &[u8]) -> Result<(), Error> {
        while !bytes.is_empty() {
            let chunk_len = bytes.len().min(u16::MAX as usize);
            self.write_chunk(&bytes[..chunk_len])?;
            bytes = &bytes[chunk_len..];
        }
        Ok(())
    }

    fn write_chunk(&mut self, chunk: &[u8]) -> Result<(), Error> {
        let status = unsafe {
            st_drivers_sys::HAL_UART_Transmit(
                &raw mut st_drivers_sys::huart1 as *mut _,
                chunk.as_ptr(),
                chunk.len() as u16,
                self.timeout_ms,
            )
        };
        Error::from_hal_status(status)
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all_crlf(s.as_bytes()).map_err(|_| fmt::Error)
    }
}
