use bitbox_hal as hal;

pub struct BitBox03Memory;

impl hal::memory::Memory for BitBox03Memory {
    const BLE_FW_FLASH_CHUNK_SIZE: u32 = 0;

    fn ble_enabled(&mut self) -> bool {
        todo!()
    }

    fn ble_enable(&mut self, _enable: bool) -> Result<(), ()> {
        todo!()
    }

    fn get_active_ble_firmware_version(
        &mut self,
    ) -> Result<alloc::string::String, bitbox_hal::memory::Error> {
        todo!()
    }

    fn ble_firmware_flash_chunk(
        &mut self,
        _slot: bitbox_hal::memory::BleFirmwareSlot,
        _chunk_index: u32,
        _chunk: &[u8],
    ) -> Result<(), bitbox_hal::memory::Error> {
        todo!()
    }

    fn ble_get_metadata(&mut self) -> bitbox_hal::memory::BleMetadata {
        todo!()
    }

    fn set_ble_metadata(
        &mut self,
        _metadata: &bitbox_hal::memory::BleMetadata,
    ) -> Result<(), bitbox_hal::memory::Error> {
        todo!()
    }

    fn get_securechip_type(&mut self) -> Result<bitbox_hal::memory::SecurechipType, ()> {
        todo!()
    }

    fn get_platform(&mut self) -> Result<bitbox_hal::memory::Platform, ()> {
        Ok(bitbox_hal::memory::Platform::BitBox03)
    }

    fn get_device_name(&mut self) -> alloc::string::String {
        todo!()
    }

    fn set_device_name(&mut self, _name: &str) -> Result<(), bitbox_hal::memory::Error> {
        todo!()
    }

    fn is_mnemonic_passphrase_enabled(&mut self) -> bool {
        todo!()
    }

    fn set_mnemonic_passphrase_enabled(&mut self, _enabled: bool) -> Result<(), ()> {
        todo!()
    }

    fn set_seed_birthdate(&mut self, _timestamp: u32) -> Result<(), ()> {
        todo!()
    }

    fn get_seed_birthdate(&mut self) -> u32 {
        todo!()
    }

    fn is_seeded(&mut self) -> bool {
        todo!()
    }

    fn is_initialized(&mut self) -> bool {
        todo!()
    }

    fn set_initialized(&mut self) -> Result<(), ()> {
        todo!()
    }

    fn get_encrypted_seed_and_hmac(
        &mut self,
    ) -> Result<(alloc::vec::Vec<u8>, bitbox_hal::memory::PasswordStretchAlgo), ()> {
        todo!()
    }

    fn set_encrypted_seed_and_hmac(
        &mut self,
        _data: &[u8],
        _password_stretch_algo: bitbox_hal::memory::PasswordStretchAlgo,
    ) -> Result<(), ()> {
        todo!()
    }

    fn reset_hww(&mut self) -> Result<(), ()> {
        todo!()
    }
    fn get_salt_root(&mut self) -> Result<zeroize::Zeroizing<alloc::vec::Vec<u8>>, ()> {
        todo!()
    }

    fn get_attestation_pubkey_and_certificate(
        &mut self,
        _pubkey_out: &mut [u8; 64],
        _certificate_out: &mut [u8; 64],
        _root_pubkey_identifier_out: &mut [u8; 32],
    ) -> Result<(), ()> {
        todo!()
    }

    fn get_attestation_bootloader_hash(&mut self) -> [u8; 32] {
        todo!()
    }

    fn multisig_set_by_hash(
        &mut self,
        _hash: &[u8; 32],
        _name: &str,
    ) -> Result<(), bitbox_hal::memory::Error> {
        todo!()
    }

    fn multisig_get_by_hash(&self, _hash: &[u8; 32]) -> Option<alloc::string::String> {
        todo!()
    }

    fn get_optiga_config_version(&mut self) -> Result<bitbox_hal::memory::OptigaConfigVersion, ()> {
        todo!()
    }

    fn set_optiga_config_version(
        &mut self,
        _version: bitbox_hal::memory::OptigaConfigVersion,
    ) -> Result<(), ()> {
        todo!()
    }

    fn get_io_protection_key(&mut self, _out: &mut [u8; 32]) {
        todo!()
    }
}
