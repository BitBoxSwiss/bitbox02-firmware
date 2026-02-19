#[repr(C)]
#[derive(Debug)]
///Cluster FLT%s, containing SITF?CR, BSMX?CR, DFLT?CR, DFLT?CICR, DFLT?RSFR, DFLT?INTR, OLD?CR, OLD?THLR, OLD?THHR, DLY?CR, SCD?CR, DFLT?IER, DFLT?ISR, OEC?CR, SNPS?DR, DFLT?DR
pub struct FLT {
    sitfcr: SITFCR,
    bsmxcr: BSMXCR,
    dfltcr: DFLTCR,
    dfltcicr: DFLTCICR,
    dfltrsfr: DFLTRSFR,
    dfltintr: DFLTINTR,
    oldcr: OLDCR,
    oldthlr: OLDTHLR,
    oldthhr: OLDTHHR,
    dlycr: DLYCR,
    scdcr: SCDCR,
    dfltier: DFLTIER,
    dfltisr: DFLTISR,
    oeccr: OECCR,
    _reserved14: [u8; 0x34],
    snpsdr: SNPSDR,
    dfltdr: DFLTDR,
    _reserved_end: [u8; 0x0c],
}
impl FLT {
    ///0x00 - This register is used to control the serial interfaces (SITFx).
    #[inline(always)]
    pub const fn sitfcr(&self) -> &SITFCR {
        &self.sitfcr
    }
    ///0x04 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    #[inline(always)]
    pub const fn bsmxcr(&self) -> &BSMXCR {
        &self.bsmxcr
    }
    ///0x08 - This register is used to control the digital filter x.
    #[inline(always)]
    pub const fn dfltcr(&self) -> &DFLTCR {
        &self.dfltcr
    }
    ///0x0c - This register is used to control the main CIC filter.
    #[inline(always)]
    pub const fn dfltcicr(&self) -> &DFLTCICR {
        &self.dfltcicr
    }
    ///0x10 - This register is used to control the reshape and HPF filters.
    #[inline(always)]
    pub const fn dfltrsfr(&self) -> &DFLTRSFR {
        &self.dfltrsfr
    }
    ///0x14 - This register is used to the integrator (INT) settings.
    #[inline(always)]
    pub const fn dfltintr(&self) -> &DFLTINTR {
        &self.dfltintr
    }
    ///0x18 - This register is used to configure the Out-of Limit Detector function.
    #[inline(always)]
    pub const fn oldcr(&self) -> &OLDCR {
        &self.oldcr
    }
    ///0x1c - This register is used for the adjustment of the Out-off Limit low threshold.
    #[inline(always)]
    pub const fn oldthlr(&self) -> &OLDTHLR {
        &self.oldthlr
    }
    ///0x20 - This register is used for the adjustment of the Out-off Limit high threshold.
    #[inline(always)]
    pub const fn oldthhr(&self) -> &OLDTHHR {
        &self.oldthhr
    }
    ///0x24 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn dlycr(&self) -> &DLYCR {
        &self.dlycr
    }
    ///0x28 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn scdcr(&self) -> &SCDCR {
        &self.scdcr
    }
    ///0x2c - This register is used for allowing or not the events to generate an interrupt.
    #[inline(always)]
    pub const fn dfltier(&self) -> &DFLTIER {
        &self.dfltier
    }
    ///0x30 - MDF DFLT0 interrupt status register 0
    #[inline(always)]
    pub const fn dfltisr(&self) -> &DFLTISR {
        &self.dfltisr
    }
    ///0x34 - This register contains the offset compensation value.
    #[inline(always)]
    pub const fn oeccr(&self) -> &OECCR {
        &self.oeccr
    }
    ///0x6c - This register is used to read the data processed by each digital filter in snapshot mode.
    #[inline(always)]
    pub const fn snpsdr(&self) -> &SNPSDR {
        &self.snpsdr
    }
    ///0x70 - This register is used to read the data processed by each digital filter.
    #[inline(always)]
    pub const fn dfltdr(&self) -> &DFLTDR {
        &self.dfltdr
    }
}
/**SITFCR (rw) register accessor: This register is used to control the serial interfaces (SITFx).

You can [`read`](crate::Reg::read) this register and get [`sitfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sitfcr`] module*/
pub type SITFCR = crate::Reg<sitfcr::SITFCRrs>;
///This register is used to control the serial interfaces (SITFx).
pub mod sitfcr;
/**BSMXCR (rw) register accessor: This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.

You can [`read`](crate::Reg::read) this register and get [`bsmxcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsmxcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bsmxcr`] module*/
pub type BSMXCR = crate::Reg<bsmxcr::BSMXCRrs>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod bsmxcr;
/**DFLTCR (rw) register accessor: This register is used to control the digital filter x.

You can [`read`](crate::Reg::read) this register and get [`dfltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfltcr`] module*/
pub type DFLTCR = crate::Reg<dfltcr::DFLTCRrs>;
///This register is used to control the digital filter x.
pub mod dfltcr;
/**DFLTCICR (rw) register accessor: This register is used to control the main CIC filter.

You can [`read`](crate::Reg::read) this register and get [`dfltcicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltcicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfltcicr`] module*/
pub type DFLTCICR = crate::Reg<dfltcicr::DFLTCICRrs>;
///This register is used to control the main CIC filter.
pub mod dfltcicr;
/**DFLTRSFR (rw) register accessor: This register is used to control the reshape and HPF filters.

You can [`read`](crate::Reg::read) this register and get [`dfltrsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltrsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfltrsfr`] module*/
pub type DFLTRSFR = crate::Reg<dfltrsfr::DFLTRSFRrs>;
///This register is used to control the reshape and HPF filters.
pub mod dfltrsfr;
/**DFLTINTR (rw) register accessor: This register is used to the integrator (INT) settings.

You can [`read`](crate::Reg::read) this register and get [`dfltintr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltintr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfltintr`] module*/
pub type DFLTINTR = crate::Reg<dfltintr::DFLTINTRrs>;
///This register is used to the integrator (INT) settings.
pub mod dfltintr;
/**OLDCR (rw) register accessor: This register is used to configure the Out-of Limit Detector function.

You can [`read`](crate::Reg::read) this register and get [`oldcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oldcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@oldcr`] module*/
pub type OLDCR = crate::Reg<oldcr::OLDCRrs>;
///This register is used to configure the Out-of Limit Detector function.
pub mod oldcr;
/**OLDTHLR (rw) register accessor: This register is used for the adjustment of the Out-off Limit low threshold.

You can [`read`](crate::Reg::read) this register and get [`oldthlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oldthlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@oldthlr`] module*/
pub type OLDTHLR = crate::Reg<oldthlr::OLDTHLRrs>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod oldthlr;
/**OLDTHHR (rw) register accessor: This register is used for the adjustment of the Out-off Limit high threshold.

You can [`read`](crate::Reg::read) this register and get [`oldthhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oldthhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@oldthhr`] module*/
pub type OLDTHHR = crate::Reg<oldthhr::OLDTHHRrs>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod oldthhr;
/**DLYCR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`dlycr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlycr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dlycr`] module*/
pub type DLYCR = crate::Reg<dlycr::DLYCRrs>;
///This register is used for the adjustment stream delays.
pub mod dlycr;
/**SCDCR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`scdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scdcr`] module*/
pub type SCDCR = crate::Reg<scdcr::SCDCRrs>;
///This register is used for the adjustment stream delays.
pub mod scdcr;
/**DFLTIER (rw) register accessor: This register is used for allowing or not the events to generate an interrupt.

You can [`read`](crate::Reg::read) this register and get [`dfltier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfltier`] module*/
pub type DFLTIER = crate::Reg<dfltier::DFLTIERrs>;
///This register is used for allowing or not the events to generate an interrupt.
pub mod dfltier;
/**DFLTISR (rw) register accessor: MDF DFLT0 interrupt status register 0

You can [`read`](crate::Reg::read) this register and get [`dfltisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfltisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfltisr`] module*/
pub type DFLTISR = crate::Reg<dfltisr::DFLTISRrs>;
///MDF DFLT0 interrupt status register 0
pub mod dfltisr;
/**OECCR (rw) register accessor: This register contains the offset compensation value.

You can [`read`](crate::Reg::read) this register and get [`oeccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oeccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@oeccr`] module*/
pub type OECCR = crate::Reg<oeccr::OECCRrs>;
///This register contains the offset compensation value.
pub mod oeccr;
/**SNPSDR (r) register accessor: This register is used to read the data processed by each digital filter in snapshot mode.

You can [`read`](crate::Reg::read) this register and get [`snpsdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@snpsdr`] module*/
pub type SNPSDR = crate::Reg<snpsdr::SNPSDRrs>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod snpsdr;
/**DFLTDR (r) register accessor: This register is used to read the data processed by each digital filter.

You can [`read`](crate::Reg::read) this register and get [`dfltdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfltdr`] module*/
pub type DFLTDR = crate::Reg<dfltdr::DFLTDRrs>;
///This register is used to read the data processed by each digital filter.
pub mod dfltdr;
