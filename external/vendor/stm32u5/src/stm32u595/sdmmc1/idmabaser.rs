///Register `IDMABASER` reader
pub type R = crate::R<IDMABASERrs>;
///Register `IDMABASER` writer
pub type W = crate::W<IDMABASERrs>;
///Field `IDMABASE` reader - Buffer memory base address bits \[31:2\], shall be word aligned (bit \[1:0\] are always 0 and read only)
pub type IDMABASE_R = crate::FieldReader<u32>;
///Field `IDMABASE` writer - Buffer memory base address bits \[31:2\], shall be word aligned (bit \[1:0\] are always 0 and read only)
pub type IDMABASE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Buffer memory base address bits \[31:2\], shall be word aligned (bit \[1:0\] are always 0 and read only)
    #[inline(always)]
    pub fn idmabase(&self) -> IDMABASE_R {
        IDMABASE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDMABASER")
            .field("idmabase", &self.idmabase())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Buffer memory base address bits \[31:2\], shall be word aligned (bit \[1:0\] are always 0 and read only)
    #[inline(always)]
    pub fn idmabase(&mut self) -> IDMABASE_W<IDMABASERrs> {
        IDMABASE_W::new(self, 0)
    }
}
/**buffer base address register

You can [`read`](crate::Reg::read) this register and get [`idmabaser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabaser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#SDMMC1:IDMABASER)*/
pub struct IDMABASERrs;
impl crate::RegisterSpec for IDMABASERrs {
    type Ux = u32;
}
///`read()` method returns [`idmabaser::R`](R) reader structure
impl crate::Readable for IDMABASERrs {}
///`write(|w| ..)` method takes [`idmabaser::W`](W) writer structure
impl crate::Writable for IDMABASERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDMABASER to value 0
impl crate::Resettable for IDMABASERrs {}
