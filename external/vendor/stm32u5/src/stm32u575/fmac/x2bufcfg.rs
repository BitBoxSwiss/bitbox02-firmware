///Register `X2BUFCFG` reader
pub type R = crate::R<X2BUFCFGrs>;
///Register `X2BUFCFG` writer
pub type W = crate::W<X2BUFCFGrs>;
///Field `X2_BASE` reader - Base address of X2 buffer
pub type X2_BASE_R = crate::FieldReader;
///Field `X2_BASE` writer - Base address of X2 buffer
pub type X2_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `X2_BUF_SIZE` reader - Size of X2 buffer in 16-bit words
pub type X2_BUF_SIZE_R = crate::FieldReader;
///Field `X2_BUF_SIZE` writer - Size of X2 buffer in 16-bit words
pub type X2_BUF_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Base address of X2 buffer
    #[inline(always)]
    pub fn x2_base(&self) -> X2_BASE_R {
        X2_BASE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Size of X2 buffer in 16-bit words
    #[inline(always)]
    pub fn x2_buf_size(&self) -> X2_BUF_SIZE_R {
        X2_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("X2BUFCFG")
            .field("x2_base", &self.x2_base())
            .field("x2_buf_size", &self.x2_buf_size())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Base address of X2 buffer
    #[inline(always)]
    pub fn x2_base(&mut self) -> X2_BASE_W<X2BUFCFGrs> {
        X2_BASE_W::new(self, 0)
    }
    ///Bits 8:15 - Size of X2 buffer in 16-bit words
    #[inline(always)]
    pub fn x2_buf_size(&mut self) -> X2_BUF_SIZE_W<X2BUFCFGrs> {
        X2_BUF_SIZE_W::new(self, 8)
    }
}
/**FMAC X2 Buffer Configuration register

You can [`read`](crate::Reg::read) this register and get [`x2bufcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x2bufcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#FMAC:X2BUFCFG)*/
pub struct X2BUFCFGrs;
impl crate::RegisterSpec for X2BUFCFGrs {
    type Ux = u32;
}
///`read()` method returns [`x2bufcfg::R`](R) reader structure
impl crate::Readable for X2BUFCFGrs {}
///`write(|w| ..)` method takes [`x2bufcfg::W`](W) writer structure
impl crate::Writable for X2BUFCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets X2BUFCFG to value 0
impl crate::Resettable for X2BUFCFGrs {}
