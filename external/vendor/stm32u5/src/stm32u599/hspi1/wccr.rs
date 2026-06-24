///Register `WCCR` reader
pub type R = crate::R<WCCRrs>;
///Register `WCCR` writer
pub type W = crate::W<WCCRrs>;
///Field `IMODE` reader - Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved
pub type IMODE_R = crate::FieldReader;
///Field `IMODE` writer - Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved
pub type IMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IDTR` reader - Instruction double transfer rate This bit sets the DTR mode for the instruction phase.
pub type IDTR_R = crate::BitReader;
///Field `IDTR` writer - Instruction double transfer rate This bit sets the DTR mode for the instruction phase.
pub type IDTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISIZE` reader - Instruction size This bit defines instruction size:
pub type ISIZE_R = crate::FieldReader;
///Field `ISIZE` writer - Instruction size This bit defines instruction size:
pub type ISIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ADMODE` reader - Address mode This field defines the address phase mode of operation. 101-111: Reserved
pub type ADMODE_R = crate::FieldReader;
///Field `ADMODE` writer - Address mode This field defines the address phase mode of operation. 101-111: Reserved
pub type ADMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ADDTR` reader - Address double transfer rate This bit sets the DTR mode for the address phase.
pub type ADDTR_R = crate::BitReader;
///Field `ADDTR` writer - Address double transfer rate This bit sets the DTR mode for the address phase.
pub type ADDTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADSIZE` reader - Address size This field defines address size.
pub type ADSIZE_R = crate::FieldReader;
///Field `ADSIZE` writer - Address size This field defines address size.
pub type ADSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ABMODE` reader - Alternate-byte mode This field defines the alternate-byte phase mode of operation. 101-111: Reserved
pub type ABMODE_R = crate::FieldReader;
///Field `ABMODE` writer - Alternate-byte mode This field defines the alternate-byte phase mode of operation. 101-111: Reserved
pub type ABMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ABDTR` reader - Alternate bytes double-transfer rate This bit sets the DTR mode for the alternate-bytes phase.
pub type ABDTR_R = crate::BitReader;
///Field `ABDTR` writer - Alternate bytes double-transfer rate This bit sets the DTR mode for the alternate-bytes phase.
pub type ABDTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABSIZE` reader - Alternate bytes size This field defines alternate bytes size:
pub type ABSIZE_R = crate::FieldReader;
///Field `ABSIZE` writer - Alternate bytes size This field defines alternate bytes size:
pub type ABSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DMODE` reader - Data mode This field defines the data phase mode of operation.
pub type DMODE_R = crate::FieldReader;
///Field `DMODE` writer - Data mode This field defines the data phase mode of operation.
pub type DMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DDTR` reader - data double transfer rate This bit sets the DTR mode for the data phase.
pub type DDTR_R = crate::BitReader;
///Field `DDTR` writer - data double transfer rate This bit sets the DTR mode for the data phase.
pub type DDTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQSE` reader - DQS enable This bit enables the data strobe management.
pub type DQSE_R = crate::BitReader;
///Field `DQSE` writer - DQS enable This bit enables the data strobe management.
pub type DQSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Instruction double transfer rate This bit sets the DTR mode for the instruction phase.
    #[inline(always)]
    pub fn idtr(&self) -> IDTR_R {
        IDTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Instruction size This bit defines instruction size:
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:10 - Address mode This field defines the address phase mode of operation. 101-111: Reserved
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - Address double transfer rate This bit sets the DTR mode for the address phase.
    #[inline(always)]
    pub fn addtr(&self) -> ADDTR_R {
        ADDTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Address size This field defines address size.
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - Alternate-byte mode This field defines the alternate-byte phase mode of operation. 101-111: Reserved
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - Alternate bytes double-transfer rate This bit sets the DTR mode for the alternate-bytes phase.
    #[inline(always)]
    pub fn abdtr(&self) -> ABDTR_R {
        ABDTR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Alternate bytes size This field defines alternate bytes size:
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:26 - Data mode This field defines the data phase mode of operation.
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - data double transfer rate This bit sets the DTR mode for the data phase.
    #[inline(always)]
    pub fn ddtr(&self) -> DDTR_R {
        DDTR_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - DQS enable This bit enables the data strobe management.
    #[inline(always)]
    pub fn dqse(&self) -> DQSE_R {
        DQSE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WCCR")
            .field("imode", &self.imode())
            .field("idtr", &self.idtr())
            .field("isize", &self.isize())
            .field("admode", &self.admode())
            .field("addtr", &self.addtr())
            .field("adsize", &self.adsize())
            .field("abmode", &self.abmode())
            .field("abdtr", &self.abdtr())
            .field("absize", &self.absize())
            .field("dmode", &self.dmode())
            .field("ddtr", &self.ddtr())
            .field("dqse", &self.dqse())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Instruction mode This field defines the instruction phase mode of operation. 101-111: Reserved
    #[inline(always)]
    pub fn imode(&mut self) -> IMODE_W<WCCRrs> {
        IMODE_W::new(self, 0)
    }
    ///Bit 3 - Instruction double transfer rate This bit sets the DTR mode for the instruction phase.
    #[inline(always)]
    pub fn idtr(&mut self) -> IDTR_W<WCCRrs> {
        IDTR_W::new(self, 3)
    }
    ///Bits 4:5 - Instruction size This bit defines instruction size:
    #[inline(always)]
    pub fn isize(&mut self) -> ISIZE_W<WCCRrs> {
        ISIZE_W::new(self, 4)
    }
    ///Bits 8:10 - Address mode This field defines the address phase mode of operation. 101-111: Reserved
    #[inline(always)]
    pub fn admode(&mut self) -> ADMODE_W<WCCRrs> {
        ADMODE_W::new(self, 8)
    }
    ///Bit 11 - Address double transfer rate This bit sets the DTR mode for the address phase.
    #[inline(always)]
    pub fn addtr(&mut self) -> ADDTR_W<WCCRrs> {
        ADDTR_W::new(self, 11)
    }
    ///Bits 12:13 - Address size This field defines address size.
    #[inline(always)]
    pub fn adsize(&mut self) -> ADSIZE_W<WCCRrs> {
        ADSIZE_W::new(self, 12)
    }
    ///Bits 16:18 - Alternate-byte mode This field defines the alternate-byte phase mode of operation. 101-111: Reserved
    #[inline(always)]
    pub fn abmode(&mut self) -> ABMODE_W<WCCRrs> {
        ABMODE_W::new(self, 16)
    }
    ///Bit 19 - Alternate bytes double-transfer rate This bit sets the DTR mode for the alternate-bytes phase.
    #[inline(always)]
    pub fn abdtr(&mut self) -> ABDTR_W<WCCRrs> {
        ABDTR_W::new(self, 19)
    }
    ///Bits 20:21 - Alternate bytes size This field defines alternate bytes size:
    #[inline(always)]
    pub fn absize(&mut self) -> ABSIZE_W<WCCRrs> {
        ABSIZE_W::new(self, 20)
    }
    ///Bits 24:26 - Data mode This field defines the data phase mode of operation.
    #[inline(always)]
    pub fn dmode(&mut self) -> DMODE_W<WCCRrs> {
        DMODE_W::new(self, 24)
    }
    ///Bit 27 - data double transfer rate This bit sets the DTR mode for the data phase.
    #[inline(always)]
    pub fn ddtr(&mut self) -> DDTR_W<WCCRrs> {
        DDTR_W::new(self, 27)
    }
    ///Bit 29 - DQS enable This bit enables the data strobe management.
    #[inline(always)]
    pub fn dqse(&mut self) -> DQSE_W<WCCRrs> {
        DQSE_W::new(self, 29)
    }
}
/**HSPI write communication configuration register

You can [`read`](crate::Reg::read) this register and get [`wccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:WCCR)*/
pub struct WCCRrs;
impl crate::RegisterSpec for WCCRrs {
    type Ux = u32;
}
///`read()` method returns [`wccr::R`](R) reader structure
impl crate::Readable for WCCRrs {}
///`write(|w| ..)` method takes [`wccr::W`](W) writer structure
impl crate::Writable for WCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WCCR to value 0
impl crate::Resettable for WCCRrs {}
