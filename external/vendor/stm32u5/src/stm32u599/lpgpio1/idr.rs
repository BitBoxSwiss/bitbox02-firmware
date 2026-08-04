///Register `IDR` reader
pub type R = crate::R<IDRrs>;
///Field `ID(0-15)` reader - ID%s
pub type ID_R = crate::BitReader;
impl R {
    ///ID(0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ID0` field.</div>
    #[inline(always)]
    pub fn id(&self, n: u8) -> ID_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ID_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///ID(0-15)
    #[inline(always)]
    pub fn id_iter(&self) -> impl Iterator<Item = ID_R> + '_ {
        (0..16).map(move |n| ID_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - ID0
    #[inline(always)]
    pub fn id0(&self) -> ID_R {
        ID_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ID1
    #[inline(always)]
    pub fn id1(&self) -> ID_R {
        ID_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ID2
    #[inline(always)]
    pub fn id2(&self) -> ID_R {
        ID_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ID3
    #[inline(always)]
    pub fn id3(&self) -> ID_R {
        ID_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ID4
    #[inline(always)]
    pub fn id4(&self) -> ID_R {
        ID_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ID5
    #[inline(always)]
    pub fn id5(&self) -> ID_R {
        ID_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ID6
    #[inline(always)]
    pub fn id6(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ID7
    #[inline(always)]
    pub fn id7(&self) -> ID_R {
        ID_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ID8
    #[inline(always)]
    pub fn id8(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ID9
    #[inline(always)]
    pub fn id9(&self) -> ID_R {
        ID_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ID10
    #[inline(always)]
    pub fn id10(&self) -> ID_R {
        ID_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ID11
    #[inline(always)]
    pub fn id11(&self) -> ID_R {
        ID_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ID12
    #[inline(always)]
    pub fn id12(&self) -> ID_R {
        ID_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ID13
    #[inline(always)]
    pub fn id13(&self) -> ID_R {
        ID_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ID14
    #[inline(always)]
    pub fn id14(&self) -> ID_R {
        ID_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ID15
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
/**LPGPIO port input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LPGPIO1:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
