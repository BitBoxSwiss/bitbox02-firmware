///Register `PLL2DIVR` reader
pub type R = crate::R<PLL2DIVRrs>;
///Register `PLL2DIVR` writer
pub type W = crate::W<PLL2DIVRrs>;
///Field `PLL2N` reader - Multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = F<sub>ref2_ck</sub> x PLL2N, when fractional value 0 has been loaded in PLL2FRACN, with: PLL2N between 4 and 512 input frequency F<sub>ref2_ck</sub> between 1MHz and 16MHz
pub type PLL2N_R = crate::FieldReader<u16>;
///Field `PLL2N` writer - Multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = F<sub>ref2_ck</sub> x PLL2N, when fractional value 0 has been loaded in PLL2FRACN, with: PLL2N between 4 and 512 input frequency F<sub>ref2_ck</sub> between 1MHz and 16MHz
pub type PLL2N_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
/**PLL2 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll2_p_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2P {
    ///0: pllx_p_ck = vcox_ck
    Div1 = 0,
    ///1: pllx_p_ck = vcox_ck / 2
    Div2 = 1,
    ///3: pllx_p_ck = vcox_ck / 4
    Div4 = 3,
    ///5: pllx_p_ck = vcox_ck / 6
    Div6 = 5,
    ///7: pllx_p_ck = vcox_ck / 8
    Div8 = 7,
    ///9: pllx_p_ck = vcox_ck / 10
    Div10 = 9,
    ///11: pllx_p_ck = vcox_ck / 12
    Div12 = 11,
    ///13: pllx_p_ck = vcox_ck / 14
    Div14 = 13,
    ///15: pllx_p_ck = vcox_ck / 16
    Div16 = 15,
    ///17: pllx_p_ck = vcox_ck / 18
    Div18 = 17,
    ///19: pllx_p_ck = vcox_ck / 20
    Div20 = 19,
    ///21: pllx_p_ck = vcox_ck / 22
    Div22 = 21,
    ///23: pllx_p_ck = vcox_ck / 24
    Div24 = 23,
    ///25: pllx_p_ck = vcox_ck / 26
    Div26 = 25,
    ///27: pllx_p_ck = vcox_ck / 28
    Div28 = 27,
    ///29: pllx_p_ck = vcox_ck / 30
    Div30 = 29,
    ///31: pllx_p_ck = vcox_ck / 32
    Div32 = 31,
    ///33: pllx_p_ck = vcox_ck / 34
    Div34 = 33,
    ///35: pllx_p_ck = vcox_ck / 36
    Div36 = 35,
    ///37: pllx_p_ck = vcox_ck / 38
    Div38 = 37,
    ///39: pllx_p_ck = vcox_ck / 40
    Div40 = 39,
    ///41: pllx_p_ck = vcox_ck / 42
    Div42 = 41,
    ///43: pllx_p_ck = vcox_ck / 44
    Div44 = 43,
    ///45: pllx_p_ck = vcox_ck / 46
    Div46 = 45,
    ///47: pllx_p_ck = vcox_ck / 48
    Div48 = 47,
    ///49: pllx_p_ck = vcox_ck / 50
    Div50 = 49,
    ///51: pllx_p_ck = vcox_ck / 52
    Div52 = 51,
    ///53: pllx_p_ck = vcox_ck / 54
    Div54 = 53,
    ///55: pllx_p_ck = vcox_ck / 56
    Div56 = 55,
    ///57: pllx_p_ck = vcox_ck / 58
    Div58 = 57,
    ///59: pllx_p_ck = vcox_ck / 60
    Div60 = 59,
    ///61: pllx_p_ck = vcox_ck / 62
    Div62 = 61,
    ///63: pllx_p_ck = vcox_ck / 64
    Div64 = 63,
    ///65: pllx_p_ck = vcox_ck / 66
    Div66 = 65,
    ///67: pllx_p_ck = vcox_ck / 68
    Div68 = 67,
    ///69: pllx_p_ck = vcox_ck / 70
    Div70 = 69,
    ///71: pllx_p_ck = vcox_ck / 72
    Div72 = 71,
    ///73: pllx_p_ck = vcox_ck / 74
    Div74 = 73,
    ///75: pllx_p_ck = vcox_ck / 76
    Div76 = 75,
    ///77: pllx_p_ck = vcox_ck / 78
    Div78 = 77,
    ///79: pllx_p_ck = vcox_ck / 80
    Div80 = 79,
    ///81: pllx_p_ck = vcox_ck / 82
    Div82 = 81,
    ///83: pllx_p_ck = vcox_ck / 84
    Div84 = 83,
    ///85: pllx_p_ck = vcox_ck / 86
    Div86 = 85,
    ///87: pllx_p_ck = vcox_ck / 88
    Div88 = 87,
    ///89: pllx_p_ck = vcox_ck / 90
    Div90 = 89,
    ///91: pllx_p_ck = vcox_ck / 92
    Div92 = 91,
    ///93: pllx_p_ck = vcox_ck / 94
    Div94 = 93,
    ///95: pllx_p_ck = vcox_ck / 96
    Div96 = 95,
    ///97: pllx_p_ck = vcox_ck / 98
    Div98 = 97,
    ///99: pllx_p_ck = vcox_ck / 100
    Div100 = 99,
    ///101: pllx_p_ck = vcox_ck / 102
    Div102 = 101,
    ///103: pllx_p_ck = vcox_ck / 104
    Div104 = 103,
    ///105: pllx_p_ck = vcox_ck / 106
    Div106 = 105,
    ///107: pllx_p_ck = vcox_ck / 108
    Div108 = 107,
    ///109: pllx_p_ck = vcox_ck / 110
    Div110 = 109,
    ///111: pllx_p_ck = vcox_ck / 112
    Div112 = 111,
    ///113: pllx_p_ck = vcox_ck / 114
    Div114 = 113,
    ///115: pllx_p_ck = vcox_ck / 116
    Div116 = 115,
    ///117: pllx_p_ck = vcox_ck / 118
    Div118 = 117,
    ///119: pllx_p_ck = vcox_ck / 120
    Div120 = 119,
    ///121: pllx_p_ck = vcox_ck / 122
    Div122 = 121,
    ///123: pllx_p_ck = vcox_ck / 124
    Div124 = 123,
    ///125: pllx_p_ck = vcox_ck / 126
    Div126 = 125,
    ///127: pllx_p_ck = vcox_ck / 128
    Div128 = 127,
}
impl From<PLL2P> for u8 {
    #[inline(always)]
    fn from(variant: PLL2P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2P {
    type Ux = u8;
}
impl crate::IsEnum for PLL2P {}
///Field `PLL2P` reader - PLL2 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll2_p_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2P_R = crate::FieldReader<PLL2P>;
impl PLL2P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL2P> {
        match self.bits {
            0 => Some(PLL2P::Div1),
            1 => Some(PLL2P::Div2),
            3 => Some(PLL2P::Div4),
            5 => Some(PLL2P::Div6),
            7 => Some(PLL2P::Div8),
            9 => Some(PLL2P::Div10),
            11 => Some(PLL2P::Div12),
            13 => Some(PLL2P::Div14),
            15 => Some(PLL2P::Div16),
            17 => Some(PLL2P::Div18),
            19 => Some(PLL2P::Div20),
            21 => Some(PLL2P::Div22),
            23 => Some(PLL2P::Div24),
            25 => Some(PLL2P::Div26),
            27 => Some(PLL2P::Div28),
            29 => Some(PLL2P::Div30),
            31 => Some(PLL2P::Div32),
            33 => Some(PLL2P::Div34),
            35 => Some(PLL2P::Div36),
            37 => Some(PLL2P::Div38),
            39 => Some(PLL2P::Div40),
            41 => Some(PLL2P::Div42),
            43 => Some(PLL2P::Div44),
            45 => Some(PLL2P::Div46),
            47 => Some(PLL2P::Div48),
            49 => Some(PLL2P::Div50),
            51 => Some(PLL2P::Div52),
            53 => Some(PLL2P::Div54),
            55 => Some(PLL2P::Div56),
            57 => Some(PLL2P::Div58),
            59 => Some(PLL2P::Div60),
            61 => Some(PLL2P::Div62),
            63 => Some(PLL2P::Div64),
            65 => Some(PLL2P::Div66),
            67 => Some(PLL2P::Div68),
            69 => Some(PLL2P::Div70),
            71 => Some(PLL2P::Div72),
            73 => Some(PLL2P::Div74),
            75 => Some(PLL2P::Div76),
            77 => Some(PLL2P::Div78),
            79 => Some(PLL2P::Div80),
            81 => Some(PLL2P::Div82),
            83 => Some(PLL2P::Div84),
            85 => Some(PLL2P::Div86),
            87 => Some(PLL2P::Div88),
            89 => Some(PLL2P::Div90),
            91 => Some(PLL2P::Div92),
            93 => Some(PLL2P::Div94),
            95 => Some(PLL2P::Div96),
            97 => Some(PLL2P::Div98),
            99 => Some(PLL2P::Div100),
            101 => Some(PLL2P::Div102),
            103 => Some(PLL2P::Div104),
            105 => Some(PLL2P::Div106),
            107 => Some(PLL2P::Div108),
            109 => Some(PLL2P::Div110),
            111 => Some(PLL2P::Div112),
            113 => Some(PLL2P::Div114),
            115 => Some(PLL2P::Div116),
            117 => Some(PLL2P::Div118),
            119 => Some(PLL2P::Div120),
            121 => Some(PLL2P::Div122),
            123 => Some(PLL2P::Div124),
            125 => Some(PLL2P::Div126),
            127 => Some(PLL2P::Div128),
            _ => None,
        }
    }
    ///pllx_p_ck = vcox_ck
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLL2P::Div1
    }
    ///pllx_p_ck = vcox_ck / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLL2P::Div2
    }
    ///pllx_p_ck = vcox_ck / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLL2P::Div4
    }
    ///pllx_p_ck = vcox_ck / 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLL2P::Div6
    }
    ///pllx_p_ck = vcox_ck / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLL2P::Div8
    }
    ///pllx_p_ck = vcox_ck / 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLL2P::Div10
    }
    ///pllx_p_ck = vcox_ck / 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLL2P::Div12
    }
    ///pllx_p_ck = vcox_ck / 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLL2P::Div14
    }
    ///pllx_p_ck = vcox_ck / 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLL2P::Div16
    }
    ///pllx_p_ck = vcox_ck / 18
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLL2P::Div18
    }
    ///pllx_p_ck = vcox_ck / 20
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLL2P::Div20
    }
    ///pllx_p_ck = vcox_ck / 22
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLL2P::Div22
    }
    ///pllx_p_ck = vcox_ck / 24
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLL2P::Div24
    }
    ///pllx_p_ck = vcox_ck / 26
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLL2P::Div26
    }
    ///pllx_p_ck = vcox_ck / 28
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLL2P::Div28
    }
    ///pllx_p_ck = vcox_ck / 30
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLL2P::Div30
    }
    ///pllx_p_ck = vcox_ck / 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLL2P::Div32
    }
    ///pllx_p_ck = vcox_ck / 34
    #[inline(always)]
    pub fn is_div34(&self) -> bool {
        *self == PLL2P::Div34
    }
    ///pllx_p_ck = vcox_ck / 36
    #[inline(always)]
    pub fn is_div36(&self) -> bool {
        *self == PLL2P::Div36
    }
    ///pllx_p_ck = vcox_ck / 38
    #[inline(always)]
    pub fn is_div38(&self) -> bool {
        *self == PLL2P::Div38
    }
    ///pllx_p_ck = vcox_ck / 40
    #[inline(always)]
    pub fn is_div40(&self) -> bool {
        *self == PLL2P::Div40
    }
    ///pllx_p_ck = vcox_ck / 42
    #[inline(always)]
    pub fn is_div42(&self) -> bool {
        *self == PLL2P::Div42
    }
    ///pllx_p_ck = vcox_ck / 44
    #[inline(always)]
    pub fn is_div44(&self) -> bool {
        *self == PLL2P::Div44
    }
    ///pllx_p_ck = vcox_ck / 46
    #[inline(always)]
    pub fn is_div46(&self) -> bool {
        *self == PLL2P::Div46
    }
    ///pllx_p_ck = vcox_ck / 48
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == PLL2P::Div48
    }
    ///pllx_p_ck = vcox_ck / 50
    #[inline(always)]
    pub fn is_div50(&self) -> bool {
        *self == PLL2P::Div50
    }
    ///pllx_p_ck = vcox_ck / 52
    #[inline(always)]
    pub fn is_div52(&self) -> bool {
        *self == PLL2P::Div52
    }
    ///pllx_p_ck = vcox_ck / 54
    #[inline(always)]
    pub fn is_div54(&self) -> bool {
        *self == PLL2P::Div54
    }
    ///pllx_p_ck = vcox_ck / 56
    #[inline(always)]
    pub fn is_div56(&self) -> bool {
        *self == PLL2P::Div56
    }
    ///pllx_p_ck = vcox_ck / 58
    #[inline(always)]
    pub fn is_div58(&self) -> bool {
        *self == PLL2P::Div58
    }
    ///pllx_p_ck = vcox_ck / 60
    #[inline(always)]
    pub fn is_div60(&self) -> bool {
        *self == PLL2P::Div60
    }
    ///pllx_p_ck = vcox_ck / 62
    #[inline(always)]
    pub fn is_div62(&self) -> bool {
        *self == PLL2P::Div62
    }
    ///pllx_p_ck = vcox_ck / 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PLL2P::Div64
    }
    ///pllx_p_ck = vcox_ck / 66
    #[inline(always)]
    pub fn is_div66(&self) -> bool {
        *self == PLL2P::Div66
    }
    ///pllx_p_ck = vcox_ck / 68
    #[inline(always)]
    pub fn is_div68(&self) -> bool {
        *self == PLL2P::Div68
    }
    ///pllx_p_ck = vcox_ck / 70
    #[inline(always)]
    pub fn is_div70(&self) -> bool {
        *self == PLL2P::Div70
    }
    ///pllx_p_ck = vcox_ck / 72
    #[inline(always)]
    pub fn is_div72(&self) -> bool {
        *self == PLL2P::Div72
    }
    ///pllx_p_ck = vcox_ck / 74
    #[inline(always)]
    pub fn is_div74(&self) -> bool {
        *self == PLL2P::Div74
    }
    ///pllx_p_ck = vcox_ck / 76
    #[inline(always)]
    pub fn is_div76(&self) -> bool {
        *self == PLL2P::Div76
    }
    ///pllx_p_ck = vcox_ck / 78
    #[inline(always)]
    pub fn is_div78(&self) -> bool {
        *self == PLL2P::Div78
    }
    ///pllx_p_ck = vcox_ck / 80
    #[inline(always)]
    pub fn is_div80(&self) -> bool {
        *self == PLL2P::Div80
    }
    ///pllx_p_ck = vcox_ck / 82
    #[inline(always)]
    pub fn is_div82(&self) -> bool {
        *self == PLL2P::Div82
    }
    ///pllx_p_ck = vcox_ck / 84
    #[inline(always)]
    pub fn is_div84(&self) -> bool {
        *self == PLL2P::Div84
    }
    ///pllx_p_ck = vcox_ck / 86
    #[inline(always)]
    pub fn is_div86(&self) -> bool {
        *self == PLL2P::Div86
    }
    ///pllx_p_ck = vcox_ck / 88
    #[inline(always)]
    pub fn is_div88(&self) -> bool {
        *self == PLL2P::Div88
    }
    ///pllx_p_ck = vcox_ck / 90
    #[inline(always)]
    pub fn is_div90(&self) -> bool {
        *self == PLL2P::Div90
    }
    ///pllx_p_ck = vcox_ck / 92
    #[inline(always)]
    pub fn is_div92(&self) -> bool {
        *self == PLL2P::Div92
    }
    ///pllx_p_ck = vcox_ck / 94
    #[inline(always)]
    pub fn is_div94(&self) -> bool {
        *self == PLL2P::Div94
    }
    ///pllx_p_ck = vcox_ck / 96
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        *self == PLL2P::Div96
    }
    ///pllx_p_ck = vcox_ck / 98
    #[inline(always)]
    pub fn is_div98(&self) -> bool {
        *self == PLL2P::Div98
    }
    ///pllx_p_ck = vcox_ck / 100
    #[inline(always)]
    pub fn is_div100(&self) -> bool {
        *self == PLL2P::Div100
    }
    ///pllx_p_ck = vcox_ck / 102
    #[inline(always)]
    pub fn is_div102(&self) -> bool {
        *self == PLL2P::Div102
    }
    ///pllx_p_ck = vcox_ck / 104
    #[inline(always)]
    pub fn is_div104(&self) -> bool {
        *self == PLL2P::Div104
    }
    ///pllx_p_ck = vcox_ck / 106
    #[inline(always)]
    pub fn is_div106(&self) -> bool {
        *self == PLL2P::Div106
    }
    ///pllx_p_ck = vcox_ck / 108
    #[inline(always)]
    pub fn is_div108(&self) -> bool {
        *self == PLL2P::Div108
    }
    ///pllx_p_ck = vcox_ck / 110
    #[inline(always)]
    pub fn is_div110(&self) -> bool {
        *self == PLL2P::Div110
    }
    ///pllx_p_ck = vcox_ck / 112
    #[inline(always)]
    pub fn is_div112(&self) -> bool {
        *self == PLL2P::Div112
    }
    ///pllx_p_ck = vcox_ck / 114
    #[inline(always)]
    pub fn is_div114(&self) -> bool {
        *self == PLL2P::Div114
    }
    ///pllx_p_ck = vcox_ck / 116
    #[inline(always)]
    pub fn is_div116(&self) -> bool {
        *self == PLL2P::Div116
    }
    ///pllx_p_ck = vcox_ck / 118
    #[inline(always)]
    pub fn is_div118(&self) -> bool {
        *self == PLL2P::Div118
    }
    ///pllx_p_ck = vcox_ck / 120
    #[inline(always)]
    pub fn is_div120(&self) -> bool {
        *self == PLL2P::Div120
    }
    ///pllx_p_ck = vcox_ck / 122
    #[inline(always)]
    pub fn is_div122(&self) -> bool {
        *self == PLL2P::Div122
    }
    ///pllx_p_ck = vcox_ck / 124
    #[inline(always)]
    pub fn is_div124(&self) -> bool {
        *self == PLL2P::Div124
    }
    ///pllx_p_ck = vcox_ck / 126
    #[inline(always)]
    pub fn is_div126(&self) -> bool {
        *self == PLL2P::Div126
    }
    ///pllx_p_ck = vcox_ck / 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PLL2P::Div128
    }
}
///Field `PLL2P` writer - PLL2 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll2_p_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2P_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PLL2P>;
impl<'a, REG> PLL2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pllx_p_ck = vcox_ck
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div1)
    }
    ///pllx_p_ck = vcox_ck / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div2)
    }
    ///pllx_p_ck = vcox_ck / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div4)
    }
    ///pllx_p_ck = vcox_ck / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div6)
    }
    ///pllx_p_ck = vcox_ck / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div8)
    }
    ///pllx_p_ck = vcox_ck / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div10)
    }
    ///pllx_p_ck = vcox_ck / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div12)
    }
    ///pllx_p_ck = vcox_ck / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div14)
    }
    ///pllx_p_ck = vcox_ck / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div16)
    }
    ///pllx_p_ck = vcox_ck / 18
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div18)
    }
    ///pllx_p_ck = vcox_ck / 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div20)
    }
    ///pllx_p_ck = vcox_ck / 22
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div22)
    }
    ///pllx_p_ck = vcox_ck / 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div24)
    }
    ///pllx_p_ck = vcox_ck / 26
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div26)
    }
    ///pllx_p_ck = vcox_ck / 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div28)
    }
    ///pllx_p_ck = vcox_ck / 30
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div30)
    }
    ///pllx_p_ck = vcox_ck / 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div32)
    }
    ///pllx_p_ck = vcox_ck / 34
    #[inline(always)]
    pub fn div34(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div34)
    }
    ///pllx_p_ck = vcox_ck / 36
    #[inline(always)]
    pub fn div36(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div36)
    }
    ///pllx_p_ck = vcox_ck / 38
    #[inline(always)]
    pub fn div38(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div38)
    }
    ///pllx_p_ck = vcox_ck / 40
    #[inline(always)]
    pub fn div40(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div40)
    }
    ///pllx_p_ck = vcox_ck / 42
    #[inline(always)]
    pub fn div42(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div42)
    }
    ///pllx_p_ck = vcox_ck / 44
    #[inline(always)]
    pub fn div44(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div44)
    }
    ///pllx_p_ck = vcox_ck / 46
    #[inline(always)]
    pub fn div46(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div46)
    }
    ///pllx_p_ck = vcox_ck / 48
    #[inline(always)]
    pub fn div48(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div48)
    }
    ///pllx_p_ck = vcox_ck / 50
    #[inline(always)]
    pub fn div50(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div50)
    }
    ///pllx_p_ck = vcox_ck / 52
    #[inline(always)]
    pub fn div52(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div52)
    }
    ///pllx_p_ck = vcox_ck / 54
    #[inline(always)]
    pub fn div54(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div54)
    }
    ///pllx_p_ck = vcox_ck / 56
    #[inline(always)]
    pub fn div56(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div56)
    }
    ///pllx_p_ck = vcox_ck / 58
    #[inline(always)]
    pub fn div58(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div58)
    }
    ///pllx_p_ck = vcox_ck / 60
    #[inline(always)]
    pub fn div60(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div60)
    }
    ///pllx_p_ck = vcox_ck / 62
    #[inline(always)]
    pub fn div62(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div62)
    }
    ///pllx_p_ck = vcox_ck / 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div64)
    }
    ///pllx_p_ck = vcox_ck / 66
    #[inline(always)]
    pub fn div66(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div66)
    }
    ///pllx_p_ck = vcox_ck / 68
    #[inline(always)]
    pub fn div68(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div68)
    }
    ///pllx_p_ck = vcox_ck / 70
    #[inline(always)]
    pub fn div70(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div70)
    }
    ///pllx_p_ck = vcox_ck / 72
    #[inline(always)]
    pub fn div72(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div72)
    }
    ///pllx_p_ck = vcox_ck / 74
    #[inline(always)]
    pub fn div74(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div74)
    }
    ///pllx_p_ck = vcox_ck / 76
    #[inline(always)]
    pub fn div76(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div76)
    }
    ///pllx_p_ck = vcox_ck / 78
    #[inline(always)]
    pub fn div78(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div78)
    }
    ///pllx_p_ck = vcox_ck / 80
    #[inline(always)]
    pub fn div80(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div80)
    }
    ///pllx_p_ck = vcox_ck / 82
    #[inline(always)]
    pub fn div82(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div82)
    }
    ///pllx_p_ck = vcox_ck / 84
    #[inline(always)]
    pub fn div84(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div84)
    }
    ///pllx_p_ck = vcox_ck / 86
    #[inline(always)]
    pub fn div86(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div86)
    }
    ///pllx_p_ck = vcox_ck / 88
    #[inline(always)]
    pub fn div88(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div88)
    }
    ///pllx_p_ck = vcox_ck / 90
    #[inline(always)]
    pub fn div90(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div90)
    }
    ///pllx_p_ck = vcox_ck / 92
    #[inline(always)]
    pub fn div92(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div92)
    }
    ///pllx_p_ck = vcox_ck / 94
    #[inline(always)]
    pub fn div94(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div94)
    }
    ///pllx_p_ck = vcox_ck / 96
    #[inline(always)]
    pub fn div96(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div96)
    }
    ///pllx_p_ck = vcox_ck / 98
    #[inline(always)]
    pub fn div98(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div98)
    }
    ///pllx_p_ck = vcox_ck / 100
    #[inline(always)]
    pub fn div100(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div100)
    }
    ///pllx_p_ck = vcox_ck / 102
    #[inline(always)]
    pub fn div102(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div102)
    }
    ///pllx_p_ck = vcox_ck / 104
    #[inline(always)]
    pub fn div104(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div104)
    }
    ///pllx_p_ck = vcox_ck / 106
    #[inline(always)]
    pub fn div106(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div106)
    }
    ///pllx_p_ck = vcox_ck / 108
    #[inline(always)]
    pub fn div108(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div108)
    }
    ///pllx_p_ck = vcox_ck / 110
    #[inline(always)]
    pub fn div110(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div110)
    }
    ///pllx_p_ck = vcox_ck / 112
    #[inline(always)]
    pub fn div112(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div112)
    }
    ///pllx_p_ck = vcox_ck / 114
    #[inline(always)]
    pub fn div114(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div114)
    }
    ///pllx_p_ck = vcox_ck / 116
    #[inline(always)]
    pub fn div116(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div116)
    }
    ///pllx_p_ck = vcox_ck / 118
    #[inline(always)]
    pub fn div118(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div118)
    }
    ///pllx_p_ck = vcox_ck / 120
    #[inline(always)]
    pub fn div120(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div120)
    }
    ///pllx_p_ck = vcox_ck / 122
    #[inline(always)]
    pub fn div122(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div122)
    }
    ///pllx_p_ck = vcox_ck / 124
    #[inline(always)]
    pub fn div124(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div124)
    }
    ///pllx_p_ck = vcox_ck / 126
    #[inline(always)]
    pub fn div126(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div126)
    }
    ///pllx_p_ck = vcox_ck / 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2P::Div128)
    }
}
/**PLL2 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll2_q_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2Q {
    ///0: pllx_q_ck = vcox_ck
    Div1 = 0,
    ///1: pllx_q_ck = vcox_ck / 2
    Div2 = 1,
    ///3: pllx_q_ck = vcox_ck / 4
    Div4 = 3,
    ///5: pllx_q_ck = vcox_ck / 6
    Div6 = 5,
    ///7: pllx_q_ck = vcox_ck / 8
    Div8 = 7,
    ///9: pllx_q_ck = vcox_ck / 10
    Div10 = 9,
    ///11: pllx_q_ck = vcox_ck / 12
    Div12 = 11,
    ///13: pllx_q_ck = vcox_ck / 14
    Div14 = 13,
    ///15: pllx_q_ck = vcox_ck / 16
    Div16 = 15,
    ///17: pllx_q_ck = vcox_ck / 18
    Div18 = 17,
    ///19: pllx_q_ck = vcox_ck / 20
    Div20 = 19,
    ///21: pllx_q_ck = vcox_ck / 22
    Div22 = 21,
    ///23: pllx_q_ck = vcox_ck / 24
    Div24 = 23,
    ///25: pllx_q_ck = vcox_ck / 26
    Div26 = 25,
    ///27: pllx_q_ck = vcox_ck / 28
    Div28 = 27,
    ///29: pllx_q_ck = vcox_ck / 30
    Div30 = 29,
    ///31: pllx_q_ck = vcox_ck / 32
    Div32 = 31,
    ///33: pllx_q_ck = vcox_ck / 34
    Div34 = 33,
    ///35: pllx_q_ck = vcox_ck / 36
    Div36 = 35,
    ///37: pllx_q_ck = vcox_ck / 38
    Div38 = 37,
    ///39: pllx_q_ck = vcox_ck / 40
    Div40 = 39,
    ///41: pllx_q_ck = vcox_ck / 42
    Div42 = 41,
    ///43: pllx_q_ck = vcox_ck / 44
    Div44 = 43,
    ///45: pllx_q_ck = vcox_ck / 46
    Div46 = 45,
    ///47: pllx_q_ck = vcox_ck / 48
    Div48 = 47,
    ///49: pllx_q_ck = vcox_ck / 50
    Div50 = 49,
    ///51: pllx_q_ck = vcox_ck / 52
    Div52 = 51,
    ///53: pllx_q_ck = vcox_ck / 54
    Div54 = 53,
    ///55: pllx_q_ck = vcox_ck / 56
    Div56 = 55,
    ///57: pllx_q_ck = vcox_ck / 58
    Div58 = 57,
    ///59: pllx_q_ck = vcox_ck / 60
    Div60 = 59,
    ///61: pllx_q_ck = vcox_ck / 62
    Div62 = 61,
    ///63: pllx_q_ck = vcox_ck / 64
    Div64 = 63,
    ///65: pllx_q_ck = vcox_ck / 66
    Div66 = 65,
    ///67: pllx_q_ck = vcox_ck / 68
    Div68 = 67,
    ///69: pllx_q_ck = vcox_ck / 70
    Div70 = 69,
    ///71: pllx_q_ck = vcox_ck / 72
    Div72 = 71,
    ///73: pllx_q_ck = vcox_ck / 74
    Div74 = 73,
    ///75: pllx_q_ck = vcox_ck / 76
    Div76 = 75,
    ///77: pllx_q_ck = vcox_ck / 78
    Div78 = 77,
    ///79: pllx_q_ck = vcox_ck / 80
    Div80 = 79,
    ///81: pllx_q_ck = vcox_ck / 82
    Div82 = 81,
    ///83: pllx_q_ck = vcox_ck / 84
    Div84 = 83,
    ///85: pllx_q_ck = vcox_ck / 86
    Div86 = 85,
    ///87: pllx_q_ck = vcox_ck / 88
    Div88 = 87,
    ///89: pllx_q_ck = vcox_ck / 90
    Div90 = 89,
    ///91: pllx_q_ck = vcox_ck / 92
    Div92 = 91,
    ///93: pllx_q_ck = vcox_ck / 94
    Div94 = 93,
    ///95: pllx_q_ck = vcox_ck / 96
    Div96 = 95,
    ///97: pllx_q_ck = vcox_ck / 98
    Div98 = 97,
    ///99: pllx_q_ck = vcox_ck / 100
    Div100 = 99,
    ///101: pllx_q_ck = vcox_ck / 102
    Div102 = 101,
    ///103: pllx_q_ck = vcox_ck / 104
    Div104 = 103,
    ///105: pllx_q_ck = vcox_ck / 106
    Div106 = 105,
    ///107: pllx_q_ck = vcox_ck / 108
    Div108 = 107,
    ///109: pllx_q_ck = vcox_ck / 110
    Div110 = 109,
    ///111: pllx_q_ck = vcox_ck / 112
    Div112 = 111,
    ///113: pllx_q_ck = vcox_ck / 114
    Div114 = 113,
    ///115: pllx_q_ck = vcox_ck / 116
    Div116 = 115,
    ///117: pllx_q_ck = vcox_ck / 118
    Div118 = 117,
    ///119: pllx_q_ck = vcox_ck / 120
    Div120 = 119,
    ///121: pllx_q_ck = vcox_ck / 122
    Div122 = 121,
    ///123: pllx_q_ck = vcox_ck / 124
    Div124 = 123,
    ///125: pllx_q_ck = vcox_ck / 126
    Div126 = 125,
    ///127: pllx_q_ck = vcox_ck / 128
    Div128 = 127,
}
impl From<PLL2Q> for u8 {
    #[inline(always)]
    fn from(variant: PLL2Q) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2Q {
    type Ux = u8;
}
impl crate::IsEnum for PLL2Q {}
///Field `PLL2Q` reader - PLL2 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll2_q_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2Q_R = crate::FieldReader<PLL2Q>;
impl PLL2Q_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL2Q> {
        match self.bits {
            0 => Some(PLL2Q::Div1),
            1 => Some(PLL2Q::Div2),
            3 => Some(PLL2Q::Div4),
            5 => Some(PLL2Q::Div6),
            7 => Some(PLL2Q::Div8),
            9 => Some(PLL2Q::Div10),
            11 => Some(PLL2Q::Div12),
            13 => Some(PLL2Q::Div14),
            15 => Some(PLL2Q::Div16),
            17 => Some(PLL2Q::Div18),
            19 => Some(PLL2Q::Div20),
            21 => Some(PLL2Q::Div22),
            23 => Some(PLL2Q::Div24),
            25 => Some(PLL2Q::Div26),
            27 => Some(PLL2Q::Div28),
            29 => Some(PLL2Q::Div30),
            31 => Some(PLL2Q::Div32),
            33 => Some(PLL2Q::Div34),
            35 => Some(PLL2Q::Div36),
            37 => Some(PLL2Q::Div38),
            39 => Some(PLL2Q::Div40),
            41 => Some(PLL2Q::Div42),
            43 => Some(PLL2Q::Div44),
            45 => Some(PLL2Q::Div46),
            47 => Some(PLL2Q::Div48),
            49 => Some(PLL2Q::Div50),
            51 => Some(PLL2Q::Div52),
            53 => Some(PLL2Q::Div54),
            55 => Some(PLL2Q::Div56),
            57 => Some(PLL2Q::Div58),
            59 => Some(PLL2Q::Div60),
            61 => Some(PLL2Q::Div62),
            63 => Some(PLL2Q::Div64),
            65 => Some(PLL2Q::Div66),
            67 => Some(PLL2Q::Div68),
            69 => Some(PLL2Q::Div70),
            71 => Some(PLL2Q::Div72),
            73 => Some(PLL2Q::Div74),
            75 => Some(PLL2Q::Div76),
            77 => Some(PLL2Q::Div78),
            79 => Some(PLL2Q::Div80),
            81 => Some(PLL2Q::Div82),
            83 => Some(PLL2Q::Div84),
            85 => Some(PLL2Q::Div86),
            87 => Some(PLL2Q::Div88),
            89 => Some(PLL2Q::Div90),
            91 => Some(PLL2Q::Div92),
            93 => Some(PLL2Q::Div94),
            95 => Some(PLL2Q::Div96),
            97 => Some(PLL2Q::Div98),
            99 => Some(PLL2Q::Div100),
            101 => Some(PLL2Q::Div102),
            103 => Some(PLL2Q::Div104),
            105 => Some(PLL2Q::Div106),
            107 => Some(PLL2Q::Div108),
            109 => Some(PLL2Q::Div110),
            111 => Some(PLL2Q::Div112),
            113 => Some(PLL2Q::Div114),
            115 => Some(PLL2Q::Div116),
            117 => Some(PLL2Q::Div118),
            119 => Some(PLL2Q::Div120),
            121 => Some(PLL2Q::Div122),
            123 => Some(PLL2Q::Div124),
            125 => Some(PLL2Q::Div126),
            127 => Some(PLL2Q::Div128),
            _ => None,
        }
    }
    ///pllx_q_ck = vcox_ck
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLL2Q::Div1
    }
    ///pllx_q_ck = vcox_ck / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLL2Q::Div2
    }
    ///pllx_q_ck = vcox_ck / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLL2Q::Div4
    }
    ///pllx_q_ck = vcox_ck / 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLL2Q::Div6
    }
    ///pllx_q_ck = vcox_ck / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLL2Q::Div8
    }
    ///pllx_q_ck = vcox_ck / 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLL2Q::Div10
    }
    ///pllx_q_ck = vcox_ck / 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLL2Q::Div12
    }
    ///pllx_q_ck = vcox_ck / 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLL2Q::Div14
    }
    ///pllx_q_ck = vcox_ck / 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLL2Q::Div16
    }
    ///pllx_q_ck = vcox_ck / 18
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLL2Q::Div18
    }
    ///pllx_q_ck = vcox_ck / 20
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLL2Q::Div20
    }
    ///pllx_q_ck = vcox_ck / 22
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLL2Q::Div22
    }
    ///pllx_q_ck = vcox_ck / 24
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLL2Q::Div24
    }
    ///pllx_q_ck = vcox_ck / 26
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLL2Q::Div26
    }
    ///pllx_q_ck = vcox_ck / 28
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLL2Q::Div28
    }
    ///pllx_q_ck = vcox_ck / 30
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLL2Q::Div30
    }
    ///pllx_q_ck = vcox_ck / 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLL2Q::Div32
    }
    ///pllx_q_ck = vcox_ck / 34
    #[inline(always)]
    pub fn is_div34(&self) -> bool {
        *self == PLL2Q::Div34
    }
    ///pllx_q_ck = vcox_ck / 36
    #[inline(always)]
    pub fn is_div36(&self) -> bool {
        *self == PLL2Q::Div36
    }
    ///pllx_q_ck = vcox_ck / 38
    #[inline(always)]
    pub fn is_div38(&self) -> bool {
        *self == PLL2Q::Div38
    }
    ///pllx_q_ck = vcox_ck / 40
    #[inline(always)]
    pub fn is_div40(&self) -> bool {
        *self == PLL2Q::Div40
    }
    ///pllx_q_ck = vcox_ck / 42
    #[inline(always)]
    pub fn is_div42(&self) -> bool {
        *self == PLL2Q::Div42
    }
    ///pllx_q_ck = vcox_ck / 44
    #[inline(always)]
    pub fn is_div44(&self) -> bool {
        *self == PLL2Q::Div44
    }
    ///pllx_q_ck = vcox_ck / 46
    #[inline(always)]
    pub fn is_div46(&self) -> bool {
        *self == PLL2Q::Div46
    }
    ///pllx_q_ck = vcox_ck / 48
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == PLL2Q::Div48
    }
    ///pllx_q_ck = vcox_ck / 50
    #[inline(always)]
    pub fn is_div50(&self) -> bool {
        *self == PLL2Q::Div50
    }
    ///pllx_q_ck = vcox_ck / 52
    #[inline(always)]
    pub fn is_div52(&self) -> bool {
        *self == PLL2Q::Div52
    }
    ///pllx_q_ck = vcox_ck / 54
    #[inline(always)]
    pub fn is_div54(&self) -> bool {
        *self == PLL2Q::Div54
    }
    ///pllx_q_ck = vcox_ck / 56
    #[inline(always)]
    pub fn is_div56(&self) -> bool {
        *self == PLL2Q::Div56
    }
    ///pllx_q_ck = vcox_ck / 58
    #[inline(always)]
    pub fn is_div58(&self) -> bool {
        *self == PLL2Q::Div58
    }
    ///pllx_q_ck = vcox_ck / 60
    #[inline(always)]
    pub fn is_div60(&self) -> bool {
        *self == PLL2Q::Div60
    }
    ///pllx_q_ck = vcox_ck / 62
    #[inline(always)]
    pub fn is_div62(&self) -> bool {
        *self == PLL2Q::Div62
    }
    ///pllx_q_ck = vcox_ck / 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PLL2Q::Div64
    }
    ///pllx_q_ck = vcox_ck / 66
    #[inline(always)]
    pub fn is_div66(&self) -> bool {
        *self == PLL2Q::Div66
    }
    ///pllx_q_ck = vcox_ck / 68
    #[inline(always)]
    pub fn is_div68(&self) -> bool {
        *self == PLL2Q::Div68
    }
    ///pllx_q_ck = vcox_ck / 70
    #[inline(always)]
    pub fn is_div70(&self) -> bool {
        *self == PLL2Q::Div70
    }
    ///pllx_q_ck = vcox_ck / 72
    #[inline(always)]
    pub fn is_div72(&self) -> bool {
        *self == PLL2Q::Div72
    }
    ///pllx_q_ck = vcox_ck / 74
    #[inline(always)]
    pub fn is_div74(&self) -> bool {
        *self == PLL2Q::Div74
    }
    ///pllx_q_ck = vcox_ck / 76
    #[inline(always)]
    pub fn is_div76(&self) -> bool {
        *self == PLL2Q::Div76
    }
    ///pllx_q_ck = vcox_ck / 78
    #[inline(always)]
    pub fn is_div78(&self) -> bool {
        *self == PLL2Q::Div78
    }
    ///pllx_q_ck = vcox_ck / 80
    #[inline(always)]
    pub fn is_div80(&self) -> bool {
        *self == PLL2Q::Div80
    }
    ///pllx_q_ck = vcox_ck / 82
    #[inline(always)]
    pub fn is_div82(&self) -> bool {
        *self == PLL2Q::Div82
    }
    ///pllx_q_ck = vcox_ck / 84
    #[inline(always)]
    pub fn is_div84(&self) -> bool {
        *self == PLL2Q::Div84
    }
    ///pllx_q_ck = vcox_ck / 86
    #[inline(always)]
    pub fn is_div86(&self) -> bool {
        *self == PLL2Q::Div86
    }
    ///pllx_q_ck = vcox_ck / 88
    #[inline(always)]
    pub fn is_div88(&self) -> bool {
        *self == PLL2Q::Div88
    }
    ///pllx_q_ck = vcox_ck / 90
    #[inline(always)]
    pub fn is_div90(&self) -> bool {
        *self == PLL2Q::Div90
    }
    ///pllx_q_ck = vcox_ck / 92
    #[inline(always)]
    pub fn is_div92(&self) -> bool {
        *self == PLL2Q::Div92
    }
    ///pllx_q_ck = vcox_ck / 94
    #[inline(always)]
    pub fn is_div94(&self) -> bool {
        *self == PLL2Q::Div94
    }
    ///pllx_q_ck = vcox_ck / 96
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        *self == PLL2Q::Div96
    }
    ///pllx_q_ck = vcox_ck / 98
    #[inline(always)]
    pub fn is_div98(&self) -> bool {
        *self == PLL2Q::Div98
    }
    ///pllx_q_ck = vcox_ck / 100
    #[inline(always)]
    pub fn is_div100(&self) -> bool {
        *self == PLL2Q::Div100
    }
    ///pllx_q_ck = vcox_ck / 102
    #[inline(always)]
    pub fn is_div102(&self) -> bool {
        *self == PLL2Q::Div102
    }
    ///pllx_q_ck = vcox_ck / 104
    #[inline(always)]
    pub fn is_div104(&self) -> bool {
        *self == PLL2Q::Div104
    }
    ///pllx_q_ck = vcox_ck / 106
    #[inline(always)]
    pub fn is_div106(&self) -> bool {
        *self == PLL2Q::Div106
    }
    ///pllx_q_ck = vcox_ck / 108
    #[inline(always)]
    pub fn is_div108(&self) -> bool {
        *self == PLL2Q::Div108
    }
    ///pllx_q_ck = vcox_ck / 110
    #[inline(always)]
    pub fn is_div110(&self) -> bool {
        *self == PLL2Q::Div110
    }
    ///pllx_q_ck = vcox_ck / 112
    #[inline(always)]
    pub fn is_div112(&self) -> bool {
        *self == PLL2Q::Div112
    }
    ///pllx_q_ck = vcox_ck / 114
    #[inline(always)]
    pub fn is_div114(&self) -> bool {
        *self == PLL2Q::Div114
    }
    ///pllx_q_ck = vcox_ck / 116
    #[inline(always)]
    pub fn is_div116(&self) -> bool {
        *self == PLL2Q::Div116
    }
    ///pllx_q_ck = vcox_ck / 118
    #[inline(always)]
    pub fn is_div118(&self) -> bool {
        *self == PLL2Q::Div118
    }
    ///pllx_q_ck = vcox_ck / 120
    #[inline(always)]
    pub fn is_div120(&self) -> bool {
        *self == PLL2Q::Div120
    }
    ///pllx_q_ck = vcox_ck / 122
    #[inline(always)]
    pub fn is_div122(&self) -> bool {
        *self == PLL2Q::Div122
    }
    ///pllx_q_ck = vcox_ck / 124
    #[inline(always)]
    pub fn is_div124(&self) -> bool {
        *self == PLL2Q::Div124
    }
    ///pllx_q_ck = vcox_ck / 126
    #[inline(always)]
    pub fn is_div126(&self) -> bool {
        *self == PLL2Q::Div126
    }
    ///pllx_q_ck = vcox_ck / 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PLL2Q::Div128
    }
}
///Field `PLL2Q` writer - PLL2 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll2_q_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2Q_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PLL2Q>;
impl<'a, REG> PLL2Q_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pllx_q_ck = vcox_ck
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div1)
    }
    ///pllx_q_ck = vcox_ck / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div2)
    }
    ///pllx_q_ck = vcox_ck / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div4)
    }
    ///pllx_q_ck = vcox_ck / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div6)
    }
    ///pllx_q_ck = vcox_ck / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div8)
    }
    ///pllx_q_ck = vcox_ck / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div10)
    }
    ///pllx_q_ck = vcox_ck / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div12)
    }
    ///pllx_q_ck = vcox_ck / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div14)
    }
    ///pllx_q_ck = vcox_ck / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div16)
    }
    ///pllx_q_ck = vcox_ck / 18
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div18)
    }
    ///pllx_q_ck = vcox_ck / 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div20)
    }
    ///pllx_q_ck = vcox_ck / 22
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div22)
    }
    ///pllx_q_ck = vcox_ck / 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div24)
    }
    ///pllx_q_ck = vcox_ck / 26
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div26)
    }
    ///pllx_q_ck = vcox_ck / 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div28)
    }
    ///pllx_q_ck = vcox_ck / 30
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div30)
    }
    ///pllx_q_ck = vcox_ck / 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div32)
    }
    ///pllx_q_ck = vcox_ck / 34
    #[inline(always)]
    pub fn div34(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div34)
    }
    ///pllx_q_ck = vcox_ck / 36
    #[inline(always)]
    pub fn div36(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div36)
    }
    ///pllx_q_ck = vcox_ck / 38
    #[inline(always)]
    pub fn div38(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div38)
    }
    ///pllx_q_ck = vcox_ck / 40
    #[inline(always)]
    pub fn div40(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div40)
    }
    ///pllx_q_ck = vcox_ck / 42
    #[inline(always)]
    pub fn div42(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div42)
    }
    ///pllx_q_ck = vcox_ck / 44
    #[inline(always)]
    pub fn div44(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div44)
    }
    ///pllx_q_ck = vcox_ck / 46
    #[inline(always)]
    pub fn div46(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div46)
    }
    ///pllx_q_ck = vcox_ck / 48
    #[inline(always)]
    pub fn div48(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div48)
    }
    ///pllx_q_ck = vcox_ck / 50
    #[inline(always)]
    pub fn div50(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div50)
    }
    ///pllx_q_ck = vcox_ck / 52
    #[inline(always)]
    pub fn div52(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div52)
    }
    ///pllx_q_ck = vcox_ck / 54
    #[inline(always)]
    pub fn div54(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div54)
    }
    ///pllx_q_ck = vcox_ck / 56
    #[inline(always)]
    pub fn div56(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div56)
    }
    ///pllx_q_ck = vcox_ck / 58
    #[inline(always)]
    pub fn div58(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div58)
    }
    ///pllx_q_ck = vcox_ck / 60
    #[inline(always)]
    pub fn div60(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div60)
    }
    ///pllx_q_ck = vcox_ck / 62
    #[inline(always)]
    pub fn div62(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div62)
    }
    ///pllx_q_ck = vcox_ck / 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div64)
    }
    ///pllx_q_ck = vcox_ck / 66
    #[inline(always)]
    pub fn div66(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div66)
    }
    ///pllx_q_ck = vcox_ck / 68
    #[inline(always)]
    pub fn div68(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div68)
    }
    ///pllx_q_ck = vcox_ck / 70
    #[inline(always)]
    pub fn div70(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div70)
    }
    ///pllx_q_ck = vcox_ck / 72
    #[inline(always)]
    pub fn div72(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div72)
    }
    ///pllx_q_ck = vcox_ck / 74
    #[inline(always)]
    pub fn div74(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div74)
    }
    ///pllx_q_ck = vcox_ck / 76
    #[inline(always)]
    pub fn div76(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div76)
    }
    ///pllx_q_ck = vcox_ck / 78
    #[inline(always)]
    pub fn div78(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div78)
    }
    ///pllx_q_ck = vcox_ck / 80
    #[inline(always)]
    pub fn div80(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div80)
    }
    ///pllx_q_ck = vcox_ck / 82
    #[inline(always)]
    pub fn div82(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div82)
    }
    ///pllx_q_ck = vcox_ck / 84
    #[inline(always)]
    pub fn div84(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div84)
    }
    ///pllx_q_ck = vcox_ck / 86
    #[inline(always)]
    pub fn div86(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div86)
    }
    ///pllx_q_ck = vcox_ck / 88
    #[inline(always)]
    pub fn div88(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div88)
    }
    ///pllx_q_ck = vcox_ck / 90
    #[inline(always)]
    pub fn div90(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div90)
    }
    ///pllx_q_ck = vcox_ck / 92
    #[inline(always)]
    pub fn div92(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div92)
    }
    ///pllx_q_ck = vcox_ck / 94
    #[inline(always)]
    pub fn div94(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div94)
    }
    ///pllx_q_ck = vcox_ck / 96
    #[inline(always)]
    pub fn div96(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div96)
    }
    ///pllx_q_ck = vcox_ck / 98
    #[inline(always)]
    pub fn div98(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div98)
    }
    ///pllx_q_ck = vcox_ck / 100
    #[inline(always)]
    pub fn div100(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div100)
    }
    ///pllx_q_ck = vcox_ck / 102
    #[inline(always)]
    pub fn div102(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div102)
    }
    ///pllx_q_ck = vcox_ck / 104
    #[inline(always)]
    pub fn div104(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div104)
    }
    ///pllx_q_ck = vcox_ck / 106
    #[inline(always)]
    pub fn div106(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div106)
    }
    ///pllx_q_ck = vcox_ck / 108
    #[inline(always)]
    pub fn div108(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div108)
    }
    ///pllx_q_ck = vcox_ck / 110
    #[inline(always)]
    pub fn div110(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div110)
    }
    ///pllx_q_ck = vcox_ck / 112
    #[inline(always)]
    pub fn div112(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div112)
    }
    ///pllx_q_ck = vcox_ck / 114
    #[inline(always)]
    pub fn div114(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div114)
    }
    ///pllx_q_ck = vcox_ck / 116
    #[inline(always)]
    pub fn div116(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div116)
    }
    ///pllx_q_ck = vcox_ck / 118
    #[inline(always)]
    pub fn div118(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div118)
    }
    ///pllx_q_ck = vcox_ck / 120
    #[inline(always)]
    pub fn div120(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div120)
    }
    ///pllx_q_ck = vcox_ck / 122
    #[inline(always)]
    pub fn div122(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div122)
    }
    ///pllx_q_ck = vcox_ck / 124
    #[inline(always)]
    pub fn div124(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div124)
    }
    ///pllx_q_ck = vcox_ck / 126
    #[inline(always)]
    pub fn div126(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div126)
    }
    ///pllx_q_ck = vcox_ck / 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2Q::Div128)
    }
}
/**PLL2 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll2_r_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2R {
    ///0: pllx_r_ck = vcox_ck
    Div1 = 0,
    ///1: pllx_r_ck = vcox_ck / 2
    Div2 = 1,
    ///3: pllx_r_ck = vcox_ck / 4
    Div4 = 3,
    ///5: pllx_r_ck = vcox_ck / 6
    Div6 = 5,
    ///7: pllx_r_ck = vcox_ck / 8
    Div8 = 7,
    ///9: pllx_r_ck = vcox_ck / 10
    Div10 = 9,
    ///11: pllx_r_ck = vcox_ck / 12
    Div12 = 11,
    ///13: pllx_r_ck = vcox_ck / 14
    Div14 = 13,
    ///15: pllx_r_ck = vcox_ck / 16
    Div16 = 15,
    ///17: pllx_r_ck = vcox_ck / 18
    Div18 = 17,
    ///19: pllx_r_ck = vcox_ck / 20
    Div20 = 19,
    ///21: pllx_r_ck = vcox_ck / 22
    Div22 = 21,
    ///23: pllx_r_ck = vcox_ck / 24
    Div24 = 23,
    ///25: pllx_r_ck = vcox_ck / 26
    Div26 = 25,
    ///27: pllx_r_ck = vcox_ck / 28
    Div28 = 27,
    ///29: pllx_r_ck = vcox_ck / 30
    Div30 = 29,
    ///31: pllx_r_ck = vcox_ck / 32
    Div32 = 31,
    ///33: pllx_r_ck = vcox_ck / 34
    Div34 = 33,
    ///35: pllx_r_ck = vcox_ck / 36
    Div36 = 35,
    ///37: pllx_r_ck = vcox_ck / 38
    Div38 = 37,
    ///39: pllx_r_ck = vcox_ck / 40
    Div40 = 39,
    ///41: pllx_r_ck = vcox_ck / 42
    Div42 = 41,
    ///43: pllx_r_ck = vcox_ck / 44
    Div44 = 43,
    ///45: pllx_r_ck = vcox_ck / 46
    Div46 = 45,
    ///47: pllx_r_ck = vcox_ck / 48
    Div48 = 47,
    ///49: pllx_r_ck = vcox_ck / 50
    Div50 = 49,
    ///51: pllx_r_ck = vcox_ck / 52
    Div52 = 51,
    ///53: pllx_r_ck = vcox_ck / 54
    Div54 = 53,
    ///55: pllx_r_ck = vcox_ck / 56
    Div56 = 55,
    ///57: pllx_r_ck = vcox_ck / 58
    Div58 = 57,
    ///59: pllx_r_ck = vcox_ck / 60
    Div60 = 59,
    ///61: pllx_r_ck = vcox_ck / 62
    Div62 = 61,
    ///63: pllx_r_ck = vcox_ck / 64
    Div64 = 63,
    ///65: pllx_r_ck = vcox_ck / 66
    Div66 = 65,
    ///67: pllx_r_ck = vcox_ck / 68
    Div68 = 67,
    ///69: pllx_r_ck = vcox_ck / 70
    Div70 = 69,
    ///71: pllx_r_ck = vcox_ck / 72
    Div72 = 71,
    ///73: pllx_r_ck = vcox_ck / 74
    Div74 = 73,
    ///75: pllx_r_ck = vcox_ck / 76
    Div76 = 75,
    ///77: pllx_r_ck = vcox_ck / 78
    Div78 = 77,
    ///79: pllx_r_ck = vcox_ck / 80
    Div80 = 79,
    ///81: pllx_r_ck = vcox_ck / 82
    Div82 = 81,
    ///83: pllx_r_ck = vcox_ck / 84
    Div84 = 83,
    ///85: pllx_r_ck = vcox_ck / 86
    Div86 = 85,
    ///87: pllx_r_ck = vcox_ck / 88
    Div88 = 87,
    ///89: pllx_r_ck = vcox_ck / 90
    Div90 = 89,
    ///91: pllx_r_ck = vcox_ck / 92
    Div92 = 91,
    ///93: pllx_r_ck = vcox_ck / 94
    Div94 = 93,
    ///95: pllx_r_ck = vcox_ck / 96
    Div96 = 95,
    ///97: pllx_r_ck = vcox_ck / 98
    Div98 = 97,
    ///99: pllx_r_ck = vcox_ck / 100
    Div100 = 99,
    ///101: pllx_r_ck = vcox_ck / 102
    Div102 = 101,
    ///103: pllx_r_ck = vcox_ck / 104
    Div104 = 103,
    ///105: pllx_r_ck = vcox_ck / 106
    Div106 = 105,
    ///107: pllx_r_ck = vcox_ck / 108
    Div108 = 107,
    ///109: pllx_r_ck = vcox_ck / 110
    Div110 = 109,
    ///111: pllx_r_ck = vcox_ck / 112
    Div112 = 111,
    ///113: pllx_r_ck = vcox_ck / 114
    Div114 = 113,
    ///115: pllx_r_ck = vcox_ck / 116
    Div116 = 115,
    ///117: pllx_r_ck = vcox_ck / 118
    Div118 = 117,
    ///119: pllx_r_ck = vcox_ck / 120
    Div120 = 119,
    ///121: pllx_r_ck = vcox_ck / 122
    Div122 = 121,
    ///123: pllx_r_ck = vcox_ck / 124
    Div124 = 123,
    ///125: pllx_r_ck = vcox_ck / 126
    Div126 = 125,
    ///127: pllx_r_ck = vcox_ck / 128
    Div128 = 127,
}
impl From<PLL2R> for u8 {
    #[inline(always)]
    fn from(variant: PLL2R) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2R {
    type Ux = u8;
}
impl crate::IsEnum for PLL2R {}
///Field `PLL2R` reader - PLL2 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll2_r_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2R_R = crate::FieldReader<PLL2R>;
impl PLL2R_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL2R> {
        match self.bits {
            0 => Some(PLL2R::Div1),
            1 => Some(PLL2R::Div2),
            3 => Some(PLL2R::Div4),
            5 => Some(PLL2R::Div6),
            7 => Some(PLL2R::Div8),
            9 => Some(PLL2R::Div10),
            11 => Some(PLL2R::Div12),
            13 => Some(PLL2R::Div14),
            15 => Some(PLL2R::Div16),
            17 => Some(PLL2R::Div18),
            19 => Some(PLL2R::Div20),
            21 => Some(PLL2R::Div22),
            23 => Some(PLL2R::Div24),
            25 => Some(PLL2R::Div26),
            27 => Some(PLL2R::Div28),
            29 => Some(PLL2R::Div30),
            31 => Some(PLL2R::Div32),
            33 => Some(PLL2R::Div34),
            35 => Some(PLL2R::Div36),
            37 => Some(PLL2R::Div38),
            39 => Some(PLL2R::Div40),
            41 => Some(PLL2R::Div42),
            43 => Some(PLL2R::Div44),
            45 => Some(PLL2R::Div46),
            47 => Some(PLL2R::Div48),
            49 => Some(PLL2R::Div50),
            51 => Some(PLL2R::Div52),
            53 => Some(PLL2R::Div54),
            55 => Some(PLL2R::Div56),
            57 => Some(PLL2R::Div58),
            59 => Some(PLL2R::Div60),
            61 => Some(PLL2R::Div62),
            63 => Some(PLL2R::Div64),
            65 => Some(PLL2R::Div66),
            67 => Some(PLL2R::Div68),
            69 => Some(PLL2R::Div70),
            71 => Some(PLL2R::Div72),
            73 => Some(PLL2R::Div74),
            75 => Some(PLL2R::Div76),
            77 => Some(PLL2R::Div78),
            79 => Some(PLL2R::Div80),
            81 => Some(PLL2R::Div82),
            83 => Some(PLL2R::Div84),
            85 => Some(PLL2R::Div86),
            87 => Some(PLL2R::Div88),
            89 => Some(PLL2R::Div90),
            91 => Some(PLL2R::Div92),
            93 => Some(PLL2R::Div94),
            95 => Some(PLL2R::Div96),
            97 => Some(PLL2R::Div98),
            99 => Some(PLL2R::Div100),
            101 => Some(PLL2R::Div102),
            103 => Some(PLL2R::Div104),
            105 => Some(PLL2R::Div106),
            107 => Some(PLL2R::Div108),
            109 => Some(PLL2R::Div110),
            111 => Some(PLL2R::Div112),
            113 => Some(PLL2R::Div114),
            115 => Some(PLL2R::Div116),
            117 => Some(PLL2R::Div118),
            119 => Some(PLL2R::Div120),
            121 => Some(PLL2R::Div122),
            123 => Some(PLL2R::Div124),
            125 => Some(PLL2R::Div126),
            127 => Some(PLL2R::Div128),
            _ => None,
        }
    }
    ///pllx_r_ck = vcox_ck
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLL2R::Div1
    }
    ///pllx_r_ck = vcox_ck / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLL2R::Div2
    }
    ///pllx_r_ck = vcox_ck / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLL2R::Div4
    }
    ///pllx_r_ck = vcox_ck / 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLL2R::Div6
    }
    ///pllx_r_ck = vcox_ck / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLL2R::Div8
    }
    ///pllx_r_ck = vcox_ck / 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLL2R::Div10
    }
    ///pllx_r_ck = vcox_ck / 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLL2R::Div12
    }
    ///pllx_r_ck = vcox_ck / 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLL2R::Div14
    }
    ///pllx_r_ck = vcox_ck / 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLL2R::Div16
    }
    ///pllx_r_ck = vcox_ck / 18
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLL2R::Div18
    }
    ///pllx_r_ck = vcox_ck / 20
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLL2R::Div20
    }
    ///pllx_r_ck = vcox_ck / 22
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLL2R::Div22
    }
    ///pllx_r_ck = vcox_ck / 24
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLL2R::Div24
    }
    ///pllx_r_ck = vcox_ck / 26
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLL2R::Div26
    }
    ///pllx_r_ck = vcox_ck / 28
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLL2R::Div28
    }
    ///pllx_r_ck = vcox_ck / 30
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLL2R::Div30
    }
    ///pllx_r_ck = vcox_ck / 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLL2R::Div32
    }
    ///pllx_r_ck = vcox_ck / 34
    #[inline(always)]
    pub fn is_div34(&self) -> bool {
        *self == PLL2R::Div34
    }
    ///pllx_r_ck = vcox_ck / 36
    #[inline(always)]
    pub fn is_div36(&self) -> bool {
        *self == PLL2R::Div36
    }
    ///pllx_r_ck = vcox_ck / 38
    #[inline(always)]
    pub fn is_div38(&self) -> bool {
        *self == PLL2R::Div38
    }
    ///pllx_r_ck = vcox_ck / 40
    #[inline(always)]
    pub fn is_div40(&self) -> bool {
        *self == PLL2R::Div40
    }
    ///pllx_r_ck = vcox_ck / 42
    #[inline(always)]
    pub fn is_div42(&self) -> bool {
        *self == PLL2R::Div42
    }
    ///pllx_r_ck = vcox_ck / 44
    #[inline(always)]
    pub fn is_div44(&self) -> bool {
        *self == PLL2R::Div44
    }
    ///pllx_r_ck = vcox_ck / 46
    #[inline(always)]
    pub fn is_div46(&self) -> bool {
        *self == PLL2R::Div46
    }
    ///pllx_r_ck = vcox_ck / 48
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == PLL2R::Div48
    }
    ///pllx_r_ck = vcox_ck / 50
    #[inline(always)]
    pub fn is_div50(&self) -> bool {
        *self == PLL2R::Div50
    }
    ///pllx_r_ck = vcox_ck / 52
    #[inline(always)]
    pub fn is_div52(&self) -> bool {
        *self == PLL2R::Div52
    }
    ///pllx_r_ck = vcox_ck / 54
    #[inline(always)]
    pub fn is_div54(&self) -> bool {
        *self == PLL2R::Div54
    }
    ///pllx_r_ck = vcox_ck / 56
    #[inline(always)]
    pub fn is_div56(&self) -> bool {
        *self == PLL2R::Div56
    }
    ///pllx_r_ck = vcox_ck / 58
    #[inline(always)]
    pub fn is_div58(&self) -> bool {
        *self == PLL2R::Div58
    }
    ///pllx_r_ck = vcox_ck / 60
    #[inline(always)]
    pub fn is_div60(&self) -> bool {
        *self == PLL2R::Div60
    }
    ///pllx_r_ck = vcox_ck / 62
    #[inline(always)]
    pub fn is_div62(&self) -> bool {
        *self == PLL2R::Div62
    }
    ///pllx_r_ck = vcox_ck / 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PLL2R::Div64
    }
    ///pllx_r_ck = vcox_ck / 66
    #[inline(always)]
    pub fn is_div66(&self) -> bool {
        *self == PLL2R::Div66
    }
    ///pllx_r_ck = vcox_ck / 68
    #[inline(always)]
    pub fn is_div68(&self) -> bool {
        *self == PLL2R::Div68
    }
    ///pllx_r_ck = vcox_ck / 70
    #[inline(always)]
    pub fn is_div70(&self) -> bool {
        *self == PLL2R::Div70
    }
    ///pllx_r_ck = vcox_ck / 72
    #[inline(always)]
    pub fn is_div72(&self) -> bool {
        *self == PLL2R::Div72
    }
    ///pllx_r_ck = vcox_ck / 74
    #[inline(always)]
    pub fn is_div74(&self) -> bool {
        *self == PLL2R::Div74
    }
    ///pllx_r_ck = vcox_ck / 76
    #[inline(always)]
    pub fn is_div76(&self) -> bool {
        *self == PLL2R::Div76
    }
    ///pllx_r_ck = vcox_ck / 78
    #[inline(always)]
    pub fn is_div78(&self) -> bool {
        *self == PLL2R::Div78
    }
    ///pllx_r_ck = vcox_ck / 80
    #[inline(always)]
    pub fn is_div80(&self) -> bool {
        *self == PLL2R::Div80
    }
    ///pllx_r_ck = vcox_ck / 82
    #[inline(always)]
    pub fn is_div82(&self) -> bool {
        *self == PLL2R::Div82
    }
    ///pllx_r_ck = vcox_ck / 84
    #[inline(always)]
    pub fn is_div84(&self) -> bool {
        *self == PLL2R::Div84
    }
    ///pllx_r_ck = vcox_ck / 86
    #[inline(always)]
    pub fn is_div86(&self) -> bool {
        *self == PLL2R::Div86
    }
    ///pllx_r_ck = vcox_ck / 88
    #[inline(always)]
    pub fn is_div88(&self) -> bool {
        *self == PLL2R::Div88
    }
    ///pllx_r_ck = vcox_ck / 90
    #[inline(always)]
    pub fn is_div90(&self) -> bool {
        *self == PLL2R::Div90
    }
    ///pllx_r_ck = vcox_ck / 92
    #[inline(always)]
    pub fn is_div92(&self) -> bool {
        *self == PLL2R::Div92
    }
    ///pllx_r_ck = vcox_ck / 94
    #[inline(always)]
    pub fn is_div94(&self) -> bool {
        *self == PLL2R::Div94
    }
    ///pllx_r_ck = vcox_ck / 96
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        *self == PLL2R::Div96
    }
    ///pllx_r_ck = vcox_ck / 98
    #[inline(always)]
    pub fn is_div98(&self) -> bool {
        *self == PLL2R::Div98
    }
    ///pllx_r_ck = vcox_ck / 100
    #[inline(always)]
    pub fn is_div100(&self) -> bool {
        *self == PLL2R::Div100
    }
    ///pllx_r_ck = vcox_ck / 102
    #[inline(always)]
    pub fn is_div102(&self) -> bool {
        *self == PLL2R::Div102
    }
    ///pllx_r_ck = vcox_ck / 104
    #[inline(always)]
    pub fn is_div104(&self) -> bool {
        *self == PLL2R::Div104
    }
    ///pllx_r_ck = vcox_ck / 106
    #[inline(always)]
    pub fn is_div106(&self) -> bool {
        *self == PLL2R::Div106
    }
    ///pllx_r_ck = vcox_ck / 108
    #[inline(always)]
    pub fn is_div108(&self) -> bool {
        *self == PLL2R::Div108
    }
    ///pllx_r_ck = vcox_ck / 110
    #[inline(always)]
    pub fn is_div110(&self) -> bool {
        *self == PLL2R::Div110
    }
    ///pllx_r_ck = vcox_ck / 112
    #[inline(always)]
    pub fn is_div112(&self) -> bool {
        *self == PLL2R::Div112
    }
    ///pllx_r_ck = vcox_ck / 114
    #[inline(always)]
    pub fn is_div114(&self) -> bool {
        *self == PLL2R::Div114
    }
    ///pllx_r_ck = vcox_ck / 116
    #[inline(always)]
    pub fn is_div116(&self) -> bool {
        *self == PLL2R::Div116
    }
    ///pllx_r_ck = vcox_ck / 118
    #[inline(always)]
    pub fn is_div118(&self) -> bool {
        *self == PLL2R::Div118
    }
    ///pllx_r_ck = vcox_ck / 120
    #[inline(always)]
    pub fn is_div120(&self) -> bool {
        *self == PLL2R::Div120
    }
    ///pllx_r_ck = vcox_ck / 122
    #[inline(always)]
    pub fn is_div122(&self) -> bool {
        *self == PLL2R::Div122
    }
    ///pllx_r_ck = vcox_ck / 124
    #[inline(always)]
    pub fn is_div124(&self) -> bool {
        *self == PLL2R::Div124
    }
    ///pllx_r_ck = vcox_ck / 126
    #[inline(always)]
    pub fn is_div126(&self) -> bool {
        *self == PLL2R::Div126
    }
    ///pllx_r_ck = vcox_ck / 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PLL2R::Div128
    }
}
///Field `PLL2R` writer - PLL2 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll2_r_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2R_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PLL2R>;
impl<'a, REG> PLL2R_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pllx_r_ck = vcox_ck
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div1)
    }
    ///pllx_r_ck = vcox_ck / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div2)
    }
    ///pllx_r_ck = vcox_ck / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div4)
    }
    ///pllx_r_ck = vcox_ck / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div6)
    }
    ///pllx_r_ck = vcox_ck / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div8)
    }
    ///pllx_r_ck = vcox_ck / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div10)
    }
    ///pllx_r_ck = vcox_ck / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div12)
    }
    ///pllx_r_ck = vcox_ck / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div14)
    }
    ///pllx_r_ck = vcox_ck / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div16)
    }
    ///pllx_r_ck = vcox_ck / 18
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div18)
    }
    ///pllx_r_ck = vcox_ck / 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div20)
    }
    ///pllx_r_ck = vcox_ck / 22
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div22)
    }
    ///pllx_r_ck = vcox_ck / 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div24)
    }
    ///pllx_r_ck = vcox_ck / 26
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div26)
    }
    ///pllx_r_ck = vcox_ck / 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div28)
    }
    ///pllx_r_ck = vcox_ck / 30
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div30)
    }
    ///pllx_r_ck = vcox_ck / 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div32)
    }
    ///pllx_r_ck = vcox_ck / 34
    #[inline(always)]
    pub fn div34(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div34)
    }
    ///pllx_r_ck = vcox_ck / 36
    #[inline(always)]
    pub fn div36(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div36)
    }
    ///pllx_r_ck = vcox_ck / 38
    #[inline(always)]
    pub fn div38(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div38)
    }
    ///pllx_r_ck = vcox_ck / 40
    #[inline(always)]
    pub fn div40(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div40)
    }
    ///pllx_r_ck = vcox_ck / 42
    #[inline(always)]
    pub fn div42(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div42)
    }
    ///pllx_r_ck = vcox_ck / 44
    #[inline(always)]
    pub fn div44(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div44)
    }
    ///pllx_r_ck = vcox_ck / 46
    #[inline(always)]
    pub fn div46(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div46)
    }
    ///pllx_r_ck = vcox_ck / 48
    #[inline(always)]
    pub fn div48(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div48)
    }
    ///pllx_r_ck = vcox_ck / 50
    #[inline(always)]
    pub fn div50(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div50)
    }
    ///pllx_r_ck = vcox_ck / 52
    #[inline(always)]
    pub fn div52(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div52)
    }
    ///pllx_r_ck = vcox_ck / 54
    #[inline(always)]
    pub fn div54(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div54)
    }
    ///pllx_r_ck = vcox_ck / 56
    #[inline(always)]
    pub fn div56(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div56)
    }
    ///pllx_r_ck = vcox_ck / 58
    #[inline(always)]
    pub fn div58(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div58)
    }
    ///pllx_r_ck = vcox_ck / 60
    #[inline(always)]
    pub fn div60(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div60)
    }
    ///pllx_r_ck = vcox_ck / 62
    #[inline(always)]
    pub fn div62(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div62)
    }
    ///pllx_r_ck = vcox_ck / 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div64)
    }
    ///pllx_r_ck = vcox_ck / 66
    #[inline(always)]
    pub fn div66(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div66)
    }
    ///pllx_r_ck = vcox_ck / 68
    #[inline(always)]
    pub fn div68(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div68)
    }
    ///pllx_r_ck = vcox_ck / 70
    #[inline(always)]
    pub fn div70(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div70)
    }
    ///pllx_r_ck = vcox_ck / 72
    #[inline(always)]
    pub fn div72(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div72)
    }
    ///pllx_r_ck = vcox_ck / 74
    #[inline(always)]
    pub fn div74(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div74)
    }
    ///pllx_r_ck = vcox_ck / 76
    #[inline(always)]
    pub fn div76(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div76)
    }
    ///pllx_r_ck = vcox_ck / 78
    #[inline(always)]
    pub fn div78(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div78)
    }
    ///pllx_r_ck = vcox_ck / 80
    #[inline(always)]
    pub fn div80(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div80)
    }
    ///pllx_r_ck = vcox_ck / 82
    #[inline(always)]
    pub fn div82(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div82)
    }
    ///pllx_r_ck = vcox_ck / 84
    #[inline(always)]
    pub fn div84(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div84)
    }
    ///pllx_r_ck = vcox_ck / 86
    #[inline(always)]
    pub fn div86(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div86)
    }
    ///pllx_r_ck = vcox_ck / 88
    #[inline(always)]
    pub fn div88(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div88)
    }
    ///pllx_r_ck = vcox_ck / 90
    #[inline(always)]
    pub fn div90(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div90)
    }
    ///pllx_r_ck = vcox_ck / 92
    #[inline(always)]
    pub fn div92(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div92)
    }
    ///pllx_r_ck = vcox_ck / 94
    #[inline(always)]
    pub fn div94(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div94)
    }
    ///pllx_r_ck = vcox_ck / 96
    #[inline(always)]
    pub fn div96(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div96)
    }
    ///pllx_r_ck = vcox_ck / 98
    #[inline(always)]
    pub fn div98(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div98)
    }
    ///pllx_r_ck = vcox_ck / 100
    #[inline(always)]
    pub fn div100(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div100)
    }
    ///pllx_r_ck = vcox_ck / 102
    #[inline(always)]
    pub fn div102(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div102)
    }
    ///pllx_r_ck = vcox_ck / 104
    #[inline(always)]
    pub fn div104(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div104)
    }
    ///pllx_r_ck = vcox_ck / 106
    #[inline(always)]
    pub fn div106(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div106)
    }
    ///pllx_r_ck = vcox_ck / 108
    #[inline(always)]
    pub fn div108(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div108)
    }
    ///pllx_r_ck = vcox_ck / 110
    #[inline(always)]
    pub fn div110(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div110)
    }
    ///pllx_r_ck = vcox_ck / 112
    #[inline(always)]
    pub fn div112(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div112)
    }
    ///pllx_r_ck = vcox_ck / 114
    #[inline(always)]
    pub fn div114(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div114)
    }
    ///pllx_r_ck = vcox_ck / 116
    #[inline(always)]
    pub fn div116(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div116)
    }
    ///pllx_r_ck = vcox_ck / 118
    #[inline(always)]
    pub fn div118(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div118)
    }
    ///pllx_r_ck = vcox_ck / 120
    #[inline(always)]
    pub fn div120(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div120)
    }
    ///pllx_r_ck = vcox_ck / 122
    #[inline(always)]
    pub fn div122(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div122)
    }
    ///pllx_r_ck = vcox_ck / 124
    #[inline(always)]
    pub fn div124(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div124)
    }
    ///pllx_r_ck = vcox_ck / 126
    #[inline(always)]
    pub fn div126(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div126)
    }
    ///pllx_r_ck = vcox_ck / 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2R::Div128)
    }
}
impl R {
    ///Bits 0:8 - Multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = F<sub>ref2_ck</sub> x PLL2N, when fractional value 0 has been loaded in PLL2FRACN, with: PLL2N between 4 and 512 input frequency F<sub>ref2_ck</sub> between 1MHz and 16MHz
    #[inline(always)]
    pub fn pll2n(&self) -> PLL2N_R {
        PLL2N_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL2 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll2_p_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2p(&self) -> PLL2P_R {
        PLL2P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL2 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll2_q_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2q(&self) -> PLL2Q_R {
        PLL2Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL2 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll2_r_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2r(&self) -> PLL2R_R {
        PLL2R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2DIVR")
            .field("pll2n", &self.pll2n())
            .field("pll2p", &self.pll2p())
            .field("pll2q", &self.pll2q())
            .field("pll2r", &self.pll2r())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Multiplication factor for PLL2 VCO This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = F<sub>ref2_ck</sub> x PLL2N, when fractional value 0 has been loaded in PLL2FRACN, with: PLL2N between 4 and 512 input frequency F<sub>ref2_ck</sub> between 1MHz and 16MHz
    #[inline(always)]
    pub fn pll2n(&mut self) -> PLL2N_W<PLL2DIVRrs> {
        PLL2N_W::new(self, 0)
    }
    ///Bits 9:15 - PLL2 DIVP division factor This bitfield is set and reset by software to control the frequency of the pll2_p_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2p(&mut self) -> PLL2P_W<PLL2DIVRrs> {
        PLL2P_W::new(self, 9)
    }
    ///Bits 16:22 - PLL2 DIVQ division factor This bitfield is set and reset by software to control the frequency of the pll2_q_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2q(&mut self) -> PLL2Q_W<PLL2DIVRrs> {
        PLL2Q_W::new(self, 16)
    }
    ///Bits 24:30 - PLL2 DIVR division factor This bitfield is set and reset by software to control the frequency of the pll2_r_ck clock. It can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2r(&mut self) -> PLL2R_W<PLL2DIVRrs> {
        PLL2R_W::new(self, 24)
    }
}
/**RCC PLL2 dividers configuration register

You can [`read`](crate::Reg::read) this register and get [`pll2divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:PLL2DIVR)*/
pub struct PLL2DIVRrs;
impl crate::RegisterSpec for PLL2DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`pll2divr::R`](R) reader structure
impl crate::Readable for PLL2DIVRrs {}
///`write(|w| ..)` method takes [`pll2divr::W`](W) writer structure
impl crate::Writable for PLL2DIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2DIVR to value 0x0101_0280
impl crate::Resettable for PLL2DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
