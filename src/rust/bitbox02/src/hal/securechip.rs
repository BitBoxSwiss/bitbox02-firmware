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

fn to_c_password_stretch_algo(algo: PasswordStretchAlgo) -> crate::securechip::PasswordStretchAlgo {
    match algo {
        PasswordStretchAlgo::V0 => {
            crate::securechip::PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0
        }
        PasswordStretchAlgo::V1 => {
            crate::securechip::PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1
        }
    }
}

impl SecureChip for BitBox02SecureChip {
    fn init_new_password(
        &mut self,
        password: &str,
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
        crate::securechip::imp::init_new_password(
            password,
            to_c_password_stretch_algo(password_stretch_algo),
        )
        .map_err(to_hal_error)
    }

    fn stretch_password(
        &mut self,
        password: &str,
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
        crate::securechip::imp::stretch_password(
            password,
            to_c_password_stretch_algo(password_stretch_algo),
        )
        .map_err(to_hal_error)
    }

    fn kdf(&mut self, msg: &[u8]) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
        crate::securechip::imp::kdf(msg).map_err(to_hal_error)
    }

    fn attestation_sign(
        &mut self,
        challenge: &[u8; 32],
        signature: &mut [u8; 64],
    ) -> Result<(), ()> {
        crate::securechip::imp::attestation_sign(challenge, signature)
    }

    fn monotonic_increments_remaining(&mut self) -> Result<u32, ()> {
        crate::securechip::imp::monotonic_increments_remaining()
    }

    fn model(&mut self) -> Result<Model, ()> {
        crate::securechip::imp::model().map(to_hal_model)
    }

    fn reset_keys(&mut self) -> Result<(), ()> {
        crate::securechip::imp::reset_keys()
    }

    #[cfg(feature = "app-u2f")]
    fn u2f_counter_set(&mut self, counter: u32) -> Result<(), ()> {
        crate::securechip::imp::u2f_counter_set(counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_lit::hex;

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

    #[test]
    fn test_to_c_password_stretch_algo() {
        assert_eq!(
            to_c_password_stretch_algo(PasswordStretchAlgo::V0),
            crate::securechip::PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
        );
        assert_eq!(
            to_c_password_stretch_algo(PasswordStretchAlgo::V1),
            crate::securechip::PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
        );
    }

    #[test]
    fn test_kdf() {
        let mut securechip = BitBox02SecureChip;
        let result = securechip.kdf(b"stub input").unwrap();
        let expected = hex!("3d7caa0407f18f6b15a6202843c883f326d614996df67940af210d91aff5b9c8");
        assert_eq!(result.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_init_new_password_invalid_password_stretch_algo() {
        let mut securechip = BitBox02SecureChip;
        assert_eq!(
            securechip.init_new_password("password", PasswordStretchAlgo::V0),
            Err(Error::SecureChip(
                SecureChipError::InvalidPasswordStretchAlgo,
            )),
        );
    }
}
