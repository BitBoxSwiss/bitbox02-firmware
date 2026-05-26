///Register `DCFG` reader
pub type R = crate::R<DCFGrs>;
///Register `DCFG` writer
pub type W = crate::W<DCFGrs>;
///Field `DSPD` reader - DSPD
pub type DSPD_R = crate::FieldReader;
///Field `DSPD` writer - DSPD
pub type DSPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NZLSOHSK` reader - NZLSOHSK
pub type NZLSOHSK_R = crate::BitReader;
///Field `NZLSOHSK` writer - NZLSOHSK
pub type NZLSOHSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAD` reader - DAD
pub type DAD_R = crate::FieldReader;
///Field `DAD` writer - DAD
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PFIVL` reader - PFIVL
pub type PFIVL_R = crate::FieldReader;
///Field `PFIVL` writer - PFIVL
pub type PFIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ERRATIM` reader - ERRATIM
pub type ERRATIM_R = crate::BitReader;
///Field `ERRATIM` writer - ERRATIM
pub type ERRATIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - DSPD
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - NZLSOHSK
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:10 - DAD
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    ///Bits 11:12 - PFIVL
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 15 - ERRATIM
    #[inline(always)]
    pub fn erratim(&self) -> ERRATIM_R {
        ERRATIM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCFG")
            .field("dspd", &self.dspd())
            .field("nzlsohsk", &self.nzlsohsk())
            .field("dad", &self.dad())
            .field("pfivl", &self.pfivl())
            .field("erratim", &self.erratim())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - DSPD
    #[inline(always)]
    pub fn dspd(&mut self) -> DSPD_W<DCFGrs> {
        DSPD_W::new(self, 0)
    }
    ///Bit 2 - NZLSOHSK
    #[inline(always)]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W<DCFGrs> {
        NZLSOHSK_W::new(self, 2)
    }
    ///Bits 4:10 - DAD
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W<DCFGrs> {
        DAD_W::new(self, 4)
    }
    ///Bits 11:12 - PFIVL
    #[inline(always)]
    pub fn pfivl(&mut self) -> PFIVL_W<DCFGrs> {
        PFIVL_W::new(self, 11)
    }
    ///Bit 15 - ERRATIM
    #[inline(always)]
    pub fn erratim(&mut self) -> ERRATIM_W<DCFGrs> {
        ERRATIM_W::new(self, 15)
    }
}
/**This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.

You can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#OTG_FS:DCFG)*/
pub struct DCFGrs;
impl crate::RegisterSpec for DCFGrs {
    type Ux = u32;
}
///`read()` method returns [`dcfg::R`](R) reader structure
impl crate::Readable for DCFGrs {}
///`write(|w| ..)` method takes [`dcfg::W`](W) writer structure
impl crate::Writable for DCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCFG to value 0x0220_0000
impl crate::Resettable for DCFGrs {
    const RESET_VALUE: u32 = 0x0220_0000;
}
