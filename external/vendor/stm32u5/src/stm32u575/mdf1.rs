#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gcr: GCR,
    ckgcr: CKGCR,
    _reserved2: [u8; 0x78],
    flt: [FLT; 6],
}
impl RegisterBlock {
    ///0x00 - MDF global control register
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    ///0x04 - MDF clock generator control register
    #[inline(always)]
    pub const fn ckgcr(&self) -> &CKGCR {
        &self.ckgcr
    }
    ///0x80..0x380 - Cluster FLT%s, containing SITF?CR, BSMX?CR, DFLT?CR, DFLT?CICR, DFLT?RSFR, DFLT?INTR, OLD?CR, OLD?THLR, OLD?THHR, DLY?CR, SCD?CR, DFLT?IER, DFLT?ISR, OEC?CR, SNPS?DR, DFLT?DR
    #[inline(always)]
    pub const fn flt(&self, n: usize) -> &FLT {
        &self.flt[n]
    }
    ///Iterator for array of:
    ///0x80..0x380 - Cluster FLT%s, containing SITF?CR, BSMX?CR, DFLT?CR, DFLT?CICR, DFLT?RSFR, DFLT?INTR, OLD?CR, OLD?THLR, OLD?THHR, DLY?CR, SCD?CR, DFLT?IER, DFLT?ISR, OEC?CR, SNPS?DR, DFLT?DR
    #[inline(always)]
    pub fn flt_iter(&self) -> impl Iterator<Item = &FLT> {
        self.flt.iter()
    }
}
/**GCR (rw) register accessor: MDF global control register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#MDF1:GCR)

For information about available fields see [`mod@gcr`] module*/
pub type GCR = crate::Reg<gcr::GCRrs>;
///MDF global control register
pub mod gcr;
/**CKGCR (rw) register accessor: MDF clock generator control register

You can [`read`](crate::Reg::read) this register and get [`ckgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#MDF1:CKGCR)

For information about available fields see [`mod@ckgcr`] module*/
pub type CKGCR = crate::Reg<ckgcr::CKGCRrs>;
///MDF clock generator control register
pub mod ckgcr;
///Cluster FLT%s, containing SITF?CR, BSMX?CR, DFLT?CR, DFLT?CICR, DFLT?RSFR, DFLT?INTR, OLD?CR, OLD?THLR, OLD?THHR, DLY?CR, SCD?CR, DFLT?IER, DFLT?ISR, OEC?CR, SNPS?DR, DFLT?DR
pub use self::flt::FLT;
///Cluster
///Cluster FLT%s, containing SITF?CR, BSMX?CR, DFLT?CR, DFLT?CICR, DFLT?RSFR, DFLT?INTR, OLD?CR, OLD?THLR, OLD?THHR, DLY?CR, SCD?CR, DFLT?IER, DFLT?ISR, OEC?CR, SNPS?DR, DFLT?DR
pub mod flt;
