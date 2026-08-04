///Register `WPR` writer
pub type W = crate::W<WPRrs>;
/**Write protection key

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY {
    ///0: Activate write protection (any value that is not the keys)
    Activate = 0,
    ///83: Key 2
    Deactivate2 = 83,
    ///202: Key 1
    Deactivate1 = 202,
}
impl From<KEY> for u8 {
    #[inline(always)]
    fn from(variant: KEY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY {
    type Ux = u8;
}
impl crate::IsEnum for KEY {}
///Field `KEY` writer - Write protection key
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Activate write protection (any value that is not the keys)
    #[inline(always)]
    pub fn activate(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Activate)
    }
    ///Key 2
    #[inline(always)]
    pub fn deactivate2(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Deactivate2)
    }
    ///Key 1
    #[inline(always)]
    pub fn deactivate1(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Deactivate1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<WPRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Write protection key
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<WPRrs> {
        KEY_W::new(self, 0)
    }
}
/**write protection register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RTC:WPR)*/
pub struct WPRrs;
impl crate::RegisterSpec for WPRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wpr::W`](W) writer structure
impl crate::Writable for WPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPR to value 0
impl crate::Resettable for WPRrs {}
