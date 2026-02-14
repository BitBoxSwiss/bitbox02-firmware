// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use crate::hal::SecureChip;
use crate::hal::memory::PasswordStretchAlgo;
use crate::hal::securechip::{Error, Model, SecureChipError};

pub(crate) struct BitBox02SecureChip;

fn to_hal_model(model: bitbox02::securechip::Model) -> Model {
    match model {
        bitbox02::securechip::Model::ATECC_ATECC608A => Model::Atecc608A,
        bitbox02::securechip::Model::ATECC_ATECC608B => Model::Atecc608B,
        bitbox02::securechip::Model::OPTIGA_TRUST_M_V3 => Model::OptigaTrustM3,
    }
}

fn to_hal_error(error: bitbox02::securechip::Error) -> Error {
    match error {
        bitbox02::securechip::Error::SecureChip(sc_err) => Error::SecureChip(match sc_err {
            bitbox02::securechip::SecureChipError::SC_ERR_IFS => SecureChipError::Ifs,
            bitbox02::securechip::SecureChipError::SC_ERR_INVALID_ARGS => {
                SecureChipError::InvalidArgs
            }
            bitbox02::securechip::SecureChipError::SC_ERR_CONFIG_MISMATCH => {
                SecureChipError::ConfigMismatch
            }
            bitbox02::securechip::SecureChipError::SC_ERR_SALT => SecureChipError::Salt,
            bitbox02::securechip::SecureChipError::SC_ERR_INCORRECT_PASSWORD => {
                SecureChipError::IncorrectPassword
            }
            bitbox02::securechip::SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO => {
                SecureChipError::InvalidPasswordStretchAlgo
            }
            bitbox02::securechip::SecureChipError::SC_ERR_MEMORY => SecureChipError::Memory,
            bitbox02::securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_CONFIG => {
                SecureChipError::AteccZoneUnlockedConfig
            }
            bitbox02::securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_DATA => {
                SecureChipError::AteccZoneUnlockedData
            }
            bitbox02::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_IO => {
                SecureChipError::AteccSlotUnlockedIo
            }
            bitbox02::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_AUTH => {
                SecureChipError::AteccSlotUnlockedAuth
            }
            bitbox02::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_ENC => {
                SecureChipError::AteccSlotUnlockedEnc
            }
            bitbox02::securechip::SecureChipError::SC_ATECC_ERR_RESET_KEYS => {
                SecureChipError::AteccResetKeys
            }
            bitbox02::securechip::SecureChipError::SC_OPTIGA_ERR_CREATE => {
                SecureChipError::OptigaCreate
            }
            bitbox02::securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_METADATA => {
                SecureChipError::OptigaUnexpectedMetadata
            }
            bitbox02::securechip::SecureChipError::SC_OPTIGA_ERR_PAL => SecureChipError::OptigaPal,
            bitbox02::securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN => {
                SecureChipError::OptigaUnexpectedLen
            }
        }),
        bitbox02::securechip::Error::Status(status) => Error::Status(status),
    }
}

impl SecureChip for BitBox02SecureChip {
    fn init_new_password(
        &mut self,
        password: &str,
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
        bitbox02::securechip::init_new_password(
            password,
            super::memory::to_bitbox02_password_stretch_algo(password_stretch_algo),
        )
        .map_err(to_hal_error)
    }

    fn stretch_password(
        &mut self,
        password: &str,
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
        bitbox02::securechip::stretch_password(
            password,
            super::memory::to_bitbox02_password_stretch_algo(password_stretch_algo),
        )
        .map_err(to_hal_error)
    }

    fn kdf(&mut self, msg: &[u8]) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
        bitbox02::securechip::kdf(msg).map_err(to_hal_error)
    }

    fn attestation_sign(
        &mut self,
        challenge: &[u8; 32],
        signature: &mut [u8; 64],
    ) -> Result<(), ()> {
        bitbox02::securechip::attestation_sign(challenge, signature)
    }

    fn monotonic_increments_remaining(&mut self) -> Result<u32, ()> {
        bitbox02::securechip::monotonic_increments_remaining()
    }

    fn model(&mut self) -> Result<Model, ()> {
        bitbox02::securechip::model().map(to_hal_model)
    }

    fn reset_keys(&mut self) -> Result<(), ()> {
        bitbox02::securechip::reset_keys()
    }

    #[cfg(feature = "app-u2f")]
    fn u2f_counter_set(&mut self, counter: u32) -> Result<(), ()> {
        bitbox02::securechip::u2f_counter_set(counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hal_model() {
        assert_eq!(
            to_hal_model(bitbox02::securechip::Model::ATECC_ATECC608A),
            Model::Atecc608A,
        );
        assert_eq!(
            to_hal_model(bitbox02::securechip::Model::ATECC_ATECC608B),
            Model::Atecc608B,
        );
        assert_eq!(
            to_hal_model(bitbox02::securechip::Model::OPTIGA_TRUST_M_V3),
            Model::OptigaTrustM3,
        );
    }

    #[test]
    fn test_to_hal_error_securechip() {
        let cases = [
            (
                bitbox02::securechip::SecureChipError::SC_ERR_IFS,
                SecureChipError::Ifs,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ERR_INVALID_ARGS,
                SecureChipError::InvalidArgs,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ERR_CONFIG_MISMATCH,
                SecureChipError::ConfigMismatch,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ERR_SALT,
                SecureChipError::Salt,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ERR_INCORRECT_PASSWORD,
                SecureChipError::IncorrectPassword,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO,
                SecureChipError::InvalidPasswordStretchAlgo,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ERR_MEMORY,
                SecureChipError::Memory,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_CONFIG,
                SecureChipError::AteccZoneUnlockedConfig,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_DATA,
                SecureChipError::AteccZoneUnlockedData,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_IO,
                SecureChipError::AteccSlotUnlockedIo,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_AUTH,
                SecureChipError::AteccSlotUnlockedAuth,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_ENC,
                SecureChipError::AteccSlotUnlockedEnc,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_ATECC_ERR_RESET_KEYS,
                SecureChipError::AteccResetKeys,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_OPTIGA_ERR_CREATE,
                SecureChipError::OptigaCreate,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_METADATA,
                SecureChipError::OptigaUnexpectedMetadata,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_OPTIGA_ERR_PAL,
                SecureChipError::OptigaPal,
            ),
            (
                bitbox02::securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN,
                SecureChipError::OptigaUnexpectedLen,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(
                to_hal_error(bitbox02::securechip::Error::SecureChip(input)),
                Error::SecureChip(expected),
            );
        }
    }

    #[test]
    fn test_to_hal_error_status() {
        assert_eq!(
            to_hal_error(bitbox02::securechip::Error::Status(7)),
            Error::Status(7)
        );
    }
}
