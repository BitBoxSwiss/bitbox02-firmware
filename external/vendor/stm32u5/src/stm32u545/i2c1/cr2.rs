///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `SADD` reader - Slave address bit (master mode)
pub type SADD_R = crate::FieldReader<u16>;
///Field `SADD` writer - Slave address bit (master mode)
pub type SADD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
/**Transfer direction (master mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_WRN {
    ///0: Master requests a write transfer
    Write = 0,
    ///1: Master requests a read transfer
    Read = 1,
}
impl From<RD_WRN> for bool {
    #[inline(always)]
    fn from(variant: RD_WRN) -> Self {
        variant as u8 != 0
    }
}
///Field `RD_WRN` reader - Transfer direction (master mode)
pub type RD_WRN_R = crate::BitReader<RD_WRN>;
impl RD_WRN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RD_WRN {
        match self.bits {
            false => RD_WRN::Write,
            true => RD_WRN::Read,
        }
    }
    ///Master requests a write transfer
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == RD_WRN::Write
    }
    ///Master requests a read transfer
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == RD_WRN::Read
    }
}
///Field `RD_WRN` writer - Transfer direction (master mode)
pub type RD_WRN_W<'a, REG> = crate::BitWriter<'a, REG, RD_WRN>;
impl<'a, REG> RD_WRN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master requests a write transfer
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(RD_WRN::Write)
    }
    ///Master requests a read transfer
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(RD_WRN::Read)
    }
}
/**10-bit addressing mode (master mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD10 {
    ///0: The master operates in 7-bit addressing mode
    Bit7 = 0,
    ///1: The master operates in 10-bit addressing mode
    Bit10 = 1,
}
impl From<ADD10> for bool {
    #[inline(always)]
    fn from(variant: ADD10) -> Self {
        variant as u8 != 0
    }
}
///Field `ADD10` reader - 10-bit addressing mode (master mode)
pub type ADD10_R = crate::BitReader<ADD10>;
impl ADD10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADD10 {
        match self.bits {
            false => ADD10::Bit7,
            true => ADD10::Bit10,
        }
    }
    ///The master operates in 7-bit addressing mode
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADD10::Bit7
    }
    ///The master operates in 10-bit addressing mode
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == ADD10::Bit10
    }
}
///Field `ADD10` writer - 10-bit addressing mode (master mode)
pub type ADD10_W<'a, REG> = crate::BitWriter<'a, REG, ADD10>;
impl<'a, REG> ADD10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The master operates in 7-bit addressing mode
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(ADD10::Bit7)
    }
    ///The master operates in 10-bit addressing mode
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(ADD10::Bit10)
    }
}
/**10-bit address header only read direction (master receiver mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEAD10R {
    ///0: The master sends the complete 10 bit slave address read sequence
    Complete = 0,
    ///1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction
    Partial = 1,
}
impl From<HEAD10R> for bool {
    #[inline(always)]
    fn from(variant: HEAD10R) -> Self {
        variant as u8 != 0
    }
}
///Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode)
pub type HEAD10R_R = crate::BitReader<HEAD10R>;
impl HEAD10R_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HEAD10R {
        match self.bits {
            false => HEAD10R::Complete,
            true => HEAD10R::Partial,
        }
    }
    ///The master sends the complete 10 bit slave address read sequence
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == HEAD10R::Complete
    }
    ///The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == HEAD10R::Partial
    }
}
///Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode)
pub type HEAD10R_W<'a, REG> = crate::BitWriter<'a, REG, HEAD10R>;
impl<'a, REG> HEAD10R_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The master sends the complete 10 bit slave address read sequence
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(HEAD10R::Complete)
    }
    ///The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction
    #[inline(always)]
    pub fn partial(self) -> &'a mut crate::W<REG> {
        self.variant(HEAD10R::Partial)
    }
}
/**Start generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTR {
    ///0: No Start generation
    NoStart = 0,
    ///1: Restart/Start generation
    Start = 1,
}
impl From<STARTR> for bool {
    #[inline(always)]
    fn from(variant: STARTR) -> Self {
        variant as u8 != 0
    }
}
///Field `START` reader - Start generation
pub type START_R = crate::BitReader<STARTR>;
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STARTR {
        match self.bits {
            false => STARTR::NoStart,
            true => STARTR::Start,
        }
    }
    ///No Start generation
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == STARTR::NoStart
    }
    ///Restart/Start generation
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STARTR::Start
    }
}
/**Start generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTW {
    ///1: Restart/Start generation
    Start = 1,
}
impl From<STARTW> for bool {
    #[inline(always)]
    fn from(variant: STARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `START` writer - Start generation
pub type START_W<'a, REG> = crate::BitWriter1S<'a, REG, STARTW>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Restart/Start generation
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STARTW::Start)
    }
}
/**Stop generation (master mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPR {
    ///0: No Stop generation
    NoStop = 0,
    ///1: Stop generation after current byte transfer
    Stop = 1,
}
impl From<STOPR> for bool {
    #[inline(always)]
    fn from(variant: STOPR) -> Self {
        variant as u8 != 0
    }
}
///Field `STOP` reader - Stop generation (master mode)
pub type STOP_R = crate::BitReader<STOPR>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOPR {
        match self.bits {
            false => STOPR::NoStop,
            true => STOPR::Stop,
        }
    }
    ///No Stop generation
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPR::NoStop
    }
    ///Stop generation after current byte transfer
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPR::Stop
    }
}
/**Stop generation (master mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPW {
    ///1: Stop generation after current byte transfer
    Stop = 1,
}
impl From<STOPW> for bool {
    #[inline(always)]
    fn from(variant: STOPW) -> Self {
        variant as u8 != 0
    }
}
///Field `STOP` writer - Stop generation (master mode)
pub type STOP_W<'a, REG> = crate::BitWriter1S<'a, REG, STOPW>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop generation after current byte transfer
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(STOPW::Stop)
    }
}
/**NACK generation (slave mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKR {
    ///0: an ACK is sent after current received byte
    Ack = 0,
    ///1: a NACK is sent after current received byte
    Nack = 1,
}
impl From<NACKR> for bool {
    #[inline(always)]
    fn from(variant: NACKR) -> Self {
        variant as u8 != 0
    }
}
///Field `NACK` reader - NACK generation (slave mode)
pub type NACK_R = crate::BitReader<NACKR>;
impl NACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NACKR {
        match self.bits {
            false => NACKR::Ack,
            true => NACKR::Nack,
        }
    }
    ///an ACK is sent after current received byte
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == NACKR::Ack
    }
    ///a NACK is sent after current received byte
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACKR::Nack
    }
}
/**NACK generation (slave mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKW {
    ///1: a NACK is sent after current received byte
    Nack = 1,
}
impl From<NACKW> for bool {
    #[inline(always)]
    fn from(variant: NACKW) -> Self {
        variant as u8 != 0
    }
}
///Field `NACK` writer - NACK generation (slave mode)
pub type NACK_W<'a, REG> = crate::BitWriter1S<'a, REG, NACKW>;
impl<'a, REG> NACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///a NACK is sent after current received byte
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(NACKW::Nack)
    }
}
///Field `NBYTES` reader - Number of bytes
pub type NBYTES_R = crate::FieldReader;
///Field `NBYTES` writer - Number of bytes
pub type NBYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
/**NBYTES reload mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RELOAD {
    ///0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)
    Completed = 0,
    ///1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)
    NotCompleted = 1,
}
impl From<RELOAD> for bool {
    #[inline(always)]
    fn from(variant: RELOAD) -> Self {
        variant as u8 != 0
    }
}
///Field `RELOAD` reader - NBYTES reload mode
pub type RELOAD_R = crate::BitReader<RELOAD>;
impl RELOAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RELOAD {
        match self.bits {
            false => RELOAD::Completed,
            true => RELOAD::NotCompleted,
        }
    }
    ///The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == RELOAD::Completed
    }
    ///The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == RELOAD::NotCompleted
    }
}
///Field `RELOAD` writer - NBYTES reload mode
pub type RELOAD_W<'a, REG> = crate::BitWriter<'a, REG, RELOAD>;
impl<'a, REG> RELOAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)
    #[inline(always)]
    pub fn completed(self) -> &'a mut crate::W<REG> {
        self.variant(RELOAD::Completed)
    }
    ///The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)
    #[inline(always)]
    pub fn not_completed(self) -> &'a mut crate::W<REG> {
        self.variant(RELOAD::NotCompleted)
    }
}
/**Automatic end mode (master mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOEND {
    ///0: Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low
    Software = 0,
    ///1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred
    Automatic = 1,
}
impl From<AUTOEND> for bool {
    #[inline(always)]
    fn from(variant: AUTOEND) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTOEND` reader - Automatic end mode (master mode)
pub type AUTOEND_R = crate::BitReader<AUTOEND>;
impl AUTOEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUTOEND {
        match self.bits {
            false => AUTOEND::Software,
            true => AUTOEND::Automatic,
        }
    }
    ///Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == AUTOEND::Software
    }
    ///Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AUTOEND::Automatic
    }
}
///Field `AUTOEND` writer - Automatic end mode (master mode)
pub type AUTOEND_W<'a, REG> = crate::BitWriter<'a, REG, AUTOEND>;
impl<'a, REG> AUTOEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOEND::Software)
    }
    ///Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOEND::Automatic)
    }
}
/**Packet error checking byte

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECBYTER {
    ///0: No PEC transfer
    NoPec = 0,
    ///1: PEC transmission/reception is requested
    Pec = 1,
}
impl From<PECBYTER> for bool {
    #[inline(always)]
    fn from(variant: PECBYTER) -> Self {
        variant as u8 != 0
    }
}
///Field `PECBYTE` reader - Packet error checking byte
pub type PECBYTE_R = crate::BitReader<PECBYTER>;
impl PECBYTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PECBYTER {
        match self.bits {
            false => PECBYTER::NoPec,
            true => PECBYTER::Pec,
        }
    }
    ///No PEC transfer
    #[inline(always)]
    pub fn is_no_pec(&self) -> bool {
        *self == PECBYTER::NoPec
    }
    ///PEC transmission/reception is requested
    #[inline(always)]
    pub fn is_pec(&self) -> bool {
        *self == PECBYTER::Pec
    }
}
/**Packet error checking byte

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECBYTEW {
    ///1: PEC transmission/reception is requested
    Pec = 1,
}
impl From<PECBYTEW> for bool {
    #[inline(always)]
    fn from(variant: PECBYTEW) -> Self {
        variant as u8 != 0
    }
}
///Field `PECBYTE` writer - Packet error checking byte
pub type PECBYTE_W<'a, REG> = crate::BitWriter1S<'a, REG, PECBYTEW>;
impl<'a, REG> PECBYTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PEC transmission/reception is requested
    #[inline(always)]
    pub fn pec(self) -> &'a mut crate::W<REG> {
        self.variant(PECBYTEW::Pec)
    }
}
impl R {
    ///Bits 0:9 - Slave address bit (master mode)
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - Transfer direction (master mode)
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - 10-bit addressing mode (master mode)
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode)
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Start generation
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Stop generation (master mode)
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - NACK generation (slave mode)
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Number of bytes
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - NBYTES reload mode
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Automatic end mode (master mode)
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Packet error checking byte
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("pecbyte", &self.pecbyte())
            .field("autoend", &self.autoend())
            .field("reload", &self.reload())
            .field("nbytes", &self.nbytes())
            .field("nack", &self.nack())
            .field("stop", &self.stop())
            .field("start", &self.start())
            .field("head10r", &self.head10r())
            .field("add10", &self.add10())
            .field("rd_wrn", &self.rd_wrn())
            .field("sadd", &self.sadd())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Slave address bit (master mode)
    #[inline(always)]
    pub fn sadd(&mut self) -> SADD_W<CR2rs> {
        SADD_W::new(self, 0)
    }
    ///Bit 10 - Transfer direction (master mode)
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<CR2rs> {
        RD_WRN_W::new(self, 10)
    }
    ///Bit 11 - 10-bit addressing mode (master mode)
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W<CR2rs> {
        ADD10_W::new(self, 11)
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode)
    #[inline(always)]
    pub fn head10r(&mut self) -> HEAD10R_W<CR2rs> {
        HEAD10R_W::new(self, 12)
    }
    ///Bit 13 - Start generation
    #[inline(always)]
    pub fn start(&mut self) -> START_W<CR2rs> {
        START_W::new(self, 13)
    }
    ///Bit 14 - Stop generation (master mode)
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 14)
    }
    ///Bit 15 - NACK generation (slave mode)
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<CR2rs> {
        NACK_W::new(self, 15)
    }
    ///Bits 16:23 - Number of bytes
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W<CR2rs> {
        NBYTES_W::new(self, 16)
    }
    ///Bit 24 - NBYTES reload mode
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<CR2rs> {
        RELOAD_W::new(self, 24)
    }
    ///Bit 25 - Automatic end mode (master mode)
    #[inline(always)]
    pub fn autoend(&mut self) -> AUTOEND_W<CR2rs> {
        AUTOEND_W::new(self, 25)
    }
    ///Bit 26 - Packet error checking byte
    #[inline(always)]
    pub fn pecbyte(&mut self) -> PECBYTE_W<CR2rs> {
        PECBYTE_W::new(self, 26)
    }
}
/**Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#I2C1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0400_e000;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
