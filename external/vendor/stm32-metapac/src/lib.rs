#![no_std]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![doc(html_no_source)]
#![cfg_attr(
    docsrs,
    doc = "<div style='padding:30px;background:#810;color:#fff;text-align:center;'><p>You might want to <a href='https://docs.embassy.dev/stm32-metapac'>browse the `stm32-metapac` documentation on the Embassy website</a> instead.</p><p>The documentation here on `docs.rs` is built for a single chip only (stm32h755zi-cm7 in particular), while on the Embassy website you can pick your exact chip from the top menu. Available peripherals and their APIs change depending on the chip.</p></div>\n\n"
)]
#![doc = include_str!("../README.md")]

pub mod common;

#[cfg(feature = "pac")]
include!(env!("STM32_METAPAC_PAC_PATH"));

#[cfg(feature = "metadata")]
pub mod metadata {
    include!("metadata.rs");
    include!(env!("STM32_METAPAC_METADATA_PATH"));
    include!("all_chips.rs");
    include!("all_peripheral_versions.rs");
}
