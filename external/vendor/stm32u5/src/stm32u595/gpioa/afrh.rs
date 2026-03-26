///Register `AFRH` reader
pub type R = crate::R<AFRHrs>;
///Register `AFRH` writer
pub type W = crate::W<AFRHrs>;
///Field `AFR(EL8,EL9,EL10,EL11,EL12,EL13,EL14,EL15)` reader - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use super::afrl::AFR_R;
///Field `AFR(EL8,EL9,EL10,EL11,EL12,EL13,EL14,EL15)` writer - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use super::afrl::AFR_W;
///Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use super::afrl::ALTERNATE_FUNCTION;
impl R {
    ///Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AFREL8` field.</div>
    #[inline(always)]
    pub fn afr(&self, n: u8) -> AFR_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afr_iter(&self) -> impl Iterator<Item = AFR_R> + '_ {
        (0..8).map(move |n| AFR_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    ///Bits 0:3 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel8(&self) -> AFR_R {
        AFR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel9(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel10(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel11(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel12(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel13(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel14(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel15(&self) -> AFR_R {
        AFR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRH")
            .field("afrel8", &self.afrel8())
            .field("afrel9", &self.afrel9())
            .field("afrel10", &self.afrel10())
            .field("afrel11", &self.afrel11())
            .field("afrel12", &self.afrel12())
            .field("afrel13", &self.afrel13())
            .field("afrel14", &self.afrel14())
            .field("afrel15", &self.afrel15())
            .finish()
    }
}
impl W {
    ///Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AFREL8` field.</div>
    #[inline(always)]
    pub fn afr(&mut self, n: u8) -> AFR_W<AFRHrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AFR_W::new(self, n * 4)
    }
    ///Bits 0:3 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel8(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 0)
    }
    ///Bits 4:7 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel9(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 4)
    }
    ///Bits 8:11 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel10(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 8)
    }
    ///Bits 12:15 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel11(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 12)
    }
    ///Bits 16:19 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel12(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 16)
    }
    ///Bits 20:23 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel13(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 20)
    }
    ///Bits 24:27 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel14(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 24)
    }
    ///Bits 28:31 - Alternate function selection for port x I/O pin y These bits are written by the software to configure alternate function I/Os. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn afrel15(&mut self) -> AFR_W<AFRHrs> {
        AFR_W::new(self, 28)
    }
}
/**GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#GPIOA:AFRH)*/
pub struct AFRHrs;
impl crate::RegisterSpec for AFRHrs {
    type Ux = u32;
}
///`read()` method returns [`afrh::R`](R) reader structure
impl crate::Readable for AFRHrs {}
///`write(|w| ..)` method takes [`afrh::W`](W) writer structure
impl crate::Writable for AFRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFRH to value 0
impl crate::Resettable for AFRHrs {}
