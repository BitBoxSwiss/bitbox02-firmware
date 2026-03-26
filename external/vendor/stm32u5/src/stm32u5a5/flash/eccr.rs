///Register `ECCR` reader
pub type R = crate::R<ECCRrs>;
///Register `ECCR` writer
pub type W = crate::W<ECCRrs>;
///Field `ADDR_ECC` reader - ECC fail address This field indicates which address is concerned by the ECC error correction or by the double ECC error detection. The address is given by bank from address 0x0 0000 to 0x1F FFF0.
pub type ADDR_ECC_R = crate::FieldReader<u32>;
///Field `BK_ECC` reader - ECC fail bank This bit indicates which bank is concerned by the ECC error correction or by the double ECC error detection.
pub type BK_ECC_R = crate::BitReader;
///Field `SYSF_ECC` reader - System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory.
pub type SYSF_ECC_R = crate::BitReader;
///Field `ECCIE` reader - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set.
pub type ECCIE_R = crate::BitReader;
///Field `ECCIE` writer - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set.
pub type ECCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCC` reader - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1.
pub type ECCC_R = crate::BitReader;
///Field `ECCC` writer - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1.
pub type ECCC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCD` reader - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1.
pub type ECCD_R = crate::BitReader;
///Field `ECCD` writer - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1.
pub type ECCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:20 - ECC fail address This field indicates which address is concerned by the ECC error correction or by the double ECC error detection. The address is given by bank from address 0x0 0000 to 0x1F FFF0.
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new(self.bits & 0x001f_ffff)
    }
    ///Bit 21 - ECC fail bank This bit indicates which bank is concerned by the ECC error correction or by the double ECC error detection.
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory.
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set.
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 30 - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCR")
            .field("addr_ecc", &self.addr_ecc())
            .field("bk_ecc", &self.bk_ecc())
            .field("sysf_ecc", &self.sysf_ecc())
            .field("eccie", &self.eccie())
            .field("eccc", &self.eccc())
            .field("eccd", &self.eccd())
            .finish()
    }
}
impl W {
    ///Bit 24 - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set.
    #[inline(always)]
    pub fn eccie(&mut self) -> ECCIE_W<ECCRrs> {
        ECCIE_W::new(self, 24)
    }
    ///Bit 30 - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn eccc(&mut self) -> ECCC_W<ECCRrs> {
        ECCC_W::new(self, 30)
    }
    ///Bit 31 - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn eccd(&mut self) -> ECCD_W<ECCRrs> {
        ECCD_W::new(self, 31)
    }
}
/**FLASH ECC register

You can [`read`](crate::Reg::read) this register and get [`eccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FLASH:ECCR)*/
pub struct ECCRrs;
impl crate::RegisterSpec for ECCRrs {
    type Ux = u32;
}
///`read()` method returns [`eccr::R`](R) reader structure
impl crate::Readable for ECCRrs {}
///`write(|w| ..)` method takes [`eccr::W`](W) writer structure
impl crate::Writable for ECCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECCR to value 0
impl crate::Resettable for ECCRrs {}
