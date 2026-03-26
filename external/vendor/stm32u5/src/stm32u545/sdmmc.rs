#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    power: POWER,
    clkcr: CLKCR,
    argr: ARGR,
    cmdr: CMDR,
    respcmdr: RESPCMDR,
    resp: [RESP; 4],
    dtimer: DTIMER,
    dlenr: DLENR,
    dctrl: DCTRL,
    dcntr: DCNTR,
    star: STAR,
    icr: ICR,
    maskr: MASKR,
    acktimer: ACKTIMER,
    _reserved14: [u8; 0x0c],
    idmactrlr: IDMACTRLR,
    idmabsizer: IDMABSIZER,
    idmabaser: IDMABASER,
    _reserved17: [u8; 0x08],
    idmalar: IDMALAR,
    idmabar: IDMABAR,
    _reserved19: [u8; 0x14],
    fifor: [FIFOR; 16],
}
impl RegisterBlock {
    ///0x00 - power control register
    #[inline(always)]
    pub const fn power(&self) -> &POWER {
        &self.power
    }
    ///0x04 - clock control register
    #[inline(always)]
    pub const fn clkcr(&self) -> &CLKCR {
        &self.clkcr
    }
    ///0x08 - argument register
    #[inline(always)]
    pub const fn argr(&self) -> &ARGR {
        &self.argr
    }
    ///0x0c - command register
    #[inline(always)]
    pub const fn cmdr(&self) -> &CMDR {
        &self.cmdr
    }
    ///0x10 - command response register
    #[inline(always)]
    pub const fn respcmdr(&self) -> &RESPCMDR {
        &self.respcmdr
    }
    ///0x14..0x24 - SDIO response %s register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `RESP1` register.</div>
    #[inline(always)]
    pub const fn resp(&self, n: usize) -> &RESP {
        &self.resp[n]
    }
    ///Iterator for array of:
    ///0x14..0x24 - SDIO response %s register
    #[inline(always)]
    pub fn resp_iter(&self) -> impl Iterator<Item = &RESP> {
        self.resp.iter()
    }
    ///0x14 - SDIO response 1 register
    #[inline(always)]
    pub const fn resp1(&self) -> &RESP {
        self.resp(0)
    }
    ///0x18 - SDIO response 2 register
    #[inline(always)]
    pub const fn resp2(&self) -> &RESP {
        self.resp(1)
    }
    ///0x1c - SDIO response 3 register
    #[inline(always)]
    pub const fn resp3(&self) -> &RESP {
        self.resp(2)
    }
    ///0x20 - SDIO response 4 register
    #[inline(always)]
    pub const fn resp4(&self) -> &RESP {
        self.resp(3)
    }
    ///0x24 - data timer register
    #[inline(always)]
    pub const fn dtimer(&self) -> &DTIMER {
        &self.dtimer
    }
    ///0x28 - data length register
    #[inline(always)]
    pub const fn dlenr(&self) -> &DLENR {
        &self.dlenr
    }
    ///0x2c - data control register
    #[inline(always)]
    pub const fn dctrl(&self) -> &DCTRL {
        &self.dctrl
    }
    ///0x30 - data counter register
    #[inline(always)]
    pub const fn dcntr(&self) -> &DCNTR {
        &self.dcntr
    }
    ///0x34 - status register
    #[inline(always)]
    pub const fn star(&self) -> &STAR {
        &self.star
    }
    ///0x38 - interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x3c - mask register
    #[inline(always)]
    pub const fn maskr(&self) -> &MASKR {
        &self.maskr
    }
    ///0x40 - acknowledgment timer register
    #[inline(always)]
    pub const fn acktimer(&self) -> &ACKTIMER {
        &self.acktimer
    }
    ///0x50 - DMA control register
    #[inline(always)]
    pub const fn idmactrlr(&self) -> &IDMACTRLR {
        &self.idmactrlr
    }
    ///0x54 - buffer size register
    #[inline(always)]
    pub const fn idmabsizer(&self) -> &IDMABSIZER {
        &self.idmabsizer
    }
    ///0x58 - buffer base address register
    #[inline(always)]
    pub const fn idmabaser(&self) -> &IDMABASER {
        &self.idmabaser
    }
    ///0x64 - linked list address register
    #[inline(always)]
    pub const fn idmalar(&self) -> &IDMALAR {
        &self.idmalar
    }
    ///0x68 - linked list memory base register
    #[inline(always)]
    pub const fn idmabar(&self) -> &IDMABAR {
        &self.idmabar
    }
    ///0x80..0xc0 - data FIFO register %s
    #[inline(always)]
    pub const fn fifor(&self, n: usize) -> &FIFOR {
        &self.fifor[n]
    }
    ///Iterator for array of:
    ///0x80..0xc0 - data FIFO register %s
    #[inline(always)]
    pub fn fifor_iter(&self) -> impl Iterator<Item = &FIFOR> {
        self.fifor.iter()
    }
}
/**POWER (rw) register accessor: power control register

You can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:POWER)

For information about available fields see [`mod@power`] module*/
pub type POWER = crate::Reg<power::POWERrs>;
///power control register
pub mod power;
/**CLKCR (rw) register accessor: clock control register

You can [`read`](crate::Reg::read) this register and get [`clkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:CLKCR)

For information about available fields see [`mod@clkcr`] module*/
pub type CLKCR = crate::Reg<clkcr::CLKCRrs>;
///clock control register
pub mod clkcr;
/**ARGR (rw) register accessor: argument register

You can [`read`](crate::Reg::read) this register and get [`argr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:ARGR)

For information about available fields see [`mod@argr`] module*/
pub type ARGR = crate::Reg<argr::ARGRrs>;
///argument register
pub mod argr;
/**CMDR (rw) register accessor: command register

You can [`read`](crate::Reg::read) this register and get [`cmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:CMDR)

For information about available fields see [`mod@cmdr`] module*/
pub type CMDR = crate::Reg<cmdr::CMDRrs>;
///command register
pub mod cmdr;
/**RESPCMDR (r) register accessor: command response register

You can [`read`](crate::Reg::read) this register and get [`respcmdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:RESPCMDR)

For information about available fields see [`mod@respcmdr`] module*/
pub type RESPCMDR = crate::Reg<respcmdr::RESPCMDRrs>;
///command response register
pub mod respcmdr;
/**RESP (r) register accessor: SDIO response %s register

You can [`read`](crate::Reg::read) this register and get [`resp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:RESP[1])

For information about available fields see [`mod@resp`] module*/
pub type RESP = crate::Reg<resp::RESPrs>;
///SDIO response %s register
pub mod resp;
/**DTIMER (rw) register accessor: data timer register

You can [`read`](crate::Reg::read) this register and get [`dtimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:DTIMER)

For information about available fields see [`mod@dtimer`] module*/
pub type DTIMER = crate::Reg<dtimer::DTIMERrs>;
///data timer register
pub mod dtimer;
/**DLENR (rw) register accessor: data length register

You can [`read`](crate::Reg::read) this register and get [`dlenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:DLENR)

For information about available fields see [`mod@dlenr`] module*/
pub type DLENR = crate::Reg<dlenr::DLENRrs>;
///data length register
pub mod dlenr;
/**DCTRL (rw) register accessor: data control register

You can [`read`](crate::Reg::read) this register and get [`dctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:DCTRL)

For information about available fields see [`mod@dctrl`] module*/
pub type DCTRL = crate::Reg<dctrl::DCTRLrs>;
///data control register
pub mod dctrl;
/**DCNTR (r) register accessor: data counter register

You can [`read`](crate::Reg::read) this register and get [`dcntr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:DCNTR)

For information about available fields see [`mod@dcntr`] module*/
pub type DCNTR = crate::Reg<dcntr::DCNTRrs>;
///data counter register
pub mod dcntr;
/**STAR (r) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`star::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:STAR)

For information about available fields see [`mod@star`] module*/
pub type STAR = crate::Reg<star::STARrs>;
///status register
pub mod star;
/**ICR (rw) register accessor: interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///interrupt clear register
pub mod icr;
/**MASKR (rw) register accessor: mask register

You can [`read`](crate::Reg::read) this register and get [`maskr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:MASKR)

For information about available fields see [`mod@maskr`] module*/
pub type MASKR = crate::Reg<maskr::MASKRrs>;
///mask register
pub mod maskr;
/**ACKTIMER (rw) register accessor: acknowledgment timer register

You can [`read`](crate::Reg::read) this register and get [`acktimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acktimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:ACKTIMER)

For information about available fields see [`mod@acktimer`] module*/
pub type ACKTIMER = crate::Reg<acktimer::ACKTIMERrs>;
///acknowledgment timer register
pub mod acktimer;
/**FIFOR (rw) register accessor: data FIFO register %s

You can [`read`](crate::Reg::read) this register and get [`fifor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:FIFOR[0])

For information about available fields see [`mod@fifor`] module*/
pub type FIFOR = crate::Reg<fifor::FIFORrs>;
///data FIFO register %s
pub mod fifor;
/**IDMACTRLR (rw) register accessor: DMA control register

You can [`read`](crate::Reg::read) this register and get [`idmactrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmactrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:IDMACTRLR)

For information about available fields see [`mod@idmactrlr`] module*/
pub type IDMACTRLR = crate::Reg<idmactrlr::IDMACTRLRrs>;
///DMA control register
pub mod idmactrlr;
/**IDMABSIZER (rw) register accessor: buffer size register

You can [`read`](crate::Reg::read) this register and get [`idmabsizer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabsizer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:IDMABSIZER)

For information about available fields see [`mod@idmabsizer`] module*/
pub type IDMABSIZER = crate::Reg<idmabsizer::IDMABSIZERrs>;
///buffer size register
pub mod idmabsizer;
/**IDMABASER (rw) register accessor: buffer base address register

You can [`read`](crate::Reg::read) this register and get [`idmabaser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabaser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:IDMABASER)

For information about available fields see [`mod@idmabaser`] module*/
pub type IDMABASER = crate::Reg<idmabaser::IDMABASERrs>;
///buffer base address register
pub mod idmabaser;
/**IDMALAR (rw) register accessor: linked list address register

You can [`read`](crate::Reg::read) this register and get [`idmalar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmalar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:IDMALAR)

For information about available fields see [`mod@idmalar`] module*/
pub type IDMALAR = crate::Reg<idmalar::IDMALARrs>;
///linked list address register
pub mod idmalar;
/**IDMABAR (rw) register accessor: linked list memory base register

You can [`read`](crate::Reg::read) this register and get [`idmabar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:IDMABAR)

For information about available fields see [`mod@idmabar`] module*/
pub type IDMABAR = crate::Reg<idmabar::IDMABARrs>;
///linked list memory base register
pub mod idmabar;
