// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, vec::Vec};

use bitbox_hal::SecureChip;
use bitbox_hal::memory::PasswordStretchAlgo;
use bitbox_hal::securechip::{Error, Model, SecureChipError};

pub struct BitBox02SecureChip;

fn to_hal_model(model: bitbox_securechip::Model) -> Model {
    match model {
        bitbox_securechip::Model::ATECC_ATECC608A => Model::Atecc608A,
        bitbox_securechip::Model::ATECC_ATECC608B => Model::Atecc608B,
        bitbox_securechip::Model::OPTIGA_TRUST_M_V3 => Model::OptigaTrustM3,
    }
}

fn to_hal_error(error: bitbox_securechip::Error) -> Error {
    match error {
        bitbox_securechip::Error::SecureChip(sc_err) => Error::SecureChip(match sc_err {
            bitbox_securechip::SecureChipError::SC_ERR_IFS => SecureChipError::Ifs,
            bitbox_securechip::SecureChipError::SC_ERR_INVALID_ARGS => SecureChipError::InvalidArgs,
            bitbox_securechip::SecureChipError::SC_ERR_CONFIG_MISMATCH => {
                SecureChipError::ConfigMismatch
            }
            bitbox_securechip::SecureChipError::SC_ERR_SALT => SecureChipError::Salt,
            bitbox_securechip::SecureChipError::SC_ERR_INCORRECT_PASSWORD => {
                SecureChipError::IncorrectPassword
            }
            bitbox_securechip::SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO => {
                SecureChipError::InvalidPasswordStretchAlgo
            }
            bitbox_securechip::SecureChipError::SC_ERR_MEMORY => SecureChipError::Memory,
            bitbox_securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_CONFIG => {
                SecureChipError::AteccZoneUnlockedConfig
            }
            bitbox_securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_DATA => {
                SecureChipError::AteccZoneUnlockedData
            }
            bitbox_securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_IO => {
                SecureChipError::AteccSlotUnlockedIo
            }
            bitbox_securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_AUTH => {
                SecureChipError::AteccSlotUnlockedAuth
            }
            bitbox_securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_ENC => {
                SecureChipError::AteccSlotUnlockedEnc
            }
            bitbox_securechip::SecureChipError::SC_ATECC_ERR_RESET_KEYS => {
                SecureChipError::AteccResetKeys
            }
            bitbox_securechip::SecureChipError::SC_OPTIGA_ERR_CREATE => {
                SecureChipError::OptigaCreate
            }
            bitbox_securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_METADATA => {
                SecureChipError::OptigaUnexpectedMetadata
            }
            bitbox_securechip::SecureChipError::SC_OPTIGA_ERR_PAL => SecureChipError::OptigaPal,
            bitbox_securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN => {
                SecureChipError::OptigaUnexpectedLen
            }
        }),
        bitbox_securechip::Error::Status(status) => Error::Status(status),
    }
}

fn to_c_password_stretch_algo(algo: PasswordStretchAlgo) -> bitbox_securechip::PasswordStretchAlgo {
    match algo {
        PasswordStretchAlgo::V0 => {
            bitbox_securechip::PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0
        }
        PasswordStretchAlgo::V1 => {
            bitbox_securechip::PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1
        }
    }
}

impl SecureChip for BitBox02SecureChip {
    fn random(&mut self) -> Result<Box<zeroize::Zeroizing<[u8; 32]>>, Error> {
        crate::securechip::random().map_err(to_hal_error)
    }

    fn init_new_password(
        &mut self,
        password: &str,
        password_stretch_algo: PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
        crate::securechip::init_new_password(
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
        crate::securechip::stretch_password(
            password,
            to_c_password_stretch_algo(password_stretch_algo),
        )
        .map_err(to_hal_error)
    }

    fn kdf(&mut self, msg: &[u8; 32]) -> Result<zeroize::Zeroizing<Vec<u8>>, Error> {
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
    use hex_lit::hex;

    #[test]
    fn test_to_hal_model() {
        assert_eq!(
            to_hal_model(bitbox_securechip::Model::ATECC_ATECC608A),
            Model::Atecc608A,
        );
        assert_eq!(
            to_hal_model(bitbox_securechip::Model::ATECC_ATECC608B),
            Model::Atecc608B,
        );
        assert_eq!(
            to_hal_model(bitbox_securechip::Model::OPTIGA_TRUST_M_V3),
            Model::OptigaTrustM3,
        );
    }

    #[test]
    fn test_to_hal_error_securechip() {
        let cases = [
            (
                bitbox_securechip::SecureChipError::SC_ERR_IFS,
                SecureChipError::Ifs,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ERR_INVALID_ARGS,
                SecureChipError::InvalidArgs,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ERR_CONFIG_MISMATCH,
                SecureChipError::ConfigMismatch,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ERR_SALT,
                SecureChipError::Salt,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ERR_INCORRECT_PASSWORD,
                SecureChipError::IncorrectPassword,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO,
                SecureChipError::InvalidPasswordStretchAlgo,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ERR_MEMORY,
                SecureChipError::Memory,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_CONFIG,
                SecureChipError::AteccZoneUnlockedConfig,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ATECC_ERR_ZONE_UNLOCKED_DATA,
                SecureChipError::AteccZoneUnlockedData,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_IO,
                SecureChipError::AteccSlotUnlockedIo,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_AUTH,
                SecureChipError::AteccSlotUnlockedAuth,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ATECC_ERR_SLOT_UNLOCKED_ENC,
                SecureChipError::AteccSlotUnlockedEnc,
            ),
            (
                bitbox_securechip::SecureChipError::SC_ATECC_ERR_RESET_KEYS,
                SecureChipError::AteccResetKeys,
            ),
            (
                bitbox_securechip::SecureChipError::SC_OPTIGA_ERR_CREATE,
                SecureChipError::OptigaCreate,
            ),
            (
                bitbox_securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_METADATA,
                SecureChipError::OptigaUnexpectedMetadata,
            ),
            (
                bitbox_securechip::SecureChipError::SC_OPTIGA_ERR_PAL,
                SecureChipError::OptigaPal,
            ),
            (
                bitbox_securechip::SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN,
                SecureChipError::OptigaUnexpectedLen,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(
                to_hal_error(bitbox_securechip::Error::SecureChip(input)),
                Error::SecureChip(expected),
            );
        }
    }

    #[test]
    fn test_to_hal_error_status() {
        assert_eq!(
            to_hal_error(bitbox_securechip::Error::Status(7)),
            Error::Status(7)
        );
    }

    #[test]
    fn test_to_c_password_stretch_algo() {
        assert_eq!(
            to_c_password_stretch_algo(PasswordStretchAlgo::V0),
            bitbox_securechip::PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
        );
        assert_eq!(
            to_c_password_stretch_algo(PasswordStretchAlgo::V1),
            bitbox_securechip::PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V1,
        );
    }

    #[test]
    fn test_kdf() {
        let mut securechip = BitBox02SecureChip;
        let msg = [0u8; 32];
        let result = securechip.kdf(&msg).unwrap();
        let expected = hex!("1c723ccd9597e76deb55f9fd6808014007bcb3d67fc060f1149aefb9be88f423");
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
