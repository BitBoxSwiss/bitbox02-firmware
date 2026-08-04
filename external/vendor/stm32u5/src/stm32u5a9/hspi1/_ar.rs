///Register `_AR` reader
pub type R = crate::R<_ARrs>;
///Register `_AR` writer
pub type W = crate::W<_ARrs>;
///Field `ADDRESS` reader - Address Address to be sent to the external device. In HyperBus mode, this field must be even as this protocol is 16-bit word oriented. In dual-memory mode, AR\[0\] is forced to 1. Writes to this field are ignored when BUSYÂ =Â 1 or when FMODE = 11 (Memory-mapped mode).
pub type ADDRESS_R = crate::FieldReader<u32>;
///Field `ADDRESS` writer - Address Address to be sent to the external device. In HyperBus mode, this field must be even as this protocol is 16-bit word oriented. In dual-memory mode, AR\[0\] is forced to 1. Writes to this field are ignored when BUSYÂ =Â 1 or when FMODE = 11 (Memory-mapped mode).
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Address Address to be sent to the external device. In HyperBus mode, this field must be even as this protocol is 16-bit word oriented. In dual-memory mode, AR\[0\] is forced to 1. Writes to this field are ignored when BUSYÂ =Â 1 or when FMODE = 11 (Memory-mapped mode).
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_AR")
            .field("address", &self.address())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Address Address to be sent to the external device. In HyperBus mode, this field must be even as this protocol is 16-bit word oriented. In dual-memory mode, AR\[0\] is forced to 1. Writes to this field are ignored when BUSYÂ =Â 1 or when FMODE = 11 (Memory-mapped mode).
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W<_ARrs> {
        ADDRESS_W::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`_ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_AR)*/
pub struct _ARrs;
impl crate::RegisterSpec for _ARrs {
    type Ux = u32;
}
///`read()` method returns [`_ar::R`](R) reader structure
impl crate::Readable for _ARrs {}
///`write(|w| ..)` method takes [`_ar::W`](W) writer structure
impl crate::Writable for _ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _AR to value 0
impl crate::Resettable for _ARrs {}
