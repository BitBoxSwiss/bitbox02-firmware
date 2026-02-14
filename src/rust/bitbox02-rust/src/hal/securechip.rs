// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use super::memory::PasswordStretchAlgo;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Model {
    Atecc608A,
    Atecc608B,
    OptigaTrustM3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    SecureChip(SecureChipError),
    Status(i32),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
// Keep in sync with securechip.h's securechip_error_t.
pub enum SecureChipError {
    // Errors common to any securechip implementation
    Ifs = -1,
    InvalidArgs = -2,
    ConfigMismatch = -3,
    Salt = -4,
    // Currently only used by Optiga, but it is in the common errors so that the API of the
    // securechip is consistent and the caller does not need to distinguish between the chips at
    // the callsite.
    IncorrectPassword = -6,
    // The password stretch algo is not supported
    InvalidPasswordStretchAlgo = -7,
    Memory = -8,
    // Errors specific to the ATECC
    AteccZoneUnlockedConfig = -100,
    AteccZoneUnlockedData = -101,
    AteccSlotUnlockedIo = -103,
    AteccSlotUnlockedAuth = -104,
    AteccSlotUnlockedEnc = -105,
    AteccResetKeys = -106,
    // Errors specific to the Optiga
    OptigaCreate = -201,
    OptigaUnexpectedMetadata = -204,
    OptigaPal = -205,
    OptigaUnexpectedLen = -206,
}

pub trait SecureChip {
    fn init_new_password(
        &mut self,
        password: &str,
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, Error>;
    fn stretch_password(
        &mut self,
        password: &str,
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, Error>;
    fn kdf(&mut self, msg: &[u8]) -> Result<zeroize::Zeroizing<Vec<u8>>, Error>;
    fn attestation_sign(
        &mut self,
        challenge: &[u8; 32],
        signature: &mut [u8; 64],
    ) -> Result<(), ()>;
    fn monotonic_increments_remaining(&mut self) -> Result<u32, ()>;
    fn model(&mut self) -> Result<Model, ()>;
    fn reset_keys(&mut self) -> Result<(), ()>;
    #[cfg(feature = "app-u2f")]
    fn u2f_counter_set(&mut self, counter: u32) -> Result<(), ()>;
}
