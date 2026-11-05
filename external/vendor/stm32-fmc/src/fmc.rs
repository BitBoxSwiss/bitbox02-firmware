//! HAL for Flexible memory controller (FMC)

/// FMC banks
///
/// For example, see RM0433 rev 7 Figure 98.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(unused)]
pub enum FmcBank {
    /// Bank1: NOR/PSRAM/SRAM
    Bank1,
    /// Bank2:
    Bank2,
    /// Bank3: NAND Flash
    Bank3,
    /// Bank4:
    Bank4,
    /// Bank5: SDRAM 1
    Bank5,
    /// Bank6: SDRAM 2
    Bank6,
}
impl FmcBank {
    /// Return a pointer to this FMC bank
    pub fn ptr(self) -> *mut u32 {
        use FmcBank::*;
        (match self {
            Bank1 => 0x6000_0000u32,
            Bank2 => 0x7000_0000u32,
            Bank3 => 0x8000_0000u32,
            Bank4 => 0x9000_0000u32, // Not used
            Bank5 => 0xC000_0000u32,
            Bank6 => 0xD000_0000u32,
        }) as *mut u32
    }
}

/// Set of address pins
pub trait AddressPinSet {
    /// The number of address pins in this set of pins
    const ADDRESS_PINS: u8;
}

macro_rules! address_pin_markers {
    ($($AddressPins:ident, $addr:tt, $doc:expr;)+) => {
        $(
            /// Type to mark that there are
            #[doc=$doc]
            /// address pins
            #[derive(Clone, Copy, Debug)]
            pub struct $AddressPins;
            impl AddressPinSet for $AddressPins {
                const ADDRESS_PINS: u8 = $addr;
            }
        )+
    };
}
address_pin_markers!(
    AddressPins11, 11, "11";
    AddressPins12, 12, "12";
    AddressPins13, 13, "13";
);

// ---- SDRAM ----

#[cfg(feature = "sdram")]
use crate::sdram::{PinsSdram, SdramBank1, SdramBank2};

#[cfg(feature = "sdram")]
macro_rules! impl_16bit_sdram {
    ($($pins:tt: [$ckeN:tt, $neN:tt,
                  $nInternalB:expr
                  $(, $pba1:ident: $ba1:tt)* ; // BA1 pins
                  $addressPins:ident
                  [ $($pa:ident: $a:ident),* ] // Address pins
    ]),+) => {
        $(
            #[rustfmt::skip]
            /// 16-bit SDRAM
            impl<PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, $($pa,)*
            PBA0, $($pba1,)* PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7, PD8,
            PD9, PD10, PD11, PD12, PD13, PD14, PD15, PNBL0, PNBL1, PSDCKEn,
            PSDCLK, PSDNCAS, PSDNEn, PSDNRAS, PSDNWE>
                PinsSdram<$pins, $addressPins>
                for (PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, $($pa,)*
                     PBA0, $($pba1,)* PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7,
                     PD8, PD9, PD10, PD11, PD12, PD13, PD14, PD15, PNBL0, PNBL1,
                     PSDCKEn, PSDCLK, PSDNCAS, PSDNEn, PSDNRAS, PSDNWE)
            where PA0: A0, PA1: A1, PA2: A2, PA3: A3, PA4: A4, PA5: A5, PA6: A6,
                  PA7: A7, PA8: A8, PA9: A9, PA10: A10, $($pa:$a,)*
                  PBA0: BA0, $($pba1:$ba1,)*
                  PD0: D0, PD1: D1, PD2: D2, PD3: D3, PD4: D4, PD5: D5, PD6: D6,
                  PD7: D7, PD8: D8, PD9: D9, PD10: D10, PD11: D11, PD12: D12,
                  PD13: D13, PD14: D14, PD15: D15,
                  PNBL0: NBL0, PNBL1: NBL1, PSDCKEn: $ckeN, PSDCLK: SDCLK,
                  PSDNCAS: SDNCAS, PSDNEn: $neN, PSDNRAS: SDNRAS, PSDNWE: SDNWE {

                const NUMBER_INTERNAL_BANKS: u8 = $nInternalB;
            }
        )+
    }
}

