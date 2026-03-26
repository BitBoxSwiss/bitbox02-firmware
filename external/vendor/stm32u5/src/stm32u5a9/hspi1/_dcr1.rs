///Register `_DCR1` reader
pub type R = crate::R<_DCR1rs>;
///Register `_DCR1` writer
pub type W = crate::W<_DCR1rs>;
///Field `CKMODE` reader - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when nCSÂ =Â 1).
pub type CKMODE_R = crate::BitReader;
///Field `CKMODE` writer - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when nCSÂ =Â 1).
pub type CKMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRCK` reader - Free running clock This bit configures the free running clock.
pub type FRCK_R = crate::BitReader;
///Field `FRCK` writer - Free running clock This bit configures the free running clock.
pub type FRCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLYBYP` reader - Delay block bypass
pub type DLYBYP_R = crate::BitReader;
///Field `DLYBYP` writer - Delay block bypass
pub type DLYBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSHT` reader - Chip-select high time CSHTÂ +Â 1 defines the minimum number of CLK cycles where the chip-select (nCS) must remain high between commands issued to the external device. ... 63: nCS stays high for at least 64 cycles between external device commands. Note: When the extended CSHT timeout feature is not supported, CSHT\[5:3\] are reserved and the number of cycles is limited to eight (refer to implementation).
pub type CSHT_R = crate::FieldReader;
///Field `CSHT` writer - Chip-select high time CSHTÂ +Â 1 defines the minimum number of CLK cycles where the chip-select (nCS) must remain high between commands issued to the external device. ... 63: nCS stays high for at least 64 cycles between external device commands. Note: When the extended CSHT timeout feature is not supported, CSHT\[5:3\] are reserved and the number of cycles is limited to eight (refer to implementation).
pub type CSHT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DEVSIZE` reader - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\[DEVSIZE+1\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4Â Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256Â Mbytes. In Regular-command mode, if DMMÂ =Â 1, DEVSIZE\[4:0\] indicates the total capacity of the two devices together.
pub type DEVSIZE_R = crate::FieldReader;
///Field `DEVSIZE` writer - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\[DEVSIZE+1\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4Â Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256Â Mbytes. In Regular-command mode, if DMMÂ =Â 1, DEVSIZE\[4:0\] indicates the total capacity of the two devices together.
pub type DEVSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `MTYP` reader - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\[2:0\] for memories different from Micron. Others: Reserved
pub type MTYP_R = crate::FieldReader;
///Field `MTYP` writer - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\[2:0\] for memories different from Micron. Others: Reserved
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when nCSÂ =Â 1).
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Free running clock This bit configures the free running clock.
    #[inline(always)]
    pub fn frck(&self) -> FRCK_R {
        FRCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Delay block bypass
    #[inline(always)]
    pub fn dlybyp(&self) -> DLYBYP_R {
        DLYBYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:13 - Chip-select high time CSHTÂ +Â 1 defines the minimum number of CLK cycles where the chip-select (nCS) must remain high between commands issued to the external device. ... 63: nCS stays high for at least 64 cycles between external device commands. Note: When the extended CSHT timeout feature is not supported, CSHT\[5:3\] are reserved and the number of cycles is limited to eight (refer to implementation).
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:20 - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\[DEVSIZE+1\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4Â Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256Â Mbytes. In Regular-command mode, if DMMÂ =Â 1, DEVSIZE\[4:0\] indicates the total capacity of the two devices together.
    #[inline(always)]
    pub fn devsize(&self) -> DEVSIZE_R {
        DEVSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:26 - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\[2:0\] for memories different from Micron. Others: Reserved
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_DCR1")
            .field("ckmode", &self.ckmode())
            .field("frck", &self.frck())
            .field("dlybyp", &self.dlybyp())
            .field("csht", &self.csht())
            .field("devsize", &self.devsize())
            .field("mtyp", &self.mtyp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when nCSÂ =Â 1).
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<_DCR1rs> {
        CKMODE_W::new(self, 0)
    }
    ///Bit 1 - Free running clock This bit configures the free running clock.
    #[inline(always)]
    pub fn frck(&mut self) -> FRCK_W<_DCR1rs> {
        FRCK_W::new(self, 1)
    }
    ///Bit 3 - Delay block bypass
    #[inline(always)]
    pub fn dlybyp(&mut self) -> DLYBYP_W<_DCR1rs> {
        DLYBYP_W::new(self, 3)
    }
    ///Bits 8:13 - Chip-select high time CSHTÂ +Â 1 defines the minimum number of CLK cycles where the chip-select (nCS) must remain high between commands issued to the external device. ... 63: nCS stays high for at least 64 cycles between external device commands. Note: When the extended CSHT timeout feature is not supported, CSHT\[5:3\] are reserved and the number of cycles is limited to eight (refer to implementation).
    #[inline(always)]
    pub fn csht(&mut self) -> CSHT_W<_DCR1rs> {
        CSHT_W::new(self, 8)
    }
    ///Bits 16:20 - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\[DEVSIZE+1\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4Â Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256Â Mbytes. In Regular-command mode, if DMMÂ =Â 1, DEVSIZE\[4:0\] indicates the total capacity of the two devices together.
    #[inline(always)]
    pub fn devsize(&mut self) -> DEVSIZE_W<_DCR1rs> {
        DEVSIZE_W::new(self, 16)
    }
    ///Bits 24:26 - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\[2:0\] for memories different from Micron. Others: Reserved
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W<_DCR1rs> {
        MTYP_W::new(self, 24)
    }
}
/**HSPI device configuration register 1

You can [`read`](crate::Reg::read) this register and get [`_dcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:_DCR1)*/
pub struct _DCR1rs;
impl crate::RegisterSpec for _DCR1rs {
    type Ux = u32;
}
///`read()` method returns [`_dcr1::R`](R) reader structure
impl crate::Readable for _DCR1rs {}
///`write(|w| ..)` method takes [`_dcr1::W`](W) writer structure
impl crate::Writable for _DCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _DCR1 to value 0
impl crate::Resettable for _DCR1rs {}
