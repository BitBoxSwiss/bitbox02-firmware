#[repr(C)]
#[derive(Debug)]
///Extended channel cluster
pub struct CH2D {
    lbar: LBAR,
    _reserved1: [u8; 0x08],
    fcr: FCR,
    sr: SR,
    cr: CR,
    _reserved4: [u8; 0x28],
    tr1: TR1,
    tr2: TR2,
    br1: BR1,
    sar: SAR,
    dar: DAR,
    tr3: TR3,
    br2: BR2,
    _reserved11: [u8; 0x20],
    llr: LLR,
}
impl CH2D {
    ///0x00 - GPDMA channel 12 linked-list base address register
    #[inline(always)]
    pub const fn lbar(&self) -> &LBAR {
        &self.lbar
    }
    ///0x0c - GPDMA channel 12 flag clear register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x10 - GPDMA channel 12 status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x14 - GPDMA channel 12 control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x40 - GPDMA channel 12 transfer register 1
    #[inline(always)]
    pub const fn tr1(&self) -> &TR1 {
        &self.tr1
    }
    ///0x44 - GPDMA channel 12 transfer register 2
    #[inline(always)]
    pub const fn tr2(&self) -> &TR2 {
        &self.tr2
    }
    ///0x48 - GPDMA channel 12 alternate block register 1
    #[inline(always)]
    pub const fn br1(&self) -> &BR1 {
        &self.br1
    }
    ///0x4c - GPDMA channel 12 source address register
    #[inline(always)]
    pub const fn sar(&self) -> &SAR {
        &self.sar
    }
    ///0x50 - GPDMA channel 12 destination address register
    #[inline(always)]
    pub const fn dar(&self) -> &DAR {
        &self.dar
    }
    ///0x54 - GPDMA channel 12 transfer register 3
    #[inline(always)]
    pub const fn tr3(&self) -> &TR3 {
        &self.tr3
    }
    ///0x58 - GPDMA channel 12 block register 2
    #[inline(always)]
    pub const fn br2(&self) -> &BR2 {
        &self.br2
    }
    ///0x7c - GPDMA channel 12 alternate linked-list address register
    #[inline(always)]
    pub const fn llr(&self) -> &LLR {
        &self.llr
    }
}
pub use crate::stm32u575::gpdma1::ch::cr;
pub use crate::stm32u575::gpdma1::ch::fcr;
pub use crate::stm32u575::gpdma1::ch::lbar;
pub use crate::stm32u575::gpdma1::ch::sr;
pub use crate::stm32u575::gpdma1::ch::tr1;
pub use crate::stm32u575::gpdma1::ch::tr2;
pub use crate::stm32u575::gpdma1::ch::CR;
pub use crate::stm32u575::gpdma1::ch::FCR;
pub use crate::stm32u575::gpdma1::ch::LBAR;
pub use crate::stm32u575::gpdma1::ch::SR;
pub use crate::stm32u575::gpdma1::ch::TR1;
pub use crate::stm32u575::gpdma1::ch::TR2;
/**BR1 (rw) register accessor: GPDMA channel 12 alternate block register 1

You can [`read`](crate::Reg::read) this register and get [`br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@br1`] module*/
pub type BR1 = crate::Reg<br1::BR1rs>;
///GPDMA channel 12 alternate block register 1
pub mod br1;
pub use crate::stm32u575::gpdma1::ch::dar;
pub use crate::stm32u575::gpdma1::ch::sar;
pub use crate::stm32u575::gpdma1::ch::DAR;
pub use crate::stm32u575::gpdma1::ch::SAR;
/**TR3 (rw) register accessor: GPDMA channel 12 transfer register 3

You can [`read`](crate::Reg::read) this register and get [`tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tr3`] module*/
pub type TR3 = crate::Reg<tr3::TR3rs>;
///GPDMA channel 12 transfer register 3
pub mod tr3;
/**BR2 (rw) register accessor: GPDMA channel 12 block register 2

You can [`read`](crate::Reg::read) this register and get [`br2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@br2`] module*/
pub type BR2 = crate::Reg<br2::BR2rs>;
///GPDMA channel 12 block register 2
pub mod br2;
/**LLR (rw) register accessor: GPDMA channel 12 alternate linked-list address register

You can [`read`](crate::Reg::read) this register and get [`llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@llr`] module*/
pub type LLR = crate::Reg<llr::LLRrs>;
///GPDMA channel 12 alternate linked-list address register
pub mod llr;