#[cfg(feature = "sdram")]
macro_rules! impl_32bit_sdram {
    ($($pins:tt: [$ckeN:tt, $neN:tt,
                  $nInternalB:expr
                  $(, $pba1:ident: $ba1:tt)* ; // BA1 pins
                  $addressPins:ident
                  [ $($pa:ident: $a:ident),* ] // Address pins
            ]),+) => {
        $(
            #[rustfmt::skip]
            /// 32-bit SDRAM
            impl<PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, $($pa,)*
            PBA0, $($pba1,)* PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7, PD8,
            PD9, PD10, PD11, PD12, PD13, PD14, PD15, PD16, PD17, PD18, PD19,
            PD20, PD21, PD22, PD23, PD24, PD25, PD26, PD27, PD28, PD29, PD30,
            PD31, PNBL0, PNBL1, PNBL2, PNBL3, PSDCKEn, PSDCLK, PSDNCAS,
            PSDNEn, PSDNRAS, PSDNWE>
                PinsSdram<$pins, $addressPins>
                for (PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, $($pa,)*
                     PBA0, $($pba1,)* PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7,
                     PD8, PD9, PD10, PD11, PD12, PD13, PD14, PD15, PD16, PD17,
                     PD18, PD19, PD20, PD21, PD22, PD23, PD24, PD25, PD26, PD27,
                     PD28, PD29, PD30, PD31, PNBL0, PNBL1, PNBL2, PNBL3, PSDCKEn,
                     PSDCLK, PSDNCAS, PSDNEn, PSDNRAS, PSDNWE)
            where PA0: A0, PA1: A1, PA2: A2, PA3: A3, PA4: A4, PA5: A5, PA6: A6,
                  PA7: A7, PA8: A8, PA9: A9, PA10: A10, $($pa:$a,)*
                  PBA0: BA0, $($pba1:$ba1,)*
                  PD0: D0, PD1: D1, PD2: D2, PD3: D3, PD4: D4, PD5: D5, PD6: D6,
                  PD7: D7, PD8: D8, PD9: D9, PD10: D10, PD11: D11, PD12: D12,
                  PD13: D13, PD14: D14, PD15: D15, PD16: D16, PD17: D17,
                  PD18: D18, PD19: D19, PD20: D20, PD21: D21, PD22: D22,
                  PD23: D23, PD24: D24, PD25: D25, PD26: D26, PD27: D27,
                  PD28: D28, PD29: D29, PD30: D30, PD31: D31,
                  PNBL0: NBL0, PNBL1: NBL1, PNBL2: NBL2, PNBL3: NBL3,
                  PSDCKEn: $ckeN, PSDCLK: SDCLK,
                  PSDNCAS: SDNCAS, PSDNEn: $neN, PSDNRAS: SDNRAS, PSDNWE: SDNWE {

                const NUMBER_INTERNAL_BANKS: u8 = $nInternalB;
            }
        )+
    }
}

#[cfg(feature = "sdram")]
impl_16bit_sdram! {
    // 16-bit SDRAM with 11 address lines, BA0 only
    SdramBank1: [SDCKE0, SDNE0, 2; AddressPins11 []],
    SdramBank2: [SDCKE1, SDNE1, 2; AddressPins11 []],
    // 16-bit SDRAM with 11 address lines, BA0 and BA1
    SdramBank1: [SDCKE0, SDNE0, 4, PBA1: BA1; AddressPins11 []],
    SdramBank2: [SDCKE1, SDNE1, 4, PBA1: BA1; AddressPins11 []],
    // 16-bit SDRAM with 12 address lines, BA0 only
    SdramBank1: [SDCKE0, SDNE0, 2; AddressPins12 [PA11: A11]],
    SdramBank2: [SDCKE1, SDNE1, 2; AddressPins12 [PA11: A11]],
    // 16-bit SDRAM with 12 address lines, BA0 and BA1
    SdramBank1: [SDCKE0, SDNE0, 4, PBA1: BA1; AddressPins12 [PA11: A11]],
    SdramBank2: [SDCKE1, SDNE1, 4, PBA1: BA1; AddressPins12 [PA11: A11]],
    // 16-bit SDRAM with 13 address lines, BA0 only
    SdramBank1: [SDCKE0, SDNE0, 2; AddressPins13 [PA11: A11, PA12: A12]],
    SdramBank2: [SDCKE1, SDNE1, 2; AddressPins13 [PA11: A11, PA12: A12]],
    // 16-bit SDRAM with 13 address lines, BA0 and BA1
    SdramBank1: [SDCKE0, SDNE0, 4, PBA1: BA1; AddressPins13 [PA11: A11, PA12: A12]],
    SdramBank2: [SDCKE1, SDNE1, 4, PBA1: BA1; AddressPins13 [PA11: A11, PA12: A12]]
}

