///Register `NSBOOTADD1R` reader
pub type R = crate::R<NSBOOTADD1Rrs>;
///Register `NSBOOTADD1R` writer
pub type W = crate::W<NSBOOTADD1Rrs>;
///Field `NSBOOTADD1` reader - Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\[24:0\] = 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\[24:0\] = 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\[24:0\] = 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)
pub type NSBOOTADD1_R = crate::FieldReader<u32>;
///Field `NSBOOTADD1` writer - Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\[24:0\] = 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\[24:0\] = 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\[24:0\] = 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)
pub type NSBOOTADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 7:31 - Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\[24:0\] = 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\[24:0\] = 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\[24:0\] = 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)
    #[inline(always)]
    pub fn nsbootadd1(&self) -> NSBOOTADD1_R {
        NSBOOTADD1_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSBOOTADD1R")
            .field("nsbootadd1", &self.nsbootadd1())
            .finish()
    }
}
impl W {
    ///Bits 7:31 - Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\[24:0\] = 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\[24:0\] = 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\[24:0\] = 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)
    #[inline(always)]
    pub fn nsbootadd1(&mut self) -> NSBOOTADD1_W<NSBOOTADD1Rrs> {
        NSBOOTADD1_W::new(self, 7)
    }
}
/**FLASH non-secure boot address 1 register

You can [`read`](crate::Reg::read) this register and get [`nsbootadd1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootadd1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FLASH:NSBOOTADD1R)*/
pub struct NSBOOTADD1Rrs;
impl crate::RegisterSpec for NSBOOTADD1Rrs {
    type Ux = u32;
}
///`read()` method returns [`nsbootadd1r::R`](R) reader structure
impl crate::Readable for NSBOOTADD1Rrs {}
///`write(|w| ..)` method takes [`nsbootadd1r::W`](W) writer structure
impl crate::Writable for NSBOOTADD1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSBOOTADD1R to value 0x0f
impl crate::Resettable for NSBOOTADD1Rrs {
    const RESET_VALUE: u32 = 0x0f;
}
