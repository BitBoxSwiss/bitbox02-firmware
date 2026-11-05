//! Management of external NAND Flash through the STM32 FMC peripheral
//!
//! Commands and parameters are referenced to the Open NAND Flash Interface
//! (ONFI) Specification Revision 5.1 3 May 2022
//!
//! Addressing supports up to 64Gb / 4GByte (8-bit data) or 128Gb / 8Gbyte (16-bit data).

use core::convert::TryInto;
use core::sync::atomic::{fence, Ordering};
use core::{fmt, ptr, str};

/// NAND Commands defined in ONFI Specification 5.1
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(unused)]
enum Command {
    /// 0xFF Reset: ONFI Section 5.3
    Reset = 0xFF,
    /// 0x90 Read ID: ONFI Section 5.6
    ReadID = 0x90,
    /// 0xEC Read Parameter Page: ONFI Section 5.7
    ReadParameterPage = 0xEC,
    /// 0xED Read Unique ID: ONFI Section 5.8
    ReadUniqueID = 0xED,
    /// Block Erase: ONFI Section 5.9
    BlockErase = 0x60,
    /// 0x70 Read Status: ONFI Section 5.10
    ReadStatus = 0x70,
}

/// Status returned from 0x70 Read Status: ONFI Section 5.10
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    /// Status Register indicated Pass
    Success(u8),
    /// Status Register indicates Fail
    Fail(u8),
}
impl Status {
    fn from_register(reg: u8) -> Self {
        match reg & 1 {
            1 => Self::Fail(reg),
            _ => Self::Success(reg),
        }
    }
}

/// Identifier returned from 0x90 Read ID: ONFI Section 5.6
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ID {
    manufacturer_jedec: u8,
    device_jedec: u8,
    internal_chip_count: usize,
    page_size: usize,
}

/// Parameter Page returned from 0xEC Read Parameter Page: ONFI Section 5.7
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, PartialEq)]
pub struct ParameterPage {
    signature: [u8; 4],
    onfi_revision: u16,
    manufacturer: [u8; 12],
    model: [u8; 20],
    date_code: u16,
    data_bytes_per_page: u32,
    spare_bytes_per_page: u16,
    pages_per_block: u32,
    blocks_per_lun: u32,
    lun_count: u8,
    ecc_bits: u8,
}
impl ParameterPage {
    /// Manufacturer of the device
    pub fn manufacturer(&self) -> &str {
        str::from_utf8(&self.manufacturer).unwrap_or("<ERR>")
    }
    /// Model number of the deviceo
    pub fn model(&self) -> &str {
        str::from_utf8(&self.model).unwrap_or("<ERR>")
    }
}
impl fmt::Debug for ParameterPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ONFI Parameter Page")
            .field("ONFI Revision", &self.onfi_revision)
            .field("Manufacturer", &self.manufacturer())
            .field("Model", &self.model())
            .field("Date Code", &self.date_code)
            .field("Data bytes per Page", &self.data_bytes_per_page)
            .field("Spare bytes per Page", &self.spare_bytes_per_page)
            .field("Pages per Block", &self.pages_per_block)
            .field("Blocks per LUN", &self.blocks_per_lun)
            .field("LUN Count", &self.lun_count)
            .field("ECC Bits Correctability", &self.ecc_bits)
            .finish()
    }
}

/// NAND Device
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_copy_implementations)]
pub struct NandDevice {
    common_command: *mut u8,
    common_address: *mut u8,
    attribute_command: *mut u8,
    common_data: *mut u8,

    /// Number of address bits C that are used for the column address. The
    /// number of data bytes per page is typically 2^C
    column_bits: Option<usize>,
}

unsafe fn write_volatile_sync<T>(dest: *mut T, src: T) {
    ptr::write_volatile(dest, src);

    // Ensure that the write is committed before continuing. In the default
    // ARMv7-M address map the space 0x8000_0000-0x9FFF_FFFF is Normal Memory
    // with write-though cache attribute.
    fence(Ordering::SeqCst);
}

