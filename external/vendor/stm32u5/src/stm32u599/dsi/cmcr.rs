///Register `CMCR` reader
pub type R = crate::R<CMCRrs>;
///Register `CMCR` writer
pub type W = crate::W<CMCRrs>;
///Field `TEARE` reader - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:
pub type TEARE_R = crate::BitReader;
///Field `TEARE` writer - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:
pub type TEARE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARE` reader - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:
pub type ARE_R = crate::BitReader;
///Field `ARE` writer - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:
pub type ARE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GSW0TX` reader - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:
pub type GSW0TX_R = crate::BitReader;
///Field `GSW0TX` writer - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:
pub type GSW0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GSW1TX` reader - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:
pub type GSW1TX_R = crate::BitReader;
///Field `GSW1TX` writer - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:
pub type GSW1TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GSW2TX` reader - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:
pub type GSW2TX_R = crate::BitReader;
///Field `GSW2TX` writer - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:
pub type GSW2TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GSR0TX` reader - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:
pub type GSR0TX_R = crate::BitReader;
///Field `GSR0TX` writer - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:
pub type GSR0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GSR1TX` reader - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:
pub type GSR1TX_R = crate::BitReader;
///Field `GSR1TX` writer - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:
pub type GSR1TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GSR2TX` reader - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:
pub type GSR2TX_R = crate::BitReader;
///Field `GSR2TX` writer - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:
pub type GSR2TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GLWTX` reader - Generic long write transmission This bit configures the generic long write packet command transmission type :
pub type GLWTX_R = crate::BitReader;
///Field `GLWTX` writer - Generic long write transmission This bit configures the generic long write packet command transmission type :
pub type GLWTX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSW0TX` reader - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:
pub type DSW0TX_R = crate::BitReader;
///Field `DSW0TX` writer - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:
pub type DSW0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSW1TX` reader - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:
pub type DSW1TX_R = crate::BitReader;
///Field `DSW1TX` writer - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:
pub type DSW1TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSR0TX` reader - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:
pub type DSR0TX_R = crate::BitReader;
///Field `DSR0TX` writer - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:
pub type DSR0TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLWTX` reader - DCS long write transmission This bit configures the DCS long write packet command transmission type:
pub type DLWTX_R = crate::BitReader;
///Field `DLWTX` writer - DCS long write transmission This bit configures the DCS long write packet command transmission type:
pub type DLWTX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MRDPS` reader - Maximum read packet size This bit configures the maximum read packet size command transmission type:
pub type MRDPS_R = crate::BitReader;
///Field `MRDPS` writer - Maximum read packet size This bit configures the maximum read packet size command transmission type:
pub type MRDPS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:
    #[inline(always)]
    pub fn teare(&self) -> TEARE_R {
        TEARE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:
    #[inline(always)]
    pub fn are(&self) -> ARE_R {
        ARE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:
    #[inline(always)]
    pub fn gsw0tx(&self) -> GSW0TX_R {
        GSW0TX_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:
    #[inline(always)]
    pub fn gsw1tx(&self) -> GSW1TX_R {
        GSW1TX_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:
    #[inline(always)]
    pub fn gsw2tx(&self) -> GSW2TX_R {
        GSW2TX_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:
    #[inline(always)]
    pub fn gsr0tx(&self) -> GSR0TX_R {
        GSR0TX_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:
    #[inline(always)]
    pub fn gsr1tx(&self) -> GSR1TX_R {
        GSR1TX_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:
    #[inline(always)]
    pub fn gsr2tx(&self) -> GSR2TX_R {
        GSR2TX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Generic long write transmission This bit configures the generic long write packet command transmission type :
    #[inline(always)]
    pub fn glwtx(&self) -> GLWTX_R {
        GLWTX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:
    #[inline(always)]
    pub fn dsw0tx(&self) -> DSW0TX_R {
        DSW0TX_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:
    #[inline(always)]
    pub fn dsw1tx(&self) -> DSW1TX_R {
        DSW1TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:
    #[inline(always)]
    pub fn dsr0tx(&self) -> DSR0TX_R {
        DSR0TX_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DCS long write transmission This bit configures the DCS long write packet command transmission type:
    #[inline(always)]
    pub fn dlwtx(&self) -> DLWTX_R {
        DLWTX_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - Maximum read packet size This bit configures the maximum read packet size command transmission type:
    #[inline(always)]
    pub fn mrdps(&self) -> MRDPS_R {
        MRDPS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMCR")
            .field("teare", &self.teare())
            .field("are", &self.are())
            .field("gsw0tx", &self.gsw0tx())
            .field("gsw1tx", &self.gsw1tx())
            .field("gsw2tx", &self.gsw2tx())
            .field("gsr0tx", &self.gsr0tx())
            .field("gsr1tx", &self.gsr1tx())
            .field("gsr2tx", &self.gsr2tx())
            .field("glwtx", &self.glwtx())
            .field("dsw0tx", &self.dsw0tx())
            .field("dsw1tx", &self.dsw1tx())
            .field("dsr0tx", &self.dsr0tx())
            .field("dlwtx", &self.dlwtx())
            .field("mrdps", &self.mrdps())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tearing effect acknowledge request enable This bit enables the tearing effect acknowledge request:
    #[inline(always)]
    pub fn teare(&mut self) -> TEARE_W<CMCRrs> {
        TEARE_W::new(self, 0)
    }
    ///Bit 1 - Acknowledge request enable This bit enables the acknowledge request after each packet transmission:
    #[inline(always)]
    pub fn are(&mut self) -> ARE_W<CMCRrs> {
        ARE_W::new(self, 1)
    }
    ///Bit 8 - Generic short write zero parameters transmission This bit configures the generic short write packet with zero parameters command transmission type:
    #[inline(always)]
    pub fn gsw0tx(&mut self) -> GSW0TX_W<CMCRrs> {
        GSW0TX_W::new(self, 8)
    }
    ///Bit 9 - Generic short write one parameters transmission This bit configures the generic short write packet with one parameters command transmission type:
    #[inline(always)]
    pub fn gsw1tx(&mut self) -> GSW1TX_W<CMCRrs> {
        GSW1TX_W::new(self, 9)
    }
    ///Bit 10 - Generic short write two parameters transmission This bit configures the generic short write packet with two parameters command transmission type:
    #[inline(always)]
    pub fn gsw2tx(&mut self) -> GSW2TX_W<CMCRrs> {
        GSW2TX_W::new(self, 10)
    }
    ///Bit 11 - Generic short read zero parameters transmission This bit configures the generic short read packet with zero parameters command transmission type:
    #[inline(always)]
    pub fn gsr0tx(&mut self) -> GSR0TX_W<CMCRrs> {
        GSR0TX_W::new(self, 11)
    }
    ///Bit 12 - Generic short read one parameters transmission This bit configures the generic short read packet with one parameters command transmission type:
    #[inline(always)]
    pub fn gsr1tx(&mut self) -> GSR1TX_W<CMCRrs> {
        GSR1TX_W::new(self, 12)
    }
    ///Bit 13 - Generic short read two parameters transmission This bit configures the generic short read packet with two parameters command transmission type:
    #[inline(always)]
    pub fn gsr2tx(&mut self) -> GSR2TX_W<CMCRrs> {
        GSR2TX_W::new(self, 13)
    }
    ///Bit 14 - Generic long write transmission This bit configures the generic long write packet command transmission type :
    #[inline(always)]
    pub fn glwtx(&mut self) -> GLWTX_W<CMCRrs> {
        GLWTX_W::new(self, 14)
    }
    ///Bit 16 - DCS short write zero parameter transmission This bit configures the DCS short write packet with zero parameter command transmission type:
    #[inline(always)]
    pub fn dsw0tx(&mut self) -> DSW0TX_W<CMCRrs> {
        DSW0TX_W::new(self, 16)
    }
    ///Bit 17 - DCS short read one parameter transmission This bit configures the DCS short read packet with one parameter command transmission type:
    #[inline(always)]
    pub fn dsw1tx(&mut self) -> DSW1TX_W<CMCRrs> {
        DSW1TX_W::new(self, 17)
    }
    ///Bit 18 - DCS short read zero parameter transmission This bit configures the DCS short read packet with zero parameter command transmission type:
    #[inline(always)]
    pub fn dsr0tx(&mut self) -> DSR0TX_W<CMCRrs> {
        DSR0TX_W::new(self, 18)
    }
    ///Bit 19 - DCS long write transmission This bit configures the DCS long write packet command transmission type:
    #[inline(always)]
    pub fn dlwtx(&mut self) -> DLWTX_W<CMCRrs> {
        DLWTX_W::new(self, 19)
    }
    ///Bit 24 - Maximum read packet size This bit configures the maximum read packet size command transmission type:
    #[inline(always)]
    pub fn mrdps(&mut self) -> MRDPS_W<CMCRrs> {
        MRDPS_W::new(self, 24)
    }
}
/**DSI Host command mode configuration register

You can [`read`](crate::Reg::read) this register and get [`cmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:CMCR)*/
pub struct CMCRrs;
impl crate::RegisterSpec for CMCRrs {
    type Ux = u32;
}
///`read()` method returns [`cmcr::R`](R) reader structure
impl crate::Readable for CMCRrs {}
///`write(|w| ..)` method takes [`cmcr::W`](W) writer structure
impl crate::Writable for CMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMCR to value 0
impl crate::Resettable for CMCRrs {}
