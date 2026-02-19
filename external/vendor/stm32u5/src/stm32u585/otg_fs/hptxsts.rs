///Register `HPTXSTS` reader
pub type R = crate::R<HPTXSTSrs>;
///Field `PTXFSAVL` reader - PTXFSAVL
pub type PTXFSAVL_R = crate::FieldReader<u16>;
///Field `PTXQSAV` reader - PTXQSAV
pub type PTXQSAV_R = crate::FieldReader;
///Field `PTXQTOP` reader - PTXQTOP
pub type PTXQTOP_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - PTXFSAVL
    #[inline(always)]
    pub fn ptxfsavl(&self) -> PTXFSAVL_R {
        PTXFSAVL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - PTXQSAV
    #[inline(always)]
    pub fn ptxqsav(&self) -> PTXQSAV_R {
        PTXQSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - PTXQTOP
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXSTS")
            .field("ptxfsavl", &self.ptxfsavl())
            .field("ptxqsav", &self.ptxqsav())
            .field("ptxqtop", &self.ptxqtop())
            .finish()
    }
}
/**This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.

You can [`read`](crate::Reg::read) this register and get [`hptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HPTXSTS)*/
pub struct HPTXSTSrs;
impl crate::RegisterSpec for HPTXSTSrs {
    type Ux = u32;
}
///`read()` method returns [`hptxsts::R`](R) reader structure
impl crate::Readable for HPTXSTSrs {}
///`reset()` method sets HPTXSTS to value 0x0008_0100
impl crate::Resettable for HPTXSTSrs {
    const RESET_VALUE: u32 = 0x0008_0100;
}
