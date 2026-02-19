///Register `BTR%s` reader
pub type R = crate::R<BTRrs>;
///Register `BTR%s` writer
pub type W = crate::W<BTRrs>;
///Field `ADDSET` reader - Address setup phase duration
pub type ADDSET_R = crate::FieldReader;
///Field `ADDSET` writer - Address setup phase duration
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `ADDHLD` reader - Address-hold phase duration
pub type ADDHLD_R = crate::FieldReader;
///Field `ADDHLD` writer - Address-hold phase duration
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATAST` reader - Data-phase duration
pub type DATAST_R = crate::FieldReader;
///Field `DATAST` writer - Data-phase duration
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BUSTURN` reader - Bus turnaround phase duration
pub type BUSTURN_R = crate::FieldReader;
///Field `BUSTURN` writer - Bus turnaround phase duration
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `CLKDIV` reader - Clock divide ratio (for FMC_CLK signal)
pub type CLKDIV_R = crate::FieldReader;
///Field `CLKDIV` writer - Clock divide ratio (for FMC_CLK signal)
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATLAT` reader - Data latency for synchronous memory
pub type DATLAT_R = crate::FieldReader;
///Field `DATLAT` writer - Data latency for synchronous memory
pub type DATLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**Access mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACCMOD {
    ///0: Access mode A
    A = 0,
    ///1: Access mode B
    B = 1,
    ///2: Access mode C
    C = 2,
    ///3: Access mode D
    D = 3,
}
impl From<ACCMOD> for u8 {
    #[inline(always)]
    fn from(variant: ACCMOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACCMOD {
    type Ux = u8;
}
impl crate::IsEnum for ACCMOD {}
///Field `ACCMOD` reader - Access mode
pub type ACCMOD_R = crate::FieldReader<ACCMOD>;
impl ACCMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACCMOD {
        match self.bits {
            0 => ACCMOD::A,
            1 => ACCMOD::B,
            2 => ACCMOD::C,
            3 => ACCMOD::D,
            _ => unreachable!(),
        }
    }
    ///Access mode A
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == ACCMOD::A
    }
    ///Access mode B
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == ACCMOD::B
    }
    ///Access mode C
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == ACCMOD::C
    }
    ///Access mode D
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == ACCMOD::D
    }
}
///Field `ACCMOD` writer - Access mode
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ACCMOD, crate::Safe>;
impl<'a, REG> ACCMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Access mode A
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(ACCMOD::A)
    }
    ///Access mode B
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(ACCMOD::B)
    }
    ///Access mode C
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(ACCMOD::C)
    }
    ///Access mode D
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(ACCMOD::D)
    }
}
///Field `DATAHLD` reader - Data hold phase duration
pub type DATAHLD_R = crate::FieldReader;
///Field `DATAHLD` writer - Data hold phase duration
pub type DATAHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - Address setup phase duration
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Address-hold phase duration
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15 - Data-phase duration
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Bus turnaround phase duration
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Clock divide ratio (for FMC_CLK signal)
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Data latency for synchronous memory
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:29 - Access mode
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Data hold phase duration
    #[inline(always)]
    pub fn datahld(&self) -> DATAHLD_R {
        DATAHLD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTR")
            .field("addset", &self.addset())
            .field("addhld", &self.addhld())
            .field("datast", &self.datast())
            .field("busturn", &self.busturn())
            .field("clkdiv", &self.clkdiv())
            .field("datlat", &self.datlat())
            .field("accmod", &self.accmod())
            .field("datahld", &self.datahld())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Address setup phase duration
    #[inline(always)]
    pub fn addset(&mut self) -> ADDSET_W<BTRrs> {
        ADDSET_W::new(self, 0)
    }
    ///Bits 4:7 - Address-hold phase duration
    #[inline(always)]
    pub fn addhld(&mut self) -> ADDHLD_W<BTRrs> {
        ADDHLD_W::new(self, 4)
    }
    ///Bits 8:15 - Data-phase duration
    #[inline(always)]
    pub fn datast(&mut self) -> DATAST_W<BTRrs> {
        DATAST_W::new(self, 8)
    }
    ///Bits 16:19 - Bus turnaround phase duration
    #[inline(always)]
    pub fn busturn(&mut self) -> BUSTURN_W<BTRrs> {
        BUSTURN_W::new(self, 16)
    }
    ///Bits 20:23 - Clock divide ratio (for FMC_CLK signal)
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<BTRrs> {
        CLKDIV_W::new(self, 20)
    }
    ///Bits 24:27 - Data latency for synchronous memory
    #[inline(always)]
    pub fn datlat(&mut self) -> DATLAT_W<BTRrs> {
        DATLAT_W::new(self, 24)
    }
    ///Bits 28:29 - Access mode
    #[inline(always)]
    pub fn accmod(&mut self) -> ACCMOD_W<BTRrs> {
        ACCMOD_W::new(self, 28)
    }
    ///Bits 30:31 - Data hold phase duration
    #[inline(always)]
    pub fn datahld(&mut self) -> DATAHLD_W<BTRrs> {
        DATAHLD_W::new(self, 30)
    }
}
/**SRAM/NOR-Flash chip-select timing register for bank %s

You can [`read`](crate::Reg::read) this register and get [`btr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#FMC:BTR[1])*/
pub struct BTRrs;
impl crate::RegisterSpec for BTRrs {
    type Ux = u32;
}
///`read()` method returns [`btr::R`](R) reader structure
impl crate::Readable for BTRrs {}
///`write(|w| ..)` method takes [`btr::W`](W) writer structure
impl crate::Writable for BTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BTR%s to value 0x0fff_ffff
impl crate::Resettable for BTRrs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
