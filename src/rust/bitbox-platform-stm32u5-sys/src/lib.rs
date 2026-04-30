// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[unsafe(export_name = "SystemCoreClock")]
static mut SYSTEM_CORE_CLOCK: u32 = 4_000_000;

#[unsafe(export_name = "AHBPrescTable")]
static AHB_PRESC_TABLE: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 7, 8, 9];

#[unsafe(export_name = "APBPrescTable")]
static APB_PRESC_TABLE: [u8; 8] = [0, 0, 0, 0, 1, 2, 3, 4];

#[unsafe(export_name = "MSIRangeTable")]
static MSI_RANGE_TABLE: [u32; 16] = [
    48_000_000, 24_000_000, 16_000_000, 12_000_000, 4_000_000, 2_000_000, 1_330_000, 1_000_000,
    3_072_000, 1_536_000, 1_024_000, 768_000, 400_000, 200_000, 133_000, 100_000,
];
