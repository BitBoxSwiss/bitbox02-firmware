use core::marker::PhantomData;

/// Host to Card commands
pub struct Cmd<R: Resp> {
    pub cmd: u8,
    pub arg: u32,
    resp: PhantomData<R>,
}

impl<R: Resp> Cmd<R> {
    pub fn response_len(&self) -> ResponseLen {
        R::LENGTH
    }
}

/// Marker for commands that don't have any response
pub struct Rz;
/// R1: Normal response
pub struct R1;
/// R2: CID and CSD register
pub struct R2;
/// R3: OCR register
pub struct R3;

pub trait Resp {
    const LENGTH: ResponseLen = ResponseLen::R48;
}

impl Resp for Rz {
    const LENGTH: ResponseLen = ResponseLen::Zero;
}

impl Resp for R2 {
    const LENGTH: ResponseLen = ResponseLen::R136;
}

impl Resp for R1 {}
impl Resp for R3 {}

/// Command Response type
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ResponseLen {
    /// No response expected
    Zero,
    /// Short (48 bit) response
    R48,
    /// Long (136 bit) response
    R136,
}

pub fn cmd<R: Resp>(cmd: u8, arg: u32) -> Cmd<R> {
    Cmd {
        cmd,
        arg,
        resp: PhantomData,
    }
}

/// CMD0: Put card in idle mode
pub fn idle() -> Cmd<Rz> {
    cmd(0, 0)
}

/// CMD2: Ask any card to send their CID
pub fn all_send_cid() -> Cmd<R2> {
    cmd(2, 0)
}

/// CMD7: Select or deselect card
pub fn select_card(rca: u16) -> Cmd<R1> {
    cmd(7, u32::from(rca) << 16)
}

/// CMD9: Send CSD
pub fn send_csd(rca: u16) -> Cmd<R2> {
    cmd(9, u32::from(rca) << 16)
}

/// CMD10: Send CID
pub fn send_cid(rca: u16) -> Cmd<R2> {
    cmd(10, u32::from(rca) << 16)
}

/// CMD12: Stop transmission
pub fn stop_transmission() -> Cmd<R1> {
    cmd(12, 0)
}

/// CMD13: Ask card to send status or task status
pub fn card_status(rca: u16, task_status: bool) -> Cmd<R1> {
    let arg = u32::from(rca) << 16 | u32::from(task_status) << 15;
    cmd(13, arg)
}

/// CMD15: Sends card to inactive state
pub fn go_inactive_state(rca: u16) -> Cmd<Rz> {
    cmd(15, u32::from(rca) << 16)
}

/// CMD16: Set block len
pub fn set_block_length(blocklen: u32) -> Cmd<R1> {
    cmd(16, blocklen)
}

/// CMD17: Read a single block from the card
pub fn read_single_block(addr: u32) -> Cmd<R1> {
    cmd(17, addr)
}

/// CMD18: Read multiple block from the card
pub fn read_multiple_blocks(addr: u32) -> Cmd<R1> {
    cmd(18, addr)
}

/// CMD24: Write block
pub fn write_single_block(addr: u32) -> Cmd<R1> {
    cmd(24, addr)
}

/// CMD25: Write multiple blocks
pub fn write_multiple_blocks(addr: u32) -> Cmd<R1> {
    cmd(25, addr)
}

/// CMD27: Program CSD
pub fn program_csd() -> Cmd<R1> {
    cmd(27, 0)
}

/// CMD38: Erase all previously selected write blocks
pub fn erase() -> Cmd<R1> {
    cmd(38, 0)
}

/// CMD55: App Command. Indicates that next command will be a app command
pub fn app_cmd(rca: u16) -> Cmd<R1> {
    cmd(55, u32::from(rca) << 16)
}