#[cfg(feature = "sdram")]
impl_32bit_sdram! {
    // 32-bit SDRAM with 11 address lines, BA0 only
    SdramBank1: [SDCKE0, SDNE0, 2; AddressPins11 []],
    SdramBank2: [SDCKE1, SDNE1, 2; AddressPins11 []],
    // 32-bit SDRAM with 11 address lines, BA0 and BA1
    SdramBank1: [SDCKE0, SDNE0, 4, PBA1: BA1; AddressPins11 []],
    SdramBank2: [SDCKE1, SDNE1, 4, PBA1: BA1; AddressPins11 []],
    // 32-bit SDRAM with 12 address lines, BA0 only
    SdramBank1: [SDCKE0, SDNE0, 2; AddressPins12 [PA11: A11]],
    SdramBank2: [SDCKE1, SDNE1, 2; AddressPins12 [PA11: A11]],
    // 32-bit SDRAM with 12 address lines, BA0 and BA1
    SdramBank1: [SDCKE0, SDNE0, 4, PBA1: BA1; AddressPins12 [PA11: A11]],
    SdramBank2: [SDCKE1, SDNE1, 4, PBA1: BA1; AddressPins12 [PA11: A11]],
    // 32-bit SDRAM with 13 address lines, BA0 only
    SdramBank1: [SDCKE0, SDNE0, 2; AddressPins13 [PA11: A11, PA12: A12]],
    SdramBank2: [SDCKE1, SDNE1, 2; AddressPins13 [PA11: A11, PA12: A12]],
    // 32-bit SDRAM with 13 address lines, BA0 and BA1
    SdramBank1: [SDCKE0, SDNE0, 4, PBA1: BA1; AddressPins13 [PA11: A11, PA12: A12]],
    SdramBank2: [SDCKE1, SDNE1, 4, PBA1: BA1; AddressPins13 [PA11: A11, PA12: A12]]
}

// ---- NAND ----

#[cfg(feature = "nand")]
use crate::nand::PinsNand;

#[cfg(feature = "nand")]
#[rustfmt::skip]
/// 8-bit NAND
impl<ALE, CLE, PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7, PNCE, PNOE, PNWE, PNWAIT>
    PinsNand
    for (ALE, CLE, PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7, PNCE, PNOE, PNWE, PNWAIT)
where ALE: A17, CLE: A16,
      PD0: D0, PD1: D1, PD2: D2, PD3: D3, PD4: D4, PD5: D5, PD6: D6, PD7: D7,
      PNCE: NCE, PNOE: NOE, PNWE: NWE, PNWAIT: NWAIT {
    const N_DATA: usize = 8;
}

