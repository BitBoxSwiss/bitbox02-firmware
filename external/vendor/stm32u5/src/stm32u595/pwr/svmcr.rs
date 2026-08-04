///Register `SVMCR` reader
pub type R = crate::R<SVMCRrs>;
///Register `SVMCR` writer
pub type W = crate::W<SVMCRrs>;
/**Power voltage detector enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE {
    ///0: PVD disabled
    Disabled = 0,
    ///1: PVD enabled
    Enabled = 1,
}
impl From<PVDE> for bool {
    #[inline(always)]
    fn from(variant: PVDE) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader<PVDE>;
impl PVDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDE {
        match self.bits {
            false => PVDE::Disabled,
            true => PVDE::Enabled,
        }
    }
    ///PVD disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVDE::Disabled
    }
    ///PVD enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVDE::Enabled
    }
}
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG, PVDE>;
impl<'a, REG> PVDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVD disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Disabled)
    }
    ///PVD enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Enabled)
    }
}
/**Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PVDLS {
    ///0: VPVD0 around 2.0 V
    Vpvd0 = 0,
    ///1: VPVD1 around 2.2 V
    Vpvd1 = 1,
    ///2: VPVD2 around 2.4 V
    Vpvd2 = 2,
    ///3: VPVD3 around 2.5 V
    Vpvd3 = 3,
    ///4: VPVD4 around 2.6 V
    Vpvd4 = 4,
    ///5: VPVD5 around 2.8 V
    Vpvd5 = 5,
    ///6: VPVD6 around 2.9 V
    Vpvd6 = 6,
    ///7: External input analog voltage PVD_IN (compared internally to VREFINT)
    Pvdin = 7,
}
impl From<PVDLS> for u8 {
    #[inline(always)]
    fn from(variant: PVDLS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PVDLS {
    type Ux = u8;
}
impl crate::IsEnum for PVDLS {}
///Field `PVDLS` reader - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
pub type PVDLS_R = crate::FieldReader<PVDLS>;
impl PVDLS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDLS {
        match self.bits {
            0 => PVDLS::Vpvd0,
            1 => PVDLS::Vpvd1,
            2 => PVDLS::Vpvd2,
            3 => PVDLS::Vpvd3,
            4 => PVDLS::Vpvd4,
            5 => PVDLS::Vpvd5,
            6 => PVDLS::Vpvd6,
            7 => PVDLS::Pvdin,
            _ => unreachable!(),
        }
    }
    ///VPVD0 around 2.0 V
    #[inline(always)]
    pub fn is_vpvd0(&self) -> bool {
        *self == PVDLS::Vpvd0
    }
    ///VPVD1 around 2.2 V
    #[inline(always)]
    pub fn is_vpvd1(&self) -> bool {
        *self == PVDLS::Vpvd1
    }
    ///VPVD2 around 2.4 V
    #[inline(always)]
    pub fn is_vpvd2(&self) -> bool {
        *self == PVDLS::Vpvd2
    }
    ///VPVD3 around 2.5 V
    #[inline(always)]
    pub fn is_vpvd3(&self) -> bool {
        *self == PVDLS::Vpvd3
    }
    ///VPVD4 around 2.6 V
    #[inline(always)]
    pub fn is_vpvd4(&self) -> bool {
        *self == PVDLS::Vpvd4
    }
    ///VPVD5 around 2.8 V
    #[inline(always)]
    pub fn is_vpvd5(&self) -> bool {
        *self == PVDLS::Vpvd5
    }
    ///VPVD6 around 2.9 V
    #[inline(always)]
    pub fn is_vpvd6(&self) -> bool {
        *self == PVDLS::Vpvd6
    }
    ///External input analog voltage PVD_IN (compared internally to VREFINT)
    #[inline(always)]
    pub fn is_pvdin(&self) -> bool {
        *self == PVDLS::Pvdin
    }
}
///Field `PVDLS` writer - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
pub type PVDLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PVDLS, crate::Safe>;
impl<'a, REG> PVDLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///VPVD0 around 2.0 V
    #[inline(always)]
    pub fn vpvd0(self) -> &'a mut crate::W<REG> {
        self.variant(PVDLS::Vpvd0)
    }
    ///VPVD1 around 2.2 V
    #[inline(always)]
    pub fn vpvd1(self) -> &'a mut crate::W<REG> {
        self.variant(PVDLS::Vpvd1)
    }
    ///VPVD2 around 2.4 V
    #[inline(always)]
    pub fn vpvd2(self) -> &'a mut crate::W<REG> {
        self.variant(PVDLS::Vpvd2)
    }
    ///VPVD3 around 2.5 V
    #[inline(always)]
    pub fn vpvd3(self) -> &'a mut crate::W<REG> {
        self.variant(PVDLS::Vpvd3)
    }
    ///VPVD4 around 2.6 V
    #[inline(always)]
    pub fn vpvd4(self) -> &'a mut crate::W<REG> {
        self.variant(PVDLS::Vpvd4)
    }
    ///VPVD5 around 2.8 V
    #[inline(always)]
    pub fn vpvd5(self) -> &'a mut crate::W<REG> {
        self.variant(PVDLS::Vpvd5)
    }
    ///VPVD6 around 2.9 V
    #[inline(always)]
    pub fn vpvd6(self) -> &'a mut crate::W<REG> {
        self.variant(PVDLS::Vpvd6)
    }
    ///External input analog voltage PVD_IN (compared internally to VREFINT)
    #[inline(always)]
    pub fn pvdin(self) -> &'a mut crate::W<REG> {
        self.variant(PVDLS::Pvdin)
    }
}
/**VDDUSB independent USB voltage monitor enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UVMEN {
    ///0: VDDUSB voltage monitor disabled
    Disabled = 0,
    ///1: VDDUSB voltage monitor enabled
    Enabled = 1,
}
impl From<UVMEN> for bool {
    #[inline(always)]
    fn from(variant: UVMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `UVMEN` reader - VDDUSB independent USB voltage monitor enable
pub type UVMEN_R = crate::BitReader<UVMEN>;
impl UVMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UVMEN {
        match self.bits {
            false => UVMEN::Disabled,
            true => UVMEN::Enabled,
        }
    }
    ///VDDUSB voltage monitor disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UVMEN::Disabled
    }
    ///VDDUSB voltage monitor enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UVMEN::Enabled
    }
}
///Field `UVMEN` writer - VDDUSB independent USB voltage monitor enable
pub type UVMEN_W<'a, REG> = crate::BitWriter<'a, REG, UVMEN>;
impl<'a, REG> UVMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VDDUSB voltage monitor disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UVMEN::Disabled)
    }
    ///VDDUSB voltage monitor enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UVMEN::Enabled)
    }
}
/**VDDIO2 independent I/Os voltage monitor enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IO2VMEN {
    ///0: VDDIO2 voltage monitor disabled
    Disabled = 0,
    ///1: VDDIO2 voltage monitor enabled
    Enabled = 1,
}
impl From<IO2VMEN> for bool {
    #[inline(always)]
    fn from(variant: IO2VMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `IO2VMEN` reader - VDDIO2 independent I/Os voltage monitor enable
pub type IO2VMEN_R = crate::BitReader<IO2VMEN>;
impl IO2VMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IO2VMEN {
        match self.bits {
            false => IO2VMEN::Disabled,
            true => IO2VMEN::Enabled,
        }
    }
    ///VDDIO2 voltage monitor disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IO2VMEN::Disabled
    }
    ///VDDIO2 voltage monitor enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IO2VMEN::Enabled
    }
}
///Field `IO2VMEN` writer - VDDIO2 independent I/Os voltage monitor enable
pub type IO2VMEN_W<'a, REG> = crate::BitWriter<'a, REG, IO2VMEN>;
impl<'a, REG> IO2VMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VDDIO2 voltage monitor disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IO2VMEN::Disabled)
    }
    ///VDDIO2 voltage monitor enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IO2VMEN::Enabled)
    }
}
/**VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVM1EN {
    ///0: VDDA voltage monitor 1 disabled
    Disabled = 0,
    ///1: VDDA voltage monitor 1 enabled
    Enabled = 1,
}
impl From<AVM1EN> for bool {
    #[inline(always)]
    fn from(variant: AVM1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `AVM1EN` reader - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
pub type AVM1EN_R = crate::BitReader<AVM1EN>;
impl AVM1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AVM1EN {
        match self.bits {
            false => AVM1EN::Disabled,
            true => AVM1EN::Enabled,
        }
    }
    ///VDDA voltage monitor 1 disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AVM1EN::Disabled
    }
    ///VDDA voltage monitor 1 enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AVM1EN::Enabled
    }
}
///Field `AVM1EN` writer - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
pub type AVM1EN_W<'a, REG> = crate::BitWriter<'a, REG, AVM1EN>;
impl<'a, REG> AVM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VDDA voltage monitor 1 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AVM1EN::Disabled)
    }
    ///VDDA voltage monitor 1 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AVM1EN::Enabled)
    }
}
/**VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVM2EN {
    ///0: VDDA voltage monitor 2 disabled
    Disabled = 0,
    ///1: VDDA voltage monitor 2 enabled
    Enabled = 1,
}
impl From<AVM2EN> for bool {
    #[inline(always)]
    fn from(variant: AVM2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `AVM2EN` reader - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
pub type AVM2EN_R = crate::BitReader<AVM2EN>;
impl AVM2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AVM2EN {
        match self.bits {
            false => AVM2EN::Disabled,
            true => AVM2EN::Enabled,
        }
    }
    ///VDDA voltage monitor 2 disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AVM2EN::Disabled
    }
    ///VDDA voltage monitor 2 enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AVM2EN::Enabled
    }
}
///Field `AVM2EN` writer - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
pub type AVM2EN_W<'a, REG> = crate::BitWriter<'a, REG, AVM2EN>;
impl<'a, REG> AVM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VDDA voltage monitor 2 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AVM2EN::Disabled)
    }
    ///VDDA voltage monitor 2 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AVM2EN::Enabled)
    }
}
/**VDDUSB independent USB supply valid This bit is used to validate the VDDUSB supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB OTG peripheral. If VDDUSB is not always present in the application, the VDDUSB voltage monitor can be used to determine whether this supply is ready or not.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USV {
    ///0: VDDUSB not present: logical and electrical isolation is applied to ignore this supply
    NotPresent = 0,
    ///1: VDDUSB valid
    Present = 1,
}
impl From<USV> for bool {
    #[inline(always)]
    fn from(variant: USV) -> Self {
        variant as u8 != 0
    }
}
///Field `USV` reader - VDDUSB independent USB supply valid This bit is used to validate the VDDUSB supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB OTG peripheral. If VDDUSB is not always present in the application, the VDDUSB voltage monitor can be used to determine whether this supply is ready or not.
pub type USV_R = crate::BitReader<USV>;
impl USV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USV {
        match self.bits {
            false => USV::NotPresent,
            true => USV::Present,
        }
    }
    ///VDDUSB not present: logical and electrical isolation is applied to ignore this supply
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == USV::NotPresent
    }
    ///VDDUSB valid
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == USV::Present
    }
}
///Field `USV` writer - VDDUSB independent USB supply valid This bit is used to validate the VDDUSB supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB OTG peripheral. If VDDUSB is not always present in the application, the VDDUSB voltage monitor can be used to determine whether this supply is ready or not.
pub type USV_W<'a, REG> = crate::BitWriter<'a, REG, USV>;
impl<'a, REG> USV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VDDUSB not present: logical and electrical isolation is applied to ignore this supply
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(USV::NotPresent)
    }
    ///VDDUSB valid
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(USV::Present)
    }
}
/**VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IO2SV {
    ///0: VDDIO2 not present: logical and electrical isolation is applied to ignore this supply
    NotPresent = 0,
    ///1: VDDIO2 valid
    Present = 1,
}
impl From<IO2SV> for bool {
    #[inline(always)]
    fn from(variant: IO2SV) -> Self {
        variant as u8 != 0
    }
}
///Field `IO2SV` reader - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
pub type IO2SV_R = crate::BitReader<IO2SV>;
impl IO2SV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IO2SV {
        match self.bits {
            false => IO2SV::NotPresent,
            true => IO2SV::Present,
        }
    }
    ///VDDIO2 not present: logical and electrical isolation is applied to ignore this supply
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == IO2SV::NotPresent
    }
    ///VDDIO2 valid
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == IO2SV::Present
    }
}
///Field `IO2SV` writer - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
pub type IO2SV_W<'a, REG> = crate::BitWriter<'a, REG, IO2SV>;
impl<'a, REG> IO2SV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VDDIO2 not present: logical and electrical isolation is applied to ignore this supply
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(IO2SV::NotPresent)
    }
    ///VDDIO2 valid
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(IO2SV::Present)
    }
}
/**VDDA independent analog supply valid This bit is used to validate the VDDA supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the analog peripherals. If VDDA is not always present in the application, the VDDA voltage monitor can be used to determine whether this supply is ready or not.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASV {
    ///0: VDDA not present: logical and electrical isolation is applied to ignore this supply
    NotPresent = 0,
    ///1: VDDA valid
    Present = 1,
}
impl From<ASV> for bool {
    #[inline(always)]
    fn from(variant: ASV) -> Self {
        variant as u8 != 0
    }
}
///Field `ASV` reader - VDDA independent analog supply valid This bit is used to validate the VDDA supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the analog peripherals. If VDDA is not always present in the application, the VDDA voltage monitor can be used to determine whether this supply is ready or not.
pub type ASV_R = crate::BitReader<ASV>;
impl ASV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASV {
        match self.bits {
            false => ASV::NotPresent,
            true => ASV::Present,
        }
    }
    ///VDDA not present: logical and electrical isolation is applied to ignore this supply
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == ASV::NotPresent
    }
    ///VDDA valid
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == ASV::Present
    }
}
///Field `ASV` writer - VDDA independent analog supply valid This bit is used to validate the VDDA supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the analog peripherals. If VDDA is not always present in the application, the VDDA voltage monitor can be used to determine whether this supply is ready or not.
pub type ASV_W<'a, REG> = crate::BitWriter<'a, REG, ASV>;
impl<'a, REG> ASV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VDDA not present: logical and electrical isolation is applied to ignore this supply
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(ASV::NotPresent)
    }
    ///VDDA valid
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(ASV::Present)
    }
}
impl R {
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
    #[inline(always)]
    pub fn pvdls(&self) -> PVDLS_R {
        PVDLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 24 - VDDUSB independent USB voltage monitor enable
    #[inline(always)]
    pub fn uvmen(&self) -> UVMEN_R {
        UVMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VDDIO2 independent I/Os voltage monitor enable
    #[inline(always)]
    pub fn io2vmen(&self) -> IO2VMEN_R {
        IO2VMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
    #[inline(always)]
    pub fn avm1en(&self) -> AVM1EN_R {
        AVM1EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
    #[inline(always)]
    pub fn avm2en(&self) -> AVM2EN_R {
        AVM2EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - VDDUSB independent USB supply valid This bit is used to validate the VDDUSB supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB OTG peripheral. If VDDUSB is not always present in the application, the VDDUSB voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn io2sv(&self) -> IO2SV_R {
        IO2SV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - VDDA independent analog supply valid This bit is used to validate the VDDA supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the analog peripherals. If VDDA is not always present in the application, the VDDA voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SVMCR")
            .field("pvde", &self.pvde())
            .field("pvdls", &self.pvdls())
            .field("uvmen", &self.uvmen())
            .field("io2vmen", &self.io2vmen())
            .field("avm1en", &self.avm1en())
            .field("avm2en", &self.avm2en())
            .field("usv", &self.usv())
            .field("io2sv", &self.io2sv())
            .field("asv", &self.asv())
            .finish()
    }
}
impl W {
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<SVMCRrs> {
        PVDE_W::new(self, 4)
    }
    ///Bits 5:7 - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
    #[inline(always)]
    pub fn pvdls(&mut self) -> PVDLS_W<SVMCRrs> {
        PVDLS_W::new(self, 5)
    }
    ///Bit 24 - VDDUSB independent USB voltage monitor enable
    #[inline(always)]
    pub fn uvmen(&mut self) -> UVMEN_W<SVMCRrs> {
        UVMEN_W::new(self, 24)
    }
    ///Bit 25 - VDDIO2 independent I/Os voltage monitor enable
    #[inline(always)]
    pub fn io2vmen(&mut self) -> IO2VMEN_W<SVMCRrs> {
        IO2VMEN_W::new(self, 25)
    }
    ///Bit 26 - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
    #[inline(always)]
    pub fn avm1en(&mut self) -> AVM1EN_W<SVMCRrs> {
        AVM1EN_W::new(self, 26)
    }
    ///Bit 27 - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
    #[inline(always)]
    pub fn avm2en(&mut self) -> AVM2EN_W<SVMCRrs> {
        AVM2EN_W::new(self, 27)
    }
    ///Bit 28 - VDDUSB independent USB supply valid This bit is used to validate the VDDUSB supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB OTG peripheral. If VDDUSB is not always present in the application, the VDDUSB voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn usv(&mut self) -> USV_W<SVMCRrs> {
        USV_W::new(self, 28)
    }
    ///Bit 29 - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn io2sv(&mut self) -> IO2SV_W<SVMCRrs> {
        IO2SV_W::new(self, 29)
    }
    ///Bit 30 - VDDA independent analog supply valid This bit is used to validate the VDDA supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the analog peripherals. If VDDA is not always present in the application, the VDDA voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn asv(&mut self) -> ASV_W<SVMCRrs> {
        ASV_W::new(self, 30)
    }
}
/**PWR supply voltage monitoring control register

You can [`read`](crate::Reg::read) this register and get [`svmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#PWR:SVMCR)*/
pub struct SVMCRrs;
impl crate::RegisterSpec for SVMCRrs {
    type Ux = u32;
}
///`read()` method returns [`svmcr::R`](R) reader structure
impl crate::Readable for SVMCRrs {}
///`write(|w| ..)` method takes [`svmcr::W`](W) writer structure
impl crate::Writable for SVMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SVMCR to value 0
impl crate::Resettable for SVMCRrs {}
