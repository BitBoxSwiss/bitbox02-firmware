///Register `X1BUFCFG` reader
pub type R = crate::R<X1BUFCFGrs>;
///Register `X1BUFCFG` writer
pub type W = crate::W<X1BUFCFGrs>;
///Field `X1_BASE` reader - Base address of X1 buffer
pub type X1_BASE_R = crate::FieldReader;
///Field `X1_BASE` writer - Base address of X1 buffer
pub type X1_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `X1_BUF_SIZE` reader - Allocated size of X1 buffer in 16-bit words
pub type X1_BUF_SIZE_R = crate::FieldReader;
///Field `X1_BUF_SIZE` writer - Allocated size of X1 buffer in 16-bit words
pub type X1_BUF_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FULL_WM` reader - Watermark for buffer full flag
pub type FULL_WM_R = crate::FieldReader;
///Field `FULL_WM` writer - Watermark for buffer full flag
pub type FULL_WM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:7 - Base address of X1 buffer
    #[inline(always)]
    pub fn x1_base(&self) -> X1_BASE_R {
        X1_BASE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Allocated size of X1 buffer in 16-bit words
    #[inline(always)]
    pub fn x1_buf_size(&self) -> X1_BUF_SIZE_R {
        X1_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 24:25 - Watermark for buffer full flag
    #[inline(always)]
    pub fn full_wm(&self) -> FULL_WM_R {
        FULL_WM_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("X1BUFCFG")
            .field("x1_base", &self.x1_base())
            .field("x1_buf_size", &self.x1_buf_size())
            .field("full_wm", &self.full_wm())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Base address of X1 buffer
    #[inline(always)]
    pub fn x1_base(&mut self) -> X1_BASE_W<X1BUFCFGrs> {
        X1_BASE_W::new(self, 0)
    }
    ///Bits 8:15 - Allocated size of X1 buffer in 16-bit words
    #[inline(always)]
    pub fn x1_buf_size(&mut self) -> X1_BUF_SIZE_W<X1BUFCFGrs> {
        X1_BUF_SIZE_W::new(self, 8)
    }
    ///Bits 24:25 - Watermark for buffer full flag
    #[inline(always)]
    pub fn full_wm(&mut self) -> FULL_WM_W<X1BUFCFGrs> {
        FULL_WM_W::new(self, 24)
    }
}
/**FMAC X1 Buffer Configuration register

You can [`read`](crate::Reg::read) this register and get [`x1bufcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x1bufcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#FMAC:X1BUFCFG)*/
pub struct X1BUFCFGrs;
impl crate::RegisterSpec for X1BUFCFGrs {
    type Ux = u32;
}
///`read()` method returns [`x1bufcfg::R`](R) reader structure
impl crate::Readable for X1BUFCFGrs {}
///`write(|w| ..)` method takes [`x1bufcfg::W`](W) writer structure
impl crate::Writable for X1BUFCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets X1BUFCFG to value 0
impl crate::Resettable for X1BUFCFGrs {}
