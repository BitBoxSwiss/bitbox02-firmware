///Register `ECR` reader
pub type R = crate::R<ECRrs>;
///Register `ECR` writer
pub type W = crate::W<ECRrs>;
///Field `TEC` reader - Transmit Error Counter
pub type TEC_R = crate::FieldReader;
///Field `REC` reader - Receive Error Counter
pub type REC_R = crate::FieldReader;
///Field `RP` reader - Receive Error Passive
pub type RP_R = crate::BitReader;
///Field `CEL` reader - AN Error Logging
pub type CEL_R = crate::FieldReader;
///Field `CEL` writer - AN Error Logging
pub type CEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Transmit Error Counter
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - Receive Error Counter
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - Receive Error Passive
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - AN Error Logging
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECR")
            .field("cel", &self.cel())
            .field("rp", &self.rp())
            .field("rec", &self.rec())
            .field("tec", &self.tec())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - AN Error Logging
    #[inline(always)]
    pub fn cel(&mut self) -> CEL_W<ECRrs> {
        CEL_W::new(self, 16)
    }
}
/**FDCAN Error Counter Register

You can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FDCAN1_RAM:ECR)*/
pub struct ECRrs;
impl crate::RegisterSpec for ECRrs {
    type Ux = u32;
}
///`read()` method returns [`ecr::R`](R) reader structure
impl crate::Readable for ECRrs {}
///`write(|w| ..)` method takes [`ecr::W`](W) writer structure
impl crate::Writable for ECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECR to value 0
impl crate::Resettable for ECRrs {}
