///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `FUNC` reader - Function
pub type FUNC_R = crate::FieldReader;
///Field `FUNC` writer - Function
pub type FUNC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PRECISION` reader - Precision required (number of iterations)
pub type PRECISION_R = crate::FieldReader;
///Field `PRECISION` writer - Precision required (number of iterations)
pub type PRECISION_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SCALE` reader - Scaling factor
pub type SCALE_R = crate::FieldReader;
///Field `SCALE` writer - Scaling factor
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IEN` reader - Enable interrupt
pub type IEN_R = crate::BitReader;
///Field `IEN` writer - Enable interrupt
pub type IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAREN` reader - Enable DMA read channel
pub type DMAREN_R = crate::BitReader;
///Field `DMAREN` writer - Enable DMA read channel
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAWEN` reader - Enable DMA write channel
pub type DMAWEN_R = crate::BitReader;
///Field `DMAWEN` writer - Enable DMA write channel
pub type DMAWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRES` reader - Number of results in the CORDIC_RDATA register
pub type NRES_R = crate::BitReader;
///Field `NRES` writer - Number of results in the CORDIC_RDATA register
pub type NRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NARGS` reader - Number of arguments expected by the CORDIC_WDATA register
pub type NARGS_R = crate::BitReader;
///Field `NARGS` writer - Number of arguments expected by the CORDIC_WDATA register
pub type NARGS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESSIZE` reader - Width of output data
pub type RESSIZE_R = crate::BitReader;
///Field `RESSIZE` writer - Width of output data
pub type RESSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARGSIZE` reader - Width of input data
pub type ARGSIZE_R = crate::BitReader;
///Field `ARGSIZE` writer - Width of input data
pub type ARGSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRDY` reader - Result ready flag
pub type RRDY_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Function
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Precision required (number of iterations)
    #[inline(always)]
    pub fn precision(&self) -> PRECISION_R {
        PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - Scaling factor
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 16 - Enable interrupt
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Enable DMA read channel
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Enable DMA write channel
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Number of results in the CORDIC_RDATA register
    #[inline(always)]
    pub fn nres(&self) -> NRES_R {
        NRES_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Number of arguments expected by the CORDIC_WDATA register
    #[inline(always)]
    pub fn nargs(&self) -> NARGS_R {
        NARGS_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Width of output data
    #[inline(always)]
    pub fn ressize(&self) -> RESSIZE_R {
        RESSIZE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Width of input data
    #[inline(always)]
    pub fn argsize(&self) -> ARGSIZE_R {
        ARGSIZE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 31 - Result ready flag
    #[inline(always)]
    pub fn rrdy(&self) -> RRDY_R {
        RRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("func", &self.func())
            .field("precision", &self.precision())
            .field("scale", &self.scale())
            .field("ien", &self.ien())
            .field("dmaren", &self.dmaren())
            .field("dmawen", &self.dmawen())
            .field("nres", &self.nres())
            .field("nargs", &self.nargs())
            .field("ressize", &self.ressize())
            .field("argsize", &self.argsize())
            .field("rrdy", &self.rrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Function
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W<CSRrs> {
        FUNC_W::new(self, 0)
    }
    ///Bits 4:7 - Precision required (number of iterations)
    #[inline(always)]
    pub fn precision(&mut self) -> PRECISION_W<CSRrs> {
        PRECISION_W::new(self, 4)
    }
    ///Bits 8:10 - Scaling factor
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<CSRrs> {
        SCALE_W::new(self, 8)
    }
    ///Bit 16 - Enable interrupt
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W<CSRrs> {
        IEN_W::new(self, 16)
    }
    ///Bit 17 - Enable DMA read channel
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W<CSRrs> {
        DMAREN_W::new(self, 17)
    }
    ///Bit 18 - Enable DMA write channel
    #[inline(always)]
    pub fn dmawen(&mut self) -> DMAWEN_W<CSRrs> {
        DMAWEN_W::new(self, 18)
    }
    ///Bit 19 - Number of results in the CORDIC_RDATA register
    #[inline(always)]
    pub fn nres(&mut self) -> NRES_W<CSRrs> {
        NRES_W::new(self, 19)
    }
    ///Bit 20 - Number of arguments expected by the CORDIC_WDATA register
    #[inline(always)]
    pub fn nargs(&mut self) -> NARGS_W<CSRrs> {
        NARGS_W::new(self, 20)
    }
    ///Bit 21 - Width of output data
    #[inline(always)]
    pub fn ressize(&mut self) -> RESSIZE_W<CSRrs> {
        RESSIZE_W::new(self, 21)
    }
    ///Bit 22 - Width of input data
    #[inline(always)]
    pub fn argsize(&mut self) -> ARGSIZE_W<CSRrs> {
        ARGSIZE_W::new(self, 22)
    }
}
/**CORDIC Control Status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#CORDIC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0x50
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x50;
}
