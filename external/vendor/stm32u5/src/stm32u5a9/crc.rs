#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_dr: [u8; 0x04],
    idr: IDR,
    cr: CR,
    _reserved3: [u8; 0x04],
    init: INIT,
    pol: POL,
}
impl RegisterBlock {
    ///0x00 - Data register - half-word sized
    #[inline(always)]
    pub const fn dr16(&self) -> &DR16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - Data register - byte sized
    #[inline(always)]
    pub const fn dr8(&self) -> &DR8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - Data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - Independent data register
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x08 - Control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - Initial CRC value
    #[inline(always)]
    pub const fn init(&self) -> &INIT {
        &self.init
    }
    ///0x14 - polynomial
    #[inline(always)]
    pub const fn pol(&self) -> &POL {
        &self.pol
    }
}
/**DR (rw) register accessor: Data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#CRC:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///Data register
pub mod dr;
/**DR8 (rw) register accessor: Data register - byte sized

You can [`read`](crate::Reg::read) this register and get [`dr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#CRC:DR8)

For information about available fields see [`mod@dr8`] module*/
pub type DR8 = crate::Reg<dr8::DR8rs>;
///Data register - byte sized
pub mod dr8;
/**DR16 (rw) register accessor: Data register - half-word sized

You can [`read`](crate::Reg::read) this register and get [`dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#CRC:DR16)

For information about available fields see [`mod@dr16`] module*/
pub type DR16 = crate::Reg<dr16::DR16rs>;
///Data register - half-word sized
pub mod dr16;
/**IDR (rw) register accessor: Independent data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#CRC:IDR)

For information about available fields see [`mod@idr`] module*/
pub type IDR = crate::Reg<idr::IDRrs>;
///Independent data register
pub mod idr;
/**CR (rw) register accessor: Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#CRC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Control register
pub mod cr;
/**INIT (rw) register accessor: Initial CRC value

You can [`read`](crate::Reg::read) this register and get [`init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#CRC:INIT)

For information about available fields see [`mod@init`] module*/
pub type INIT = crate::Reg<init::INITrs>;
///Initial CRC value
pub mod init;
/**POL (rw) register accessor: polynomial

You can [`read`](crate::Reg::read) this register and get [`pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#CRC:POL)

For information about available fields see [`mod@pol`] module*/
pub type POL = crate::Reg<pol::POLrs>;
///polynomial
pub mod pol;