/// Marks a type as an A0 pin
pub trait A0 {}
/// Marks a type as an A1 pin
pub trait A1 {}
/// Marks a type as an A10 pin
pub trait A10 {}
/// Marks a type as an A11 pin
pub trait A11 {}
/// Marks a type as an A12 pin
pub trait A12 {}
/// Marks a type as an A13 pin
pub trait A13 {}
/// Marks a type as an A14 pin
pub trait A14 {}
/// Marks a type as an A15 pin
pub trait A15 {}
/// Marks a type as an A16 pin
pub trait A16 {}
/// Marks a type as an A17 pin
pub trait A17 {}
/// Marks a type as an A18 pin
pub trait A18 {}
/// Marks a type as an A19 pin
pub trait A19 {}
/// Marks a type as an A2 pin
pub trait A2 {}
/// Marks a type as an A20 pin
pub trait A20 {}
/// Marks a type as an A21 pin
pub trait A21 {}
/// Marks a type as an A22 pin
pub trait A22 {}
/// Marks a type as an A23 pin
pub trait A23 {}
/// Marks a type as an A24 pin
pub trait A24 {}
/// Marks a type as an A25 pin
pub trait A25 {}
/// Marks a type as an A3 pin
pub trait A3 {}
/// Marks a type as an A4 pin
pub trait A4 {}
/// Marks a type as an A5 pin
pub trait A5 {}
/// Marks a type as an A6 pin
pub trait A6 {}
/// Marks a type as an A7 pin
pub trait A7 {}
/// Marks a type as an A8 pin
pub trait A8 {}
/// Marks a type as an A9 pin
pub trait A9 {}
/// Marks a type as a BA0 pin
pub trait BA0 {}
/// Marks a type as a BA1 pin
pub trait BA1 {}
/// Marks a type as a CLK pin
pub trait CLK {}
/// Marks a type as a D0 pin
pub trait D0 {}
/// Marks a type as a D1 pin
pub trait D1 {}
/// Marks a type as a D10 pin
pub trait D10 {}
/// Marks a type as a D11 pin
pub trait D11 {}
/// Marks a type as a D12 pin
pub trait D12 {}
/// Marks a type as a D13 pin
pub trait D13 {}
/// Marks a type as a D14 pin
pub trait D14 {}
/// Marks a type as a D15 pin
pub trait D15 {}
/// Marks a type as a D16 pin
pub trait D16 {}
/// Marks a type as a D17 pin
pub trait D17 {}
/// Marks a type as a D18 pin
pub trait D18 {}
/// Marks a type as a D19 pin
pub trait D19 {}
/// Marks a type as a D2 pin
pub trait D2 {}
/// Marks a type as a D20 pin
pub trait D20 {}
/// Marks a type as a D21 pin
pub trait D21 {}
/// Marks a type as a D22 pin
pub trait D22 {}
/// Marks a type as a D23 pin
pub trait D23 {}
/// Marks a type as a D24 pin
pub trait D24 {}
/// Marks a type as a D25 pin
pub trait D25 {}
/// Marks a type as a D26 pin
pub trait D26 {}
/// Marks a type as a D27 pin
pub trait D27 {}
/// Marks a type as a D28 pin
pub trait D28 {}
/// Marks a type as a D29 pin
pub trait D29 {}
/// Marks a type as a D3 pin
pub trait D3 {}
/// Marks a type as a D30 pin
pub trait D30 {}
/// Marks a type as a D31 pin
pub trait D31 {}
/// Marks a type as a D4 pin
pub trait D4 {}
/// Marks a type as a D5 pin
pub trait D5 {}
/// Marks a type as a D6 pin
pub trait D6 {}
/// Marks a type as a D7 pin
pub trait D7 {}
/// Marks a type as a D8 pin
pub trait D8 {}
/// Marks a type as a D9 pin
pub trait D9 {}
/// Marks a type as a DA0 pin
pub trait DA0 {}
/// Marks a type as a DA1 pin
pub trait DA1 {}
/// Marks a type as a DA10 pin
pub trait DA10 {}
/// Marks a type as a DA11 pin
pub trait DA11 {}
/// Marks a type as a DA12 pin
pub trait DA12 {}
/// Marks a type as a DA13 pin
pub trait DA13 {}
/// Marks a type as a DA14 pin
pub trait DA14 {}
/// Marks a type as a DA15 pin
pub trait DA15 {}
/// Marks a type as a DA2 pin
pub trait DA2 {}
/// Marks a type as a DA3 pin
pub trait DA3 {}
/// Marks a type as a DA4 pin
pub trait DA4 {}
/// Marks a type as a DA5 pin
pub trait DA5 {}
/// Marks a type as a DA6 pin
pub trait DA6 {}
/// Marks a type as a DA7 pin
pub trait DA7 {}
/// Marks a type as a DA8 pin
pub trait DA8 {}
/// Marks a type as a DA9 pin
pub trait DA9 {}
/// Marks a type as an INT pin
pub trait INT {}
/// Marks a type as a NBL0 pin
pub trait NBL0 {}
/// Marks a type as a NBL1 pin
pub trait NBL1 {}
/// Marks a type as a NBL2 pin
pub trait NBL2 {}
/// Marks a type as a NBL3 pin
pub trait NBL3 {}
/// Marks a type as a NE1 pin
pub trait NE1 {}
/// Marks a type as a NE2 pin
pub trait NE2 {}
/// Marks a type as a NE3 pin
pub trait NE3 {}
/// Marks a type as a NE4 pin
pub trait NE4 {}
/// Marks a type as a NL pin
pub trait NL {}
/// Marks a type as a NCE pin
pub trait NCE {}
/// Marks a type as a NOE pin
pub trait NOE {}
/// Marks a type as a NWAIT pin
pub trait NWAIT {}
/// Marks a type as a NWE pin
pub trait NWE {}
/// Marks a type as a SDCKE0 pin
pub trait SDCKE0 {}
/// Marks a type as a SDCKE1 pin
pub trait SDCKE1 {}
/// Marks a type as a SDCLK pin
pub trait SDCLK {}
/// Marks a type as a SDNCAS pin
pub trait SDNCAS {}
/// Marks a type as a SDNE0 pin
pub trait SDNE0 {}
/// Marks a type as a SDNE1 pin
pub trait SDNE1 {}
/// Marks a type as a SDNRAS pin
pub trait SDNRAS {}
/// Marks a type as a SDNWE pin
pub trait SDNWE {}

use crate::ral::fmc;
use crate::FmcPeripheral;

#[derive(Copy, Clone)]
pub(crate) struct FmcRegisters(usize);

impl FmcRegisters {
    #[inline(always)]
    pub fn new<FMC: FmcPeripheral>() -> Self {
        Self(FMC::REGISTERS as usize)
    }

    #[inline(always)]
    pub fn global(&self) -> &'static fmc::RegisterBlock {
        unsafe { &*(self.0 as *const _) }
    }
}
