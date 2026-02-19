///Register `ICSCR3` reader
pub type R = crate::R<ICSCR3rs>;
///Register `ICSCR3` writer
pub type W = crate::W<ICSCR3rs>;
///Field `HSICAL` reader - HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value.
pub type HSICAL_R = crate::FieldReader<u16>;
///Field `HSITRIM` reader - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to HSICAL\[11:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI.
pub type HSITRIM_R = crate::FieldReader;
///Field `HSITRIM` writer - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to HSICAL\[11:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI.
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bits 0:11 - HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value.
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:20 - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to HSICAL\[11:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI.
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSCR3")
            .field("hsical", &self.hsical())
            .field("hsitrim", &self.hsitrim())
            .finish()
    }
}
impl W {
    ///Bits 16:20 - HSI clock trimming These bits provide an additional user-programmable trimming value that is added to HSICAL\[11:0\] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI.
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W<ICSCR3rs> {
        HSITRIM_W::new(self, 16)
    }
}
/**RCC internal clock sources calibration register 3

You can [`read`](crate::Reg::read) this register and get [`icscr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:ICSCR3)*/
pub struct ICSCR3rs;
impl crate::RegisterSpec for ICSCR3rs {
    type Ux = u32;
}
///`read()` method returns [`icscr3::R`](R) reader structure
impl crate::Readable for ICSCR3rs {}
///`write(|w| ..)` method takes [`icscr3::W`](W) writer structure
impl crate::Writable for ICSCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICSCR3 to value 0x0010_0000
impl crate::Resettable for ICSCR3rs {
    const RESET_VALUE: u32 = 0x0010_0000;
}
