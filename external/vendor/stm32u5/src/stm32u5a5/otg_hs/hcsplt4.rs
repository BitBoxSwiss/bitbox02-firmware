///Register `HCSPLT4` reader
pub type R = crate::R<HCSPLT4rs>;
///Register `HCSPLT4` writer
pub type W = crate::W<HCSPLT4rs>;
///Field `PRTADDR` reader - Port address This field is the port number of the recipient transaction translator.
pub type PRTADDR_R = crate::FieldReader;
///Field `PRTADDR` writer - Port address This field is the port number of the recipient transaction translator.
pub type PRTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `HUBADDR` reader - Hub address This field holds the device address of the transaction translatorâs hub.
pub type HUBADDR_R = crate::FieldReader;
///Field `HUBADDR` writer - Hub address This field holds the device address of the transaction translatorâs hub.
pub type HUBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `XACTPOS` reader - Transaction position This field is used to determine whether to send all, first, middle, or last payloads with each OUT transaction.
pub type XACTPOS_R = crate::FieldReader;
///Field `XACTPOS` writer - Transaction position This field is used to determine whether to send all, first, middle, or last payloads with each OUT transaction.
pub type XACTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMPLSPLT` reader - Do complete split The application sets this bit to request the OTG host to perform a complete split transaction.
pub type COMPLSPLT_R = crate::BitReader;
///Field `COMPLSPLT` writer - Do complete split The application sets this bit to request the OTG host to perform a complete split transaction.
pub type COMPLSPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLITEN` reader - Split enable The application sets this bit to indicate that this channel is enabled to perform split transactions.
pub type SPLITEN_R = crate::BitReader;
///Field `SPLITEN` writer - Split enable The application sets this bit to indicate that this channel is enabled to perform split transactions.
pub type SPLITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - Port address This field is the port number of the recipient transaction translator.
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:13 - Hub address This field holds the device address of the transaction translatorâs hub.
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    ///Bits 14:15 - Transaction position This field is used to determine whether to send all, first, middle, or last payloads with each OUT transaction.
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Do complete split The application sets this bit to request the OTG host to perform a complete split transaction.
    #[inline(always)]
    pub fn complsplt(&self) -> COMPLSPLT_R {
        COMPLSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - Split enable The application sets this bit to indicate that this channel is enabled to perform split transactions.
    #[inline(always)]
    pub fn spliten(&self) -> SPLITEN_R {
        SPLITEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCSPLT4")
            .field("prtaddr", &self.prtaddr())
            .field("hubaddr", &self.hubaddr())
            .field("xactpos", &self.xactpos())
            .field("complsplt", &self.complsplt())
            .field("spliten", &self.spliten())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Port address This field is the port number of the recipient transaction translator.
    #[inline(always)]
    pub fn prtaddr(&mut self) -> PRTADDR_W<HCSPLT4rs> {
        PRTADDR_W::new(self, 0)
    }
    ///Bits 7:13 - Hub address This field holds the device address of the transaction translatorâs hub.
    #[inline(always)]
    pub fn hubaddr(&mut self) -> HUBADDR_W<HCSPLT4rs> {
        HUBADDR_W::new(self, 7)
    }
    ///Bits 14:15 - Transaction position This field is used to determine whether to send all, first, middle, or last payloads with each OUT transaction.
    #[inline(always)]
    pub fn xactpos(&mut self) -> XACTPOS_W<HCSPLT4rs> {
        XACTPOS_W::new(self, 14)
    }
    ///Bit 16 - Do complete split The application sets this bit to request the OTG host to perform a complete split transaction.
    #[inline(always)]
    pub fn complsplt(&mut self) -> COMPLSPLT_W<HCSPLT4rs> {
        COMPLSPLT_W::new(self, 16)
    }
    ///Bit 31 - Split enable The application sets this bit to indicate that this channel is enabled to perform split transactions.
    #[inline(always)]
    pub fn spliten(&mut self) -> SPLITEN_W<HCSPLT4rs> {
        SPLITEN_W::new(self, 31)
    }
}
/**OTG host channel 4 split control register

You can [`read`](crate::Reg::read) this register and get [`hcsplt4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#OTG_HS:HCSPLT4)*/
pub struct HCSPLT4rs;
impl crate::RegisterSpec for HCSPLT4rs {
    type Ux = u32;
}
///`read()` method returns [`hcsplt4::R`](R) reader structure
impl crate::Readable for HCSPLT4rs {}
///`write(|w| ..)` method takes [`hcsplt4::W`](W) writer structure
impl crate::Writable for HCSPLT4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCSPLT4 to value 0
impl crate::Resettable for HCSPLT4rs {}
