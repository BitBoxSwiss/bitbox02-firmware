///Register `DR` reader
pub type R = crate::R<DRrs>;
///Field `BYTE(0-3)` reader - Data byte %s
pub type BYTE_R = crate::FieldReader;
impl R {
    ///Data byte (0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BYTE0` field.</div>
    #[inline(always)]
    pub fn byte(&self, n: u8) -> BYTE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        BYTE_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    ///Iterator for array of:
    ///Data byte (0-3)
    #[inline(always)]
    pub fn byte_iter(&self) -> impl Iterator<Item = BYTE_R> + '_ {
        (0..4).map(move |n| BYTE_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    ///Bits 0:7 - Data byte 0
    #[inline(always)]
    pub fn byte0(&self) -> BYTE_R {
        BYTE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data byte 1
    #[inline(always)]
    pub fn byte1(&self) -> BYTE_R {
        BYTE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data byte 2
    #[inline(always)]
    pub fn byte2(&self) -> BYTE_R {
        BYTE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data byte 3
    #[inline(always)]
    pub fn byte3(&self) -> BYTE_R {
        BYTE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("byte0", &self.byte0())
            .field("byte1", &self.byte1())
            .field("byte2", &self.byte2())
            .field("byte3", &self.byte3())
            .finish()
    }
}
/**data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DCMI:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {}
