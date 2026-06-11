///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `B0OF` reader - Buffer 0 overflow flag This bit is set when an overflow occurs during the offset calculation of the buffer 0. It is cleared by writing 1 to CB0OF.
pub type B0OF_R = crate::BitReader;
///Field `B1OF` reader - Buffer 1 overflow flag This bit is set when an overflow occurs during the offset calculation of the buffer 1. It is cleared by writing 1 to CB1OF.
pub type B1OF_R = crate::BitReader;
///Field `B2OF` reader - Buffer 2 overflow flag This bit is set when an overflow occurs during the offset calculation of the buffer 2. It is cleared by writing 1 to CB2OF.
pub type B2OF_R = crate::BitReader;
///Field `B3OF` reader - Buffer 3 overflow flag This bit is set when an overflow occurs during the offset calculation of the buffer 3. It is cleared by writing 1 to CB3OF.
pub type B3OF_R = crate::BitReader;
///Field `AMEF` reader - AHB master error flag This bit is set when an AHB error happens during a transaction. It is cleared by writing 1 to CAMEF.
pub type AMEF_R = crate::BitReader;
impl R {
    ///Bit 0 - Buffer 0 overflow flag This bit is set when an overflow occurs during the offset calculation of the buffer 0. It is cleared by writing 1 to CB0OF.
    #[inline(always)]
    pub fn b0of(&self) -> B0OF_R {
        B0OF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Buffer 1 overflow flag This bit is set when an overflow occurs during the offset calculation of the buffer 1. It is cleared by writing 1 to CB1OF.
    #[inline(always)]
    pub fn b1of(&self) -> B1OF_R {
        B1OF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Buffer 2 overflow flag This bit is set when an overflow occurs during the offset calculation of the buffer 2. It is cleared by writing 1 to CB2OF.
    #[inline(always)]
    pub fn b2of(&self) -> B2OF_R {
        B2OF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Buffer 3 overflow flag This bit is set when an overflow occurs during the offset calculation of the buffer 3. It is cleared by writing 1 to CB3OF.
    #[inline(always)]
    pub fn b3of(&self) -> B3OF_R {
        B3OF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AHB master error flag This bit is set when an AHB error happens during a transaction. It is cleared by writing 1 to CAMEF.
    #[inline(always)]
    pub fn amef(&self) -> AMEF_R {
        AMEF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("b0of", &self.b0of())
            .field("b1of", &self.b1of())
            .field("b2of", &self.b2of())
            .field("b3of", &self.b3of())
            .field("amef", &self.amef())
            .finish()
    }
}
/**GFXMMU status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#GFXMMU:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
