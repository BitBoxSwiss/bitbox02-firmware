///Register `TCR` reader
pub type R = crate::R<TCRrs>;
///Register `TCR` writer
pub type W = crate::W<TCRrs>;
///Field `DCYC` reader - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31).
pub type DCYC_R = crate::FieldReader;
///Field `DCYC` writer - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31).
pub type DCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DHQC` reader - Delay hold quarter cycle
pub type DHQC_R = crate::BitReader;
///Field `DHQC` writer - Delay hold quarter cycle
pub type DHQC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSHIFT` reader - Sample shift By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFTÂ =Â 0 when the data phase is configured in DTR mode (when DDTRÂ =Â 1.)
pub type SSHIFT_R = crate::BitReader;
///Field `SSHIFT` writer - Sample shift By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFTÂ =Â 0 when the data phase is configured in DTR mode (when DDTRÂ =Â 1.)
pub type SSHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31).
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 28 - Delay hold quarter cycle
    #[inline(always)]
    pub fn dhqc(&self) -> DHQC_R {
        DHQC_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Sample shift By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFTÂ =Â 0 when the data phase is configured in DTR mode (when DDTRÂ =Â 1.)
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("dcyc", &self.dcyc())
            .field("dhqc", &self.dhqc())
            .field("sshift", &self.sshift())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31).
    #[inline(always)]
    pub fn dcyc(&mut self) -> DCYC_W<TCRrs> {
        DCYC_W::new(self, 0)
    }
    ///Bit 28 - Delay hold quarter cycle
    #[inline(always)]
    pub fn dhqc(&mut self) -> DHQC_W<TCRrs> {
        DHQC_W::new(self, 28)
    }
    ///Bit 30 - Sample shift By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFTÂ =Â 0 when the data phase is configured in DTR mode (when DDTRÂ =Â 1.)
    #[inline(always)]
    pub fn sshift(&mut self) -> SSHIFT_W<TCRrs> {
        SSHIFT_W::new(self, 30)
    }
}
/**HSPI timing configuration register

You can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:TCR)*/
pub struct TCRrs;
impl crate::RegisterSpec for TCRrs {
    type Ux = u32;
}
///`read()` method returns [`tcr::R`](R) reader structure
impl crate::Readable for TCRrs {}
///`write(|w| ..)` method takes [`tcr::W`](W) writer structure
impl crate::Writable for TCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCR to value 0
impl crate::Resettable for TCRrs {}
