#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    din: DIN,
    str: STR,
    hra0: HRA0,
    hra1: HRA1,
    hra2: HRA2,
    hra3: HRA3,
    hra4: HRA4,
    imr: IMR,
    sr: SR,
    _reserved10: [u8; 0xd0],
    csr0: CSR0,
    csr1: CSR1,
    csr2: CSR2,
    csr3: CSR3,
    csr4: CSR4,
    csr5: CSR5,
    csr6: CSR6,
    csr7: CSR7,
    csr8: CSR8,
    csr9: CSR9,
    csr10: CSR10,
    csr11: CSR11,
    csr12: CSR12,
    csr13: CSR13,
    csr14: CSR14,
    csr15: CSR15,
    csr16: CSR16,
    csr17: CSR17,
    csr18: CSR18,
    csr19: CSR19,
    csr20: CSR20,
    csr21: CSR21,
    csr22: CSR22,
    csr23: CSR23,
    csr24: CSR24,
    csr25: CSR25,
    csr26: CSR26,
    csr27: CSR27,
    csr28: CSR28,
    csr29: CSR29,
    csr30: CSR30,
    csr31: CSR31,
    csr32: CSR32,
    csr33: CSR33,
    csr34: CSR34,
    csr35: CSR35,
    csr36: CSR36,
    csr37: CSR37,
    csr38: CSR38,
    csr39: CSR39,
    csr40: CSR40,
    csr41: CSR41,
    csr42: CSR42,
    csr43: CSR43,
    csr44: CSR44,
    csr45: CSR45,
    csr46: CSR46,
    csr47: CSR47,
    csr48: CSR48,
    csr49: CSR49,
    csr50: CSR50,
    csr51: CSR51,
    csr52: CSR52,
    csr53: CSR53,
    _reserved64: [u8; 0x0140],
    hr0: HR0,
    hr1: HR1,
    hr2: HR2,
    hr3: HR3,
    hr4: HR4,
    hr5: HR5,
    hr6: HR6,
    hr7: HR7,
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - data input register
    #[inline(always)]
    pub const fn din(&self) -> &DIN {
        &self.din
    }
    ///0x08 - start register
    #[inline(always)]
    pub const fn str(&self) -> &STR {
        &self.str
    }
    ///0x0c - HASH aliased digest register 0
    #[inline(always)]
    pub const fn hra0(&self) -> &HRA0 {
        &self.hra0
    }
    ///0x10 - HASH aliased digest register 1
    #[inline(always)]
    pub const fn hra1(&self) -> &HRA1 {
        &self.hra1
    }
    ///0x14 - HASH aliased digest register 2
    #[inline(always)]
    pub const fn hra2(&self) -> &HRA2 {
        &self.hra2
    }
    ///0x18 - HASH aliased digest register 3
    #[inline(always)]
    pub const fn hra3(&self) -> &HRA3 {
        &self.hra3
    }
    ///0x1c - HASH aliased digest register 4
    #[inline(always)]
    pub const fn hra4(&self) -> &HRA4 {
        &self.hra4
    }
    ///0x20 - interrupt enable register
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    ///0x24 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0xf8 - context swap registers
    #[inline(always)]
    pub const fn csr0(&self) -> &CSR0 {
        &self.csr0
    }
    ///0xfc - context swap registers
    #[inline(always)]
    pub const fn csr1(&self) -> &CSR1 {
        &self.csr1
    }
    ///0x100 - context swap registers
    #[inline(always)]
    pub const fn csr2(&self) -> &CSR2 {
        &self.csr2
    }
    ///0x104 - context swap registers
    #[inline(always)]
    pub const fn csr3(&self) -> &CSR3 {
        &self.csr3
    }
    ///0x108 - context swap registers
    #[inline(always)]
    pub const fn csr4(&self) -> &CSR4 {
        &self.csr4
    }
    ///0x10c - context swap registers
    #[inline(always)]
    pub const fn csr5(&self) -> &CSR5 {
        &self.csr5
    }
    ///0x110 - context swap registers
    #[inline(always)]
    pub const fn csr6(&self) -> &CSR6 {
        &self.csr6
    }
    ///0x114 - context swap registers
    #[inline(always)]
    pub const fn csr7(&self) -> &CSR7 {
        &self.csr7
    }
    ///0x118 - context swap registers
    #[inline(always)]
    pub const fn csr8(&self) -> &CSR8 {
        &self.csr8
    }
    ///0x11c - context swap registers
    #[inline(always)]
    pub const fn csr9(&self) -> &CSR9 {
        &self.csr9
    }
    ///0x120 - context swap registers
    #[inline(always)]
    pub const fn csr10(&self) -> &CSR10 {
        &self.csr10
    }
    ///0x124 - context swap registers
    #[inline(always)]
    pub const fn csr11(&self) -> &CSR11 {
        &self.csr11
    }
    ///0x128 - context swap registers
    #[inline(always)]
    pub const fn csr12(&self) -> &CSR12 {
        &self.csr12
    }
    ///0x12c - context swap registers
    #[inline(always)]
    pub const fn csr13(&self) -> &CSR13 {
        &self.csr13
    }
    ///0x130 - context swap registers
    #[inline(always)]
    pub const fn csr14(&self) -> &CSR14 {
        &self.csr14
    }
    ///0x134 - context swap registers
    #[inline(always)]
    pub const fn csr15(&self) -> &CSR15 {
        &self.csr15
    }
    ///0x138 - context swap registers
    #[inline(always)]
    pub const fn csr16(&self) -> &CSR16 {
        &self.csr16
    }
    ///0x13c - context swap registers
    #[inline(always)]
    pub const fn csr17(&self) -> &CSR17 {
        &self.csr17
    }
    ///0x140 - context swap registers
    #[inline(always)]
    pub const fn csr18(&self) -> &CSR18 {
        &self.csr18
    }
    ///0x144 - context swap registers
    #[inline(always)]
    pub const fn csr19(&self) -> &CSR19 {
        &self.csr19
    }
    ///0x148 - context swap registers
    #[inline(always)]
    pub const fn csr20(&self) -> &CSR20 {
        &self.csr20
    }
    ///0x14c - context swap registers
    #[inline(always)]
    pub const fn csr21(&self) -> &CSR21 {
        &self.csr21
    }
    ///0x150 - context swap registers
    #[inline(always)]
    pub const fn csr22(&self) -> &CSR22 {
        &self.csr22
    }
    ///0x154 - context swap registers
    #[inline(always)]
    pub const fn csr23(&self) -> &CSR23 {
        &self.csr23
    }
    ///0x158 - context swap registers
    #[inline(always)]
    pub const fn csr24(&self) -> &CSR24 {
        &self.csr24
    }
    ///0x15c - context swap registers
    #[inline(always)]
    pub const fn csr25(&self) -> &CSR25 {
        &self.csr25
    }
    ///0x160 - context swap registers
    #[inline(always)]
    pub const fn csr26(&self) -> &CSR26 {
        &self.csr26
    }
    ///0x164 - context swap registers
    #[inline(always)]
    pub const fn csr27(&self) -> &CSR27 {
        &self.csr27
    }
    ///0x168 - context swap registers
    #[inline(always)]
    pub const fn csr28(&self) -> &CSR28 {
        &self.csr28
    }
    ///0x16c - context swap registers
    #[inline(always)]
    pub const fn csr29(&self) -> &CSR29 {
        &self.csr29
    }
    ///0x170 - context swap registers
    #[inline(always)]
    pub const fn csr30(&self) -> &CSR30 {
        &self.csr30
    }
    ///0x174 - context swap registers
    #[inline(always)]
    pub const fn csr31(&self) -> &CSR31 {
        &self.csr31
    }
    ///0x178 - context swap registers
    #[inline(always)]
    pub const fn csr32(&self) -> &CSR32 {
        &self.csr32
    }
    ///0x17c - context swap registers
    #[inline(always)]
    pub const fn csr33(&self) -> &CSR33 {
        &self.csr33
    }
    ///0x180 - context swap registers
    #[inline(always)]
    pub const fn csr34(&self) -> &CSR34 {
        &self.csr34
    }
    ///0x184 - context swap registers
    #[inline(always)]
    pub const fn csr35(&self) -> &CSR35 {
        &self.csr35
    }
    ///0x188 - context swap registers
    #[inline(always)]
    pub const fn csr36(&self) -> &CSR36 {
        &self.csr36
    }
    ///0x18c - context swap registers
    #[inline(always)]
    pub const fn csr37(&self) -> &CSR37 {
        &self.csr37
    }
    ///0x190 - context swap registers
    #[inline(always)]
    pub const fn csr38(&self) -> &CSR38 {
        &self.csr38
    }
    ///0x194 - context swap registers
    #[inline(always)]
    pub const fn csr39(&self) -> &CSR39 {
        &self.csr39
    }
    ///0x198 - context swap registers
    #[inline(always)]
    pub const fn csr40(&self) -> &CSR40 {
        &self.csr40
    }
    ///0x19c - context swap registers
    #[inline(always)]
    pub const fn csr41(&self) -> &CSR41 {
        &self.csr41
    }
    ///0x1a0 - context swap registers
    #[inline(always)]
    pub const fn csr42(&self) -> &CSR42 {
        &self.csr42
    }
    ///0x1a4 - context swap registers
    #[inline(always)]
    pub const fn csr43(&self) -> &CSR43 {
        &self.csr43
    }
    ///0x1a8 - context swap registers
    #[inline(always)]
    pub const fn csr44(&self) -> &CSR44 {
        &self.csr44
    }
    ///0x1ac - context swap registers
    #[inline(always)]
    pub const fn csr45(&self) -> &CSR45 {
        &self.csr45
    }
    ///0x1b0 - context swap registers
    #[inline(always)]
    pub const fn csr46(&self) -> &CSR46 {
        &self.csr46
    }
    ///0x1b4 - context swap registers
    #[inline(always)]
    pub const fn csr47(&self) -> &CSR47 {
        &self.csr47
    }
    ///0x1b8 - context swap registers
    #[inline(always)]
    pub const fn csr48(&self) -> &CSR48 {
        &self.csr48
    }
    ///0x1bc - context swap registers
    #[inline(always)]
    pub const fn csr49(&self) -> &CSR49 {
        &self.csr49
    }
    ///0x1c0 - context swap registers
    #[inline(always)]
    pub const fn csr50(&self) -> &CSR50 {
        &self.csr50
    }
    ///0x1c4 - context swap registers
    #[inline(always)]
    pub const fn csr51(&self) -> &CSR51 {
        &self.csr51
    }
    ///0x1c8 - context swap registers
    #[inline(always)]
    pub const fn csr52(&self) -> &CSR52 {
        &self.csr52
    }
    ///0x1cc - context swap registers
    #[inline(always)]
    pub const fn csr53(&self) -> &CSR53 {
        &self.csr53
    }
    ///0x310 - digest register 0
    #[inline(always)]
    pub const fn hr0(&self) -> &HR0 {
        &self.hr0
    }
    ///0x314 - digest register 1
    #[inline(always)]
    pub const fn hr1(&self) -> &HR1 {
        &self.hr1
    }
    ///0x318 - digest register 4
    #[inline(always)]
    pub const fn hr2(&self) -> &HR2 {
        &self.hr2
    }
    ///0x31c - digest register 3
    #[inline(always)]
    pub const fn hr3(&self) -> &HR3 {
        &self.hr3
    }
    ///0x320 - digest register 4
    #[inline(always)]
    pub const fn hr4(&self) -> &HR4 {
        &self.hr4
    }
    ///0x324 - supplementary digest register 5
    #[inline(always)]
    pub const fn hr5(&self) -> &HR5 {
        &self.hr5
    }
    ///0x328 - supplementary digest register 6
    #[inline(always)]
    pub const fn hr6(&self) -> &HR6 {
        &self.hr6
    }
    ///0x32c - supplementary digest register 7
    #[inline(always)]
    pub const fn hr7(&self) -> &HR7 {
        &self.hr7
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**DIN (w) register accessor: data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:DIN)

