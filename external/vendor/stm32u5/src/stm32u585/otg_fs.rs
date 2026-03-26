#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gotgctl: GOTGCTL,
    gotgint: GOTGINT,
    gahbcfg: GAHBCFG,
    gusbcfg: GUSBCFG,
    grstctl: GRSTCTL,
    gintsts: GINTSTS,
    gintmsk: GINTMSK,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    grxfsiz: GRXFSIZ,
    hnptxfsiz: HNPTXFSIZ,
    hnptxsts: HNPTXSTS,
    _reserved12: [u8; 0x08],
    gccfg: GCCFG,
    cid: CID,
    _reserved14: [u8; 0x14],
    glpmcfg: GLPMCFG,
    _reserved15: [u8; 0xa8],
    hptxfsiz: HPTXFSIZ,
    dieptxf1: DIEPTXF1,
    dieptxf2: DIEPTXF2,
    dieptxf3: DIEPTXF3,
    dieptxf4: DIEPTXF4,
    dieptxf5: DIEPTXF5,
    _reserved21: [u8; 0x02e8],
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved24: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    _reserved27: [u8; 0x24],
    hprt: HPRT,
    _reserved28: [u8; 0xbc],
    hcchar0: HCCHAR0,
    _reserved29: [u8; 0x04],
    hcint0: HCINT0,
    hcintmsk0: HCINTMSK0,
    hctsiz0: HCTSIZ0,
    _reserved32: [u8; 0x0c],
    hcchar1: HCCHAR1,
    _reserved33: [u8; 0x04],
    hcint1: HCINT1,
    hcintmsk1: HCINTMSK1,
    hctsiz1: HCTSIZ1,
    _reserved36: [u8; 0x0c],
    hcchar2: HCCHAR2,
    _reserved37: [u8; 0x04],
    hcint2: HCINT2,
    hcintmsk2: HCINTMSK2,
    hctsiz2: HCTSIZ2,
    _reserved40: [u8; 0x0c],
    hcchar3: HCCHAR3,
    _reserved41: [u8; 0x04],
    hcint3: HCINT3,
    hcintmsk3: HCINTMSK3,
    hctsiz3: HCTSIZ3,
    _reserved44: [u8; 0x0c],
    hcchar4: HCCHAR4,
    _reserved45: [u8; 0x04],
    hcint4: HCINT4,
    hcintmsk4: HCINTMSK4,
    hctsiz4: HCTSIZ4,
    _reserved48: [u8; 0x0c],
    hcchar5: HCCHAR5,
    _reserved49: [u8; 0x04],
    hcint5: HCINT5,
    hcintmsk5: HCINTMSK5,
    hctsiz5: HCTSIZ5,
    _reserved52: [u8; 0x0c],
    hcchar6: HCCHAR6,
    _reserved53: [u8; 0x04],
    hcint6: HCINT6,
    hcintmsk6: HCINTMSK6,
    hctsiz6: HCTSIZ6,
    _reserved56: [u8; 0x0c],
    hcchar7: HCCHAR7,
    _reserved57: [u8; 0x04],
    hcint7: HCINT7,
    hcintmsk7: HCINTMSK7,
    hctsiz7: HCTSIZ7,
    _reserved60: [u8; 0x0c],
    hcchar8: HCCHAR8,
    _reserved61: [u8; 0x04],
    hcint8: HCINT8,
    hcintmsk8: HCINTMSK8,
    hctsiz8: HCTSIZ8,
    _reserved64: [u8; 0x0c],
    hcchar9: HCCHAR9,
    _reserved65: [u8; 0x04],
    hcint9: HCINT9,
    hcintmsk9: HCINTMSK9,
    hctsiz9: HCTSIZ9,
    _reserved68: [u8; 0x0c],
    hcchar10: HCCHAR10,
    _reserved69: [u8; 0x04],
    hcint10: HCINT10,
    hcintmsk10: HCINTMSK10,
    hctsiz10: HCTSIZ10,
    _reserved72: [u8; 0x0c],
    hcchar11: HCCHAR11,
    _reserved73: [u8; 0x04],
    hcint11: HCINT11,
    hcintmsk11: HCINTMSK11,
    hctsiz11: HCTSIZ11,
    _reserved76: [u8; 0x018c],
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved79: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved83: [u8; 0x08],
    dvbusdis: DVBUSDIS,
    dvbuspulse: DVBUSPULSE,
    _reserved85: [u8; 0x04],
    diepempmsk: DIEPEMPMSK,
    _reserved86: [u8; 0xc8],
    diepctl0: DIEPCTL0,
    _reserved87: [u8; 0x04],
    diepint0: DIEPINT0,
    _reserved88: [u8; 0x04],
    dieptsiz0: DIEPTSIZ0,
    _reserved89: [u8; 0x04],
    dtxfsts0: DTXFSTS0,
    _reserved90: [u8; 0x04],
    diepctl1: DIEPCTL1,
    _reserved91: [u8; 0x04],
    diepint1: DIEPINT1,
    _reserved92: [u8; 0x04],
    dieptsiz1: DIEPTSIZ1,
    _reserved93: [u8; 0x04],
    dtxfsts1: DTXFSTS1,
    _reserved94: [u8; 0x04],
    diepctl2: DIEPCTL2,
    _reserved95: [u8; 0x04],
    diepint2: DIEPINT2,
    _reserved96: [u8; 0x04],
    dieptsiz2: DIEPTSIZ2,
    _reserved97: [u8; 0x04],
    dtxfsts2: DTXFSTS2,
    _reserved98: [u8; 0x04],
    diepctl3: DIEPCTL3,
    _reserved99: [u8; 0x04],
    diepint3: DIEPINT3,
    _reserved100: [u8; 0x04],
    dieptsiz3: DIEPTSIZ3,
    _reserved101: [u8; 0x04],
    dtxfsts3: DTXFSTS3,
    _reserved102: [u8; 0x04],
    diepctl4: DIEPCTL4,
    _reserved103: [u8; 0x04],
    diepint4: DIEPINT4,
    _reserved104: [u8; 0x04],
    dieptsiz4: DIEPTSIZ4,
    _reserved105: [u8; 0x04],
    dtxfsts4: DTXFSTS4,
    _reserved106: [u8; 0x04],
    diepctl5: DIEPCTL5,
    _reserved107: [u8; 0x04],
    diepint5: DIEPINT5,
    _reserved108: [u8; 0x04],
    dieptsiz5: DIEPTSIZ5,
    _reserved109: [u8; 0x04],
    dtxfsts5: DTXFSTS5,
    _reserved110: [u8; 0x0144],
    doepctl0: DOEPCTL0,
    _reserved111: [u8; 0x04],
    doepint0: DOEPINT0,
    _reserved112: [u8; 0x04],
    doeptsiz0: DOEPTSIZ0,
    _reserved113: [u8; 0x0c],
    doepctl1: DOEPCTL1,
    _reserved114: [u8; 0x04],
    doepint1: DOEPINT1,
    _reserved115: [u8; 0x04],
    doeptsiz1: DOEPTSIZ1,
    _reserved116: [u8; 0x0c],
    doepctl2: DOEPCTL2,
    _reserved117: [u8; 0x04],
    doepint2: DOEPINT2,
    _reserved118: [u8; 0x04],
    doeptsiz2: DOEPTSIZ2,
    _reserved119: [u8; 0x0c],
    doepctl3: DOEPCTL3,
    _reserved120: [u8; 0x04],
    doepint3: DOEPINT3,
    _reserved121: [u8; 0x04],
    doeptsiz3: DOEPTSIZ3,
    _reserved122: [u8; 0x0c],
    doepctl4: DOEPCTL4,
    _reserved123: [u8; 0x04],
    doepint4: DOEPINT4,
    _reserved124: [u8; 0x04],
    doeptsiz4: DOEPTSIZ4,
    _reserved125: [u8; 0x0c],
    doepctl5: DOEPCTL5,
    _reserved126: [u8; 0x04],
    doepint5: DOEPINT5,
    _reserved127: [u8; 0x04],
    doeptsiz5: DOEPTSIZ5,
    _reserved128: [u8; 0x024c],
    pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    ///0x00 - The GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.
    #[inline(always)]
    pub const fn gotgctl(&self) -> &GOTGCTL {
        &self.gotgctl
    }
    ///0x04 - The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.
    #[inline(always)]
    pub const fn gotgint(&self) -> &GOTGINT {
        &self.gotgint
    }
    ///0x08 - This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    ///0x0c - This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    ///0x10 - The application uses this register to reset various hardware features inside the core.
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    ///0x14 - This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.
    #[inline(always)]
    pub const fn gintsts(&self) -> &GINTSTS {
        &self.gintsts
    }
    ///0x18 - This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (GINTSTS) register bit corresponding to that interrupt is still set.
    #[inline(always)]
    pub const fn gintmsk(&self) -> &GINTMSK {
        &self.gintmsk
    }
    ///0x1c - This description is for register GRXSTSR in Host mode
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - This description is for register GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000.
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - This description is for register GRXSTSP in HOST mode
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - This description is for register GRXSTSP in Device mode. Similarly to GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in GINTSTS) is asserted.
    #[inline(always)]
    pub const fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x24 - The application can program the RAM size that must be allocated to the Rx FIFO.
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    ///0x28 - Host mode
    #[inline(always)]
    pub const fn hnptxfsiz(&self) -> &HNPTXFSIZ {
        &self.hnptxfsiz
    }
    ///0x2c - In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.
    #[inline(always)]
    pub const fn hnptxsts(&self) -> &HNPTXSTS {
        &self.hnptxsts
    }
    ///0x38 - OTG general core configuration register
    #[inline(always)]
    pub const fn gccfg(&self) -> &GCCFG {
        &self.gccfg
    }
    ///0x3c - This is a register containing the Product ID as reset value.
    #[inline(always)]
    pub const fn cid(&self) -> &CID {
        &self.cid
    }
    ///0x54 - OTG core LPM configuration register
    #[inline(always)]
    pub const fn glpmcfg(&self) -> &GLPMCFG {
        &self.glpmcfg
    }
    ///0x100 - OTG host periodic transmit FIFO size register
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &HPTXFSIZ {
        &self.hptxfsiz
    }
    ///0x104 - OTG device IN endpoint transmit FIFO 1 size register
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF1 {
        &self.dieptxf1
    }
    ///0x108 - OTG device IN endpoint transmit FIFO 2 size register
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF2 {
        &self.dieptxf2
    }
    ///0x10c - OTG device IN endpoint transmit FIFO 3 size register
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF3 {
        &self.dieptxf3
    }
    ///0x110 - OTG device IN endpoint transmit FIFO 4 size register
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF4 {
        &self.dieptxf4
    }
    ///0x114 - OTG device IN endpoint transmit FIFO 5 size register
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &DIEPTXF5 {
        &self.dieptxf5
    }
    ///0x400 - This register configures the core after power-on. Do not make changes to this register after initializing the host.
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    ///0x404 - This register stores the frame interval information for the current speed to which the OTG controller has enumerated.
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    ///0x408 - This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    ///0x410 - This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    ///0x414 - When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    ///0x418 - The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    ///0x440 - This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    ///0x500 - OTG host channel 0 characteristics register
    #[inline(always)]
    pub const fn hcchar0(&self) -> &HCCHAR0 {
        &self.hcchar0
    }
    ///0x508 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint0(&self) -> &HCINT0 {
        &self.hcint0
    }
    ///0x50c - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk0(&self) -> &HCINTMSK0 {
        &self.hcintmsk0
    }
    ///0x510 - OTG host channel 0 transfer size register
    #[inline(always)]
    pub const fn hctsiz0(&self) -> &HCTSIZ0 {
        &self.hctsiz0
    }
    ///0x520 - OTG host channel 1 characteristics register
    #[inline(always)]
    pub const fn hcchar1(&self) -> &HCCHAR1 {
        &self.hcchar1
    }
    ///0x528 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint1(&self) -> &HCINT1 {
        &self.hcint1
    }
    ///0x52c - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk1(&self) -> &HCINTMSK1 {
        &self.hcintmsk1
    }
    ///0x530 - OTG host channel 1 transfer size register
    #[inline(always)]
    pub const fn hctsiz1(&self) -> &HCTSIZ1 {
        &self.hctsiz1
    }
    ///0x540 - OTG host channel 2 characteristics register
    #[inline(always)]
    pub const fn hcchar2(&self) -> &HCCHAR2 {
        &self.hcchar2
    }
    ///0x548 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint2(&self) -> &HCINT2 {
        &self.hcint2
    }
    ///0x54c - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk2(&self) -> &HCINTMSK2 {
        &self.hcintmsk2
    }
    ///0x550 - OTG host channel 2 transfer size register
    #[inline(always)]
    pub const fn hctsiz2(&self) -> &HCTSIZ2 {
        &self.hctsiz2
    }
    ///0x560 - OTG host channel 3 characteristics register
    #[inline(always)]
    pub const fn hcchar3(&self) -> &HCCHAR3 {
        &self.hcchar3
    }
    ///0x568 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint3(&self) -> &HCINT3 {
        &self.hcint3
    }
    ///0x56c - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk3(&self) -> &HCINTMSK3 {
        &self.hcintmsk3
    }
    ///0x570 - OTG host channel 3 transfer size register
    #[inline(always)]
    pub const fn hctsiz3(&self) -> &HCTSIZ3 {
        &self.hctsiz3
    }
    ///0x580 - OTG host channel 4 characteristics register
    #[inline(always)]
    pub const fn hcchar4(&self) -> &HCCHAR4 {
        &self.hcchar4
    }
    ///0x588 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint4(&self) -> &HCINT4 {
        &self.hcint4
    }
    ///0x58c - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk4(&self) -> &HCINTMSK4 {
        &self.hcintmsk4
    }
    ///0x590 - OTG host channel 4 transfer size register
    #[inline(always)]
    pub const fn hctsiz4(&self) -> &HCTSIZ4 {
        &self.hctsiz4
    }
    ///0x5a0 - OTG host channel 5 characteristics register
    #[inline(always)]
    pub const fn hcchar5(&self) -> &HCCHAR5 {
        &self.hcchar5
    }
    ///0x5a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint5(&self) -> &HCINT5 {
        &self.hcint5
    }
    ///0x5ac - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk5(&self) -> &HCINTMSK5 {
        &self.hcintmsk5
    }
    ///0x5b0 - OTG host channel 5 transfer size register
    #[inline(always)]
    pub const fn hctsiz5(&self) -> &HCTSIZ5 {
        &self.hctsiz5
    }
    ///0x5c0 - OTG host channel 6 characteristics register
    #[inline(always)]
    pub const fn hcchar6(&self) -> &HCCHAR6 {
        &self.hcchar6
    }
    ///0x5c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint6(&self) -> &HCINT6 {
        &self.hcint6
    }
    ///0x5cc - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk6(&self) -> &HCINTMSK6 {
        &self.hcintmsk6
    }
    ///0x5d0 - OTG host channel 6 transfer size register
    #[inline(always)]
    pub const fn hctsiz6(&self) -> &HCTSIZ6 {
        &self.hctsiz6
    }
    ///0x5e0 - OTG host channel 7 characteristics register
    #[inline(always)]
    pub const fn hcchar7(&self) -> &HCCHAR7 {
        &self.hcchar7
    }
    ///0x5e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint7(&self) -> &HCINT7 {
        &self.hcint7
    }
    ///0x5ec - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk7(&self) -> &HCINTMSK7 {
        &self.hcintmsk7
    }
    ///0x5f0 - OTG host channel 7 transfer size register
    #[inline(always)]
    pub const fn hctsiz7(&self) -> &HCTSIZ7 {
        &self.hctsiz7
    }
    ///0x600 - OTG host channel 8 characteristics register
    #[inline(always)]
    pub const fn hcchar8(&self) -> &HCCHAR8 {
        &self.hcchar8
    }
    ///0x608 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint8(&self) -> &HCINT8 {
        &self.hcint8
    }
    ///0x60c - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk8(&self) -> &HCINTMSK8 {
        &self.hcintmsk8
    }
    ///0x610 - OTG host channel 8 transfer size register
    #[inline(always)]
    pub const fn hctsiz8(&self) -> &HCTSIZ8 {
        &self.hctsiz8
    }
    ///0x620 - OTG host channel 9 characteristics register
    #[inline(always)]
    pub const fn hcchar9(&self) -> &HCCHAR9 {
        &self.hcchar9
    }
    ///0x628 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint9(&self) -> &HCINT9 {
        &self.hcint9
    }
    ///0x62c - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk9(&self) -> &HCINTMSK9 {
        &self.hcintmsk9
    }
    ///0x630 - OTG host channel 9 transfer size register
    #[inline(always)]
    pub const fn hctsiz9(&self) -> &HCTSIZ9 {
        &self.hctsiz9
    }
    ///0x640 - OTG host channel 10 characteristics register
    #[inline(always)]
    pub const fn hcchar10(&self) -> &HCCHAR10 {
        &self.hcchar10
    }
    ///0x648 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint10(&self) -> &HCINT10 {
        &self.hcint10
    }
    ///0x64c - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk10(&self) -> &HCINTMSK10 {
        &self.hcintmsk10
    }
    ///0x650 - OTG host channel 10 transfer size register
    #[inline(always)]
    pub const fn hctsiz10(&self) -> &HCTSIZ10 {
        &self.hctsiz10
    }
    ///0x660 - OTG host channel 11 characteristics register
    #[inline(always)]
    pub const fn hcchar11(&self) -> &HCCHAR11 {
        &self.hcchar11
    }
    ///0x668 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn hcint11(&self) -> &HCINT11 {
        &self.hcint11
    }
    ///0x66c - This register reflects the mask for each channel status described in the previous section.
    #[inline(always)]
    pub const fn hcintmsk11(&self) -> &HCINTMSK11 {
        &self.hcintmsk11
    }
    ///0x670 - OTG host channel 11 transfer size register
    #[inline(always)]
    pub const fn hctsiz11(&self) -> &HCTSIZ11 {
        &self.hctsiz11
    }
    ///0x800 - This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    ///0x804 - OTG device control register
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    ///0x808 - This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (DAINT) register.
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    ///0x810 - This register works with each of the DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    ///0x814 - This register works with each of the DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    ///0x818 - When a significant event occurs on an endpoint, a DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the GINTSTS register (OEPINT or IEPINT in GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (DIEPINTx/DOEPINTx).
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    ///0x81c - The DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the DAINT register bit corresponding to that interrupt is still set.
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    ///0x828 - This register specifies the VBUS discharge time after VBUS pulsing during SRP.
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &DVBUSDIS {
        &self.dvbusdis
    }
    ///0x82c - This register specifies the VBUS pulsing time during SRP.
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &DVBUSPULSE {
        &self.dvbuspulse
    }
    ///0x834 - This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_DIEPINTx).
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    ///0x900 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    #[inline(always)]
    pub const fn diepctl0(&self) -> &DIEPCTL0 {
        &self.diepctl0
    }
    ///0x908 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn diepint0(&self) -> &DIEPINT0 {
        &self.diepint0
    }
    ///0x910 - The application must modify this register before enabling endpoint 0.
    #[inline(always)]
    pub const fn dieptsiz0(&self) -> &DIEPTSIZ0 {
        &self.dieptsiz0
    }
    ///0x918 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    #[inline(always)]
    pub const fn dtxfsts0(&self) -> &DTXFSTS0 {
        &self.dtxfsts0
    }
    ///0x920 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    #[inline(always)]
    pub const fn diepctl1(&self) -> &DIEPCTL1 {
        &self.diepctl1
    }
    ///0x928 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn diepint1(&self) -> &DIEPINT1 {
        &self.diepint1
    }
    ///0x930 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    #[inline(always)]
    pub const fn dieptsiz1(&self) -> &DIEPTSIZ1 {
        &self.dieptsiz1
    }
    ///0x938 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    #[inline(always)]
    pub const fn dtxfsts1(&self) -> &DTXFSTS1 {
        &self.dtxfsts1
    }
    ///0x940 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    #[inline(always)]
    pub const fn diepctl2(&self) -> &DIEPCTL2 {
        &self.diepctl2
    }
    ///0x948 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn diepint2(&self) -> &DIEPINT2 {
        &self.diepint2
    }
    ///0x950 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    #[inline(always)]
    pub const fn dieptsiz2(&self) -> &DIEPTSIZ2 {
        &self.dieptsiz2
    }
    ///0x958 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    #[inline(always)]
    pub const fn dtxfsts2(&self) -> &DTXFSTS2 {
        &self.dtxfsts2
    }
    ///0x960 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    #[inline(always)]
    pub const fn diepctl3(&self) -> &DIEPCTL3 {
        &self.diepctl3
    }
    ///0x968 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn diepint3(&self) -> &DIEPINT3 {
        &self.diepint3
    }
    ///0x970 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    #[inline(always)]
    pub const fn dieptsiz3(&self) -> &DIEPTSIZ3 {
        &self.dieptsiz3
    }
    ///0x978 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    #[inline(always)]
    pub const fn dtxfsts3(&self) -> &DTXFSTS3 {
        &self.dtxfsts3
    }
    ///0x980 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    #[inline(always)]
    pub const fn diepctl4(&self) -> &DIEPCTL4 {
        &self.diepctl4
    }
    ///0x988 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn diepint4(&self) -> &DIEPINT4 {
        &self.diepint4
    }
    ///0x990 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    #[inline(always)]
    pub const fn dieptsiz4(&self) -> &DIEPTSIZ4 {
        &self.dieptsiz4
    }
    ///0x998 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    #[inline(always)]
    pub const fn dtxfsts4(&self) -> &DTXFSTS4 {
        &self.dtxfsts4
    }
    ///0x9a0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    #[inline(always)]
    pub const fn diepctl5(&self) -> &DIEPCTL5 {
        &self.diepctl5
    }
    ///0x9a8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn diepint5(&self) -> &DIEPINT5 {
        &self.diepint5
    }
    ///0x9b0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    #[inline(always)]
    pub const fn dieptsiz5(&self) -> &DIEPTSIZ5 {
        &self.dieptsiz5
    }
    ///0x9b8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    #[inline(always)]
    pub const fn dtxfsts5(&self) -> &DTXFSTS5 {
        &self.dtxfsts5
    }
    ///0xb00 - This section describes the DOEPCTL0 register.
    #[inline(always)]
    pub const fn doepctl0(&self) -> &DOEPCTL0 {
        &self.doepctl0
    }
    ///0xb08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn doepint0(&self) -> &DOEPINT0 {
        &self.doepint0
    }
    ///0xb10 - The application must modify this register before enabling endpoint 0.
    #[inline(always)]
    pub const fn doeptsiz0(&self) -> &DOEPTSIZ0 {
        &self.doeptsiz0
    }
    ///0xb20 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    #[inline(always)]
    pub const fn doepctl1(&self) -> &DOEPCTL1 {
        &self.doepctl1
    }
    ///0xb28 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn doepint1(&self) -> &DOEPINT1 {
        &self.doepint1
    }
    ///0xb30 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    #[inline(always)]
    pub const fn doeptsiz1(&self) -> &DOEPTSIZ1 {
        &self.doeptsiz1
    }
    ///0xb40 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    #[inline(always)]
    pub const fn doepctl2(&self) -> &DOEPCTL2 {
        &self.doepctl2
    }
    ///0xb48 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn doepint2(&self) -> &DOEPINT2 {
        &self.doepint2
    }
    ///0xb50 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    #[inline(always)]
    pub const fn doeptsiz2(&self) -> &DOEPTSIZ2 {
        &self.doeptsiz2
    }
    ///0xb60 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    #[inline(always)]
    pub const fn doepctl3(&self) -> &DOEPCTL3 {
        &self.doepctl3
    }
    ///0xb68 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn doepint3(&self) -> &DOEPINT3 {
        &self.doepint3
    }
    ///0xb70 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    #[inline(always)]
    pub const fn doeptsiz3(&self) -> &DOEPTSIZ3 {
        &self.doeptsiz3
    }
    ///0xb80 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    #[inline(always)]
    pub const fn doepctl4(&self) -> &DOEPCTL4 {
        &self.doepctl4
    }
    ///0xb88 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn doepint4(&self) -> &DOEPINT4 {
        &self.doepint4
    }
    ///0xb90 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    #[inline(always)]
    pub const fn doeptsiz4(&self) -> &DOEPTSIZ4 {
        &self.doeptsiz4
    }
    ///0xba0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    #[inline(always)]
    pub const fn doepctl5(&self) -> &DOEPCTL5 {
        &self.doepctl5
    }
    ///0xba8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
    #[inline(always)]
    pub const fn doepint5(&self) -> &DOEPINT5 {
        &self.doepint5
    }
    ///0xbb0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    #[inline(always)]
    pub const fn doeptsiz5(&self) -> &DOEPTSIZ5 {
        &self.doeptsiz5
    }
    ///0xe00 - This register is available in host and device modes.
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
}
/**GOTGCTL (rw) register accessor: The GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.

You can [`read`](crate::Reg::read) this register and get [`gotgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GOTGCTL)

For information about available fields see [`mod@gotgctl`] module*/
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTLrs>;
///The GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.
pub mod gotgctl;
/**GOTGINT (rw) register accessor: The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.

You can [`read`](crate::Reg::read) this register and get [`gotgint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GOTGINT)

For information about available fields see [`mod@gotgint`] module*/
pub type GOTGINT = crate::Reg<gotgint::GOTGINTrs>;
///The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.
pub mod gotgint;
/**GAHBCFG (rw) register accessor: This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.

You can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GAHBCFG)

For information about available fields see [`mod@gahbcfg`] module*/
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFGrs>;
///This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.
pub mod gahbcfg;
/**GUSBCFG (rw) register accessor: This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.

You can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GUSBCFG)

For information about available fields see [`mod@gusbcfg`] module*/
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFGrs>;
///This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.
pub mod gusbcfg;
/**GRSTCTL (rw) register accessor: The application uses this register to reset various hardware features inside the core.

You can [`read`](crate::Reg::read) this register and get [`grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GRSTCTL)

For information about available fields see [`mod@grstctl`] module*/
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTLrs>;
///The application uses this register to reset various hardware features inside the core.
pub mod grstctl;
/**GINTSTS (rw) register accessor: This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.

You can [`read`](crate::Reg::read) this register and get [`gintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GINTSTS)

For information about available fields see [`mod@gintsts`] module*/
pub type GINTSTS = crate::Reg<gintsts::GINTSTSrs>;
///This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.
pub mod gintsts;
/**GINTMSK (rw) register accessor: This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (GINTSTS) register bit corresponding to that interrupt is still set.

You can [`read`](crate::Reg::read) this register and get [`gintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GINTMSK)

For information about available fields see [`mod@gintmsk`] module*/
pub type GINTMSK = crate::Reg<gintmsk::GINTMSKrs>;
///This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (GINTSTS) register bit corresponding to that interrupt is still set.
pub mod gintmsk;
/**GRXSTSR_DEVICE (r) register accessor: This description is for register GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000.

You can [`read`](crate::Reg::read) this register and get [`grxstsr_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GRXSTSR_DEVICE)

For information about available fields see [`mod@grxstsr_device`] module*/
pub type GRXSTSR_DEVICE = crate::Reg<grxstsr_device::GRXSTSR_DEVICErs>;
///This description is for register GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000.
pub mod grxstsr_device;
/**GRXSTSR_HOST (r) register accessor: This description is for register GRXSTSR in Host mode

You can [`read`](crate::Reg::read) this register and get [`grxstsr_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GRXSTSR_HOST)

For information about available fields see [`mod@grxstsr_host`] module*/
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOSTrs>;
///This description is for register GRXSTSR in Host mode
pub mod grxstsr_host;
/**GRXSTSP_DEVICE (r) register accessor: This description is for register GRXSTSP in Device mode. Similarly to GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in GINTSTS) is asserted.

You can [`read`](crate::Reg::read) this register and get [`grxstsp_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GRXSTSP_DEVICE)

For information about available fields see [`mod@grxstsp_device`] module*/
pub type GRXSTSP_DEVICE = crate::Reg<grxstsp_device::GRXSTSP_DEVICErs>;
///This description is for register GRXSTSP in Device mode. Similarly to GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in GINTSTS) is asserted.
pub mod grxstsp_device;
/**GRXSTSP_HOST (r) register accessor: This description is for register GRXSTSP in HOST mode

You can [`read`](crate::Reg::read) this register and get [`grxstsp_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GRXSTSP_HOST)

For information about available fields see [`mod@grxstsp_host`] module*/
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOSTrs>;
///This description is for register GRXSTSP in HOST mode
pub mod grxstsp_host;
/**GRXFSIZ (rw) register accessor: The application can program the RAM size that must be allocated to the Rx FIFO.

You can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GRXFSIZ)

For information about available fields see [`mod@grxfsiz`] module*/
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZrs>;
///The application can program the RAM size that must be allocated to the Rx FIFO.
pub mod grxfsiz;
/**HNPTXFSIZ (rw) register accessor: Host mode

You can [`read`](crate::Reg::read) this register and get [`hnptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hnptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HNPTXFSIZ)

For information about available fields see [`mod@hnptxfsiz`] module*/
pub type HNPTXFSIZ = crate::Reg<hnptxfsiz::HNPTXFSIZrs>;
///Host mode
pub mod hnptxfsiz;
/**HNPTXSTS (r) register accessor: In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.

You can [`read`](crate::Reg::read) this register and get [`hnptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HNPTXSTS)

For information about available fields see [`mod@hnptxsts`] module*/
pub type HNPTXSTS = crate::Reg<hnptxsts::HNPTXSTSrs>;
///In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.
pub mod hnptxsts;
/**GCCFG (rw) register accessor: OTG general core configuration register

You can [`read`](crate::Reg::read) this register and get [`gccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GCCFG)

For information about available fields see [`mod@gccfg`] module*/
pub type GCCFG = crate::Reg<gccfg::GCCFGrs>;
///OTG general core configuration register
pub mod gccfg;
/**CID (rw) register accessor: This is a register containing the Product ID as reset value.

You can [`read`](crate::Reg::read) this register and get [`cid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:CID)

For information about available fields see [`mod@cid`] module*/
pub type CID = crate::Reg<cid::CIDrs>;
///This is a register containing the Product ID as reset value.
pub mod cid;
/**GLPMCFG (rw) register accessor: OTG core LPM configuration register

You can [`read`](crate::Reg::read) this register and get [`glpmcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glpmcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:GLPMCFG)

For information about available fields see [`mod@glpmcfg`] module*/
pub type GLPMCFG = crate::Reg<glpmcfg::GLPMCFGrs>;
///OTG core LPM configuration register
pub mod glpmcfg;
/**HPTXFSIZ (rw) register accessor: OTG host periodic transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HPTXFSIZ)

For information about available fields see [`mod@hptxfsiz`] module*/
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZrs>;
///OTG host periodic transmit FIFO size register
pub mod hptxfsiz;
/**DIEPTXF1 (rw) register accessor: OTG device IN endpoint transmit FIFO 1 size register

You can [`read`](crate::Reg::read) this register and get [`dieptxf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPTXF1)

For information about available fields see [`mod@dieptxf1`] module*/
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1rs>;
///OTG device IN endpoint transmit FIFO 1 size register
pub mod dieptxf1;
/**DIEPTXF2 (rw) register accessor: OTG device IN endpoint transmit FIFO 2 size register

You can [`read`](crate::Reg::read) this register and get [`dieptxf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPTXF2)

For information about available fields see [`mod@dieptxf2`] module*/
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2rs>;
///OTG device IN endpoint transmit FIFO 2 size register
pub mod dieptxf2;
/**DIEPTXF3 (rw) register accessor: OTG device IN endpoint transmit FIFO 3 size register

You can [`read`](crate::Reg::read) this register and get [`dieptxf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPTXF3)

For information about available fields see [`mod@dieptxf3`] module*/
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3rs>;
///OTG device IN endpoint transmit FIFO 3 size register
pub mod dieptxf3;
/**DIEPTXF4 (rw) register accessor: OTG device IN endpoint transmit FIFO 4 size register

You can [`read`](crate::Reg::read) this register and get [`dieptxf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPTXF4)

For information about available fields see [`mod@dieptxf4`] module*/
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4rs>;
///OTG device IN endpoint transmit FIFO 4 size register
pub mod dieptxf4;
/**DIEPTXF5 (rw) register accessor: OTG device IN endpoint transmit FIFO 5 size register

You can [`read`](crate::Reg::read) this register and get [`dieptxf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPTXF5)

For information about available fields see [`mod@dieptxf5`] module*/
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5rs>;
///OTG device IN endpoint transmit FIFO 5 size register
pub mod dieptxf5;
/**HCFG (rw) register accessor: This register configures the core after power-on. Do not make changes to this register after initializing the host.

You can [`read`](crate::Reg::read) this register and get [`hcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCFG)

For information about available fields see [`mod@hcfg`] module*/
pub type HCFG = crate::Reg<hcfg::HCFGrs>;
///This register configures the core after power-on. Do not make changes to this register after initializing the host.
pub mod hcfg;
/**HFIR (rw) register accessor: This register stores the frame interval information for the current speed to which the OTG controller has enumerated.

You can [`read`](crate::Reg::read) this register and get [`hfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HFIR)

For information about available fields see [`mod@hfir`] module*/
pub type HFIR = crate::Reg<hfir::HFIRrs>;
///This register stores the frame interval information for the current speed to which the OTG controller has enumerated.
pub mod hfir;
/**HFNUM (r) register accessor: This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.

You can [`read`](crate::Reg::read) this register and get [`hfnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HFNUM)

For information about available fields see [`mod@hfnum`] module*/
pub type HFNUM = crate::Reg<hfnum::HFNUMrs>;
///This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.
pub mod hfnum;
/**HPTXSTS (r) register accessor: This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.

You can [`read`](crate::Reg::read) this register and get [`hptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HPTXSTS)

For information about available fields see [`mod@hptxsts`] module*/
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTSrs>;
///This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.
pub mod hptxsts;
/**HAINT (r) register accessor: When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.

You can [`read`](crate::Reg::read) this register and get [`haint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HAINT)

For information about available fields see [`mod@haint`] module*/
pub type HAINT = crate::Reg<haint::HAINTrs>;
///When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.
pub mod haint;
/**HAINTMSK (rw) register accessor: The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.

You can [`read`](crate::Reg::read) this register and get [`haintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HAINTMSK)

For information about available fields see [`mod@haintmsk`] module*/
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSKrs>;
///The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.
pub mod haintmsk;
/**HPRT (rw) register accessor: This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.

You can [`read`](crate::Reg::read) this register and get [`hprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HPRT)

For information about available fields see [`mod@hprt`] module*/
pub type HPRT = crate::Reg<hprt::HPRTrs>;
///This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.
pub mod hprt;
/**HCCHAR0 (rw) register accessor: OTG host channel 0 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR0)

For information about available fields see [`mod@hcchar0`] module*/
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0rs>;
///OTG host channel 0 characteristics register
pub mod hcchar0;
/**HCINT0 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT0)

For information about available fields see [`mod@hcint0`] module*/
pub type HCINT0 = crate::Reg<hcint0::HCINT0rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint0;
/**HCINTMSK0 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK0)

For information about available fields see [`mod@hcintmsk0`] module*/
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk0;
/**HCTSIZ0 (rw) register accessor: OTG host channel 0 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ0)

For information about available fields see [`mod@hctsiz0`] module*/
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0rs>;
///OTG host channel 0 transfer size register
pub mod hctsiz0;
/**HCCHAR1 (rw) register accessor: OTG host channel 1 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR1)

For information about available fields see [`mod@hcchar1`] module*/
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1rs>;
///OTG host channel 1 characteristics register
pub mod hcchar1;
/**HCINT1 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT1)

For information about available fields see [`mod@hcint1`] module*/
pub type HCINT1 = crate::Reg<hcint1::HCINT1rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint1;
/**HCINTMSK1 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK1)

For information about available fields see [`mod@hcintmsk1`] module*/
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk1;
/**HCTSIZ1 (rw) register accessor: OTG host channel 1 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ1)

For information about available fields see [`mod@hctsiz1`] module*/
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1rs>;
///OTG host channel 1 transfer size register
pub mod hctsiz1;
/**HCCHAR2 (rw) register accessor: OTG host channel 2 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR2)

For information about available fields see [`mod@hcchar2`] module*/
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2rs>;
///OTG host channel 2 characteristics register
pub mod hcchar2;
/**HCINT2 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT2)

For information about available fields see [`mod@hcint2`] module*/
pub type HCINT2 = crate::Reg<hcint2::HCINT2rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint2;
/**HCINTMSK2 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK2)

For information about available fields see [`mod@hcintmsk2`] module*/
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk2;
/**HCTSIZ2 (rw) register accessor: OTG host channel 2 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ2)

For information about available fields see [`mod@hctsiz2`] module*/
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2rs>;
///OTG host channel 2 transfer size register
pub mod hctsiz2;
/**HCCHAR3 (rw) register accessor: OTG host channel 3 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR3)

For information about available fields see [`mod@hcchar3`] module*/
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3rs>;
///OTG host channel 3 characteristics register
pub mod hcchar3;
/**HCINT3 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT3)

For information about available fields see [`mod@hcint3`] module*/
pub type HCINT3 = crate::Reg<hcint3::HCINT3rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint3;
/**HCINTMSK3 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK3)

For information about available fields see [`mod@hcintmsk3`] module*/
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk3;
/**HCTSIZ3 (rw) register accessor: OTG host channel 3 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ3)

For information about available fields see [`mod@hctsiz3`] module*/
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3rs>;
///OTG host channel 3 transfer size register
pub mod hctsiz3;
/**HCCHAR4 (rw) register accessor: OTG host channel 4 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR4)

For information about available fields see [`mod@hcchar4`] module*/
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4rs>;
///OTG host channel 4 characteristics register
pub mod hcchar4;
/**HCINT4 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT4)

For information about available fields see [`mod@hcint4`] module*/
pub type HCINT4 = crate::Reg<hcint4::HCINT4rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint4;
/**HCINTMSK4 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK4)

For information about available fields see [`mod@hcintmsk4`] module*/
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk4;
/**HCTSIZ4 (rw) register accessor: OTG host channel 4 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ4)

For information about available fields see [`mod@hctsiz4`] module*/
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4rs>;
///OTG host channel 4 transfer size register
pub mod hctsiz4;
/**HCCHAR5 (rw) register accessor: OTG host channel 5 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR5)

For information about available fields see [`mod@hcchar5`] module*/
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5rs>;
///OTG host channel 5 characteristics register
pub mod hcchar5;
/**HCINT5 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT5)

For information about available fields see [`mod@hcint5`] module*/
pub type HCINT5 = crate::Reg<hcint5::HCINT5rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint5;
/**HCINTMSK5 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK5)

For information about available fields see [`mod@hcintmsk5`] module*/
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk5;
/**HCTSIZ5 (rw) register accessor: OTG host channel 5 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ5)

For information about available fields see [`mod@hctsiz5`] module*/
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5rs>;
///OTG host channel 5 transfer size register
pub mod hctsiz5;
/**HCCHAR6 (rw) register accessor: OTG host channel 6 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR6)

For information about available fields see [`mod@hcchar6`] module*/
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6rs>;
///OTG host channel 6 characteristics register
pub mod hcchar6;
/**HCINT6 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT6)

For information about available fields see [`mod@hcint6`] module*/
pub type HCINT6 = crate::Reg<hcint6::HCINT6rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint6;
/**HCINTMSK6 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK6)

For information about available fields see [`mod@hcintmsk6`] module*/
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk6;
/**HCTSIZ6 (rw) register accessor: OTG host channel 6 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ6)

For information about available fields see [`mod@hctsiz6`] module*/
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6rs>;
///OTG host channel 6 transfer size register
pub mod hctsiz6;
/**HCCHAR7 (rw) register accessor: OTG host channel 7 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR7)

For information about available fields see [`mod@hcchar7`] module*/
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7rs>;
///OTG host channel 7 characteristics register
pub mod hcchar7;
/**HCINT7 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT7)

For information about available fields see [`mod@hcint7`] module*/
pub type HCINT7 = crate::Reg<hcint7::HCINT7rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint7;
/**HCINTMSK7 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK7)

For information about available fields see [`mod@hcintmsk7`] module*/
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk7;
/**HCTSIZ7 (rw) register accessor: OTG host channel 7 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ7)

For information about available fields see [`mod@hctsiz7`] module*/
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7rs>;
///OTG host channel 7 transfer size register
pub mod hctsiz7;
/**HCCHAR8 (rw) register accessor: OTG host channel 8 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR8)

For information about available fields see [`mod@hcchar8`] module*/
pub type HCCHAR8 = crate::Reg<hcchar8::HCCHAR8rs>;
///OTG host channel 8 characteristics register
pub mod hcchar8;
/**HCINT8 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT8)

For information about available fields see [`mod@hcint8`] module*/
pub type HCINT8 = crate::Reg<hcint8::HCINT8rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint8;
/**HCINTMSK8 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK8)

For information about available fields see [`mod@hcintmsk8`] module*/
pub type HCINTMSK8 = crate::Reg<hcintmsk8::HCINTMSK8rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk8;
/**HCTSIZ8 (rw) register accessor: OTG host channel 8 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ8)

For information about available fields see [`mod@hctsiz8`] module*/
pub type HCTSIZ8 = crate::Reg<hctsiz8::HCTSIZ8rs>;
///OTG host channel 8 transfer size register
pub mod hctsiz8;
/**HCCHAR9 (rw) register accessor: OTG host channel 9 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR9)

For information about available fields see [`mod@hcchar9`] module*/
pub type HCCHAR9 = crate::Reg<hcchar9::HCCHAR9rs>;
///OTG host channel 9 characteristics register
pub mod hcchar9;
/**HCINT9 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT9)

For information about available fields see [`mod@hcint9`] module*/
pub type HCINT9 = crate::Reg<hcint9::HCINT9rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint9;
/**HCINTMSK9 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK9)

For information about available fields see [`mod@hcintmsk9`] module*/
pub type HCINTMSK9 = crate::Reg<hcintmsk9::HCINTMSK9rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk9;
/**HCTSIZ9 (rw) register accessor: OTG host channel 9 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ9)

For information about available fields see [`mod@hctsiz9`] module*/
pub type HCTSIZ9 = crate::Reg<hctsiz9::HCTSIZ9rs>;
///OTG host channel 9 transfer size register
pub mod hctsiz9;
/**HCCHAR10 (rw) register accessor: OTG host channel 10 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR10)

For information about available fields see [`mod@hcchar10`] module*/
pub type HCCHAR10 = crate::Reg<hcchar10::HCCHAR10rs>;
///OTG host channel 10 characteristics register
pub mod hcchar10;
/**HCINT10 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT10)

For information about available fields see [`mod@hcint10`] module*/
pub type HCINT10 = crate::Reg<hcint10::HCINT10rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint10;
/**HCINTMSK10 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK10)

For information about available fields see [`mod@hcintmsk10`] module*/
pub type HCINTMSK10 = crate::Reg<hcintmsk10::HCINTMSK10rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk10;
/**HCTSIZ10 (rw) register accessor: OTG host channel 10 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ10)

For information about available fields see [`mod@hctsiz10`] module*/
pub type HCTSIZ10 = crate::Reg<hctsiz10::HCTSIZ10rs>;
///OTG host channel 10 transfer size register
pub mod hctsiz10;
/**HCCHAR11 (rw) register accessor: OTG host channel 11 characteristics register

You can [`read`](crate::Reg::read) this register and get [`hcchar11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCCHAR11)

For information about available fields see [`mod@hcchar11`] module*/
pub type HCCHAR11 = crate::Reg<hcchar11::HCCHAR11rs>;
///OTG host channel 11 characteristics register
pub mod hcchar11;
/**HCINT11 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`hcint11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINT11)

For information about available fields see [`mod@hcint11`] module*/
pub type HCINT11 = crate::Reg<hcint11::HCINT11rs>;
///This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.
pub mod hcint11;
/**HCINTMSK11 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.

You can [`read`](crate::Reg::read) this register and get [`hcintmsk11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCINTMSK11)

For information about available fields see [`mod@hcintmsk11`] module*/
pub type HCINTMSK11 = crate::Reg<hcintmsk11::HCINTMSK11rs>;
///This register reflects the mask for each channel status described in the previous section.
pub mod hcintmsk11;
/**HCTSIZ11 (rw) register accessor: OTG host channel 11 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:HCTSIZ11)

For information about available fields see [`mod@hctsiz11`] module*/
pub type HCTSIZ11 = crate::Reg<hctsiz11::HCTSIZ11rs>;
///OTG host channel 11 transfer size register
pub mod hctsiz11;
/**DCFG (rw) register accessor: This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.

You can [`read`](crate::Reg::read) this register and get [`dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DCFG)

For information about available fields see [`mod@dcfg`] module*/
pub type DCFG = crate::Reg<dcfg::DCFGrs>;
///This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.
pub mod dcfg;
/**DCTL (rw) register accessor: OTG device control register

You can [`read`](crate::Reg::read) this register and get [`dctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DCTL)

For information about available fields see [`mod@dctl`] module*/
pub type DCTL = crate::Reg<dctl::DCTLrs>;
///OTG device control register
pub mod dctl;
/**DSTS (r) register accessor: This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (DAINT) register.

You can [`read`](crate::Reg::read) this register and get [`dsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DSTS)

For information about available fields see [`mod@dsts`] module*/
pub type DSTS = crate::Reg<dsts::DSTSrs>;
///This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (DAINT) register.
pub mod dsts;
/**DIEPMSK (rw) register accessor: This register works with each of the DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.

You can [`read`](crate::Reg::read) this register and get [`diepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPMSK)

For information about available fields see [`mod@diepmsk`] module*/
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSKrs>;
///This register works with each of the DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.
pub mod diepmsk;
/**DOEPMSK (rw) register accessor: This register works with each of the DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.

You can [`read`](crate::Reg::read) this register and get [`doepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPMSK)

For information about available fields see [`mod@doepmsk`] module*/
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSKrs>;
///This register works with each of the DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.
pub mod doepmsk;
/**DAINT (r) register accessor: When a significant event occurs on an endpoint, a DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the GINTSTS register (OEPINT or IEPINT in GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (DIEPINTx/DOEPINTx).

You can [`read`](crate::Reg::read) this register and get [`daint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DAINT)

For information about available fields see [`mod@daint`] module*/
pub type DAINT = crate::Reg<daint::DAINTrs>;
///When a significant event occurs on an endpoint, a DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the GINTSTS register (OEPINT or IEPINT in GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (DIEPINTx/DOEPINTx).
pub mod daint;
/**DAINTMSK (rw) register accessor: The DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the DAINT register bit corresponding to that interrupt is still set.

You can [`read`](crate::Reg::read) this register and get [`daintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DAINTMSK)

For information about available fields see [`mod@daintmsk`] module*/
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSKrs>;
///The DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the DAINT register bit corresponding to that interrupt is still set.
pub mod daintmsk;
/**DVBUSDIS (rw) register accessor: This register specifies the VBUS discharge time after VBUS pulsing during SRP.

You can [`read`](crate::Reg::read) this register and get [`dvbusdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DVBUSDIS)

For information about available fields see [`mod@dvbusdis`] module*/
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDISrs>;
///This register specifies the VBUS discharge time after VBUS pulsing during SRP.
pub mod dvbusdis;
/**DVBUSPULSE (rw) register accessor: This register specifies the VBUS pulsing time during SRP.

You can [`read`](crate::Reg::read) this register and get [`dvbuspulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DVBUSPULSE)

For information about available fields see [`mod@dvbuspulse`] module*/
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSErs>;
///This register specifies the VBUS pulsing time during SRP.
pub mod dvbuspulse;
/**DIEPEMPMSK (rw) register accessor: This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_DIEPINTx).

You can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPEMPMSK)

For information about available fields see [`mod@diepempmsk`] module*/
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSKrs>;
///This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_DIEPINTx).
pub mod diepempmsk;
/**DIEPCTL0 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`diepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPCTL0)

For information about available fields see [`mod@diepctl0`] module*/
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0rs>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl0;
/**DIEPINT0 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`diepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPINT0)

For information about available fields see [`mod@diepint0`] module*/
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint0;
/**DIEPTSIZ0 (rw) register accessor: The application must modify this register before enabling endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`dieptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPTSIZ0)

For information about available fields see [`mod@dieptsiz0`] module*/
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0rs>;
///The application must modify this register before enabling endpoint 0.
pub mod dieptsiz0;
/**DTXFSTS0 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.

You can [`read`](crate::Reg::read) this register and get [`dtxfsts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DTXFSTS0)

For information about available fields see [`mod@dtxfsts0`] module*/
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0rs>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts0;
/**DIEPCTL1 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`diepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPCTL1)

For information about available fields see [`mod@diepctl1`] module*/
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1rs>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl1;
/**DIEPINT1 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`diepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPINT1)

For information about available fields see [`mod@diepint1`] module*/
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint1;
/**DIEPTSIZ1 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`dieptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPTSIZ1)

For information about available fields see [`mod@dieptsiz1`] module*/
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1rs>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz1;
/**DTXFSTS1 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.

You can [`read`](crate::Reg::read) this register and get [`dtxfsts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DTXFSTS1)

For information about available fields see [`mod@dtxfsts1`] module*/
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1rs>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts1;
/**DIEPCTL2 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`diepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPCTL2)

For information about available fields see [`mod@diepctl2`] module*/
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2rs>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl2;
/**DIEPINT2 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`diepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPINT2)

For information about available fields see [`mod@diepint2`] module*/
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint2;
/**DIEPTSIZ2 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`dieptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPTSIZ2)

For information about available fields see [`mod@dieptsiz2`] module*/
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2rs>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz2;
/**DTXFSTS2 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.

You can [`read`](crate::Reg::read) this register and get [`dtxfsts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DTXFSTS2)

For information about available fields see [`mod@dtxfsts2`] module*/
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2rs>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts2;
/**DIEPCTL3 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`diepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPCTL3)

For information about available fields see [`mod@diepctl3`] module*/
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3rs>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl3;
/**DIEPINT3 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`diepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPINT3)

For information about available fields see [`mod@diepint3`] module*/
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint3;
/**DIEPTSIZ3 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`dieptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPTSIZ3)

For information about available fields see [`mod@dieptsiz3`] module*/
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3rs>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz3;
/**DTXFSTS3 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.

You can [`read`](crate::Reg::read) this register and get [`dtxfsts3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DTXFSTS3)

For information about available fields see [`mod@dtxfsts3`] module*/
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3rs>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts3;
/**DIEPCTL4 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`diepctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPCTL4)

For information about available fields see [`mod@diepctl4`] module*/
pub type DIEPCTL4 = crate::Reg<diepctl4::DIEPCTL4rs>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl4;
/**DIEPINT4 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`diepint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPINT4)

For information about available fields see [`mod@diepint4`] module*/
pub type DIEPINT4 = crate::Reg<diepint4::DIEPINT4rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint4;
/**DIEPTSIZ4 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`dieptsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPTSIZ4)

For information about available fields see [`mod@dieptsiz4`] module*/
pub type DIEPTSIZ4 = crate::Reg<dieptsiz4::DIEPTSIZ4rs>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz4;
/**DTXFSTS4 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.

You can [`read`](crate::Reg::read) this register and get [`dtxfsts4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DTXFSTS4)

For information about available fields see [`mod@dtxfsts4`] module*/
pub type DTXFSTS4 = crate::Reg<dtxfsts4::DTXFSTS4rs>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts4;
/**DIEPCTL5 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`diepctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPCTL5)

For information about available fields see [`mod@diepctl5`] module*/
pub type DIEPCTL5 = crate::Reg<diepctl5::DIEPCTL5rs>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod diepctl5;
/**DIEPINT5 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`diepint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPINT5)

For information about available fields see [`mod@diepint5`] module*/
pub type DIEPINT5 = crate::Reg<diepint5::DIEPINT5rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod diepint5;
/**DIEPTSIZ5 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`dieptsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DIEPTSIZ5)

For information about available fields see [`mod@dieptsiz5`] module*/
pub type DIEPTSIZ5 = crate::Reg<dieptsiz5::DIEPTSIZ5rs>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod dieptsiz5;
/**DTXFSTS5 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.

You can [`read`](crate::Reg::read) this register and get [`dtxfsts5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DTXFSTS5)

For information about available fields see [`mod@dtxfsts5`] module*/
pub type DTXFSTS5 = crate::Reg<dtxfsts5::DTXFSTS5rs>;
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod dtxfsts5;
/**DOEPCTL0 (rw) register accessor: This section describes the DOEPCTL0 register.

You can [`read`](crate::Reg::read) this register and get [`doepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPCTL0)

For information about available fields see [`mod@doepctl0`] module*/
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0rs>;
///This section describes the DOEPCTL0 register.
pub mod doepctl0;
/**DOEPINT0 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`doepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPINT0)

For information about available fields see [`mod@doepint0`] module*/
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint0;
/**DOEPTSIZ0 (rw) register accessor: The application must modify this register before enabling endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`doeptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPTSIZ0)

For information about available fields see [`mod@doeptsiz0`] module*/
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0rs>;
///The application must modify this register before enabling endpoint 0.
pub mod doeptsiz0;
/**DOEPCTL1 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`doepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPCTL1)

For information about available fields see [`mod@doepctl1`] module*/
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1rs>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl1;
/**DOEPINT1 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`doepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPINT1)

For information about available fields see [`mod@doepint1`] module*/
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint1;
/**DOEPTSIZ1 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`doeptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPTSIZ1)

For information about available fields see [`mod@doeptsiz1`] module*/
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1rs>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz1;
/**DOEPCTL2 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`doepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPCTL2)

For information about available fields see [`mod@doepctl2`] module*/
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2rs>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl2;
/**DOEPINT2 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`doepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPINT2)

For information about available fields see [`mod@doepint2`] module*/
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint2;
/**DOEPTSIZ2 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`doeptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPTSIZ2)

For information about available fields see [`mod@doeptsiz2`] module*/
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2rs>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz2;
/**DOEPCTL3 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`doepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPCTL3)

For information about available fields see [`mod@doepctl3`] module*/
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3rs>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl3;
/**DOEPINT3 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`doepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPINT3)

For information about available fields see [`mod@doepint3`] module*/
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint3;
/**DOEPTSIZ3 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`doeptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPTSIZ3)

For information about available fields see [`mod@doeptsiz3`] module*/
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3rs>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz3;
/**DOEPCTL4 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`doepctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPCTL4)

For information about available fields see [`mod@doepctl4`] module*/
pub type DOEPCTL4 = crate::Reg<doepctl4::DOEPCTL4rs>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl4;
/**DOEPINT4 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`doepint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPINT4)

For information about available fields see [`mod@doepint4`] module*/
pub type DOEPINT4 = crate::Reg<doepint4::DOEPINT4rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint4;
/**DOEPTSIZ4 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`doeptsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPTSIZ4)

For information about available fields see [`mod@doeptsiz4`] module*/
pub type DOEPTSIZ4 = crate::Reg<doeptsiz4::DOEPTSIZ4rs>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz4;
/**DOEPCTL5 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.

You can [`read`](crate::Reg::read) this register and get [`doepctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPCTL5)

For information about available fields see [`mod@doepctl5`] module*/
pub type DOEPCTL5 = crate::Reg<doepctl5::DOEPCTL5rs>;
///The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod doepctl5;
/**DOEPINT5 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.

You can [`read`](crate::Reg::read) this register and get [`doepint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPINT5)

For information about available fields see [`mod@doepint5`] module*/
pub type DOEPINT5 = crate::Reg<doepint5::DOEPINT5rs>;
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
pub mod doepint5;
/**DOEPTSIZ5 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.

You can [`read`](crate::Reg::read) this register and get [`doeptsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:DOEPTSIZ5)

For information about available fields see [`mod@doeptsiz5`] module*/
pub type DOEPTSIZ5 = crate::Reg<doeptsiz5::DOEPTSIZ5rs>;
///The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod doeptsiz5;
/**PCGCCTL (rw) register accessor: This register is available in host and device modes.

You can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#OTG_FS:PCGCCTL)

For information about available fields see [`mod@pcgcctl`] module*/
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTLrs>;
///This register is available in host and device modes.
pub mod pcgcctl;
