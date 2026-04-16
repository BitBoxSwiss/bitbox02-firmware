///Register `DTR2` reader
pub type R = crate::R<DTR2rs>;
///Register `DTR2` writer
pub type W = crate::W<DTR2rs>;
///Field `DTGF` reader - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\[7:5\]=0xx => DTF=DTGF\[7:0\]x tdtg with tdtg=tDTS. DTGF\[7:5\]=10x => DTF=(64+DTGF\[5:0\])xtdtg with Tdtg=2xtDTS. DTGF\[7:5\]=110 => DTF=(32+DTGF\[4:0\])xtdtg with Tdtg=8xtDTS. DTGF\[7:5\]=111 => DTF=(32+DTGF\[4:0\])xtdtg with Tdtg=16xtDTS. Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type DTGF_R = crate::FieldReader;
///Field `DTGF` writer - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\[7:5\]=0xx => DTF=DTGF\[7:0\]x tdtg with tdtg=tDTS. DTGF\[7:5\]=10x => DTF=(64+DTGF\[5:0\])xtdtg with Tdtg=2xtDTS. DTGF\[7:5\]=110 => DTF=(32+DTGF\[4:0\])xtdtg with Tdtg=8xtDTS. DTGF\[7:5\]=111 => DTF=(32+DTGF\[4:0\])xtdtg with Tdtg=16xtDTS. Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type DTGF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DTAE` reader - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type DTAE_R = crate::BitReader;
///Field `DTAE` writer - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type DTAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTPE` reader - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type DTPE_R = crate::BitReader;
///Field `DTPE` writer - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type DTPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\[7:5\]=0xx => DTF=DTGF\[7:0\]x tdtg with tdtg=tDTS. DTGF\[7:5\]=10x => DTF=(64+DTGF\[5:0\])xtdtg with Tdtg=2xtDTS. DTGF\[7:5\]=110 => DTF=(32+DTGF\[4:0\])xtdtg with Tdtg=8xtDTS. DTGF\[7:5\]=111 => DTF=(32+DTGF\[4:0\])xtdtg with Tdtg=16xtDTS. Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn dtgf(&self) -> DTGF_R {
        DTGF_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 16 - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn dtae(&self) -> DTAE_R {
        DTAE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn dtpe(&self) -> DTPE_R {
        DTPE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTR2")
            .field("dtgf", &self.dtgf())
            .field("dtae", &self.dtae())
            .field("dtpe", &self.dtpe())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Dead-time falling edge generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs, on the falling edge. DTGF\[7:5\]=0xx => DTF=DTGF\[7:0\]x tdtg with tdtg=tDTS. DTGF\[7:5\]=10x => DTF=(64+DTGF\[5:0\])xtdtg with Tdtg=2xtDTS. DTGF\[7:5\]=110 => DTF=(32+DTGF\[4:0\])xtdtg with Tdtg=8xtDTS. DTGF\[7:5\]=111 => DTF=(32+DTGF\[4:0\])xtdtg with Tdtg=16xtDTS. Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn dtgf(&mut self) -> DTGF_W<DTR2rs> {
        DTGF_W::new(self, 0)
    }
    ///Bit 16 - Deadtime asymmetric enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn dtae(&mut self) -> DTAE_W<DTR2rs> {
        DTAE_W::new(self, 16)
    }
    ///Bit 17 - Deadtime preload enable Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn dtpe(&mut self) -> DTPE_W<DTR2rs> {
        DTPE_W::new(self, 17)
    }
}
/**TIM1 timer deadtime register 2

You can [`read`](crate::Reg::read) this register and get [`dtr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TIM1:DTR2)*/
pub struct DTR2rs;
impl crate::RegisterSpec for DTR2rs {
    type Ux = u32;
}
///`read()` method returns [`dtr2::R`](R) reader structure
impl crate::Readable for DTR2rs {}
///`write(|w| ..)` method takes [`dtr2::W`](W) writer structure
impl crate::Writable for DTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTR2 to value 0
impl crate::Resettable for DTR2rs {}