impl NandDevice {
    /// Create a `NandDevice` from a bank pointer
    ///
    /// # Safety
    ///
    /// The FMC controller must have been initialized as NAND controller and
    /// enabled for this bank, with the correct pin settings. The bank pointer
    /// must be a singleton.
    pub(crate) unsafe fn init(ptr: *mut u8, column_bits: usize) -> NandDevice {
        let mut nand = NandDevice {
            common_command: ptr.add(0x1_0000),
            common_address: ptr.add(0x2_0000),
            attribute_command: ptr.add(0x801_0000),
            common_data: ptr,
            column_bits: Some(column_bits),
        };

        // Reset Command. May be specifically required by some devices and there
        // seems to be no disadvantage of sending it always
        nand.reset();

        nand
    }
    /// 0xFF Reset: ONFI Section 5.3
    pub fn reset(&mut self) {
        unsafe {
            write_volatile_sync(self.common_command, 0xFF);
        }
    }
    /// Generic Command
    fn command(&mut self, cmd: Command, address: u8, buffer: &mut [u8]) {
        unsafe {
            write_volatile_sync(self.common_command, cmd as u8);
            write_volatile_sync(self.common_address, address);
            for x in buffer {
                *x = ptr::read_volatile(self.common_data);
            }
        }
    }
    /// Generic Address
    ///
    /// column_bits must be set first!
    fn address(&mut self, address: usize, spare: bool) {
        let column_bits = self
            .column_bits
            .expect("Number of column bits must be configured first");
        let column = (address & ((1 << column_bits) - 1))
            + if spare { 1 << column_bits } else { 0 };
        let row = address >> column_bits;

        let mut addr_cycles = [0u8; 5];

        // Assuming 5-cycle address
        addr_cycles[0] = (column & 0xFF) as u8;
        addr_cycles[1] = ((column >> 8) & 0xFF) as u8;
        addr_cycles[2] = (row & 0xFF) as u8;
        addr_cycles[3] = ((row >> 8) & 0xFF) as u8;
        addr_cycles[4] = ((row >> 16) & 0xFF) as u8;

        for a in addr_cycles {
            unsafe {
                write_volatile_sync(self.common_address, a);
            }
        }
    }

    /// 0x90 Read ID: ONFI Section 5.6
    pub fn read_id(&mut self) -> ID {
        let mut id = [0u8; 5];
        self.command(Command::ReadID, 0, &mut id);

        let internal_chip_count = match id[2] & 3 {
            1 => 2,
            2 => 4,
            3 => 8,
            _ => 1,
        };
        let page_size = match id[3] & 3 {
            1 => 2048,
            2 => 4096,
            _ => 0,
        };
        ID {
            manufacturer_jedec: id[0],
            device_jedec: id[1],
            internal_chip_count,
            page_size,
        }
    }
    /// 0xEC Read Parameter Page: ONFI Section 5.7
    pub fn read_parameter_page(&mut self) -> ParameterPage {
        let mut page = [0u8; 115];
        self.command(Command::ReadParameterPage, 0, &mut page);

        ParameterPage {
            signature: page[0..4].try_into().unwrap(),
            onfi_revision: u16::from_le_bytes(page[4..6].try_into().unwrap()),
            manufacturer: page[32..44].try_into().unwrap(),
            model: page[44..64].try_into().unwrap(),
            date_code: u16::from_le_bytes(page[65..67].try_into().unwrap()),
            data_bytes_per_page: u32::from_le_bytes(
                page[80..84].try_into().unwrap(),
            ),
            spare_bytes_per_page: u16::from_le_bytes(
                page[84..86].try_into().unwrap(),
            ),
            pages_per_block: u32::from_le_bytes(
                page[92..96].try_into().unwrap(),
            ),
            blocks_per_lun: u32::from_le_bytes(
                page[96..100].try_into().unwrap(),
            ),
            lun_count: page[100],
            ecc_bits: page[112],
        }
    }
    /// 0xED Read Unique ID: ONFI Section 5.8
    pub fn read_unique_id(&mut self) -> u128 {
        let mut unique = [0u8; 16];
        self.command(Command::ReadUniqueID, 0, &mut unique);
        u128::from_le_bytes(unique)
    }
    /// 0x60 Block Erase: ONFI Section 5.9
    pub fn block_erase(&mut self, address: usize) -> Status {
        unsafe {
            write_volatile_sync(self.common_command, 0x60); // auto block erase setup
        }

        let column_bits = self
            .column_bits
            .expect("Number of column bits must be configured first!");
        let row = address >> column_bits;
        unsafe {
            // write block address
            write_volatile_sync(self.common_address, (row & 0xFF) as u8);
            write_volatile_sync(self.common_address, ((row >> 8) & 0xFF) as u8);
            write_volatile_sync(
                self.common_address,
                ((row >> 16) & 0xFF) as u8,
            );

            // erase command
            write_volatile_sync(self.attribute_command, 0xD0); // t_WB
            write_volatile_sync(self.common_command, Command::ReadStatus as u8);
            let status_register = ptr::read_volatile(self.common_data);
            Status::from_register(status_register)
        }
    }

