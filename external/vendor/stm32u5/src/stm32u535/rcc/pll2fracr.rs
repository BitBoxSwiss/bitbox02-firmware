///Register `PLL2FRACR` reader
pub type R = crate::R<PLL2FRACRrs>;
///Register `PLL2FRACR` writer
pub type W = crate::W<PLL2FRACRrs>;
///Field `PLL2FRACN` reader - Fractional part of the multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. VCO output frequency = F<sub>ref2_ck</sub> x (PLL2N + (PLL2FRACN / 2<sup>13</sup>)), with PLL2N must be between 4 and 512. PLL2FRACN can be between 0 and 2<sup>13 </sup>- 1. The input frequency F<sub>ref2_ck</sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into PLL2FRACN. Set the bit PLL2FRACEN to 1.
pub type PLL2FRACN_R = crate::FieldReader<u16>;
///Field `PLL2FRACN` writer - Fractional part of the multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. VCO output frequency = F<sub>ref2_ck</sub> x (PLL2N + (PLL2FRACN / 2<sup>13</sup>)), with PLL2N must be between 4 and 512. PLL2FRACN can be between 0 and 2<sup>13 </sup>- 1. The input frequency F<sub>ref2_ck</sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into PLL2FRACN. Set the bit PLL2FRACEN to 1.
pub type PLL2FRACN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. VCO output frequency = F<sub>ref2_ck</sub> x (PLL2N + (PLL2FRACN / 2<sup>13</sup>)), with PLL2N must be between 4 and 512. PLL2FRACN can be between 0 and 2<sup>13 </sup>- 1. The input frequency F<sub>ref2_ck</sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into PLL2FRACN. Set the bit PLL2FRACEN to 1.
    #[inline(always)]
    pub fn pll2fracn(&self) -> PLL2FRACN_R {
        PLL2FRACN_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2FRACR")
            .field("pll2fracn", &self.pll2fracn())
            .finish()
    }
}
impl W {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor. It can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO. VCO output frequency = F<sub>ref2_ck</sub> x (PLL2N + (PLL2FRACN / 2<sup>13</sup>)), with PLL2N must be between 4 and 512. PLL2FRACN can be between 0 and 2<sup>13 </sup>- 1. The input frequency F<sub>ref2_ck</sub> must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL2FRACEN to 0. Write the new fractional value into PLL2FRACN. Set the bit PLL2FRACEN to 1.
    #[inline(always)]
    pub fn pll2fracn(&mut self) -> PLL2FRACN_W<PLL2FRACRrs> {
        PLL2FRACN_W::new(self, 3)
    }
}
/**RCC PLL2 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll2fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:PLL2FRACR)*/
pub struct PLL2FRACRrs;
impl crate::RegisterSpec for PLL2FRACRrs {
    type Ux = u32;
}
///`read()` method returns [`pll2fracr::R`](R) reader structure
impl crate::Readable for PLL2FRACRrs {}
///`write(|w| ..)` method takes [`pll2fracr::W`](W) writer structure
impl crate::Writable for PLL2FRACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2FRACR to value 0
impl crate::Resettable for PLL2FRACRrs {}
