///Register `SR` reader
pub type R = crate::R<SRrs>;
/**RTT4B

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTT4B {
    ///0: FIFO is not ready for a four-byte transfer
    NotReady = 0,
    ///1: FIFO is ready for a four-byte (32-bit) transfer. In receive mode, this means that at least four valid data bytes are in the FIFO. In transmit mode, this means that there are at least four bytes free in the FIFO
    Ready = 1,
}
impl From<RTT4B> for bool {
    #[inline(always)]
    fn from(variant: RTT4B) -> Self {
        variant as u8 != 0
    }
}
///Field `RTT4B` reader - RTT4B
pub type RTT4B_R = crate::BitReader<RTT4B>;
impl RTT4B_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTT4B {
        match self.bits {
            false => RTT4B::NotReady,
            true => RTT4B::Ready,
        }
    }
    ///FIFO is not ready for a four-byte transfer
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RTT4B::NotReady
    }
    ///FIFO is ready for a four-byte (32-bit) transfer. In receive mode, this means that at least four valid data bytes are in the FIFO. In transmit mode, this means that there are at least four bytes free in the FIFO
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RTT4B::Ready
    }
}
/**RTT1B

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTT1B {
    ///0: FIFO is not ready for a 1-byte transfer
    NotReady = 0,
    ///1: FIFO is ready for a one byte (32-bit) transfer. In receive mode, this means that at least one valid data byte is in the FIFO. In transmit mode, this means that there is at least one byte free in the FIFO
    Ready = 1,
}
impl From<RTT1B> for bool {
    #[inline(always)]
    fn from(variant: RTT1B) -> Self {
        variant as u8 != 0
    }
}
///Field `RTT1B` reader - RTT1B
pub type RTT1B_R = crate::BitReader<RTT1B>;
impl RTT1B_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTT1B {
        match self.bits {
            false => RTT1B::NotReady,
            true => RTT1B::Ready,
        }
    }
    ///FIFO is not ready for a 1-byte transfer
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RTT1B::NotReady
    }
    ///FIFO is ready for a one byte (32-bit) transfer. In receive mode, this means that at least one valid data byte is in the FIFO. In transmit mode, this means that there is at least one byte free in the FIFO
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RTT1B::Ready
    }
}
impl R {
    ///Bit 2 - RTT4B
    #[inline(always)]
    pub fn rtt4b(&self) -> RTT4B_R {
        RTT4B_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RTT1B
    #[inline(always)]
    pub fn rtt1b(&self) -> RTT1B_R {
        RTT1B_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rtt4b", &self.rtt4b())
            .field("rtt1b", &self.rtt1b())
            .finish()
    }
}
/**PSSI status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#PSSI:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