For information about available fields see [`mod@din`] module*/
pub type DIN = crate::Reg<din::DINrs>;
///data input register
pub mod din;
/**STR (rw) register accessor: start register

You can [`read`](crate::Reg::read) this register and get [`str::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:STR)

For information about available fields see [`mod@str`] module*/
pub type STR = crate::Reg<str::STRrs>;
///start register
pub mod str;
/**HRA0 (r) register accessor: HASH aliased digest register 0

You can [`read`](crate::Reg::read) this register and get [`hra0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HRA0)

For information about available fields see [`mod@hra0`] module*/
pub type HRA0 = crate::Reg<hra0::HRA0rs>;
///HASH aliased digest register 0
pub mod hra0;
/**HRA1 (r) register accessor: HASH aliased digest register 1

You can [`read`](crate::Reg::read) this register and get [`hra1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HRA1)

For information about available fields see [`mod@hra1`] module*/
pub type HRA1 = crate::Reg<hra1::HRA1rs>;
///HASH aliased digest register 1
pub mod hra1;
/**HRA2 (r) register accessor: HASH aliased digest register 2

You can [`read`](crate::Reg::read) this register and get [`hra2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HRA2)

For information about available fields see [`mod@hra2`] module*/
pub type HRA2 = crate::Reg<hra2::HRA2rs>;
///HASH aliased digest register 2
pub mod hra2;
/**HRA3 (r) register accessor: HASH aliased digest register 3

You can [`read`](crate::Reg::read) this register and get [`hra3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HRA3)

For information about available fields see [`mod@hra3`] module*/
pub type HRA3 = crate::Reg<hra3::HRA3rs>;
///HASH aliased digest register 3
pub mod hra3;
/**HRA4 (r) register accessor: HASH aliased digest register 4

You can [`read`](crate::Reg::read) this register and get [`hra4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HRA4)

For information about available fields see [`mod@hra4`] module*/
pub type HRA4 = crate::Reg<hra4::HRA4rs>;
///HASH aliased digest register 4
pub mod hra4;
/**HR0 (r) register accessor: digest register 0

You can [`read`](crate::Reg::read) this register and get [`hr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HR0)

For information about available fields see [`mod@hr0`] module*/
pub type HR0 = crate::Reg<hr0::HR0rs>;
///digest register 0
pub mod hr0;
/**HR1 (r) register accessor: digest register 1

You can [`read`](crate::Reg::read) this register and get [`hr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HR1)

For information about available fields see [`mod@hr1`] module*/
pub type HR1 = crate::Reg<hr1::HR1rs>;
///digest register 1
pub mod hr1;
/**HR2 (r) register accessor: digest register 4

You can [`read`](crate::Reg::read) this register and get [`hr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HR2)

For information about available fields see [`mod@hr2`] module*/
pub type HR2 = crate::Reg<hr2::HR2rs>;
///digest register 4
pub mod hr2;
/**HR3 (r) register accessor: digest register 3

You can [`read`](crate::Reg::read) this register and get [`hr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HR3)

For information about available fields see [`mod@hr3`] module*/
pub type HR3 = crate::Reg<hr3::HR3rs>;
///digest register 3
pub mod hr3;
/**HR4 (r) register accessor: digest register 4

You can [`read`](crate::Reg::read) this register and get [`hr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HR4)

For information about available fields see [`mod@hr4`] module*/
pub type HR4 = crate::Reg<hr4::HR4rs>;
///digest register 4
pub mod hr4;
/**HR5 (r) register accessor: supplementary digest register 5

You can [`read`](crate::Reg::read) this register and get [`hr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HR5)

For information about available fields see [`mod@hr5`] module*/
pub type HR5 = crate::Reg<hr5::HR5rs>;
///supplementary digest register 5
pub mod hr5;
/**HR6 (r) register accessor: supplementary digest register 6

You can [`read`](crate::Reg::read) this register and get [`hr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HR6)

For information about available fields see [`mod@hr6`] module*/
pub type HR6 = crate::Reg<hr6::HR6rs>;
///supplementary digest register 6
pub mod hr6;
/**HR7 (r) register accessor: supplementary digest register 7

You can [`read`](crate::Reg::read) this register and get [`hr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:HR7)

For information about available fields see [`mod@hr7`] module*/
pub type HR7 = crate::Reg<hr7::HR7rs>;
///supplementary digest register 7
pub mod hr7;
/**IMR (rw) register accessor: interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:IMR)

For information about available fields see [`mod@imr`] module*/
pub type IMR = crate::Reg<imr::IMRrs>;
///interrupt enable register
pub mod imr;
/**SR (rw) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**CSR0 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR0)

For information about available fields see [`mod@csr0`] module*/
pub type CSR0 = crate::Reg<csr0::CSR0rs>;
///context swap registers
pub mod csr0;
/**CSR1 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR1)

For information about available fields see [`mod@csr1`] module*/
pub type CSR1 = crate::Reg<csr1::CSR1rs>;
///context swap registers
pub mod csr1;
/**CSR2 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR2)

For information about available fields see [`mod@csr2`] module*/
pub type CSR2 = crate::Reg<csr2::CSR2rs>;
///context swap registers
pub mod csr2;
/**CSR3 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR3)

For information about available fields see [`mod@csr3`] module*/
pub type CSR3 = crate::Reg<csr3::CSR3rs>;
///context swap registers
pub mod csr3;
/**CSR4 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR4)

For information about available fields see [`mod@csr4`] module*/
pub type CSR4 = crate::Reg<csr4::CSR4rs>;
///context swap registers
pub mod csr4;
/**CSR5 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR5)

For information about available fields see [`mod@csr5`] module*/
pub type CSR5 = crate::Reg<csr5::CSR5rs>;
///context swap registers
pub mod csr5;
/**CSR6 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR6)

For information about available fields see [`mod@csr6`] module*/
pub type CSR6 = crate::Reg<csr6::CSR6rs>;
///context swap registers
pub mod csr6;
/**CSR7 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR7)

For information about available fields see [`mod@csr7`] module*/
pub type CSR7 = crate::Reg<csr7::CSR7rs>;
///context swap registers
pub mod csr7;
/**CSR8 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR8)

For information about available fields see [`mod@csr8`] module*/
pub type CSR8 = crate::Reg<csr8::CSR8rs>;
///context swap registers
pub mod csr8;
/**CSR9 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR9)

For information about available fields see [`mod@csr9`] module*/
pub type CSR9 = crate::Reg<csr9::CSR9rs>;
///context swap registers
pub mod csr9;
/**CSR10 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR10)

For information about available fields see [`mod@csr10`] module*/
pub type CSR10 = crate::Reg<csr10::CSR10rs>;
///context swap registers
pub mod csr10;
/**CSR11 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR11)

For information about available fields see [`mod@csr11`] module*/
pub type CSR11 = crate::Reg<csr11::CSR11rs>;
///context swap registers
pub mod csr11;
/**CSR12 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR12)

For information about available fields see [`mod@csr12`] module*/
pub type CSR12 = crate::Reg<csr12::CSR12rs>;
///context swap registers
pub mod csr12;
/**CSR13 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR13)

For information about available fields see [`mod@csr13`] module*/
pub type CSR13 = crate::Reg<csr13::CSR13rs>;
///context swap registers
pub mod csr13;
/**CSR14 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR14)

For information about available fields see [`mod@csr14`] module*/
pub type CSR14 = crate::Reg<csr14::CSR14rs>;
///context swap registers
pub mod csr14;
/**CSR15 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR15)

For information about available fields see [`mod@csr15`] module*/
pub type CSR15 = crate::Reg<csr15::CSR15rs>;
///context swap registers
pub mod csr15;
/**CSR16 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR16)

For information about available fields see [`mod@csr16`] module*/
pub type CSR16 = crate::Reg<csr16::CSR16rs>;
///context swap registers
pub mod csr16;
/**CSR17 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR17)

For information about available fields see [`mod@csr17`] module*/
pub type CSR17 = crate::Reg<csr17::CSR17rs>;
///context swap registers
pub mod csr17;
/**CSR18 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR18)

For information about available fields see [`mod@csr18`] module*/
pub type CSR18 = crate::Reg<csr18::CSR18rs>;
///context swap registers
pub mod csr18;
/**CSR19 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR19)

For information about available fields see [`mod@csr19`] module*/
pub type CSR19 = crate::Reg<csr19::CSR19rs>;
///context swap registers
pub mod csr19;
/**CSR20 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR20)

For information about available fields see [`mod@csr20`] module*/
pub type CSR20 = crate::Reg<csr20::CSR20rs>;
///context swap registers
pub mod csr20;
/**CSR21 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR21)

For information about available fields see [`mod@csr21`] module*/
pub type CSR21 = crate::Reg<csr21::CSR21rs>;
///context swap registers
pub mod csr21;
/**CSR22 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR22)

For information about available fields see [`mod@csr22`] module*/
pub type CSR22 = crate::Reg<csr22::CSR22rs>;
///context swap registers
pub mod csr22;
/**CSR23 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR23)

For information about available fields see [`mod@csr23`] module*/
pub type CSR23 = crate::Reg<csr23::CSR23rs>;
///context swap registers
pub mod csr23;
/**CSR24 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR24)

For information about available fields see [`mod@csr24`] module*/
pub type CSR24 = crate::Reg<csr24::CSR24rs>;
///context swap registers
pub mod csr24;
/**CSR25 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR25)

For information about available fields see [`mod@csr25`] module*/
pub type CSR25 = crate::Reg<csr25::CSR25rs>;
///context swap registers
pub mod csr25;
/**CSR26 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR26)

For information about available fields see [`mod@csr26`] module*/
pub type CSR26 = crate::Reg<csr26::CSR26rs>;
///context swap registers
pub mod csr26;
/**CSR27 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR27)

For information about available fields see [`mod@csr27`] module*/
pub type CSR27 = crate::Reg<csr27::CSR27rs>;
///context swap registers
pub mod csr27;
/**CSR28 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR28)

For information about available fields see [`mod@csr28`] module*/
pub type CSR28 = crate::Reg<csr28::CSR28rs>;
///context swap registers
pub mod csr28;
/**CSR29 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR29)

For information about available fields see [`mod@csr29`] module*/
pub type CSR29 = crate::Reg<csr29::CSR29rs>;
///context swap registers
pub mod csr29;
/**CSR30 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR30)

For information about available fields see [`mod@csr30`] module*/
pub type CSR30 = crate::Reg<csr30::CSR30rs>;
///context swap registers
pub mod csr30;
/**CSR31 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR31)

For information about available fields see [`mod@csr31`] module*/
pub type CSR31 = crate::Reg<csr31::CSR31rs>;
///context swap registers
pub mod csr31;
/**CSR32 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR32)

For information about available fields see [`mod@csr32`] module*/
pub type CSR32 = crate::Reg<csr32::CSR32rs>;
///context swap registers
pub mod csr32;
/**CSR33 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR33)

For information about available fields see [`mod@csr33`] module*/
pub type CSR33 = crate::Reg<csr33::CSR33rs>;
///context swap registers
pub mod csr33;
/**CSR34 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR34)

For information about available fields see [`mod@csr34`] module*/
pub type CSR34 = crate::Reg<csr34::CSR34rs>;
///context swap registers
pub mod csr34;
/**CSR35 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR35)

For information about available fields see [`mod@csr35`] module*/
pub type CSR35 = crate::Reg<csr35::CSR35rs>;
///context swap registers
pub mod csr35;
/**CSR36 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR36)

For information about available fields see [`mod@csr36`] module*/
pub type CSR36 = crate::Reg<csr36::CSR36rs>;
///context swap registers
pub mod csr36;
/**CSR37 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR37)

For information about available fields see [`mod@csr37`] module*/
pub type CSR37 = crate::Reg<csr37::CSR37rs>;
///context swap registers
pub mod csr37;
/**CSR38 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR38)

For information about available fields see [`mod@csr38`] module*/
pub type CSR38 = crate::Reg<csr38::CSR38rs>;
///context swap registers
pub mod csr38;
/**CSR39 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR39)

For information about available fields see [`mod@csr39`] module*/
pub type CSR39 = crate::Reg<csr39::CSR39rs>;
///context swap registers
pub mod csr39;
/**CSR40 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR40)

For information about available fields see [`mod@csr40`] module*/
pub type CSR40 = crate::Reg<csr40::CSR40rs>;
///context swap registers
pub mod csr40;
/**CSR41 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR41)

For information about available fields see [`mod@csr41`] module*/
pub type CSR41 = crate::Reg<csr41::CSR41rs>;
///context swap registers
pub mod csr41;
/**CSR42 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR42)

For information about available fields see [`mod@csr42`] module*/
pub type CSR42 = crate::Reg<csr42::CSR42rs>;
///context swap registers
pub mod csr42;
/**CSR43 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR43)

For information about available fields see [`mod@csr43`] module*/
pub type CSR43 = crate::Reg<csr43::CSR43rs>;
///context swap registers
pub mod csr43;
/**CSR44 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR44)

For information about available fields see [`mod@csr44`] module*/
pub type CSR44 = crate::Reg<csr44::CSR44rs>;
///context swap registers
pub mod csr44;
/**CSR45 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR45)

For information about available fields see [`mod@csr45`] module*/
pub type CSR45 = crate::Reg<csr45::CSR45rs>;
///context swap registers
pub mod csr45;
/**CSR46 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR46)

For information about available fields see [`mod@csr46`] module*/
pub type CSR46 = crate::Reg<csr46::CSR46rs>;
///context swap registers
pub mod csr46;
/**CSR47 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR47)

For information about available fields see [`mod@csr47`] module*/
pub type CSR47 = crate::Reg<csr47::CSR47rs>;
///context swap registers
pub mod csr47;
/**CSR48 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR48)

For information about available fields see [`mod@csr48`] module*/
pub type CSR48 = crate::Reg<csr48::CSR48rs>;
///context swap registers
pub mod csr48;
/**CSR49 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR49)

For information about available fields see [`mod@csr49`] module*/
pub type CSR49 = crate::Reg<csr49::CSR49rs>;
///context swap registers
pub mod csr49;
/**CSR50 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR50)

For information about available fields see [`mod@csr50`] module*/
pub type CSR50 = crate::Reg<csr50::CSR50rs>;
///context swap registers
pub mod csr50;
/**CSR51 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR51)

For information about available fields see [`mod@csr51`] module*/
pub type CSR51 = crate::Reg<csr51::CSR51rs>;
///context swap registers
pub mod csr51;
/**CSR52 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR52)

For information about available fields see [`mod@csr52`] module*/
pub type CSR52 = crate::Reg<csr52::CSR52rs>;
///context swap registers
pub mod csr52;
/**CSR53 (rw) register accessor: context swap registers

You can [`read`](crate::Reg::read) this register and get [`csr53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HASH:CSR53)

For information about available fields see [`mod@csr53`] module*/
pub type CSR53 = crate::Reg<csr53::CSR53rs>;
///context swap registers
pub mod csr53;
