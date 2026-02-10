// SPDX-License-Identifier: Apache-2.0

pub mod memory;
pub mod random;
pub mod sd;
pub mod securechip;
pub mod system;
pub mod ui;

#[cfg(feature = "testing")]
pub mod testing;

pub use memory::Memory;
pub use random::Random;
pub use sd::Sd;
pub use securechip::SecureChip;
pub use system::System;
pub use ui::Ui;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use futures_lite::future::yield_now;

use crate::workflow::{
    cancel, confirm, menu, mnemonic, sdcard, status, transaction, trinary_choice,
    trinary_input_string,
};

/// Hardware abstraction layer for BitBox devices.
pub trait Hal {
    fn ui(&mut self) -> &mut impl Ui;
    fn sd(&mut self) -> &mut impl Sd;
    fn random(&mut self) -> &mut impl Random;
    fn securechip(&mut self) -> &mut impl SecureChip;
    fn memory(&mut self) -> &mut impl Memory;
    fn system(&mut self) -> &mut impl System;
}

pub struct BitBox02Ui;

impl Ui for BitBox02Ui {
    #[inline(always)]
    async fn confirm(&mut self, params: &confirm::Params<'_>) -> Result<(), confirm::UserAbort> {
        confirm::confirm(params).await
    }

    #[inline(always)]
    async fn verify_recipient(
        &mut self,
        recipient: &str,
        amount: &str,
    ) -> Result<(), transaction::UserAbort> {
        transaction::verify_recipient(recipient, amount).await
    }

    #[inline(always)]
    async fn verify_total_fee(
        &mut self,
        total: &str,
        fee: &str,
        longtouch: bool,
    ) -> Result<(), transaction::UserAbort> {
        transaction::verify_total_fee(total, fee, longtouch).await
    }

    #[inline(always)]
    async fn status(&mut self, title: &str, status_success: bool) {
        status::status(title, status_success).await
    }

    #[inline(always)]
    async fn enter_string(
        &mut self,
        params: &trinary_input_string::Params<'_>,
        can_cancel: trinary_input_string::CanCancel,
        preset: &str,
    ) -> Result<zeroize::Zeroizing<String>, trinary_input_string::Error> {
        trinary_input_string::enter(params, can_cancel, preset).await
    }

    #[inline(always)]
    async fn insert_sdcard(&mut self) -> Result<(), sdcard::UserAbort> {
        sdcard::sdcard().await
    }

    #[inline(always)]
    async fn menu(&mut self, words: &[&str], title: Option<&str>) -> Result<u8, menu::CancelError> {
        menu::pick(words, title).await
    }

    #[inline(always)]
    async fn trinary_choice(
        &mut self,
        message: &str,
        label_left: Option<&str>,
        label_middle: Option<&str>,
        label_right: Option<&str>,
    ) -> trinary_choice::TrinaryChoice {
        trinary_choice::choose(message, label_left, label_middle, label_right).await
    }

    async fn show_mnemonic(&mut self, words: &[&str]) -> Result<(), cancel::Error> {
        mnemonic::show_mnemonic(words).await
    }

    async fn quiz_mnemonic_word(
        &mut self,
        choices: &[&str],
        title: &str,
    ) -> Result<u8, cancel::Error> {
        mnemonic::confirm_word(choices, title).await
    }
}

pub struct BitBox02Sd;

impl Sd for BitBox02Sd {
    #[inline(always)]
    async fn sdcard_inserted(&mut self) -> bool {
        let result = bitbox02::sd::sdcard_inserted();
        yield_now().await;
        result
    }

    #[inline(always)]
    async fn list_subdir(&mut self, subdir: Option<&str>) -> Result<Vec<String>, ()> {
        let result = bitbox02::sd::list_subdir(subdir);
        yield_now().await;
        result
    }

    #[inline(always)]
    async fn erase_file_in_subdir(&mut self, filename: &str, dir: &str) -> Result<(), ()> {
        let result = bitbox02::sd::erase_file_in_subdir(filename, dir);
        yield_now().await;
        result
    }

    #[inline(always)]
    async fn load_bin(
        &mut self,
        filename: &str,
        dir: &str,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
        let result = bitbox02::sd::load_bin(filename, dir);
        yield_now().await;
        result
    }

    #[inline(always)]
    async fn write_bin(&mut self, filename: &str, dir: &str, data: &[u8]) -> Result<(), ()> {
        let result = bitbox02::sd::write_bin(filename, dir, data);
        yield_now().await;
        result
    }
}

pub struct BitBox02Random;

impl Random for BitBox02Random {
    #[inline(always)]
    fn random_32_bytes(&mut self) -> Box<zeroize::Zeroizing<[u8; 32]>> {
        bitbox02::random::random_32_bytes()
    }
}

pub struct BitBox02SecureChip;

