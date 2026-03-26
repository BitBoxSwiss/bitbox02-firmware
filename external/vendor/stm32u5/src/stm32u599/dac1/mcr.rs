///Register `MCR` reader
pub type R = crate::R<MCRrs>;
///Register `MCR` writer
pub type W = crate::W<MCRrs>;
/**DAC channel%s mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1 {
    ///0: Normal mode - DAC channelx is connected to external pin with Buffer enabled
    NormalPinBuffer = 0,
    ///1: Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    NormalPinChipBuffer = 1,
    ///2: Normal mode - DAC channelx is connected to external pin with Buffer disabled
    NormalPinNoBuffer = 2,
    ///3: Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    NormalChipNoBuffer = 3,
    ///4: S&H mode - DAC channelx is connected to external pin with Buffer enabled
    ShpinBuffer = 4,
    ///5: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    ShpinChipBuffer = 5,
    ///6: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
    ShpinNoBuffer = 6,
    ///7: S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    ShchipNoBuffer = 7,
}
impl From<MODE1> for u8 {
    #[inline(always)]
    fn from(variant: MODE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE1 {
    type Ux = u8;
}
impl crate::IsEnum for MODE1 {}
///Field `MODE(1-2)` reader - DAC channel%s mode
pub type MODE_R = crate::FieldReader<MODE1>;
impl MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE1 {
        match self.bits {
            0 => MODE1::NormalPinBuffer,
            1 => MODE1::NormalPinChipBuffer,
            2 => MODE1::NormalPinNoBuffer,
            3 => MODE1::NormalChipNoBuffer,
            4 => MODE1::ShpinBuffer,
            5 => MODE1::ShpinChipBuffer,
            6 => MODE1::ShpinNoBuffer,
            7 => MODE1::ShchipNoBuffer,
            _ => unreachable!(),
        }
    }
    ///Normal mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn is_normal_pin_buffer(&self) -> bool {
        *self == MODE1::NormalPinBuffer
    }
    ///Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn is_normal_pin_chip_buffer(&self) -> bool {
        *self == MODE1::NormalPinChipBuffer
    }
    ///Normal mode - DAC channelx is connected to external pin with Buffer disabled
    #[inline(always)]
    pub fn is_normal_pin_no_buffer(&self) -> bool {
        *self == MODE1::NormalPinNoBuffer
    }
    ///Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn is_normal_chip_no_buffer(&self) -> bool {
        *self == MODE1::NormalChipNoBuffer
    }
    ///S&H mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn is_shpin_buffer(&self) -> bool {
        *self == MODE1::ShpinBuffer
    }
    ///S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn is_shpin_chip_buffer(&self) -> bool {
        *self == MODE1::ShpinChipBuffer
    }
    ///S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn is_shpin_no_buffer(&self) -> bool {
        *self == MODE1::ShpinNoBuffer
    }
    ///S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn is_shchip_no_buffer(&self) -> bool {
        *self == MODE1::ShchipNoBuffer
    }
}
///Field `MODE(1-2)` writer - DAC channel%s mode
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE1, crate::Safe>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Normal mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn normal_pin_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalPinBuffer)
    }
    ///Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn normal_pin_chip_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalPinChipBuffer)
    }
    ///Normal mode - DAC channelx is connected to external pin with Buffer disabled
    #[inline(always)]
    pub fn normal_pin_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalPinNoBuffer)
    }
    ///Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn normal_chip_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::NormalChipNoBuffer)
    }
    ///S&H mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn shpin_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShpinBuffer)
    }
    ///S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn shpin_chip_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShpinChipBuffer)
    }
    ///S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn shpin_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShpinNoBuffer)
    }
    ///S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn shchip_no_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::ShchipNoBuffer)
    }
}
/**DAC channel%s DMA double data mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMADOUBLE1 {
    ///0: DMA Normal mode selected
    Normal = 0,
    ///1: DMA Double data mode selected
    DoubleData = 1,
}
impl From<DMADOUBLE1> for bool {
    #[inline(always)]
    fn from(variant: DMADOUBLE1) -> Self {
        variant as u8 != 0
    }
}
///Field `DMADOUBLE(1-2)` reader - DAC channel%s DMA double data mode
pub type DMADOUBLE_R = crate::BitReader<DMADOUBLE1>;
impl DMADOUBLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMADOUBLE1 {
        match self.bits {
            false => DMADOUBLE1::Normal,
            true => DMADOUBLE1::DoubleData,
        }
    }
    ///DMA Normal mode selected
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DMADOUBLE1::Normal
    }
    ///DMA Double data mode selected
    #[inline(always)]
    pub fn is_double_data(&self) -> bool {
        *self == DMADOUBLE1::DoubleData
    }
}
///Field `DMADOUBLE(1-2)` writer - DAC channel%s DMA double data mode
pub type DMADOUBLE_W<'a, REG> = crate::BitWriter<'a, REG, DMADOUBLE1>;
impl<'a, REG> DMADOUBLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA Normal mode selected
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(DMADOUBLE1::Normal)
    }
    ///DMA Double data mode selected
    #[inline(always)]
    pub fn double_data(self) -> &'a mut crate::W<REG> {
        self.variant(DMADOUBLE1::DoubleData)
    }
}
/**Enable signed format for DAC channel%s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINFORMAT1 {
    ///0: Input data is in unsigned format
    Unsigned = 0,
    ///1: Input data is in signed format (2's complement). The MSB bit represents the sign.
    Signed = 1,
}
impl From<SINFORMAT1> for bool {
    #[inline(always)]
    fn from(variant: SINFORMAT1) -> Self {
        variant as u8 != 0
    }
}
///Field `SINFORMAT(1-2)` reader - Enable signed format for DAC channel%s
pub type SINFORMAT_R = crate::BitReader<SINFORMAT1>;
impl SINFORMAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SINFORMAT1 {
        match self.bits {
            false => SINFORMAT1::Unsigned,
            true => SINFORMAT1::Signed,
        }
    }
    ///Input data is in unsigned format
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        *self == SINFORMAT1::Unsigned
    }
    ///Input data is in signed format (2's complement). The MSB bit represents the sign.
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        *self == SINFORMAT1::Signed
    }
}
///Field `SINFORMAT(1-2)` writer - Enable signed format for DAC channel%s
pub type SINFORMAT_W<'a, REG> = crate::BitWriter<'a, REG, SINFORMAT1>;
impl<'a, REG> SINFORMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input data is in unsigned format
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut crate::W<REG> {
        self.variant(SINFORMAT1::Unsigned)
    }
    ///Input data is in signed format (2's complement). The MSB bit represents the sign.
    #[inline(always)]
    pub fn signed(self) -> &'a mut crate::W<REG> {
        self.variant(SINFORMAT1::Signed)
    }
}
/**High frequency interface mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HFSEL {
    ///0: High frequency interface mode disabled
    Disabled = 0,
    ///1: High frequency interface mode enabled for AHB clock frequency > 80 MHz
    More80mhz = 1,
    ///2: High frequency interface mode enabled for AHB clock frequency >160 MHz
    More160mhz = 2,
}
impl From<HFSEL> for u8 {
    #[inline(always)]
    fn from(variant: HFSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HFSEL {
    type Ux = u8;
}
impl crate::IsEnum for HFSEL {}
///Field `HFSEL` reader - High frequency interface mode selection
pub type HFSEL_R = crate::FieldReader<HFSEL>;
impl HFSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<HFSEL> {
        match self.bits {
            0 => Some(HFSEL::Disabled),
            1 => Some(HFSEL::More80mhz),
            2 => Some(HFSEL::More160mhz),
            _ => None,
        }
    }
    ///High frequency interface mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HFSEL::Disabled
    }
    ///High frequency interface mode enabled for AHB clock frequency > 80 MHz
    #[inline(always)]
    pub fn is_more80mhz(&self) -> bool {
        *self == HFSEL::More80mhz
    }
    ///High frequency interface mode enabled for AHB clock frequency >160 MHz
    #[inline(always)]
    pub fn is_more160mhz(&self) -> bool {
        *self == HFSEL::More160mhz
    }
}
///Field `HFSEL` writer - High frequency interface mode selection
pub type HFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HFSEL>;
impl<'a, REG> HFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///High frequency interface mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HFSEL::Disabled)
    }
    ///High frequency interface mode enabled for AHB clock frequency > 80 MHz
    #[inline(always)]
    pub fn more80mhz(self) -> &'a mut crate::W<REG> {
        self.variant(HFSEL::More80mhz)
    }
    ///High frequency interface mode enabled for AHB clock frequency >160 MHz
    #[inline(always)]
    pub fn more160mhz(self) -> &'a mut crate::W<REG> {
        self.variant(HFSEL::More160mhz)
    }
}
impl R {
    ///DAC channel(1-2) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODE1` field.</div>
    #[inline(always)]
    pub fn mode(&self, n: u8) -> MODE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MODE_R::new(((self.bits >> (n * 16)) & 7) as u8)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) mode
    #[inline(always)]
    pub fn mode_iter(&self) -> impl Iterator<Item = MODE_R> + '_ {
        (0..2).map(move |n| MODE_R::new(((self.bits >> (n * 16)) & 7) as u8))
    }
    ///Bits 0:2 - DAC channel1 mode
    #[inline(always)]
    pub fn mode1(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    ///Bits 16:18 - DAC channel2 mode
    #[inline(always)]
    pub fn mode2(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///DAC channel(1-2) DMA double data mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DMADOUBLE1` field.</div>
    #[inline(always)]
    pub fn dmadouble(&self, n: u8) -> DMADOUBLE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DMADOUBLE_R::new(((self.bits >> (n * 16 + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///DAC channel(1-2) DMA double data mode
    #[inline(always)]
    pub fn dmadouble_iter(&self) -> impl Iterator<Item = DMADOUBLE_R> + '_ {
        (0..2).map(move |n| DMADOUBLE_R::new(((self.bits >> (n * 16 + 8)) & 1) != 0))
    }
    ///Bit 8 - DAC channel1 DMA double data mode
    #[inline(always)]
    pub fn dmadouble1(&self) -> DMADOUBLE_R {
        DMADOUBLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 24 - DAC channel2 DMA double data mode
    #[inline(always)]
    pub fn dmadouble2(&self) -> DMADOUBLE_R {
        DMADOUBLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Enable signed format for DAC channel(1-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SINFORMAT1` field.</div>
    #[inline(always)]
    pub fn sinformat(&self, n: u8) -> SINFORMAT_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SINFORMAT_R::new(((self.bits >> (n * 16 + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Enable signed format for DAC channel(1-2)
    #[inline(always)]
    pub fn sinformat_iter(&self) -> impl Iterator<Item = SINFORMAT_R> + '_ {
        (0..2).map(move |n| SINFORMAT_R::new(((self.bits >> (n * 16 + 9)) & 1) != 0))
    }
    ///Bit 9 - Enable signed format for DAC channel1
    #[inline(always)]
    pub fn sinformat1(&self) -> SINFORMAT_R {
        SINFORMAT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 25 - Enable signed format for DAC channel2
    #[inline(always)]
    pub fn sinformat2(&self) -> SINFORMAT_R {
        SINFORMAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 14:15 - High frequency interface mode selection
    #[inline(always)]
    pub fn hfsel(&self) -> HFSEL_R {
        HFSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR")
            .field("mode1", &self.mode1())
            .field("mode2", &self.mode2())
            .field("dmadouble1", &self.dmadouble1())
            .field("dmadouble2", &self.dmadouble2())
            .field("sinformat1", &self.sinformat1())
            .field("sinformat2", &self.sinformat2())
            .field("hfsel", &self.hfsel())
            .finish()
    }
}
impl W {
    ///DAC channel(1-2) mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODE1` field.</div>
    #[inline(always)]
    pub fn mode(&mut self, n: u8) -> MODE_W<MCRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MODE_W::new(self, n * 16)
    }
    ///Bits 0:2 - DAC channel1 mode
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE_W<MCRrs> {
        MODE_W::new(self, 0)
    }
    ///Bits 16:18 - DAC channel2 mode
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE_W<MCRrs> {
        MODE_W::new(self, 16)
    }
    ///DAC channel(1-2) DMA double data mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DMADOUBLE1` field.</div>
    #[inline(always)]
    pub fn dmadouble(&mut self, n: u8) -> DMADOUBLE_W<MCRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DMADOUBLE_W::new(self, n * 16 + 8)
    }
    ///Bit 8 - DAC channel1 DMA double data mode
    #[inline(always)]
    pub fn dmadouble1(&mut self) -> DMADOUBLE_W<MCRrs> {
        DMADOUBLE_W::new(self, 8)
    }
    ///Bit 24 - DAC channel2 DMA double data mode
    #[inline(always)]
    pub fn dmadouble2(&mut self) -> DMADOUBLE_W<MCRrs> {
        DMADOUBLE_W::new(self, 24)
    }
    ///Enable signed format for DAC channel(1-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SINFORMAT1` field.</div>
    #[inline(always)]
    pub fn sinformat(&mut self, n: u8) -> SINFORMAT_W<MCRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SINFORMAT_W::new(self, n * 16 + 9)
    }
    ///Bit 9 - Enable signed format for DAC channel1
    #[inline(always)]
    pub fn sinformat1(&mut self) -> SINFORMAT_W<MCRrs> {
        SINFORMAT_W::new(self, 9)
    }
    ///Bit 25 - Enable signed format for DAC channel2
    #[inline(always)]
    pub fn sinformat2(&mut self) -> SINFORMAT_W<MCRrs> {
        SINFORMAT_W::new(self, 25)
    }
    ///Bits 14:15 - High frequency interface mode selection
    #[inline(always)]
    pub fn hfsel(&mut self) -> HFSEL_W<MCRrs> {
        HFSEL_W::new(self, 14)
    }
}
/**DAC mode control register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DAC1:MCR)*/
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
///`read()` method returns [`mcr::R`](R) reader structure
impl crate::Readable for MCRrs {}
///`write(|w| ..)` method takes [`mcr::W`](W) writer structure
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCRrs {}
