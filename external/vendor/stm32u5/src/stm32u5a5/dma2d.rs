#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    isr: ISR,
    ifcr: IFCR,
    fgmar: FGMAR,
    fgor: FGOR,
    bgmar: BGMAR,
    bgor: BGOR,
    fgpfccr: FGPFCCR,
    fgcolr: FGCOLR,
    bgpfccr: BGPFCCR,
    bgcolr: BGCOLR,
    fgcmar: FGCMAR,
    bgcmar: BGCMAR,
    opfccr: OPFCCR,
    _reserved_14_ocolr: [u8; 0x04],
    omar: OMAR,
    oor: OOR,
    nlr: NLR,
    lwr: LWR,
    amtcr: AMTCR,
    _reserved20: [u8; 0x03b0],
    fgclut: FGCLUT,
    _reserved21: [u8; 0x03fc],
    bgclut: BGCLUT,
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - Interrupt Status Register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x08 - interrupt flag clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    ///0x0c - foreground memory address register
    #[inline(always)]
    pub const fn fgmar(&self) -> &FGMAR {
        &self.fgmar
    }
    ///0x10 - foreground offset register
    #[inline(always)]
    pub const fn fgor(&self) -> &FGOR {
        &self.fgor
    }
    ///0x14 - background memory address register
    #[inline(always)]
    pub const fn bgmar(&self) -> &BGMAR {
        &self.bgmar
    }
    ///0x18 - background offset register
    #[inline(always)]
    pub const fn bgor(&self) -> &BGOR {
        &self.bgor
    }
    ///0x1c - foreground PFC control register
    #[inline(always)]
    pub const fn fgpfccr(&self) -> &FGPFCCR {
        &self.fgpfccr
    }
    ///0x20 - foreground color register
    #[inline(always)]
    pub const fn fgcolr(&self) -> &FGCOLR {
        &self.fgcolr
    }
    ///0x24 - background PFC control register
    #[inline(always)]
    pub const fn bgpfccr(&self) -> &BGPFCCR {
        &self.bgpfccr
    }
    ///0x28 - background color register
    #[inline(always)]
    pub const fn bgcolr(&self) -> &BGCOLR {
        &self.bgcolr
    }
    ///0x2c - foreground CLUT memory address register
    #[inline(always)]
    pub const fn fgcmar(&self) -> &FGCMAR {
        &self.fgcmar
    }
    ///0x30 - background CLUT memory address register
    #[inline(always)]
    pub const fn bgcmar(&self) -> &BGCMAR {
        &self.bgcmar
    }
    ///0x34 - output PFC control register
    #[inline(always)]
    pub const fn opfccr(&self) -> &OPFCCR {
        &self.opfccr
    }
    ///0x38 - output color register
    #[inline(always)]
    pub const fn ocolr_argb4444(&self) -> &OCOLR_ARGB4444 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x38 - output color register
    #[inline(always)]
    pub const fn ocolr_argb1555(&self) -> &OCOLR_ARGB1555 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x38 - output color register
    #[inline(always)]
    pub const fn ocolr_rgb565(&self) -> &OCOLR_RGB565 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x38 - output color register
    #[inline(always)]
    pub const fn ocolr_rgb888(&self) -> &OCOLR_RGB888 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x3c - output memory address register
    #[inline(always)]
    pub const fn omar(&self) -> &OMAR {
        &self.omar
    }
    ///0x40 - output offset register
    #[inline(always)]
    pub const fn oor(&self) -> &OOR {
        &self.oor
    }
    ///0x44 - number of line register
    #[inline(always)]
    pub const fn nlr(&self) -> &NLR {
        &self.nlr
    }
    ///0x48 - line watermark register
    #[inline(always)]
    pub const fn lwr(&self) -> &LWR {
        &self.lwr
    }
    ///0x4c - AHB master timer configuration register
    #[inline(always)]
    pub const fn amtcr(&self) -> &AMTCR {
        &self.amtcr
    }
    ///0x400 - FGCLUT
    #[inline(always)]
    pub const fn fgclut(&self) -> &FGCLUT {
        &self.fgclut
    }
    ///0x800 - BGCLUT
    #[inline(always)]
    pub const fn bgclut(&self) -> &BGCLUT {
        &self.bgclut
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**ISR (r) register accessor: Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///Interrupt Status Register
pub mod isr;
/**IFCR (rw) register accessor: interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`ifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:IFCR)

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///interrupt flag clear register
pub mod ifcr;
/**FGMAR (rw) register accessor: foreground memory address register

You can [`read`](crate::Reg::read) this register and get [`fgmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:FGMAR)

For information about available fields see [`mod@fgmar`] module*/
pub type FGMAR = crate::Reg<fgmar::FGMARrs>;
///foreground memory address register
pub mod fgmar;
/**FGOR (rw) register accessor: foreground offset register

You can [`read`](crate::Reg::read) this register and get [`fgor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:FGOR)

For information about available fields see [`mod@fgor`] module*/
pub type FGOR = crate::Reg<fgor::FGORrs>;
///foreground offset register
pub mod fgor;
/**BGMAR (rw) register accessor: background memory address register

You can [`read`](crate::Reg::read) this register and get [`bgmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:BGMAR)

For information about available fields see [`mod@bgmar`] module*/
pub type BGMAR = crate::Reg<bgmar::BGMARrs>;
///background memory address register
pub mod bgmar;
/**BGOR (rw) register accessor: background offset register

You can [`read`](crate::Reg::read) this register and get [`bgor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:BGOR)

For information about available fields see [`mod@bgor`] module*/
pub type BGOR = crate::Reg<bgor::BGORrs>;
///background offset register
pub mod bgor;
/**FGPFCCR (rw) register accessor: foreground PFC control register

You can [`read`](crate::Reg::read) this register and get [`fgpfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgpfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:FGPFCCR)

For information about available fields see [`mod@fgpfccr`] module*/
pub type FGPFCCR = crate::Reg<fgpfccr::FGPFCCRrs>;
///foreground PFC control register
pub mod fgpfccr;
/**FGCOLR (rw) register accessor: foreground color register

You can [`read`](crate::Reg::read) this register and get [`fgcolr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgcolr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:FGCOLR)

For information about available fields see [`mod@fgcolr`] module*/
pub type FGCOLR = crate::Reg<fgcolr::FGCOLRrs>;
///foreground color register
pub mod fgcolr;
/**BGPFCCR (rw) register accessor: background PFC control register

You can [`read`](crate::Reg::read) this register and get [`bgpfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgpfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:BGPFCCR)

For information about available fields see [`mod@bgpfccr`] module*/
pub type BGPFCCR = crate::Reg<bgpfccr::BGPFCCRrs>;
///background PFC control register
pub mod bgpfccr;
/**BGCOLR (rw) register accessor: background color register

You can [`read`](crate::Reg::read) this register and get [`bgcolr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcolr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:BGCOLR)

For information about available fields see [`mod@bgcolr`] module*/
pub type BGCOLR = crate::Reg<bgcolr::BGCOLRrs>;
///background color register
pub mod bgcolr;
/**FGCMAR (rw) register accessor: foreground CLUT memory address register

You can [`read`](crate::Reg::read) this register and get [`fgcmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgcmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:FGCMAR)

For information about available fields see [`mod@fgcmar`] module*/
pub type FGCMAR = crate::Reg<fgcmar::FGCMARrs>;
///foreground CLUT memory address register
pub mod fgcmar;
/**BGCMAR (rw) register accessor: background CLUT memory address register

You can [`read`](crate::Reg::read) this register and get [`bgcmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:BGCMAR)

For information about available fields see [`mod@bgcmar`] module*/
pub type BGCMAR = crate::Reg<bgcmar::BGCMARrs>;
///background CLUT memory address register
pub mod bgcmar;
/**OPFCCR (rw) register accessor: output PFC control register

You can [`read`](crate::Reg::read) this register and get [`opfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:OPFCCR)

For information about available fields see [`mod@opfccr`] module*/
pub type OPFCCR = crate::Reg<opfccr::OPFCCRrs>;
///output PFC control register
pub mod opfccr;
/**OCOLR_RGB888 (rw) register accessor: output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_rgb888::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_rgb888::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:OCOLR_RGB888)

For information about available fields see [`mod@ocolr_rgb888`] module*/
pub type OCOLR_RGB888 = crate::Reg<ocolr_rgb888::OCOLR_RGB888rs>;
///output color register
pub mod ocolr_rgb888;
/**OCOLR_RGB565 (rw) register accessor: output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_rgb565::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_rgb565::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:OCOLR_RGB565)

For information about available fields see [`mod@ocolr_rgb565`] module*/
pub type OCOLR_RGB565 = crate::Reg<ocolr_rgb565::OCOLR_RGB565rs>;
///output color register
pub mod ocolr_rgb565;
/**OCOLR_ARGB1555 (rw) register accessor: output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_argb1555::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_argb1555::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:OCOLR_ARGB1555)

For information about available fields see [`mod@ocolr_argb1555`] module*/
pub type OCOLR_ARGB1555 = crate::Reg<ocolr_argb1555::OCOLR_ARGB1555rs>;
///output color register
pub mod ocolr_argb1555;
/**OCOLR_ARGB4444 (rw) register accessor: output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_argb4444::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_argb4444::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:OCOLR_ARGB4444)

For information about available fields see [`mod@ocolr_argb4444`] module*/
pub type OCOLR_ARGB4444 = crate::Reg<ocolr_argb4444::OCOLR_ARGB4444rs>;
///output color register
pub mod ocolr_argb4444;
/**OMAR (rw) register accessor: output memory address register

You can [`read`](crate::Reg::read) this register and get [`omar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:OMAR)

For information about available fields see [`mod@omar`] module*/
pub type OMAR = crate::Reg<omar::OMARrs>;
///output memory address register
pub mod omar;
/**OOR (rw) register accessor: output offset register

You can [`read`](crate::Reg::read) this register and get [`oor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:OOR)

For information about available fields see [`mod@oor`] module*/
pub type OOR = crate::Reg<oor::OORrs>;
///output offset register
pub mod oor;
/**NLR (rw) register accessor: number of line register

You can [`read`](crate::Reg::read) this register and get [`nlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:NLR)

For information about available fields see [`mod@nlr`] module*/
pub type NLR = crate::Reg<nlr::NLRrs>;
///number of line register
pub mod nlr;
/**LWR (rw) register accessor: line watermark register

You can [`read`](crate::Reg::read) this register and get [`lwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:LWR)

For information about available fields see [`mod@lwr`] module*/
pub type LWR = crate::Reg<lwr::LWRrs>;
///line watermark register
pub mod lwr;
/**AMTCR (rw) register accessor: AHB master timer configuration register

You can [`read`](crate::Reg::read) this register and get [`amtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:AMTCR)

For information about available fields see [`mod@amtcr`] module*/
pub type AMTCR = crate::Reg<amtcr::AMTCRrs>;
///AHB master timer configuration register
pub mod amtcr;
/**FGCLUT (rw) register accessor: FGCLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:FGCLUT)

For information about available fields see [`mod@fgclut`] module*/
pub type FGCLUT = crate::Reg<fgclut::FGCLUTrs>;
///FGCLUT
pub mod fgclut;
/**BGCLUT (rw) register accessor: BGCLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DMA2D:BGCLUT)

For information about available fields see [`mod@bgclut`] module*/
pub type BGCLUT = crate::Reg<bgclut::BGCLUTrs>;
///BGCLUT
pub mod bgclut;