    /// Page Read: ONFI Section 5.14
    ///
    /// This method starts a Page Read operation but does not include the data
    /// phase. This method is useful when DMA is used for the data phase.
    ///
    /// For a method that completes the entire transaction see
    /// [`page_read`](Self::page_read).
    pub fn start_page_read(&mut self, address: usize, spare: bool) {
        unsafe {
            write_volatile_sync(self.common_command, 0x00);
            self.address(address, spare);
            write_volatile_sync(self.attribute_command, 0x30); // t_WB
        }
    }
    /// Page Read: ONFI Section 5.14
    ///
    /// Executes a Page Read operation from the specified address. Data is
    /// copied to the slice `page`. The length of `page` determines the read
    /// length. The read length should not exceed the number of bytes between
    /// the specified address and the end of the page. Reading beyond the end of
    /// the page results in indeterminate values being returned.
    ///
    /// If `spare` is true, then the read occours from the spare area. The
    /// address offset from the start of the page plus the slice length should
    /// not exceed the spare area size.
    pub fn page_read(&mut self, address: usize, spare: bool, page: &mut [u8]) {
        self.start_page_read(address, spare);
        for x in page {
            unsafe {
                *x = ptr::read_volatile(self.common_data);
            }
        }
    }

    /// Page Program: ONFI Section 5.16
    ///
    /// Executes a page program to the specified address and waits for it to
    /// complete. The length of `page` determines the write length. The write
    /// length should not exceed the number of bytes between the specified
    /// address and the end of the page. Writing beyond this length is
    /// undefined.
    pub fn page_program(
        &mut self,
        address: usize,
        spare: bool,
        page: &[u8],
    ) -> Status {
        unsafe {
            write_volatile_sync(self.common_command, 0x80); // data input
            self.address(address, spare);
            for x in page {
                write_volatile_sync(self.common_data, *x); // write page
            }
            write_volatile_sync(self.attribute_command, 0x10); // program command, t_WB
            let mut status_register;
            while {
                write_volatile_sync(
                    self.common_command,
                    Command::ReadStatus as u8,
                );
                status_register = ptr::read_volatile(self.common_data);

                status_register & 0x20 == 0 // program in progress
            } {}

            Status::from_register(status_register)
        }
    }
}

/// Methods to allow users to implement their own commands using `unsafe`.
///
impl NandDevice {
    /// Return a Raw Pointer to the common command space. This memory-mapped
    /// address is used to write command phase of NAND device transactions.
    ///
    /// It is recommended to use
    /// [`ptr::write_volatile`](https://doc.rust-lang.org/std/ptr/fn.write_volatile.html)
    /// to write to this pointer. Depending on the memory map in use, you may
    /// need to ensure the write is committed by using
    /// [`core::atomic::sync::fence`](https://doc.rust-lang.org/core/sync/atomic/fn.fence.html).
    pub fn common_command(&mut self) -> *mut u8 {
        self.common_command
    }
    /// Return a Raw Pointer to the attribute command space. This memory-mapped
    /// address is used to write command phase of NAND device transactions.
    ///
    /// It is recommended to use
    /// [`ptr::write_volatile`](https://doc.rust-lang.org/std/ptr/fn.write_volatile.html)
    /// to write to this pointer. Depending on the memory map in use, you may
    /// need to ensure the write is committed by using
    /// [`core::atomic::sync::fence`](https://doc.rust-lang.org/core/sync/atomic/fn.fence.html).
    pub fn attribute_command(&mut self) -> *mut u8 {
        self.attribute_command
    }
    /// Return a Raw Pointer to the common address space. This memory-mapped
    /// address is used to write the address phase of NAND device transactions.
    ///
    /// It is recommended to use
    /// [`ptr::write_volatile`](https://doc.rust-lang.org/std/ptr/fn.write_volatile.html)
    /// to write to this pointer. Depending on the memory map in use, you may
    /// need to ensure the write is committed by using
    /// [`core::atomic::sync::fence`](https://doc.rust-lang.org/core/sync/atomic/fn.fence.html).
    pub fn common_address(&mut self) -> *mut u8 {
        self.common_address
    }
    /// Return a Raw Pointer to the common data space. This memory-mapped
    /// address is used to write or read the data phase of NAND device
    /// transactions.
    ///
    /// It is recommended to use
    /// [`ptr::write_volatile`](https://doc.rust-lang.org/std/ptr/fn.write_volatile.html)
    /// to write to this pointer. Depending on the memory map in use, you may
    /// need to ensure the write is committed by using
    /// [`core::atomic::sync::fence`](https://doc.rust-lang.org/core/sync/atomic/fn.fence.html).
    pub fn common_data(&mut self) -> *mut u8 {
        self.common_data
    }
}
