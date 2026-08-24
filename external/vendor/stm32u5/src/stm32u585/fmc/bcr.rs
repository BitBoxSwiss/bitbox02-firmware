///Register `BCR%s` reader
pub type R = crate::R<BCRrs>;
///Register `BCR%s` writer
pub type W = crate::W<BCRrs>;
///Wait signal during asynchronous transfers
pub use super::bcr1::ASYNCWAIT;
///Field `ASYNCWAIT` reader - Wait signal during asynchronous transfers
pub use super::bcr1::ASYNCWAIT_R;
///Field `ASYNCWAIT` writer - Wait signal during asynchronous transfers
pub use super::bcr1::ASYNCWAIT_W;
///Burst enable bit
pub use super::bcr1::BURSTEN;
///Field `BURSTEN` reader - Burst enable bit
pub use super::bcr1::BURSTEN_R;
///Field `BURSTEN` writer - Burst enable bit
pub use super::bcr1::BURSTEN_W;
///Write burst enable
pub use super::bcr1::CBURSTRW;
///Field `CBURSTRW` reader - Write burst enable
pub use super::bcr1::CBURSTRW_R;
///Field `CBURSTRW` writer - Write burst enable
pub use super::bcr1::CBURSTRW_W;
///CRAM Page Size
pub use super::bcr1::CPSIZE;
///Field `CPSIZE` reader - CRAM Page Size
pub use super::bcr1::CPSIZE_R;
///Field `CPSIZE` writer - CRAM Page Size
pub use super::bcr1::CPSIZE_W;
///Extended mode enable
pub use super::bcr1::EXTMOD;
///Field `EXTMOD` reader - Extended mode enable
pub use super::bcr1::EXTMOD_R;
///Field `EXTMOD` writer - Extended mode enable
pub use super::bcr1::EXTMOD_W;
///Flash access enable
pub use super::bcr1::FACCEN;
///Field `FACCEN` reader - Flash access enable
pub use super::bcr1::FACCEN_R;
///Field `FACCEN` writer - Flash access enable
pub use super::bcr1::FACCEN_W;
///Memory bank enable bit
pub use super::bcr1::MBKEN;
///Field `MBKEN` reader - Memory bank enable bit
pub use super::bcr1::MBKEN_R;
///Field `MBKEN` writer - Memory bank enable bit
pub use super::bcr1::MBKEN_W;
///Memory type
pub use super::bcr1::MTYP;
///Field `MTYP` reader - Memory type
pub use super::bcr1::MTYP_R;
///Field `MTYP` writer - Memory type
pub use super::bcr1::MTYP_W;
///Address/data multiplexing enable bit
pub use super::bcr1::MUXEN;
///Field `MUXEN` reader - Address/data multiplexing enable bit
pub use super::bcr1::MUXEN_R;
///Field `MUXEN` writer - Address/data multiplexing enable bit
pub use super::bcr1::MUXEN_W;
///Memory data bus width
pub use super::bcr1::MWID;
///Field `MWID` reader - Memory data bus width
pub use super::bcr1::MWID_R;
///Field `MWID` writer - Memory data bus width
pub use super::bcr1::MWID_W;
///Wait timing configuration
pub use super::bcr1::WAITCFG;
///Field `WAITCFG` reader - Wait timing configuration
pub use super::bcr1::WAITCFG_R;
///Field `WAITCFG` writer - Wait timing configuration
pub use super::bcr1::WAITCFG_W;
///Wait enable bit
pub use super::bcr1::WAITEN;
///Field `WAITEN` reader - Wait enable bit
pub use super::bcr1::WAITEN_R;
///Field `WAITEN` writer - Wait enable bit
pub use super::bcr1::WAITEN_W;
///Wait signal polarity bit
pub use super::bcr1::WAITPOL;
///Field `WAITPOL` reader - Wait signal polarity bit
pub use super::bcr1::WAITPOL_R;
///Field `WAITPOL` writer - Wait signal polarity bit
pub use super::bcr1::WAITPOL_W;
///Write enable bit
pub use super::bcr1::WREN;
///Field `WREN` reader - Write enable bit
pub use super::bcr1::WREN_R;
///Field `WREN` writer - Write enable bit
pub use super::bcr1::WREN_W;
///Field `NBLSET` reader - Byte lane (NBL) setup
pub type NBLSET_R = crate::FieldReader;
///Field `NBLSET` writer - Byte lane (NBL) setup
pub type NBLSET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Memory bank enable bit
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Address/data multiplexing enable bit
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Memory type
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Flash access enable
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Burst enable bit
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wait signal polarity bit
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Wait timing configuration
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Write enable bit
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Wait enable bit
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Extended mode enable
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Wait signal during asynchronous transfers
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - CRAM Page Size
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - Write burst enable
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 22:23 - Byte lane (NBL) setup
    #[inline(always)]
    pub fn nblset(&self) -> NBLSET_R {
        NBLSET_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR")
            .field("mbken", &self.mbken())
            .field("muxen", &self.muxen())
            .field("mtyp", &self.mtyp())
            .field("mwid", &self.mwid())
            .field("faccen", &self.faccen())
            .field("bursten", &self.bursten())
            .field("waitpol", &self.waitpol())
            .field("waitcfg", &self.waitcfg())
            .field("wren", &self.wren())
            .field("waiten", &self.waiten())
            .field("extmod", &self.extmod())
            .field("asyncwait", &self.asyncwait())
            .field("cpsize", &self.cpsize())
            .field("cburstrw", &self.cburstrw())
            .field("nblset", &self.nblset())
            .finish()
    }
}
impl W {
    ///Bit 0 - Memory bank enable bit
    #[inline(always)]
    pub fn mbken(&mut self) -> MBKEN_W<BCRrs> {
        MBKEN_W::new(self, 0)
    }
    ///Bit 1 - Address/data multiplexing enable bit
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W<BCRrs> {
        MUXEN_W::new(self, 1)
    }
    ///Bits 2:3 - Memory type
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W<BCRrs> {
        MTYP_W::new(self, 2)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<BCRrs> {
        MWID_W::new(self, 4)
    }
    ///Bit 6 - Flash access enable
    #[inline(always)]
    pub fn faccen(&mut self) -> FACCEN_W<BCRrs> {
        FACCEN_W::new(self, 6)
    }
    ///Bit 8 - Burst enable bit
    #[inline(always)]
    pub fn bursten(&mut self) -> BURSTEN_W<BCRrs> {
        BURSTEN_W::new(self, 8)
    }
    ///Bit 9 - Wait signal polarity bit
    #[inline(always)]
    pub fn waitpol(&mut self) -> WAITPOL_W<BCRrs> {
        WAITPOL_W::new(self, 9)
    }
    ///Bit 11 - Wait timing configuration
    #[inline(always)]
    pub fn waitcfg(&mut self) -> WAITCFG_W<BCRrs> {
        WAITCFG_W::new(self, 11)
    }
    ///Bit 12 - Write enable bit
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<BCRrs> {
        WREN_W::new(self, 12)
    }
    ///Bit 13 - Wait enable bit
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W<BCRrs> {
        WAITEN_W::new(self, 13)
    }
    ///Bit 14 - Extended mode enable
    #[inline(always)]
    pub fn extmod(&mut self) -> EXTMOD_W<BCRrs> {
        EXTMOD_W::new(self, 14)
    }
    ///Bit 15 - Wait signal during asynchronous transfers
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<BCRrs> {
        ASYNCWAIT_W::new(self, 15)
    }
    ///Bits 16:18 - CRAM Page Size
    #[inline(always)]
    pub fn cpsize(&mut self) -> CPSIZE_W<BCRrs> {
        CPSIZE_W::new(self, 16)
    }
    ///Bit 19 - Write burst enable
    #[inline(always)]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<BCRrs> {
        CBURSTRW_W::new(self, 19)
    }
    ///Bits 22:23 - Byte lane (NBL) setup
    #[inline(always)]
    pub fn nblset(&mut self) -> NBLSET_W<BCRrs> {
        NBLSET_W::new(self, 22)
    }
}
/**SRAM/NOR-Flash chip-select control register for bank %s

You can [`read`](crate::Reg::read) this register and get [`bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FMC:BCR[2])*/
pub struct BCRrs;
impl crate::RegisterSpec for BCRrs {
    type Ux = u32;
}
///`read()` method returns [`bcr::R`](R) reader structure
impl crate::Readable for BCRrs {}
///`write(|w| ..)` method takes [`bcr::W`](W) writer structure
impl crate::Writable for BCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCR%s to value 0x30d2
impl crate::Resettable for BCRrs {
    const RESET_VALUE: u32 = 0x30d2;
}
