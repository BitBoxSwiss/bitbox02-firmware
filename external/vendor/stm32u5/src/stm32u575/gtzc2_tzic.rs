#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ier1: IER1,
    ier2: IER2,
    _reserved2: [u8; 0x08],
    sr1: SR1,
    sr2: SR2,
    _reserved4: [u8; 0x08],
    fcr1: FCR1,
    fcr2: FCR2,
}
impl RegisterBlock {
    ///0x00 - TZIC interrupt enable register 1
    #[inline(always)]
    pub const fn ier1(&self) -> &IER1 {
        &self.ier1
    }
    ///0x04 - TZIC interrupt enable register 2
    #[inline(always)]
    pub const fn ier2(&self) -> &IER2 {
        &self.ier2
    }
    ///0x10 - TZIC status register 1
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    ///0x14 - TZIC status register 2
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    ///0x20 - TZIC flag clear register 1
    #[inline(always)]
    pub const fn fcr1(&self) -> &FCR1 {
        &self.fcr1
    }
    ///0x24 - TZIC flag clear register 2
    #[inline(always)]
    pub const fn fcr2(&self) -> &FCR2 {
        &self.fcr2
    }
}
/**IER1 (rw) register accessor: TZIC interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_TZIC:IER1)

For information about available fields see [`mod@ier1`] module*/
pub type IER1 = crate::Reg<ier1::IER1rs>;
///TZIC interrupt enable register 1
pub mod ier1;
/**IER2 (rw) register accessor: TZIC interrupt enable register 2

You can [`read`](crate::Reg::read) this register and get [`ier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_TZIC:IER2)

For information about available fields see [`mod@ier2`] module*/
pub type IER2 = crate::Reg<ier2::IER2rs>;
///TZIC interrupt enable register 2
pub mod ier2;
/**SR1 (r) register accessor: TZIC status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_TZIC:SR1)

For information about available fields see [`mod@sr1`] module*/
pub type SR1 = crate::Reg<sr1::SR1rs>;
///TZIC status register 1
pub mod sr1;
/**SR2 (r) register accessor: TZIC status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_TZIC:SR2)

For information about available fields see [`mod@sr2`] module*/
pub type SR2 = crate::Reg<sr2::SR2rs>;
///TZIC status register 2
pub mod sr2;
/**FCR1 (w) register accessor: TZIC flag clear register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_TZIC:FCR1)

For information about available fields see [`mod@fcr1`] module*/
pub type FCR1 = crate::Reg<fcr1::FCR1rs>;
///TZIC flag clear register 1
pub mod fcr1;
/**FCR2 (w) register accessor: TZIC flag clear register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC2_TZIC:FCR2)

For information about available fields see [`mod@fcr2`] module*/
pub type FCR2 = crate::Reg<fcr2::FCR2rs>;
///TZIC flag clear register 2
pub mod fcr2;
