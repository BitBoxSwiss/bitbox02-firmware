///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
/**CHN21SEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHN21SEL {
    ///0: dac1_out1 selected
    Out1 = 0,
    ///1: dac1_out2 selected
    Out2 = 1,
}
impl From<CHN21SEL> for bool {
    #[inline(always)]
    fn from(variant: CHN21SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `CHN21SEL` reader - CHN21SEL
pub type CHN21SEL_R = crate::BitReader<CHN21SEL>;
impl CHN21SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHN21SEL {
        match self.bits {
            false => CHN21SEL::Out1,
            true => CHN21SEL::Out2,
        }
    }
    ///dac1_out1 selected
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == CHN21SEL::Out1
    }
    ///dac1_out2 selected
    #[inline(always)]
    pub fn is_out2(&self) -> bool {
        *self == CHN21SEL::Out2
    }
}
///Field `CHN21SEL` writer - CHN21SEL
pub type CHN21SEL_W<'a, REG> = crate::BitWriter<'a, REG, CHN21SEL>;
impl<'a, REG> CHN21SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///dac1_out1 selected
    #[inline(always)]
    pub fn out1(self) -> &'a mut crate::W<REG> {
        self.variant(CHN21SEL::Out1)
    }
    ///dac1_out2 selected
    #[inline(always)]
    pub fn out2(self) -> &'a mut crate::W<REG> {
        self.variant(CHN21SEL::Out2)
    }
}
impl R {
    ///Bit 0 - CHN21SEL
    #[inline(always)]
    pub fn chn21sel(&self) -> CHN21SEL_R {
        CHN21SEL_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("chn21sel", &self.chn21sel())
            .finish()
    }
}
impl W {
    ///Bit 0 - CHN21SEL
    #[inline(always)]
    pub fn chn21sel(&mut self) -> CHN21SEL_W<ORrs> {
        CHN21SEL_W::new(self, 0)
    }
}
/**ADC option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADC4:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
