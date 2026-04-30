// SPDX-License-Identifier: Apache-2.0

use bitbox_boot_utils::{FLASH_PAGE_SIZE, IMMUTABLE_PAGE_ADDR};
use bitbox03_boot_utils::ImmutablePage;

fn load_immutable_state_for_update() -> Result<ImmutablePage, ()> {
    let mut bytes = [0xff; FLASH_PAGE_SIZE];
    bitbox_platform_stm32u5::flash::read(IMMUTABLE_PAGE_ADDR, &mut bytes);
    ImmutablePage::from_page_bytes(&bytes)
}

fn store_immutable_state(state: ImmutablePage) -> Result<(), ()> {
    bitbox_platform_stm32u5::flash::write_page(IMMUTABLE_PAGE_ADDR, &state.to_page_bytes())
        .map_err(|_| ())
}

pub fn get_attestation_bootloader_hash() -> Result<[u8; 32], ()> {
    Ok(load_immutable_state_for_update()?.attestation_bootloader_hash)
}

pub fn set_attestation_bootloader_hash(hash: &[u8; 32]) -> Result<(), ()> {
    let mut state = load_immutable_state_for_update()?;
    state.attestation_bootloader_hash = *hash;
    store_immutable_state(state)
}

pub fn get_stored_attestation_device_pubkey() -> Result<Option<[u8; 64]>, ()> {
    let pubkey = load_immutable_state_for_update()?.attestation_device_pubkey;
    if pubkey.iter().all(|byte| *byte == 0) {
        Ok(None)
    } else {
        Ok(Some(pubkey))
    }
}

pub fn set_attestation_device_pubkey(pubkey: &[u8; 64]) -> Result<(), ()> {
    let mut state = load_immutable_state_for_update()?;
    state.attestation_present = 0;
    state.attestation_device_pubkey = *pubkey;
    state.attestation_certificate = [0; 64];
    state.attestation_root_pubkey_identifier = [0; 32];
    store_immutable_state(state)
}

pub fn set_attestation_certificate(
    pubkey: &[u8; 64],
    certificate: &[u8; 64],
    root_pubkey_identifier: &[u8; 32],
) -> Result<(), ()> {
    let mut state = load_immutable_state_for_update()?;
    if state.attestation_device_pubkey != *pubkey {
        return Err(());
    }
    state.attestation_present = 1;
    state.attestation_device_pubkey = *pubkey;
    state.attestation_certificate = *certificate;
    state.attestation_root_pubkey_identifier = *root_pubkey_identifier;
    store_immutable_state(state)
}
