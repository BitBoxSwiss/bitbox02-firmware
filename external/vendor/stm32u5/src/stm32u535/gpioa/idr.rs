///Register `IDR` reader
pub type R = crate::R<IDRrs>;
/**Port input data pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUT_DATA {
    ///0: Input is logic low
    Low = 0,
    ///1: Input is logic high
    High = 1,
}
impl From<INPUT_DATA> for bool {
    #[inline(always)]
    fn from(variant: INPUT_DATA) -> Self {
        variant as u8 != 0
    }
}
///Field `ID(0-15)` reader - Port input data pin %s
pub type ID_R = crate::BitReader<INPUT_DATA>;
impl ID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INPUT_DATA {
        match self.bits {
            false => INPUT_DATA::Low,
            true => INPUT_DATA::High,
        }
    }
    ///Input is logic low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == INPUT_DATA::Low
    }
    ///Input is logic high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == INPUT_DATA::High
    }
}
impl R {
    ///Port input data pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ID0` field.</div>
    #[inline(always)]
    pub fn id(&self, n: u8) -> ID_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ID_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Port input data pin (0-15)
    #[inline(always)]
    pub fn id_iter(&self) -> impl Iterator<Item = ID_R> + '_ {
        (0..16).map(move |n| ID_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Port input data pin 0
    #[inline(always)]
    pub fn id0(&self) -> ID_R {
        ID_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port input data pin 1
    #[inline(always)]
    pub fn id1(&self) -> ID_R {
        ID_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port input data pin 2
    #[inline(always)]
    pub fn id2(&self) -> ID_R {
        ID_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port input data pin 3
    #[inline(always)]
    pub fn id3(&self) -> ID_R {
        ID_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port input data pin 4
    #[inline(always)]
    pub fn id4(&self) -> ID_R {
        ID_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port input data pin 5
    #[inline(always)]
    pub fn id5(&self) -> ID_R {
        ID_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port input data pin 6
    #[inline(always)]
    pub fn id6(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port input data pin 7
    #[inline(always)]
    pub fn id7(&self) -> ID_R {
        ID_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port input data pin 8
    #[inline(always)]
    pub fn id8(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port input data pin 9
    #[inline(always)]
    pub fn id9(&self) -> ID_R {
        ID_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port input data pin 10
    #[inline(always)]
    pub fn id10(&self) -> ID_R {
        ID_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port input data pin 11
    #[inline(always)]
    pub fn id11(&self) -> ID_R {
        ID_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port input data pin 12
    #[inline(always)]
    pub fn id12(&self) -> ID_R {
        ID_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port input data pin 13
    #[inline(always)]
    pub fn id13(&self) -> ID_R {
        ID_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port input data pin 14
    #[inline(always)]
    pub fn id14(&self) -> ID_R {
        ID_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port input data pin 15
    #[inline(always)]
    pub fn id15(&self) -> ID_R {
        ID_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR")
            .field("id0", &self.id0())
            .field("id1", &self.id1())
            .field("id2", &self.id2())
            .field("id3", &self.id3())
            .field("id4", &self.id4())
            .field("id5", &self.id5())
            .field("id6", &self.id6())
            .field("id7", &self.id7())
            .field("id8", &self.id8())
            .field("id9", &self.id9())
            .field("id10", &self.id10())
            .field("id11", &self.id11())
            .field("id12", &self.id12())
            .field("id13", &self.id13())
            .field("id14", &self.id14())
            .field("id15", &self.id15())
            .finish()
    }
}
/**GPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#GPIOA:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