impl SecureChip for BitBox02SecureChip {
    fn init_new_password(
        &mut self,
        password: &str,
        password_stretch_algo: bitbox02::memory::PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error> {
        bitbox02::securechip::init_new_password(password, password_stretch_algo)
    }

    fn stretch_password(
        &mut self,
        password: &str,
        password_stretch_algo: bitbox02::memory::PasswordStretchAlgo,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error> {
        bitbox02::securechip::stretch_password(password, password_stretch_algo)
    }

    fn kdf(
        &mut self,
        msg: &[u8],
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, bitbox02::securechip::Error> {
        bitbox02::securechip::kdf(msg)
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

    fn model(&mut self) -> Result<bitbox02::securechip::Model, ()> {
        bitbox02::securechip::model()
    }

    fn reset_keys(&mut self) -> Result<(), ()> {
        bitbox02::securechip::reset_keys()
    }

    #[cfg(feature = "app-u2f")]
    fn u2f_counter_set(&mut self, counter: u32) -> Result<(), ()> {
        bitbox02::securechip::u2f_counter_set(counter)
    }
}

pub struct BitBox02Memory;

impl Memory for BitBox02Memory {
    fn get_securechip_type(&mut self) -> Result<bitbox02::memory::SecurechipType, ()> {
        bitbox02::memory::get_securechip_type()
    }

    fn get_platform(&mut self) -> Result<bitbox02::memory::Platform, ()> {
        bitbox02::memory::get_platform()
    }

    fn get_device_name(&mut self) -> String {
        bitbox02::memory::get_device_name()
    }

    fn set_device_name(&mut self, name: &str) -> Result<(), bitbox02::memory::Error> {
        bitbox02::memory::set_device_name(name)
    }

    fn is_mnemonic_passphrase_enabled(&mut self) -> bool {
        bitbox02::memory::is_mnemonic_passphrase_enabled()
    }

    fn set_mnemonic_passphrase_enabled(&mut self, enabled: bool) -> Result<(), ()> {
        bitbox02::memory::set_mnemonic_passphrase_enabled(enabled)
    }

    fn set_seed_birthdate(&mut self, timestamp: u32) -> Result<(), ()> {
        bitbox02::memory::set_seed_birthdate(timestamp)
    }

    fn get_seed_birthdate(&mut self) -> u32 {
        bitbox02::memory::get_seed_birthdate()
    }

    fn is_seeded(&mut self) -> bool {
        bitbox02::memory::is_seeded()
    }

    fn is_initialized(&mut self) -> bool {
        bitbox02::memory::is_initialized()
    }

    fn set_initialized(&mut self) -> Result<(), ()> {
        bitbox02::memory::set_initialized()
    }

    fn get_encrypted_seed_and_hmac(
        &mut self,
    ) -> Result<(alloc::vec::Vec<u8>, bitbox02::memory::PasswordStretchAlgo), ()> {
        bitbox02::memory::get_encrypted_seed_and_hmac()
    }

    fn set_encrypted_seed_and_hmac(
        &mut self,
        data: &[u8],
        password_stretch_algo: bitbox02::memory::PasswordStretchAlgo,
    ) -> Result<(), ()> {
        bitbox02::memory::set_encrypted_seed_and_hmac(data, password_stretch_algo)
    }

    fn reset_hww(&mut self) -> Result<(), ()> {
        bitbox02::memory::reset_hww()
    }

    fn get_unlock_attempts(&mut self) -> u8 {
        bitbox02::memory::smarteeprom_get_unlock_attempts()
    }
    fn increment_unlock_attempts(&mut self) {
        bitbox02::memory::smarteeprom_increment_unlock_attempts()
    }
    fn reset_unlock_attempts(&mut self) {
        bitbox02::memory::smarteeprom_reset_unlock_attempts()
    }

    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
        bitbox02::memory::get_salt_root()
    }

    fn get_attestation_pubkey_and_certificate(
        &mut self,
        pubkey_out: &mut [u8; 64],
        certificate_out: &mut [u8; 64],
        root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()> {
        bitbox02::memory::get_attestation_pubkey_and_certificate(
            pubkey_out,
            certificate_out,
            root_pubkey_identifier_out,
        )
    }

    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32] {
        bitbox02::memory::get_attestation_bootloader_hash()
    }

    fn multisig_set_by_hash(
        &mut self,
        hash: &[u8; 32],
        name: &str,
    ) -> Result<(), bitbox02::memory::MemoryError> {
        bitbox02::memory::multisig_set_by_hash(hash, name)
    }

    fn multisig_get_by_hash(&self, hash: &[u8; 32]) -> Option<String> {
        bitbox02::memory::multisig_get_by_hash(hash)
    }
}

pub struct BitBox02System;

impl System for BitBox02System {
    fn reboot_to_bootloader(&mut self) -> ! {
        bitbox02::reboot_to_bootloader()
    }
}

pub struct BitBox02Hal {
    ui: BitBox02Ui,
    sd: BitBox02Sd,
    random: BitBox02Random,
    securechip: BitBox02SecureChip,
    memory: BitBox02Memory,
    system: BitBox02System,
}

impl BitBox02Hal {
    pub const fn new() -> Self {
        Self {
            ui: BitBox02Ui,
            sd: BitBox02Sd,
            random: BitBox02Random,
            securechip: BitBox02SecureChip,
            memory: BitBox02Memory,
            system: BitBox02System,
        }
    }
}

impl Hal for BitBox02Hal {
    fn ui(&mut self) -> &mut impl Ui {
        &mut self.ui
    }
    fn sd(&mut self) -> &mut impl Sd {
        &mut self.sd
    }
    fn random(&mut self) -> &mut impl Random {
        &mut self.random
    }
    fn securechip(&mut self) -> &mut impl SecureChip {
        &mut self.securechip
    }
    fn memory(&mut self) -> &mut impl Memory {
        &mut self.memory
    }
    fn system(&mut self) -> &mut impl System {
        &mut self.system
    }
}
