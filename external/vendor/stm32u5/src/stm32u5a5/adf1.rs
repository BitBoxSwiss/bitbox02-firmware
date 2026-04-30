#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gcr: GCR,
    ckgcr: CKGCR,
    _reserved2: [u8; 0x78],
    sitf0cr: SITF0CR,
    bsmx0cr: BSMX0CR,
    dflt0cr: DFLT0CR,
    dflt0cicr: DFLT0CICR,
    dflt0rsfr: DFLT0RSFR,
    _reserved7: [u8; 0x10],
    dly0cr: DLY0CR,
    _reserved8: [u8; 0x04],
    dflt0ier: DFLT0IER,
    dflt0isr: DFLT0ISR,
    _reserved10: [u8; 0x04],
    sadcr: SADCR,
    sadcfgr: SADCFGR,
    sadsdlvr: SADSDLVR,
    sadanlvr: SADANLVR,
    _reserved14: [u8; 0x28],
    dflt0dr: DFLT0DR,
}
impl RegisterBlock {
    ///0x00 - ADF Global Control Register
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    ///0x04 - ADF clock generator control register
    #[inline(always)]
    pub const fn ckgcr(&self) -> &CKGCR {
        &self.ckgcr
    }
    ///0x80 - ADF serial interface control register 0
    #[inline(always)]
    pub const fn sitf0cr(&self) -> &SITF0CR {
        &self.sitf0cr
    }
    ///0x84 - ADF bitstream matrix control register 0
    #[inline(always)]
    pub const fn bsmx0cr(&self) -> &BSMX0CR {
        &self.bsmx0cr
    }
    ///0x88 - ADF digital filter control register 0
    #[inline(always)]
    pub const fn dflt0cr(&self) -> &DFLT0CR {
        &self.dflt0cr
    }
    ///0x8c - ADF digital filer configuration register 0
    #[inline(always)]
    pub const fn dflt0cicr(&self) -> &DFLT0CICR {
        &self.dflt0cicr
    }
    ///0x90 - ADF reshape filter configuration register 0
    #[inline(always)]
    pub const fn dflt0rsfr(&self) -> &DFLT0RSFR {
        &self.dflt0rsfr
    }
    ///0xa4 - ADF delay control register 0
    #[inline(always)]
    pub const fn dly0cr(&self) -> &DLY0CR {
        &self.dly0cr
    }
    ///0xac - ADF DFLT0 interrupt enable register
    #[inline(always)]
    pub const fn dflt0ier(&self) -> &DFLT0IER {
        &self.dflt0ier
    }
    ///0xb0 - ADF DFLT0 interrupt status register 0
    #[inline(always)]
    pub const fn dflt0isr(&self) -> &DFLT0ISR {
        &self.dflt0isr
    }
    ///0xb8 - ADF SAD control register
    #[inline(always)]
    pub const fn sadcr(&self) -> &SADCR {
        &self.sadcr
    }
    ///0xbc - ADF SAD configuration register
    #[inline(always)]
    pub const fn sadcfgr(&self) -> &SADCFGR {
        &self.sadcfgr
    }
    ///0xc0 - ADF SAD sound level register
    #[inline(always)]
    pub const fn sadsdlvr(&self) -> &SADSDLVR {
        &self.sadsdlvr
    }
    ///0xc4 - ADF SAD ambient noise level register
    #[inline(always)]
    pub const fn sadanlvr(&self) -> &SADANLVR {
        &self.sadanlvr
    }
    ///0xf0 - ADF digital filter data register 0
    #[inline(always)]
    pub const fn dflt0dr(&self) -> &DFLT0DR {
        &self.dflt0dr
    }
}
/**GCR (rw) register accessor: ADF Global Control Register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:GCR)

For information about available fields see [`mod@gcr`] module*/
pub type GCR = crate::Reg<gcr::GCRrs>;
///ADF Global Control Register
pub mod gcr;
/**CKGCR (rw) register accessor: ADF clock generator control register

You can [`read`](crate::Reg::read) this register and get [`ckgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:CKGCR)

For information about available fields see [`mod@ckgcr`] module*/
pub type CKGCR = crate::Reg<ckgcr::CKGCRrs>;
///ADF clock generator control register
pub mod ckgcr;
/**SITF0CR (rw) register accessor: ADF serial interface control register 0

You can [`read`](crate::Reg::read) this register and get [`sitf0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitf0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:SITF0CR)

For information about available fields see [`mod@sitf0cr`] module*/
pub type SITF0CR = crate::Reg<sitf0cr::SITF0CRrs>;
///ADF serial interface control register 0
pub mod sitf0cr;
/**BSMX0CR (rw) register accessor: ADF bitstream matrix control register 0

You can [`read`](crate::Reg::read) this register and get [`bsmx0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsmx0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:BSMX0CR)

For information about available fields see [`mod@bsmx0cr`] module*/
pub type BSMX0CR = crate::Reg<bsmx0cr::BSMX0CRrs>;
///ADF bitstream matrix control register 0
pub mod bsmx0cr;
/**DFLT0CR (rw) register accessor: ADF digital filter control register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:DFLT0CR)

For information about available fields see [`mod@dflt0cr`] module*/
pub type DFLT0CR = crate::Reg<dflt0cr::DFLT0CRrs>;
///ADF digital filter control register 0
pub mod dflt0cr;
/**DFLT0CICR (rw) register accessor: ADF digital filer configuration register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:DFLT0CICR)

For information about available fields see [`mod@dflt0cicr`] module*/
pub type DFLT0CICR = crate::Reg<dflt0cicr::DFLT0CICRrs>;
///ADF digital filer configuration register 0
pub mod dflt0cicr;
/**DFLT0RSFR (rw) register accessor: ADF reshape filter configuration register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:DFLT0RSFR)

For information about available fields see [`mod@dflt0rsfr`] module*/
pub type DFLT0RSFR = crate::Reg<dflt0rsfr::DFLT0RSFRrs>;
///ADF reshape filter configuration register 0
pub mod dflt0rsfr;
/**DLY0CR (rw) register accessor: ADF delay control register 0

You can [`read`](crate::Reg::read) this register and get [`dly0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:DLY0CR)

For information about available fields see [`mod@dly0cr`] module*/
pub type DLY0CR = crate::Reg<dly0cr::DLY0CRrs>;
///ADF delay control register 0
pub mod dly0cr;
/**DFLT0IER (rw) register accessor: ADF DFLT0 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dflt0ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:DFLT0IER)

For information about available fields see [`mod@dflt0ier`] module*/
pub type DFLT0IER = crate::Reg<dflt0ier::DFLT0IERrs>;
///ADF DFLT0 interrupt enable register
pub mod dflt0ier;
/**DFLT0ISR (rw) register accessor: ADF DFLT0 interrupt status register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:DFLT0ISR)

For information about available fields see [`mod@dflt0isr`] module*/
pub type DFLT0ISR = crate::Reg<dflt0isr::DFLT0ISRrs>;
///ADF DFLT0 interrupt status register 0
pub mod dflt0isr;
/**SADCR (rw) register accessor: ADF SAD control register

You can [`read`](crate::Reg::read) this register and get [`sadcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sadcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:SADCR)

For information about available fields see [`mod@sadcr`] module*/
pub type SADCR = crate::Reg<sadcr::SADCRrs>;
///ADF SAD control register
pub mod sadcr;
/**SADCFGR (rw) register accessor: ADF SAD configuration register

You can [`read`](crate::Reg::read) this register and get [`sadcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sadcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:SADCFGR)

For information about available fields see [`mod@sadcfgr`] module*/
pub type SADCFGR = crate::Reg<sadcfgr::SADCFGRrs>;
///ADF SAD configuration register
pub mod sadcfgr;
/**SADSDLVR (r) register accessor: ADF SAD sound level register

You can [`read`](crate::Reg::read) this register and get [`sadsdlvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:SADSDLVR)

For information about available fields see [`mod@sadsdlvr`] module*/
pub type SADSDLVR = crate::Reg<sadsdlvr::SADSDLVRrs>;
///ADF SAD sound level register
pub mod sadsdlvr;
/**SADANLVR (r) register accessor: ADF SAD ambient noise level register

You can [`read`](crate::Reg::read) this register and get [`sadanlvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:SADANLVR)

For information about available fields see [`mod@sadanlvr`] module*/
pub type SADANLVR = crate::Reg<sadanlvr::SADANLVRrs>;
///ADF SAD ambient noise level register
pub mod sadanlvr;
/**DFLT0DR (r) register accessor: ADF digital filter data register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADF1:DFLT0DR)

For information about available fields see [`mod@dflt0dr`] module*/
pub type DFLT0DR = crate::Reg<dflt0dr::DFLT0DRrs>;
///ADF digital filter data register 0
pub mod dflt0dr;
