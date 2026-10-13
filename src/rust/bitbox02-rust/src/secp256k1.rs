// SPDX-License-Identifier: Apache-2.0

pub use bitbox_secp256k1::*;

pub async fn init(hal: &mut impl crate::hal::Hal) -> Result<(), bitbox_hal::securechip::Error> {
    let seed = bitbox_core_utils::random::random_32_bytes_from_hal(hal).await?;
    SECP256K1.init(&seed);
    Ok(())
}
