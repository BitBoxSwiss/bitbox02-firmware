#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x04],
    dcr1: DCR1,
    dcr2: DCR2,
    dcr3: DCR3,
    dcr4: DCR4,
    _reserved5: [u8; 0x08],
    sr: SR,
    fcr: FCR,
    _reserved7: [u8; 0x18],
    dlr: DLR,
    _reserved8: [u8; 0x04],
    ar: AR,
    _reserved9: [u8; 0x04],
    dr: DR,
    _reserved10: [u8; 0x2c],
    psmkr: PSMKR,
    _reserved11: [u8; 0x04],
    psmar: PSMAR,
    _reserved12: [u8; 0x04],
    pir: PIR,
    _reserved13: [u8; 0x6c],
    ccr: CCR,
    _reserved14: [u8; 0x04],
    tcr: TCR,
    _reserved15: [u8; 0x04],
    ir: IR,
    _reserved16: [u8; 0x0c],
    abr: ABR,
    _reserved17: [u8; 0x0c],
    lptr: LPTR,
    _reserved18: [u8; 0x0c],
    wpccr: WPCCR,
    _reserved19: [u8; 0x04],
    wptcr: WPTCR,
    _reserved20: [u8; 0x04],
    wpir: WPIR,
    _reserved21: [u8; 0x0c],
    wpabr: WPABR,
    _reserved22: [u8; 0x1c],
    wccr: WCCR,
    _reserved23: [u8; 0x04],
    wtcr: WTCR,
    _reserved24: [u8; 0x04],
    wir: WIR,
    _reserved25: [u8; 0x0c],
    wabr: WABR,
    _reserved26: [u8; 0x5c],
    hlcr: HLCR,
    _reserved27: [u8; 0x0c],
    calfcr: CALFCR,
    _reserved28: [u8; 0x04],
    calmr: CALMR,
    _reserved29: [u8; 0x04],
    calsor: CALSOR,
    _reserved30: [u8; 0x04],
    calsir: CALSIR,
}
impl RegisterBlock {
    ///0x00 - HSPI control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x08 - HSPI device configuration register 1
    #[inline(always)]
    pub const fn dcr1(&self) -> &DCR1 {
        &self.dcr1
    }
    ///0x0c - HSPI device configuration register 2
    #[inline(always)]
    pub const fn dcr2(&self) -> &DCR2 {
        &self.dcr2
    }
    ///0x10 - HSPI device configuration register 3
    #[inline(always)]
    pub const fn dcr3(&self) -> &DCR3 {
        &self.dcr3
    }
    ///0x14 - HSPI device configuration register 4
    #[inline(always)]
    pub const fn dcr4(&self) -> &DCR4 {
        &self.dcr4
    }
    ///0x20 -
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x24 -
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x40 - HSPI data length register
    #[inline(always)]
    pub const fn dlr(&self) -> &DLR {
        &self.dlr
    }
    ///0x48 -
    #[inline(always)]
    pub const fn ar(&self) -> &AR {
        &self.ar
    }
    ///0x50 -
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x80 - HSPI polling status mask register
    #[inline(always)]
    pub const fn psmkr(&self) -> &PSMKR {
        &self.psmkr
    }
    ///0x88 - HSPI polling status match register
    #[inline(always)]
    pub const fn psmar(&self) -> &PSMAR {
        &self.psmar
    }
    ///0x90 - HSPI polling interval register
    #[inline(always)]
    pub const fn pir(&self) -> &PIR {
        &self.pir
    }
    ///0x100 - HSPI communication configuration register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x108 - HSPI timing configuration register
    #[inline(always)]
    pub const fn tcr(&self) -> &TCR {
        &self.tcr
    }
    ///0x110 - HSPI instruction register
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    ///0x120 - HSPI alternate bytes register
    #[inline(always)]
    pub const fn abr(&self) -> &ABR {
        &self.abr
    }
    ///0x130 - HSPI low-power timeout register
    #[inline(always)]
    pub const fn lptr(&self) -> &LPTR {
        &self.lptr
    }
    ///0x140 - HSPI wrap communication configuration register
    #[inline(always)]
    pub const fn wpccr(&self) -> &WPCCR {
        &self.wpccr
    }
    ///0x148 - HSPI wrap timing configuration register
    #[inline(always)]
    pub const fn wptcr(&self) -> &WPTCR {
        &self.wptcr
    }
    ///0x150 - HSPI wrap instruction register
    #[inline(always)]
    pub const fn wpir(&self) -> &WPIR {
        &self.wpir
    }
    ///0x160 - HSPI wrap alternate bytes register
    #[inline(always)]
    pub const fn wpabr(&self) -> &WPABR {
        &self.wpabr
    }
    ///0x180 - HSPI write communication configuration register
    #[inline(always)]
    pub const fn wccr(&self) -> &WCCR {
        &self.wccr
    }
    ///0x188 - HSPI write timing configuration register
    #[inline(always)]
    pub const fn wtcr(&self) -> &WTCR {
        &self.wtcr
    }
    ///0x190 - HSPI write instruction register
    #[inline(always)]
    pub const fn wir(&self) -> &WIR {
        &self.wir
    }
    ///0x1a0 - HSPI write alternate bytes register
    #[inline(always)]
    pub const fn wabr(&self) -> &WABR {
        &self.wabr
    }
    ///0x200 - HSPI HyperBus latency configuration register
    #[inline(always)]
    pub const fn hlcr(&self) -> &HLCR {
        &self.hlcr
    }
    ///0x210 - HSPI full-cycle calibration configuration
    #[inline(always)]
    pub const fn calfcr(&self) -> &CALFCR {
        &self.calfcr
    }
    ///0x218 - HSPI DLL master calibration configuration
    #[inline(always)]
    pub const fn calmr(&self) -> &CALMR {
        &self.calmr
    }
    ///0x220 - HSPI DLL slave output calibration configuration
    #[inline(always)]
    pub const fn calsor(&self) -> &CALSOR {
        &self.calsor
    }
    ///0x228 - HSPI DLL slave input calibration configuration
    #[inline(always)]
    pub const fn calsir(&self) -> &CALSIR {
        &self.calsir
    }
}
/**CR (rw) register accessor: HSPI control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///HSPI control register
pub mod cr;
/**DCR1 (rw) register accessor: HSPI device configuration register 1

You can [`read`](crate::Reg::read) this register and get [`dcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:DCR1)

For information about available fields see [`mod@dcr1`] module*/
pub type DCR1 = crate::Reg<dcr1::DCR1rs>;
///HSPI device configuration register 1
pub mod dcr1;
/**DCR2 (rw) register accessor: HSPI device configuration register 2

You can [`read`](crate::Reg::read) this register and get [`dcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:DCR2)

For information about available fields see [`mod@dcr2`] module*/
pub type DCR2 = crate::Reg<dcr2::DCR2rs>;
///HSPI device configuration register 2
pub mod dcr2;
/**DCR3 (rw) register accessor: HSPI device configuration register 3

You can [`read`](crate::Reg::read) this register and get [`dcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:DCR3)

For information about available fields see [`mod@dcr3`] module*/
pub type DCR3 = crate::Reg<dcr3::DCR3rs>;
///HSPI device configuration register 3
pub mod dcr3;
/**DCR4 (rw) register accessor: HSPI device configuration register 4

You can [`read`](crate::Reg::read) this register and get [`dcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:DCR4)

For information about available fields see [`mod@dcr4`] module*/
pub type DCR4 = crate::Reg<dcr4::DCR4rs>;
///HSPI device configuration register 4
pub mod dcr4;
/**SR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///
pub mod sr;
/**FCR (w) register accessor:

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:FCR)

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///
pub mod fcr;
/**DLR (rw) register accessor: HSPI data length register

You can [`read`](crate::Reg::read) this register and get [`dlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:DLR)

For information about available fields see [`mod@dlr`] module*/
pub type DLR = crate::Reg<dlr::DLRrs>;
///HSPI data length register
pub mod dlr;
/**AR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:AR)

For information about available fields see [`mod@ar`] module*/
pub type AR = crate::Reg<ar::ARrs>;
///
pub mod ar;
/**DR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///
pub mod dr;
/**PSMKR (rw) register accessor: HSPI polling status mask register

You can [`read`](crate::Reg::read) this register and get [`psmkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:PSMKR)

For information about available fields see [`mod@psmkr`] module*/
pub type PSMKR = crate::Reg<psmkr::PSMKRrs>;
///HSPI polling status mask register
pub mod psmkr;
/**PSMAR (rw) register accessor: HSPI polling status match register

You can [`read`](crate::Reg::read) this register and get [`psmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:PSMAR)

For information about available fields see [`mod@psmar`] module*/
pub type PSMAR = crate::Reg<psmar::PSMARrs>;
///HSPI polling status match register
pub mod psmar;
/**PIR (rw) register accessor: HSPI polling interval register

You can [`read`](crate::Reg::read) this register and get [`pir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:PIR)

For information about available fields see [`mod@pir`] module*/
pub type PIR = crate::Reg<pir::PIRrs>;
///HSPI polling interval register
pub mod pir;
/**CCR (rw) register accessor: HSPI communication configuration register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///HSPI communication configuration register
pub mod ccr;
/**TCR (rw) register accessor: HSPI timing configuration register

You can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:TCR)

For information about available fields see [`mod@tcr`] module*/
pub type TCR = crate::Reg<tcr::TCRrs>;
///HSPI timing configuration register
pub mod tcr;
/**IR (rw) register accessor: HSPI instruction register

You can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:IR)

For information about available fields see [`mod@ir`] module*/
pub type IR = crate::Reg<ir::IRrs>;
///HSPI instruction register
pub mod ir;
/**ABR (rw) register accessor: HSPI alternate bytes register

You can [`read`](crate::Reg::read) this register and get [`abr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:ABR)

For information about available fields see [`mod@abr`] module*/
pub type ABR = crate::Reg<abr::ABRrs>;
///HSPI alternate bytes register
pub mod abr;
/**LPTR (rw) register accessor: HSPI low-power timeout register

You can [`read`](crate::Reg::read) this register and get [`lptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:LPTR)

For information about available fields see [`mod@lptr`] module*/
pub type LPTR = crate::Reg<lptr::LPTRrs>;
///HSPI low-power timeout register
pub mod lptr;
/**WPCCR (rw) register accessor: HSPI wrap communication configuration register

You can [`read`](crate::Reg::read) this register and get [`wpccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:WPCCR)

For information about available fields see [`mod@wpccr`] module*/
pub type WPCCR = crate::Reg<wpccr::WPCCRrs>;
///HSPI wrap communication configuration register
pub mod wpccr;
/**WPTCR (rw) register accessor: HSPI wrap timing configuration register

You can [`read`](crate::Reg::read) this register and get [`wptcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wptcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:WPTCR)

For information about available fields see [`mod@wptcr`] module*/
pub type WPTCR = crate::Reg<wptcr::WPTCRrs>;
///HSPI wrap timing configuration register
pub mod wptcr;
/**WPIR (rw) register accessor: HSPI wrap instruction register

You can [`read`](crate::Reg::read) this register and get [`wpir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:WPIR)

For information about available fields see [`mod@wpir`] module*/
pub type WPIR = crate::Reg<wpir::WPIRrs>;
///HSPI wrap instruction register
pub mod wpir;
/**WPABR (rw) register accessor: HSPI wrap alternate bytes register

You can [`read`](crate::Reg::read) this register and get [`wpabr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpabr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:WPABR)

For information about available fields see [`mod@wpabr`] module*/
pub type WPABR = crate::Reg<wpabr::WPABRrs>;
///HSPI wrap alternate bytes register
pub mod wpabr;
/**WCCR (rw) register accessor: HSPI write communication configuration register

You can [`read`](crate::Reg::read) this register and get [`wccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:WCCR)

For information about available fields see [`mod@wccr`] module*/
pub type WCCR = crate::Reg<wccr::WCCRrs>;
///HSPI write communication configuration register
pub mod wccr;
/**WTCR (rw) register accessor: HSPI write timing configuration register

You can [`read`](crate::Reg::read) this register and get [`wtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:WTCR)

For information about available fields see [`mod@wtcr`] module*/
pub type WTCR = crate::Reg<wtcr::WTCRrs>;
///HSPI write timing configuration register
pub mod wtcr;
/**WIR (rw) register accessor: HSPI write instruction register

You can [`read`](crate::Reg::read) this register and get [`wir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:WIR)

For information about available fields see [`mod@wir`] module*/
pub type WIR = crate::Reg<wir::WIRrs>;
///HSPI write instruction register
pub mod wir;
/**WABR (rw) register accessor: HSPI write alternate bytes register

You can [`read`](crate::Reg::read) this register and get [`wabr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wabr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:WABR)

For information about available fields see [`mod@wabr`] module*/
pub type WABR = crate::Reg<wabr::WABRrs>;
///HSPI write alternate bytes register
pub mod wabr;
/**HLCR (rw) register accessor: HSPI HyperBus latency configuration register

You can [`read`](crate::Reg::read) this register and get [`hlcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hlcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:HLCR)

For information about available fields see [`mod@hlcr`] module*/
pub type HLCR = crate::Reg<hlcr::HLCRrs>;
///HSPI HyperBus latency configuration register
pub mod hlcr;
/**CALFCR (r) register accessor: HSPI full-cycle calibration configuration

You can [`read`](crate::Reg::read) this register and get [`calfcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:CALFCR)

For information about available fields see [`mod@calfcr`] module*/
pub type CALFCR = crate::Reg<calfcr::CALFCRrs>;
///HSPI full-cycle calibration configuration
pub mod calfcr;
/**CALMR (rw) register accessor: HSPI DLL master calibration configuration

You can [`read`](crate::Reg::read) this register and get [`calmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:CALMR)

For information about available fields see [`mod@calmr`] module*/
pub type CALMR = crate::Reg<calmr::CALMRrs>;
///HSPI DLL master calibration configuration
pub mod calmr;
/**CALSOR (rw) register accessor: HSPI DLL slave output calibration configuration

You can [`read`](crate::Reg::read) this register and get [`calsor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calsor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:CALSOR)

For information about available fields see [`mod@calsor`] module*/
pub type CALSOR = crate::Reg<calsor::CALSORrs>;
///HSPI DLL slave output calibration configuration
pub mod calsor;
/**CALSIR (rw) register accessor: HSPI DLL slave input calibration configuration

You can [`read`](crate::Reg::read) this register and get [`calsir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calsir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:CALSIR)

For information about available fields see [`mod@calsir`] module*/
pub type CALSIR = crate::Reg<calsir::CALSIRrs>;
///HSPI DLL slave input calibration configuration
pub mod calsir;
