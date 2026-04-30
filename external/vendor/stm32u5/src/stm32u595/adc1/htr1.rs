///Register `HTR1` reader
pub type R = crate::R<HTR1rs>;
///Register `HTR1` writer
pub type W = crate::W<HTR1rs>;
///Field `HTR1` reader - HTR1
pub type HTR1_R = crate::FieldReader<u32>;
///Field `HTR1` writer - HTR1
pub type HTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32, crate::Safe>;
/**AWDFILT1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWDFILT1 {
    ///0: No filtering
    NoFiltering = 0,
    ///1: Two consecutive detections generates an AWDx flag or an interrupt
    Detections2 = 1,
    ///2: Three consecutive detections generates an AWDx flag or an interrupt
    Detections3 = 2,
    ///3: Four consecutive detections generates an AWDx flag or an interrupt
    Detections4 = 3,
    ///4: Five consecutive detections generates an AWDx flag or an interrupt
    Detections5 = 4,
    ///5: Six consecutive detections generates an AWDx flag or an interrupt
    Detections6 = 5,
    ///6: Seven consecutive detections generates an AWDx flag or an interrupt
    Detections7 = 6,
    ///7: Eight consecutive detections generates an AWDx flag or an interrupt
    Detections8 = 7,
}
impl From<AWDFILT1> for u8 {
    #[inline(always)]
    fn from(variant: AWDFILT1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AWDFILT1 {
    type Ux = u8;
}
impl crate::IsEnum for AWDFILT1 {}
///Field `AWDFILT1` reader - AWDFILT1
pub type AWDFILT1_R = crate::FieldReader<AWDFILT1>;
impl AWDFILT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWDFILT1 {
        match self.bits {
            0 => AWDFILT1::NoFiltering,
            1 => AWDFILT1::Detections2,
            2 => AWDFILT1::Detections3,
            3 => AWDFILT1::Detections4,
            4 => AWDFILT1::Detections5,
            5 => AWDFILT1::Detections6,
            6 => AWDFILT1::Detections7,
            7 => AWDFILT1::Detections8,
            _ => unreachable!(),
        }
    }
    ///No filtering
    #[inline(always)]
    pub fn is_no_filtering(&self) -> bool {
        *self == AWDFILT1::NoFiltering
    }
    ///Two consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn is_detections2(&self) -> bool {
        *self == AWDFILT1::Detections2
    }
    ///Three consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn is_detections3(&self) -> bool {
        *self == AWDFILT1::Detections3
    }
    ///Four consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn is_detections4(&self) -> bool {
        *self == AWDFILT1::Detections4
    }
    ///Five consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn is_detections5(&self) -> bool {
        *self == AWDFILT1::Detections5
    }
    ///Six consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn is_detections6(&self) -> bool {
        *self == AWDFILT1::Detections6
    }
    ///Seven consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn is_detections7(&self) -> bool {
        *self == AWDFILT1::Detections7
    }
    ///Eight consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn is_detections8(&self) -> bool {
        *self == AWDFILT1::Detections8
    }
}
///Field `AWDFILT1` writer - AWDFILT1
pub type AWDFILT1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, AWDFILT1, crate::Safe>;
impl<'a, REG> AWDFILT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filtering
    #[inline(always)]
    pub fn no_filtering(self) -> &'a mut crate::W<REG> {
        self.variant(AWDFILT1::NoFiltering)
    }
    ///Two consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn detections2(self) -> &'a mut crate::W<REG> {
        self.variant(AWDFILT1::Detections2)
    }
    ///Three consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn detections3(self) -> &'a mut crate::W<REG> {
        self.variant(AWDFILT1::Detections3)
    }
    ///Four consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn detections4(self) -> &'a mut crate::W<REG> {
        self.variant(AWDFILT1::Detections4)
    }
    ///Five consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn detections5(self) -> &'a mut crate::W<REG> {
        self.variant(AWDFILT1::Detections5)
    }
    ///Six consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn detections6(self) -> &'a mut crate::W<REG> {
        self.variant(AWDFILT1::Detections6)
    }
    ///Seven consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn detections7(self) -> &'a mut crate::W<REG> {
        self.variant(AWDFILT1::Detections7)
    }
    ///Eight consecutive detections generates an AWDx flag or an interrupt
    #[inline(always)]
    pub fn detections8(self) -> &'a mut crate::W<REG> {
        self.variant(AWDFILT1::Detections8)
    }
}
impl R {
    ///Bits 0:24 - HTR1
    #[inline(always)]
    pub fn htr1(&self) -> HTR1_R {
        HTR1_R::new(self.bits & 0x01ff_ffff)
    }
    ///Bits 29:31 - AWDFILT1
    #[inline(always)]
    pub fn awdfilt1(&self) -> AWDFILT1_R {
        AWDFILT1_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HTR1")
            .field("awdfilt1", &self.awdfilt1())
            .field("htr1", &self.htr1())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - HTR1
    #[inline(always)]
    pub fn htr1(&mut self) -> HTR1_W<HTR1rs> {
        HTR1_W::new(self, 0)
    }
    ///Bits 29:31 - AWDFILT1
    #[inline(always)]
    pub fn awdfilt1(&mut self) -> AWDFILT1_W<HTR1rs> {
        AWDFILT1_W::new(self, 29)
    }
}
/**ADC watchdog threshold register 1

You can [`read`](crate::Reg::read) this register and get [`htr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ADC1:HTR1)*/
pub struct HTR1rs;
impl crate::RegisterSpec for HTR1rs {
    type Ux = u32;
}
///`read()` method returns [`htr1::R`](R) reader structure
impl crate::Readable for HTR1rs {}
///`write(|w| ..)` method takes [`htr1::W`](W) writer structure
impl crate::Writable for HTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HTR1 to value 0x01ff_ffff
impl crate::Resettable for HTR1rs {
    const RESET_VALUE: u32 = 0x01ff_ffff;
}
