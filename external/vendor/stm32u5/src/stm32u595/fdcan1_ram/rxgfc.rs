///Register `RXGFC` reader
pub type R = crate::R<RXGFCrs>;
///Register `RXGFC` writer
pub type W = crate::W<RXGFCrs>;
///Field `RRFE` reader - Reject Remote Frames Extended
pub type RRFE_R = crate::BitReader;
///Field `RRFE` writer - Reject Remote Frames Extended
pub type RRFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRFS` reader - Reject Remote Frames Standard
pub type RRFS_R = crate::BitReader;
///Field `RRFS` writer - Reject Remote Frames Standard
pub type RRFS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANFE` reader - Accept Non-matching Frames Extended
pub type ANFE_R = crate::FieldReader;
///Field `ANFE` writer - Accept Non-matching Frames Extended
pub type ANFE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ANFS` reader - Accept Non-matching Frames Standard
pub type ANFS_R = crate::FieldReader;
///Field `ANFS` writer - Accept Non-matching Frames Standard
pub type ANFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `F1OM` reader - F1OM
pub type F1OM_R = crate::BitReader;
///Field `F1OM` writer - F1OM
pub type F1OM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `F0OM` reader - F0OM
pub type F0OM_R = crate::BitReader;
///Field `F0OM` writer - F0OM
pub type F0OM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSS` reader - LSS
pub type LSS_R = crate::FieldReader;
///Field `LSS` writer - LSS
pub type LSS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `LSE` reader - LSE
pub type LSE_R = crate::FieldReader;
///Field `LSE` writer - LSE
pub type LSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Reject Remote Frames Extended
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Reject Remote Frames Standard
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Accept Non-matching Frames Extended
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Accept Non-matching Frames Standard
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - F1OM
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - F0OM
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 16:20 - LSS
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:27 - LSE
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXGFC")
            .field("rrfe", &self.rrfe())
            .field("rrfs", &self.rrfs())
            .field("anfe", &self.anfe())
            .field("anfs", &self.anfs())
            .field("f1om", &self.f1om())
            .field("f0om", &self.f0om())
            .field("lss", &self.lss())
            .field("lse", &self.lse())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reject Remote Frames Extended
    #[inline(always)]
    pub fn rrfe(&mut self) -> RRFE_W<RXGFCrs> {
        RRFE_W::new(self, 0)
    }
    ///Bit 1 - Reject Remote Frames Standard
    #[inline(always)]
    pub fn rrfs(&mut self) -> RRFS_W<RXGFCrs> {
        RRFS_W::new(self, 1)
    }
    ///Bits 2:3 - Accept Non-matching Frames Extended
    #[inline(always)]
    pub fn anfe(&mut self) -> ANFE_W<RXGFCrs> {
        ANFE_W::new(self, 2)
    }
    ///Bits 4:5 - Accept Non-matching Frames Standard
    #[inline(always)]
    pub fn anfs(&mut self) -> ANFS_W<RXGFCrs> {
        ANFS_W::new(self, 4)
    }
    ///Bit 8 - F1OM
    #[inline(always)]
    pub fn f1om(&mut self) -> F1OM_W<RXGFCrs> {
        F1OM_W::new(self, 8)
    }
    ///Bit 9 - F0OM
    #[inline(always)]
    pub fn f0om(&mut self) -> F0OM_W<RXGFCrs> {
        F0OM_W::new(self, 9)
    }
    ///Bits 16:20 - LSS
    #[inline(always)]
    pub fn lss(&mut self) -> LSS_W<RXGFCrs> {
        LSS_W::new(self, 16)
    }
    ///Bits 24:27 - LSE
    #[inline(always)]
    pub fn lse(&mut self) -> LSE_W<RXGFCrs> {
        LSE_W::new(self, 24)
    }
}
/**FDCAN Global Filter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`rxgfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxgfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#FDCAN1_RAM:RXGFC)*/
pub struct RXGFCrs;
impl crate::RegisterSpec for RXGFCrs {
    type Ux = u32;
}
///`read()` method returns [`rxgfc::R`](R) reader structure
impl crate::Readable for RXGFCrs {}
///`write(|w| ..)` method takes [`rxgfc::W`](W) writer structure
impl crate::Writable for RXGFCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXGFC to value 0
impl crate::Resettable for RXGFCrs {}
