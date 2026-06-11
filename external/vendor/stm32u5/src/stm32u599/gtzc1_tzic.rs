#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ier1: IER1,
    ier2: IER2,
    ier3: IER3,
    ier4: IER4,
    sr1: SR1,
    sr2: SR2,
    sr3: SR3,
    sr4: SR4,
    fcr1: FCR1,
    fcr2: FCR2,
    fcr3: FCR3,
    fcr4: FCR4,
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
    ///0x08 - TZIC interrupt enable register 3
    #[inline(always)]
    pub const fn ier3(&self) -> &IER3 {
        &self.ier3
    }
    ///0x0c - TZIC interrupt enable register 4
    #[inline(always)]
    pub const fn ier4(&self) -> &IER4 {
        &self.ier4
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
    ///0x18 - TZIC status register 3
    #[inline(always)]
    pub const fn sr3(&self) -> &SR3 {
        &self.sr3
    }
    ///0x1c - TZIC status register 4
    #[inline(always)]
    pub const fn sr4(&self) -> &SR4 {
        &self.sr4
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
    ///0x28 - TZIC flag clear register 3
    #[inline(always)]
    pub const fn fcr3(&self) -> &FCR3 {
        &self.fcr3
    }
    ///0x2c - TZIC flag clear register 4
    #[inline(always)]
    pub const fn fcr4(&self) -> &FCR4 {
        &self.fcr4
    }
}
/**IER1 (rw) register accessor: TZIC interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:IER1)

For information about available fields see [`mod@ier1`] module*/
pub type IER1 = crate::Reg<ier1::IER1rs>;
///TZIC interrupt enable register 1
pub mod ier1;
/**IER2 (rw) register accessor: TZIC interrupt enable register 2

You can [`read`](crate::Reg::read) this register and get [`ier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:IER2)

For information about available fields see [`mod@ier2`] module*/
pub type IER2 = crate::Reg<ier2::IER2rs>;
///TZIC interrupt enable register 2
pub mod ier2;
/**IER3 (rw) register accessor: TZIC interrupt enable register 3

You can [`read`](crate::Reg::read) this register and get [`ier3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:IER3)

For information about available fields see [`mod@ier3`] module*/
pub type IER3 = crate::Reg<ier3::IER3rs>;
///TZIC interrupt enable register 3
pub mod ier3;
/**IER4 (rw) register accessor: TZIC interrupt enable register 4

You can [`read`](crate::Reg::read) this register and get [`ier4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:IER4)

For information about available fields see [`mod@ier4`] module*/
pub type IER4 = crate::Reg<ier4::IER4rs>;
///TZIC interrupt enable register 4
pub mod ier4;
/**SR1 (r) register accessor: TZIC status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:SR1)

For information about available fields see [`mod@sr1`] module*/
pub type SR1 = crate::Reg<sr1::SR1rs>;
///TZIC status register 1
pub mod sr1;
/**SR2 (r) register accessor: TZIC status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:SR2)

For information about available fields see [`mod@sr2`] module*/
pub type SR2 = crate::Reg<sr2::SR2rs>;
///TZIC status register 2
pub mod sr2;
/**SR3 (r) register accessor: TZIC status register 3

You can [`read`](crate::Reg::read) this register and get [`sr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:SR3)

For information about available fields see [`mod@sr3`] module*/
pub type SR3 = crate::Reg<sr3::SR3rs>;
///TZIC status register 3
pub mod sr3;
/**SR4 (r) register accessor: TZIC status register 4

You can [`read`](crate::Reg::read) this register and get [`sr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:SR4)

For information about available fields see [`mod@sr4`] module*/
pub type SR4 = crate::Reg<sr4::SR4rs>;
///TZIC status register 4
pub mod sr4;
/**FCR1 (w) register accessor: TZIC flag clear register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:FCR1)

For information about available fields see [`mod@fcr1`] module*/
pub type FCR1 = crate::Reg<fcr1::FCR1rs>;
///TZIC flag clear register 1
pub mod fcr1;
/**FCR2 (w) register accessor: TZIC flag clear register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:FCR2)

For information about available fields see [`mod@fcr2`] module*/
pub type FCR2 = crate::Reg<fcr2::FCR2rs>;
///TZIC flag clear register 2
pub mod fcr2;
/**FCR3 (w) register accessor: TZIC flag clear register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:FCR3)

For information about available fields see [`mod@fcr3`] module*/
pub type FCR3 = crate::Reg<fcr3::FCR3rs>;
///TZIC flag clear register 3
pub mod fcr3;
/**FCR4 (w) register accessor: TZIC flag clear register 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GTZC1_TZIC:FCR4)

For information about available fields see [`mod@fcr4`] module*/
pub type FCR4 = crate::Reg<fcr4::FCR4rs>;
///TZIC flag clear register 4
pub mod fcr4;
