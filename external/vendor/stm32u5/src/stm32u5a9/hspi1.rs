#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _cr: _CR,
    _reserved1: [u8; 0x04],
    _dcr1: _DCR1,
    _dcr2: _DCR2,
    _dcr3: _DCR3,
    _dcr4: _DCR4,
    _reserved5: [u8; 0x08],
    _sr: _SR,
    _fcr: _FCR,
    _reserved7: [u8; 0x18],
    _dlr: _DLR,
    _reserved8: [u8; 0x04],
    _ar: _AR,
    _reserved9: [u8; 0x04],
    _dr: _DR,
    _reserved10: [u8; 0x2c],
    _psmkr: _PSMKR,
    _reserved11: [u8; 0x04],
    _psmar: _PSMAR,
    _reserved12: [u8; 0x04],
    _pir: _PIR,
    _reserved13: [u8; 0x6c],
    _ccr: _CCR,
    _reserved14: [u8; 0x04],
    _tcr: _TCR,
    _reserved15: [u8; 0x04],
    _ir: _IR,
    _reserved16: [u8; 0x0c],
    _abr: _ABR,
    _reserved17: [u8; 0x0c],
    _lptr: _LPTR,
    _reserved18: [u8; 0x0c],
    _wpccr: _WPCCR,
    _reserved19: [u8; 0x04],
    _wptcr: _WPTCR,
    _reserved20: [u8; 0x04],
    _wpir: _WPIR,
    _reserved21: [u8; 0x0c],
    _wpabr: _WPABR,
    _reserved22: [u8; 0x1c],
    _wccr: _WCCR,
    _reserved23: [u8; 0x04],
    _wtcr: _WTCR,
    _reserved24: [u8; 0x04],
    _wir: _WIR,
    _reserved25: [u8; 0x0c],
    _wabr: _WABR,
    _reserved26: [u8; 0x5c],
    _hlcr: _HLCR,
    _reserved27: [u8; 0x0c],
    _calfcr: _CALFCR,
    _reserved28: [u8; 0x04],
    _calmr: _CALMR,
    _reserved29: [u8; 0x04],
    _calsor: _CALSOR,
    _reserved30: [u8; 0x04],
    _calsir: _CALSIR,
}
impl RegisterBlock {
    ///0x00 - HSPI control register
    #[inline(always)]
    pub const fn _cr(&self) -> &_CR {
        &self._cr
    }
    ///0x08 - HSPI device configuration register 1
    #[inline(always)]
    pub const fn _dcr1(&self) -> &_DCR1 {
        &self._dcr1
    }
    ///0x0c - HSPI device configuration register 2
    #[inline(always)]
    pub const fn _dcr2(&self) -> &_DCR2 {
        &self._dcr2
    }
    ///0x10 - HSPI device configuration register 3
    #[inline(always)]
    pub const fn _dcr3(&self) -> &_DCR3 {
        &self._dcr3
    }
    ///0x14 - HSPI device configuration register 4
    #[inline(always)]
    pub const fn _dcr4(&self) -> &_DCR4 {
        &self._dcr4
    }
    ///0x20 -
    #[inline(always)]
    pub const fn _sr(&self) -> &_SR {
        &self._sr
    }
    ///0x24 -
    #[inline(always)]
    pub const fn _fcr(&self) -> &_FCR {
        &self._fcr
    }
    ///0x40 - HSPI data length register
    #[inline(always)]
    pub const fn _dlr(&self) -> &_DLR {
        &self._dlr
    }
    ///0x48 -
    #[inline(always)]
    pub const fn _ar(&self) -> &_AR {
        &self._ar
    }
    ///0x50 -
    #[inline(always)]
    pub const fn _dr(&self) -> &_DR {
        &self._dr
    }
    ///0x80 - HSPI polling status mask register
    #[inline(always)]
    pub const fn _psmkr(&self) -> &_PSMKR {
        &self._psmkr
    }
    ///0x88 - HSPI polling status match register
    #[inline(always)]
    pub const fn _psmar(&self) -> &_PSMAR {
        &self._psmar
    }
    ///0x90 - HSPI polling interval register
    #[inline(always)]
    pub const fn _pir(&self) -> &_PIR {
        &self._pir
    }
    ///0x100 - HSPI communication configuration register
    #[inline(always)]
    pub const fn _ccr(&self) -> &_CCR {
        &self._ccr
    }
    ///0x108 - HSPI timing configuration register
    #[inline(always)]
    pub const fn _tcr(&self) -> &_TCR {
        &self._tcr
    }
    ///0x110 - HSPI instruction register
    #[inline(always)]
    pub const fn _ir(&self) -> &_IR {
        &self._ir
    }
    ///0x120 - HSPI alternate bytes register
    #[inline(always)]
    pub const fn _abr(&self) -> &_ABR {
        &self._abr
    }
    ///0x130 - HSPI low-power timeout register
    #[inline(always)]
    pub const fn _lptr(&self) -> &_LPTR {
        &self._lptr
    }
    ///0x140 - HSPI wrap communication configuration register
    #[inline(always)]
    pub const fn _wpccr(&self) -> &_WPCCR {
        &self._wpccr
    }
    ///0x148 - HSPI wrap timing configuration register
    #[inline(always)]
    pub const fn _wptcr(&self) -> &_WPTCR {
        &self._wptcr
    }
    ///0x150 - HSPI wrap instruction register
    #[inline(always)]
    pub const fn _wpir(&self) -> &_WPIR {
        &self._wpir
    }
    ///0x160 - HSPI wrap alternate bytes register
    #[inline(always)]
    pub const fn _wpabr(&self) -> &_WPABR {
        &self._wpabr
    }
    ///0x180 - HSPI write communication configuration register
    #[inline(always)]
    pub const fn _wccr(&self) -> &_WCCR {
        &self._wccr
    }
    ///0x188 - HSPI write timing configuration register
    #[inline(always)]
    pub const fn _wtcr(&self) -> &_WTCR {
        &self._wtcr
    }
    ///0x190 - HSPI write instruction register
    #[inline(always)]
    pub const fn _wir(&self) -> &_WIR {
        &self._wir
    }
    ///0x1a0 - HSPI write alternate bytes register
    #[inline(always)]
    pub const fn _wabr(&self) -> &_WABR {
        &self._wabr
    }
    ///0x200 - HSPI HyperBus latency configuration register
    #[inline(always)]
    pub const fn _hlcr(&self) -> &_HLCR {
        &self._hlcr
    }
    ///0x210 - HSPI full-cycle calibration configuration
    #[inline(always)]
    pub const fn _calfcr(&self) -> &_CALFCR {
        &self._calfcr
    }
    ///0x218 - HSPI DLL master calibration configuration
    #[inline(always)]
    pub const fn _calmr(&self) -> &_CALMR {
        &self._calmr
    }
    ///0x220 - HSPI DLL slave output calibration configuration
    #[inline(always)]
    pub const fn _calsor(&self) -> &_CALSOR {
        &self._calsor
    }
    ///0x228 - HSPI DLL slave input calibration configuration
    #[inline(always)]
    pub const fn _calsir(&self) -> &_CALSIR {
        &self._calsir
    }
}
/**_CR (rw) register accessor: HSPI control register

You can [`read`](crate::Reg::read) this register and get [`_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_CR)

For information about available fields see [`mod@_cr`] module*/
pub type _CR = crate::Reg<_cr::_CRrs>;
///HSPI control register
pub mod _cr;
/**_DCR1 (rw) register accessor: HSPI device configuration register 1

You can [`read`](crate::Reg::read) this register and get [`_dcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_DCR1)

For information about available fields see [`mod@_dcr1`] module*/
pub type _DCR1 = crate::Reg<_dcr1::_DCR1rs>;
///HSPI device configuration register 1
pub mod _dcr1;
/**_DCR2 (rw) register accessor: HSPI device configuration register 2

You can [`read`](crate::Reg::read) this register and get [`_dcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_DCR2)

For information about available fields see [`mod@_dcr2`] module*/
pub type _DCR2 = crate::Reg<_dcr2::_DCR2rs>;
///HSPI device configuration register 2
pub mod _dcr2;
/**_DCR3 (rw) register accessor: HSPI device configuration register 3

You can [`read`](crate::Reg::read) this register and get [`_dcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_DCR3)

For information about available fields see [`mod@_dcr3`] module*/
pub type _DCR3 = crate::Reg<_dcr3::_DCR3rs>;
///HSPI device configuration register 3
pub mod _dcr3;
/**_DCR4 (rw) register accessor: HSPI device configuration register 4

You can [`read`](crate::Reg::read) this register and get [`_dcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_DCR4)

For information about available fields see [`mod@_dcr4`] module*/
pub type _DCR4 = crate::Reg<_dcr4::_DCR4rs>;
///HSPI device configuration register 4
pub mod _dcr4;
/**_SR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_SR)

For information about available fields see [`mod@_sr`] module*/
pub type _SR = crate::Reg<_sr::_SRrs>;
///
pub mod _sr;
/**_FCR (w) register accessor:

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_FCR)

For information about available fields see [`mod@_fcr`] module*/
pub type _FCR = crate::Reg<_fcr::_FCRrs>;
///
pub mod _fcr;
/**_DLR (rw) register accessor: HSPI data length register

You can [`read`](crate::Reg::read) this register and get [`_dlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_DLR)

For information about available fields see [`mod@_dlr`] module*/
pub type _DLR = crate::Reg<_dlr::_DLRrs>;
///HSPI data length register
pub mod _dlr;
/**_AR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`_ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_AR)

For information about available fields see [`mod@_ar`] module*/
pub type _AR = crate::Reg<_ar::_ARrs>;
///
pub mod _ar;
/**_DR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_DR)

For information about available fields see [`mod@_dr`] module*/
pub type _DR = crate::Reg<_dr::_DRrs>;
///
pub mod _dr;
/**_PSMKR (rw) register accessor: HSPI polling status mask register

You can [`read`](crate::Reg::read) this register and get [`_psmkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_psmkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_PSMKR)

For information about available fields see [`mod@_psmkr`] module*/
pub type _PSMKR = crate::Reg<_psmkr::_PSMKRrs>;
///HSPI polling status mask register
pub mod _psmkr;
/**_PSMAR (rw) register accessor: HSPI polling status match register

You can [`read`](crate::Reg::read) this register and get [`_psmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_psmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_PSMAR)

For information about available fields see [`mod@_psmar`] module*/
pub type _PSMAR = crate::Reg<_psmar::_PSMARrs>;
///HSPI polling status match register
pub mod _psmar;
/**_PIR (rw) register accessor: HSPI polling interval register

You can [`read`](crate::Reg::read) this register and get [`_pir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_pir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_PIR)

For information about available fields see [`mod@_pir`] module*/
pub type _PIR = crate::Reg<_pir::_PIRrs>;
///HSPI polling interval register
pub mod _pir;
/**_CCR (rw) register accessor: HSPI communication configuration register

You can [`read`](crate::Reg::read) this register and get [`_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_CCR)

For information about available fields see [`mod@_ccr`] module*/
pub type _CCR = crate::Reg<_ccr::_CCRrs>;
///HSPI communication configuration register
pub mod _ccr;
/**_TCR (rw) register accessor: HSPI timing configuration register

You can [`read`](crate::Reg::read) this register and get [`_tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_TCR)

For information about available fields see [`mod@_tcr`] module*/
pub type _TCR = crate::Reg<_tcr::_TCRrs>;
///HSPI timing configuration register
pub mod _tcr;
/**_IR (rw) register accessor: HSPI instruction register

You can [`read`](crate::Reg::read) this register and get [`_ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_IR)

For information about available fields see [`mod@_ir`] module*/
pub type _IR = crate::Reg<_ir::_IRrs>;
///HSPI instruction register
pub mod _ir;
/**_ABR (rw) register accessor: HSPI alternate bytes register

You can [`read`](crate::Reg::read) this register and get [`_abr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_abr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_ABR)

For information about available fields see [`mod@_abr`] module*/
pub type _ABR = crate::Reg<_abr::_ABRrs>;
///HSPI alternate bytes register
pub mod _abr;
/**_LPTR (rw) register accessor: HSPI low-power timeout register

You can [`read`](crate::Reg::read) this register and get [`_lptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_lptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_LPTR)

For information about available fields see [`mod@_lptr`] module*/
pub type _LPTR = crate::Reg<_lptr::_LPTRrs>;
///HSPI low-power timeout register
pub mod _lptr;
/**_WPCCR (rw) register accessor: HSPI wrap communication configuration register

You can [`read`](crate::Reg::read) this register and get [`_wpccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_wpccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_WPCCR)

For information about available fields see [`mod@_wpccr`] module*/
pub type _WPCCR = crate::Reg<_wpccr::_WPCCRrs>;
///HSPI wrap communication configuration register
pub mod _wpccr;
/**_WPTCR (rw) register accessor: HSPI wrap timing configuration register

You can [`read`](crate::Reg::read) this register and get [`_wptcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_wptcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_WPTCR)

For information about available fields see [`mod@_wptcr`] module*/
pub type _WPTCR = crate::Reg<_wptcr::_WPTCRrs>;
///HSPI wrap timing configuration register
pub mod _wptcr;
/**_WPIR (rw) register accessor: HSPI wrap instruction register

You can [`read`](crate::Reg::read) this register and get [`_wpir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_wpir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_WPIR)

For information about available fields see [`mod@_wpir`] module*/
pub type _WPIR = crate::Reg<_wpir::_WPIRrs>;
///HSPI wrap instruction register
pub mod _wpir;
/**_WPABR (rw) register accessor: HSPI wrap alternate bytes register

You can [`read`](crate::Reg::read) this register and get [`_wpabr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_wpabr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_WPABR)

For information about available fields see [`mod@_wpabr`] module*/
pub type _WPABR = crate::Reg<_wpabr::_WPABRrs>;
///HSPI wrap alternate bytes register
pub mod _wpabr;
/**_WCCR (rw) register accessor: HSPI write communication configuration register

You can [`read`](crate::Reg::read) this register and get [`_wccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_wccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_WCCR)

For information about available fields see [`mod@_wccr`] module*/
pub type _WCCR = crate::Reg<_wccr::_WCCRrs>;
///HSPI write communication configuration register
pub mod _wccr;
/**_WTCR (rw) register accessor: HSPI write timing configuration register

You can [`read`](crate::Reg::read) this register and get [`_wtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_wtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_WTCR)

For information about available fields see [`mod@_wtcr`] module*/
pub type _WTCR = crate::Reg<_wtcr::_WTCRrs>;
///HSPI write timing configuration register
pub mod _wtcr;
/**_WIR (rw) register accessor: HSPI write instruction register

You can [`read`](crate::Reg::read) this register and get [`_wir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_wir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_WIR)

For information about available fields see [`mod@_wir`] module*/
pub type _WIR = crate::Reg<_wir::_WIRrs>;
///HSPI write instruction register
pub mod _wir;
/**_WABR (rw) register accessor: HSPI write alternate bytes register

You can [`read`](crate::Reg::read) this register and get [`_wabr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_wabr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_WABR)

For information about available fields see [`mod@_wabr`] module*/
pub type _WABR = crate::Reg<_wabr::_WABRrs>;
///HSPI write alternate bytes register
pub mod _wabr;
/**_HLCR (rw) register accessor: HSPI HyperBus latency configuration register

You can [`read`](crate::Reg::read) this register and get [`_hlcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_hlcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_HLCR)

For information about available fields see [`mod@_hlcr`] module*/
pub type _HLCR = crate::Reg<_hlcr::_HLCRrs>;
///HSPI HyperBus latency configuration register
pub mod _hlcr;
/**_CALFCR (r) register accessor: HSPI full-cycle calibration configuration

You can [`read`](crate::Reg::read) this register and get [`_calfcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_CALFCR)

For information about available fields see [`mod@_calfcr`] module*/
pub type _CALFCR = crate::Reg<_calfcr::_CALFCRrs>;
///HSPI full-cycle calibration configuration
pub mod _calfcr;
/**_CALMR (rw) register accessor: HSPI DLL master calibration configuration

You can [`read`](crate::Reg::read) this register and get [`_calmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_calmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_CALMR)

For information about available fields see [`mod@_calmr`] module*/
pub type _CALMR = crate::Reg<_calmr::_CALMRrs>;
///HSPI DLL master calibration configuration
pub mod _calmr;
/**_CALSOR (rw) register accessor: HSPI DLL slave output calibration configuration

You can [`read`](crate::Reg::read) this register and get [`_calsor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_calsor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_CALSOR)

For information about available fields see [`mod@_calsor`] module*/
pub type _CALSOR = crate::Reg<_calsor::_CALSORrs>;
///HSPI DLL slave output calibration configuration
pub mod _calsor;
/**_CALSIR (rw) register accessor: HSPI DLL slave input calibration configuration

You can [`read`](crate::Reg::read) this register and get [`_calsir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_calsir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_CALSIR)

For information about available fields see [`mod@_calsir`] module*/
pub type _CALSIR = crate::Reg<_calsir::_CALSIRrs>;
///HSPI DLL slave input calibration configuration
pub mod _calsir;
