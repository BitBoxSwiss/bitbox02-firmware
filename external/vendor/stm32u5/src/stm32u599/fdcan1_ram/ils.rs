///Register `ILS` reader
pub type R = crate::R<ILSrs>;
///Register `ILS` writer
pub type W = crate::W<ILSrs>;
///Field `RxFIFO0` reader - RxFIFO0
pub type RX_FIFO0_R = crate::BitReader;
///Field `RxFIFO0` writer - RxFIFO0
pub type RX_FIFO0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RxFIFO1` reader - RxFIFO1
pub type RX_FIFO1_R = crate::BitReader;
///Field `RxFIFO1` writer - RxFIFO1
pub type RX_FIFO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMSG` reader - SMSG
pub type SMSG_R = crate::BitReader;
///Field `SMSG` writer - SMSG
pub type SMSG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFERR` reader - TFERR
pub type TFERR_R = crate::BitReader;
///Field `TFERR` writer - TFERR
pub type TFERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MISC` reader - MISC
pub type MISC_R = crate::BitReader;
///Field `MISC` writer - MISC
pub type MISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BERR` reader - BERR
pub type BERR_R = crate::BitReader;
///Field `BERR` writer - BERR
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERR` reader - PERR
pub type PERR_R = crate::BitReader;
///Field `PERR` writer - PERR
pub type PERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RxFIFO0
    #[inline(always)]
    pub fn rx_fifo0(&self) -> RX_FIFO0_R {
        RX_FIFO0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RxFIFO1
    #[inline(always)]
    pub fn rx_fifo1(&self) -> RX_FIFO1_R {
        RX_FIFO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SMSG
    #[inline(always)]
    pub fn smsg(&self) -> SMSG_R {
        SMSG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TFERR
    #[inline(always)]
    pub fn tferr(&self) -> TFERR_R {
        TFERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MISC
    #[inline(always)]
    pub fn misc(&self) -> MISC_R {
        MISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BERR
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PERR
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ILS")
            .field("rx_fifo0", &self.rx_fifo0())
            .field("rx_fifo1", &self.rx_fifo1())
            .field("smsg", &self.smsg())
            .field("tferr", &self.tferr())
            .field("misc", &self.misc())
            .field("berr", &self.berr())
            .field("perr", &self.perr())
            .finish()
    }
}
impl W {
    ///Bit 0 - RxFIFO0
    #[inline(always)]
    pub fn rx_fifo0(&mut self) -> RX_FIFO0_W<ILSrs> {
        RX_FIFO0_W::new(self, 0)
    }
    ///Bit 1 - RxFIFO1
    #[inline(always)]
    pub fn rx_fifo1(&mut self) -> RX_FIFO1_W<ILSrs> {
        RX_FIFO1_W::new(self, 1)
    }
    ///Bit 2 - SMSG
    #[inline(always)]
    pub fn smsg(&mut self) -> SMSG_W<ILSrs> {
        SMSG_W::new(self, 2)
    }
    ///Bit 3 - TFERR
    #[inline(always)]
    pub fn tferr(&mut self) -> TFERR_W<ILSrs> {
        TFERR_W::new(self, 3)
    }
    ///Bit 4 - MISC
    #[inline(always)]
    pub fn misc(&mut self) -> MISC_W<ILSrs> {
        MISC_W::new(self, 4)
    }
    ///Bit 5 - BERR
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W<ILSrs> {
        BERR_W::new(self, 5)
    }
    ///Bit 6 - PERR
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W<ILSrs> {
        PERR_W::new(self, 6)
    }
}
/**FDCAN Interrupt Line Select Register

You can [`read`](crate::Reg::read) this register and get [`ils::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FDCAN1_RAM:ILS)*/
pub struct ILSrs;
impl crate::RegisterSpec for ILSrs {
    type Ux = u32;
}
///`read()` method returns [`ils::R`](R) reader structure
impl crate::Readable for ILSrs {}
///`write(|w| ..)` method takes [`ils::W`](W) writer structure
impl crate::Writable for ILSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ILS to value 0
impl crate::Resettable for ILSrs {}
