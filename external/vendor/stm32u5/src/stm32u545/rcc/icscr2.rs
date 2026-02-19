///Register `ICSCR2` reader
pub type R = crate::R<ICSCR2rs>;
///Register `ICSCR2` writer
pub type W = crate::W<ICSCR2rs>;
///Field `MSITRIM3` reader - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM3_R = crate::FieldReader;
///Field `MSITRIM3` writer - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM3_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `MSITRIM2` reader - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM2_R = crate::FieldReader;
///Field `MSITRIM2` writer - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM2_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `MSITRIM1` reader - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM1_R = crate::FieldReader;
///Field `MSITRIM1` writer - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
///Field `MSITRIM0` reader - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM0_R = crate::FieldReader;
///Field `MSITRIM0` writer - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
pub type MSITRIM0_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bits 0:4 - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim3(&self) -> MSITRIM3_R {
        MSITRIM3_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim2(&self) -> MSITRIM2_R {
        MSITRIM2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim1(&self) -> MSITRIM1_R {
        MSITRIM1_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim0(&self) -> MSITRIM0_R {
        MSITRIM0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSCR2")
            .field("msitrim3", &self.msitrim3())
            .field("msitrim2", &self.msitrim2())
            .field("msitrim1", &self.msitrim1())
            .field("msitrim0", &self.msitrim0())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim3(&mut self) -> MSITRIM3_W<ICSCR2rs> {
        MSITRIM3_W::new(self, 0)
    }
    ///Bits 5:9 - MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim2(&mut self) -> MSITRIM2_W<ICSCR2rs> {
        MSITRIM2_W::new(self, 5)
    }
    ///Bits 10:14 - MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim1(&mut self) -> MSITRIM1_W<ICSCR2rs> {
        MSITRIM1_W::new(self, 10)
    }
    ///Bits 15:19 - MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\[4:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.
    #[inline(always)]
    pub fn msitrim0(&mut self) -> MSITRIM0_W<ICSCR2rs> {
        MSITRIM0_W::new(self, 15)
    }
}
/**RCC internal clock sources calibration register 2

You can [`read`](crate::Reg::read) this register and get [`icscr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RCC:ICSCR2)*/
pub struct ICSCR2rs;
impl crate::RegisterSpec for ICSCR2rs {
    type Ux = u32;
}
///`read()` method returns [`icscr2::R`](R) reader structure
impl crate::Readable for ICSCR2rs {}
///`write(|w| ..)` method takes [`icscr2::W`](W) writer structure
impl crate::Writable for ICSCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICSCR2 to value 0x0008_4210
impl crate::Resettable for ICSCR2rs {
    const RESET_VALUE: u32 = 0x0008_4210;
}
