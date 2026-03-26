///Register `GRXSTSP_DEVICE` reader
pub type R = crate::R<GRXSTSP_DEVICErs>;
///Field `EPNUM` reader - EPNUM
pub type EPNUM_R = crate::FieldReader;
///Field `BCNT` reader - BCNT
pub type BCNT_R = crate::FieldReader<u16>;
///Field `DPID` reader - DPID
pub type DPID_R = crate::FieldReader;
///Field `PKTSTS` reader - PKTSTS
pub type PKTSTS_R = crate::FieldReader;
///Field `FRMNUM` reader - FRMNUM
pub type FRMNUM_R = crate::FieldReader;
///Field `STSPHST` reader - STSPHST
pub type STSPHST_R = crate::BitReader;
impl R {
    ///Bits 0:3 - EPNUM
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:14 - BCNT
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    ///Bits 15:16 - DPID
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 17:20 - PKTSTS
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bits 21:24 - FRMNUM
    #[inline(always)]
    pub fn frmnum(&self) -> FRMNUM_R {
        FRMNUM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    ///Bit 27 - STSPHST
    #[inline(always)]
    pub fn stsphst(&self) -> STSPHST_R {
        STSPHST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSP_DEVICE")
            .field("epnum", &self.epnum())
            .field("bcnt", &self.bcnt())
            .field("dpid", &self.dpid())
            .field("pktsts", &self.pktsts())
            .field("frmnum", &self.frmnum())
            .field("stsphst", &self.stsphst())
            .finish()
    }
}
/**This description is for register GRXSTSP in Device mode. Similarly to GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in GINTSTS) is asserted.

You can [`read`](crate::Reg::read) this register and get [`grxstsp_device::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#OTG_HS:GRXSTSP_DEVICE)*/
pub struct GRXSTSP_DEVICErs;
impl crate::RegisterSpec for GRXSTSP_DEVICErs {
    type Ux = u32;
}
///`read()` method returns [`grxstsp_device::R`](R) reader structure
impl crate::Readable for GRXSTSP_DEVICErs {}
///`reset()` method sets GRXSTSP_DEVICE to value 0
impl crate::Resettable for GRXSTSP_DEVICErs {}
