///Register `SECBOOTADD0R` reader
pub type R = crate::R<SECBOOTADD0Rrs>;
///Register `SECBOOTADD0R` writer
pub type W = crate::W<SECBOOTADD0Rrs>;
///Field `BOOT_LOCK` reader - Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\[24:0\] option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0.
pub type BOOT_LOCK_R = crate::BitReader;
///Field `BOOT_LOCK` writer - Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\[24:0\] option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0.
pub type BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECBOOTADD0` reader - Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \[31:7\] The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\[24:0\] = 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\[24:0\] = 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\[24:0\] = 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)
pub type SECBOOTADD0_R = crate::FieldReader<u32>;
///Field `SECBOOTADD0` writer - Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \[31:7\] The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\[24:0\] = 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\[24:0\] = 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\[24:0\] = 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)
pub type SECBOOTADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bit 0 - Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\[24:0\] option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0.
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bits 7:31 - Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \[31:7\] The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\[24:0\] = 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\[24:0\] = 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\[24:0\] = 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)
    #[inline(always)]
    pub fn secbootadd0(&self) -> SECBOOTADD0_R {
        SECBOOTADD0_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECBOOTADD0R")
            .field("boot_lock", &self.boot_lock())
            .field("secbootadd0", &self.secbootadd0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\[24:0\] option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0.
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<SECBOOTADD0Rrs> {
        BOOT_LOCK_W::new(self, 0)
    }
    ///Bits 7:31 - Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \[31:7\] The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\[24:0\] = 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\[24:0\] = 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\[24:0\] = 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)
    #[inline(always)]
    pub fn secbootadd0(&mut self) -> SECBOOTADD0_W<SECBOOTADD0Rrs> {
        SECBOOTADD0_W::new(self, 7)
    }
}
/**FLASH secure boot address 0 register

You can [`read`](crate::Reg::read) this register and get [`secbootadd0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbootadd0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FLASH:SECBOOTADD0R)*/
pub struct SECBOOTADD0Rrs;
impl crate::RegisterSpec for SECBOOTADD0Rrs {
    type Ux = u32;
}
///`read()` method returns [`secbootadd0r::R`](R) reader structure
impl crate::Readable for SECBOOTADD0Rrs {}
///`write(|w| ..)` method takes [`secbootadd0r::W`](W) writer structure
impl crate::Writable for SECBOOTADD0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECBOOTADD0R to value 0
impl crate::Resettable for SECBOOTADD0Rrs {}
