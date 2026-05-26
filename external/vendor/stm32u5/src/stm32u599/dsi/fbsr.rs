///Register `FBSR` reader
pub type R = crate::R<FBSRrs>;
///Field `VCWFE` reader - Video mode command write FIFO empty This bit indicates the empty status of the video mode write command FIFO:
pub type VCWFE_R = crate::BitReader;
///Field `VCWFF` reader - Video mode command write FIFO full This bit indicates the full status of the video mode write command FIFO:
pub type VCWFF_R = crate::BitReader;
///Field `VPWFE` reader - Video mode payload write FIFO empty This bit indicates the empty status of the video mode write payload FIFO:
pub type VPWFE_R = crate::BitReader;
///Field `VPWFF` reader - Video mode payload write FIFO full This bit indicates the full status of the video mode write payload FIFO:
pub type VPWFF_R = crate::BitReader;
///Field `ACWFE` reader - Adapted command mode command write FIFO empty This bit indicates the empty status of the adapted command mode write command FIFO:
pub type ACWFE_R = crate::BitReader;
///Field `ACWFF` reader - Adapted command mode command write FIFO full This bit indicates the full status of the adapted command mode write command FIFO:
pub type ACWFF_R = crate::BitReader;
///Field `APWFE` reader - Adapted command mode payload write FIFO empty This bit indicates the empty status of the adapted command mode write payload FIFO:
pub type APWFE_R = crate::BitReader;
///Field `APWFF` reader - Adapted command mode payload write FIFO full This bit indicates the full status of the adapted command mode write payload FIFO:
pub type APWFF_R = crate::BitReader;
///Field `VPBE` reader - Video mode payload buffer empty This bit indicates the empty status of the video mode payload internal buffer:
pub type VPBE_R = crate::BitReader;
///Field `VPBF` reader - Video mode payload buffer full This bit indicates the full status of the video mode payload internal buffer:
pub type VPBF_R = crate::BitReader;
///Field `ACBE` reader - Adapted command mode command buffer empty This bit indicates the empty status of the adapted command mode command internal buffer:
pub type ACBE_R = crate::BitReader;
///Field `ACBF` reader - Adapted command mode command buffer full This bit indicates the full status of the adapted command mode command internal buffer:
pub type ACBF_R = crate::BitReader;
///Field `APBE` reader - Adapted command mode payload buffer empty This bit indicates the empty status of the adapted command mode payload internal buffer:
pub type APBE_R = crate::BitReader;
///Field `APBF` reader - Adapted command mode payload buffer full This bit indicates the full status of the adapted command mode payload internal buffer:
pub type APBF_R = crate::BitReader;
impl R {
    ///Bit 0 - Video mode command write FIFO empty This bit indicates the empty status of the video mode write command FIFO:
    #[inline(always)]
    pub fn vcwfe(&self) -> VCWFE_R {
        VCWFE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Video mode command write FIFO full This bit indicates the full status of the video mode write command FIFO:
    #[inline(always)]
    pub fn vcwff(&self) -> VCWFF_R {
        VCWFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Video mode payload write FIFO empty This bit indicates the empty status of the video mode write payload FIFO:
    #[inline(always)]
    pub fn vpwfe(&self) -> VPWFE_R {
        VPWFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Video mode payload write FIFO full This bit indicates the full status of the video mode write payload FIFO:
    #[inline(always)]
    pub fn vpwff(&self) -> VPWFF_R {
        VPWFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Adapted command mode command write FIFO empty This bit indicates the empty status of the adapted command mode write command FIFO:
    #[inline(always)]
    pub fn acwfe(&self) -> ACWFE_R {
        ACWFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Adapted command mode command write FIFO full This bit indicates the full status of the adapted command mode write command FIFO:
    #[inline(always)]
    pub fn acwff(&self) -> ACWFF_R {
        ACWFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Adapted command mode payload write FIFO empty This bit indicates the empty status of the adapted command mode write payload FIFO:
    #[inline(always)]
    pub fn apwfe(&self) -> APWFE_R {
        APWFE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Adapted command mode payload write FIFO full This bit indicates the full status of the adapted command mode write payload FIFO:
    #[inline(always)]
    pub fn apwff(&self) -> APWFF_R {
        APWFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Video mode payload buffer empty This bit indicates the empty status of the video mode payload internal buffer:
    #[inline(always)]
    pub fn vpbe(&self) -> VPBE_R {
        VPBE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Video mode payload buffer full This bit indicates the full status of the video mode payload internal buffer:
    #[inline(always)]
    pub fn vpbf(&self) -> VPBF_R {
        VPBF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Adapted command mode command buffer empty This bit indicates the empty status of the adapted command mode command internal buffer:
    #[inline(always)]
    pub fn acbe(&self) -> ACBE_R {
        ACBE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Adapted command mode command buffer full This bit indicates the full status of the adapted command mode command internal buffer:
    #[inline(always)]
    pub fn acbf(&self) -> ACBF_R {
        ACBF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Adapted command mode payload buffer empty This bit indicates the empty status of the adapted command mode payload internal buffer:
    #[inline(always)]
    pub fn apbe(&self) -> APBE_R {
        APBE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Adapted command mode payload buffer full This bit indicates the full status of the adapted command mode payload internal buffer:
    #[inline(always)]
    pub fn apbf(&self) -> APBF_R {
        APBF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBSR")
            .field("vcwfe", &self.vcwfe())
            .field("vcwff", &self.vcwff())
            .field("vpwfe", &self.vpwfe())
            .field("vpwff", &self.vpwff())
            .field("acwfe", &self.acwfe())
            .field("acwff", &self.acwff())
            .field("apwfe", &self.apwfe())
            .field("apwff", &self.apwff())
            .field("vpbe", &self.vpbe())
            .field("vpbf", &self.vpbf())
            .field("acbe", &self.acbe())
            .field("acbf", &self.acbf())
            .field("apbe", &self.apbe())
            .field("apbf", &self.apbf())
            .finish()
    }
}
/**DSI Host FIFO and buffer status register

You can [`read`](crate::Reg::read) this register and get [`fbsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:FBSR)*/
pub struct FBSRrs;
impl crate::RegisterSpec for FBSRrs {
    type Ux = u32;
}
///`read()` method returns [`fbsr::R`](R) reader structure
impl crate::Readable for FBSRrs {}
///`reset()` method sets FBSR to value 0x0005_0015
impl crate::Resettable for FBSRrs {
    const RESET_VALUE: u32 = 0x0005_0015;
}
