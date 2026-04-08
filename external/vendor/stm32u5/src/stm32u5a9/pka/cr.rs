///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - Peripheral Enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - Peripheral Enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Start the operation
pub type START_R = crate::BitReader;
///Field `START` writer - Start the operation
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - PKA Operation Mode
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - PKA Operation Mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PROCENDIE` reader - End of operation interrupt enable
pub type PROCENDIE_R = crate::BitReader;
///Field `PROCENDIE` writer - End of operation interrupt enable
pub type PROCENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMERRIE` reader - RAM error interrupt enable
pub type RAMERRIE_R = crate::BitReader;
///Field `RAMERRIE` writer - RAM error interrupt enable
pub type RAMERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDRERRIE` reader - Address error interrupt enable
pub type ADDRERRIE_R = crate::BitReader;
///Field `ADDRERRIE` writer - Address error interrupt enable
pub type ADDRERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPERRIE` reader - Operation error interrupt enable
pub type OPERRIE_R = crate::BitReader;
///Field `OPERRIE` writer - Operation error interrupt enable
pub type OPERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Peripheral Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start the operation
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:13 - PKA Operation Mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 17 - End of operation interrupt enable
    #[inline(always)]
    pub fn procendie(&self) -> PROCENDIE_R {
        PROCENDIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - RAM error interrupt enable
    #[inline(always)]
    pub fn ramerrie(&self) -> RAMERRIE_R {
        RAMERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Address error interrupt enable
    #[inline(always)]
    pub fn addrerrie(&self) -> ADDRERRIE_R {
        ADDRERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Operation error interrupt enable
    #[inline(always)]
    pub fn operrie(&self) -> OPERRIE_R {
        OPERRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("operrie", &self.operrie())
            .field("addrerrie", &self.addrerrie())
            .field("ramerrie", &self.ramerrie())
            .field("procendie", &self.procendie())
            .field("mode", &self.mode())
            .field("start", &self.start())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Peripheral Enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Start the operation
    #[inline(always)]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 1)
    }
    ///Bits 8:13 - PKA Operation Mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 8)
    }
    ///Bit 17 - End of operation interrupt enable
    #[inline(always)]
    pub fn procendie(&mut self) -> PROCENDIE_W<CRrs> {
        PROCENDIE_W::new(self, 17)
    }
    ///Bit 19 - RAM error interrupt enable
    #[inline(always)]
    pub fn ramerrie(&mut self) -> RAMERRIE_W<CRrs> {
        RAMERRIE_W::new(self, 19)
    }
    ///Bit 20 - Address error interrupt enable
    #[inline(always)]
    pub fn addrerrie(&mut self) -> ADDRERRIE_W<CRrs> {
        ADDRERRIE_W::new(self, 20)
    }
    ///Bit 21 - Operation error interrupt enable
    #[inline(always)]
    pub fn operrie(&mut self) -> OPERRIE_W<CRrs> {
        OPERRIE_W::new(self, 21)
    }
}
/**Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#PKA:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
