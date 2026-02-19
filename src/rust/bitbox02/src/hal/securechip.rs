// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use bitbox_hal::SecureChip;
use bitbox_hal::memory::PasswordStretchAlgo;
use bitbox_hal::securechip::{Error, Model, SecureChipError};

pub struct BitBox02SecureChip;

fn to_hal_model(model: crate::securechip::Model) -> Model {
    match model {
        crate::securechip::Model::ATECC_ATECC608A => Model::Atecc608A,
        crate::securechip::Model::ATECC_ATECC608B => Model::Atecc608B,
        crate::securechip::Model::OPTIGA_TRUST_M_V3 => Model::OptigaTrustM3,
    }
}

fn to_hal_error(error: crate::securechip::Error) -> Error {
    match error {
        crate::securechip::Error::SecureChip(sc_err) => Error::SecureChip(match sc_err {
            crate::securechip::SecureChipError::SC_ERR_IFS => SecureChipError::Ifs,
            crate::securechip::SecureChipError::SC_ERR_INVALID_ARGS => SecureChipError::InvalidArgs,
            crate::securechip::SecureChipError::SC_ERR_CONFIG_MISMATCH => {
                SecureChipError::ConfigMismatch
            }
            crate::securechip::SecureChipError::SC_ERR_SALT => SecureChipError::Salt,
            crate::securechip::SecureChipError::SC_ERR_INCORRECT_PASSWORD => {
                SecureChipError::IncorrectPassword
            }
            crate::securechip::SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO => {
                SecureChipError::InvalidPasswordStretchAlgo
            }
            crate::securechip::SecureChipError::SC_ERR_MEMORY => SecureChipError::Memory,
            crate::securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_CONFIG => {
                SecureChipError::AteccZoneUnlockedConfig
            }
            crate::securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_DATA => {
                SecureChipError::AteccZoneUnlockedData
            }
            crate::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_IO => {
                SecureChipError::AteccSlotUnlockedIo
            }
            crate::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_AUTH => {
                SecureChipError::AteccSlotUnlockedAuth
            }
            crate::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_ENC => {
                SecureChipError::AteccSlotUnlockedEnc
            }
            crate::securechip::SecureChipError::SC_ATECC_ERR_RESET_KEYS => {
                SecureChipError::AteccResetKeys
            }
            crate::securechip::SecureChipError::SC_OPTIGA_ERR_CREATE => {
                SecureChipError::OptigaCreate
            }
            crate::securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_METADATA => {
                SecureChipError::OptigaUnexpectedMetadata
            }
            crate::securechip::SecureChipError::SC_OPTIGA_ERR_PAL => SecureChipError::OptigaPal,
            crate::securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN => {
                SecureChipError::OptigaUnexpectedLen
            }
        }),
        crate::securechip::Error::Status(status) => Error::Status(status),
    }
}

impl SecureChip for BitBox02SecureChip {
    fn init_new_password(
        &mut self,
        password: &str,
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
        crate::securechip::init_new_password(
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
        crate::securechip::stretch_password(
            password,
            super::memory::to_bitbox02_password_stretch_algo(password_stretch_algo),
        )
        .map_err(to_hal_error)
    }

    fn kdf(&mut self, msg: &[u8]) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
        crate::securechip::kdf(msg).map_err(to_hal_error)
    }

    fn attestation_sign(
        &mut self,
        challenge: &[u8; 32],
        signature: &mut [u8; 64],
    ) -> Result<(), ()> {
        crate::securechip::attestation_sign(challenge, signature)
    }

    fn monotonic_increments_remaining(&mut self) -> Result<u32, ()> {
        crate::securechip::monotonic_increments_remaining()
    }

    fn model(&mut self) -> Result<Model, ()> {
        crate::securechip::model().map(to_hal_model)
    }

    fn reset_keys(&mut self) -> Result<(), ()> {
        crate::securechip::reset_keys()
    }

    #[cfg(feature = "app-u2f")]
    fn u2f_counter_set(&mut self, counter: u32) -> Result<(), ()> {
        crate::securechip::u2f_counter_set(counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hal_model() {
        assert_eq!(
            to_hal_model(crate::securechip::Model::ATECC_ATECC608A),
            Model::Atecc608A,
        );
        assert_eq!(
            to_hal_model(crate::securechip::Model::ATECC_ATECC608B),
            Model::Atecc608B,
        );
        assert_eq!(
            to_hal_model(crate::securechip::Model::OPTIGA_TRUST_M_V3),
            Model::OptigaTrustM3,
        );
    }

    #[test]
    fn test_to_hal_error_securechip() {
        let cases = [
            (
                crate::securechip::SecureChipError::SC_ERR_IFS,
                SecureChipError::Ifs,
            ),
            (
                crate::securechip::SecureChipError::SC_ERR_INVALID_ARGS,
                SecureChipError::InvalidArgs,
            ),
            (
                crate::securechip::SecureChipError::SC_ERR_CONFIG_MISMATCH,
                SecureChipError::ConfigMismatch,
            ),
            (
                crate::securechip::SecureChipError::SC_ERR_SALT,
                SecureChipError::Salt,
            ),
            (
                crate::securechip::SecureChipError::SC_ERR_INCORRECT_PASSWORD,
                SecureChipError::IncorrectPassword,
            ),
            (
                crate::securechip::SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO,
                SecureChipError::InvalidPasswordStretchAlgo,
            ),
            (
                crate::securechip::SecureChipError::SC_ERR_MEMORY,
                SecureChipError::Memory,
            ),
            (
                crate::securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_CONFIG,
                SecureChipError::AteccZoneUnlockedConfig,
            ),
            (
                crate::securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_DATA,
                SecureChipError::AteccZoneUnlockedData,
            ),
            (
                crate::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_IO,
                SecureChipError::AteccSlotUnlockedIo,
            ),
            (
                crate::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_AUTH,
                SecureChipError::AteccSlotUnlockedAuth,
            ),
            (
                crate::securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_ENC,
                SecureChipError::AteccSlotUnlockedEnc,
            ),
            (
                crate::securechip::SecureChipError::SC_ATECC_ERR_RESET_KEYS,
                SecureChipError::AteccResetKeys,
            ),
            (
                crate::securechip::SecureChipError::SC_OPTIGA_ERR_CREATE,
                SecureChipError::OptigaCreate,
            ),
            (
                crate::securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_METADATA,
                SecureChipError::OptigaUnexpectedMetadata,
            ),
            (
                crate::securechip::SecureChipError::SC_OPTIGA_ERR_PAL,
                SecureChipError::OptigaPal,
            ),
            (
                crate::securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN,
                SecureChipError::OptigaUnexpectedLen,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(
                to_hal_error(crate::securechip::Error::SecureChip(input)),
                Error::SecureChip(expected),
            );
        }
    }

    #[test]
    fn test_to_hal_error_status() {
        assert_eq!(
            to_hal_error(crate::securechip::Error::Status(7)),
            Error::Status(7)
        );
    }
}
