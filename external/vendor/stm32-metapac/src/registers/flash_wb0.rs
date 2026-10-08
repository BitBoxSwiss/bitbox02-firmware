
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Flash",
            extends: None,
            description: Some(
                "4kb addressable space.",
            ),
            items: &[
                BlockItem {
                    name: "command",
                    description: Some(
                        "COMMAND register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Command",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "config",
                    description: Some(
                        "CONFIG register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Config",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irqstat",
                    description: Some(
                        "IRQSTAT register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irqstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irqmask",
                    description: Some(
                        "IRQMASK register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irqmask",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irqraw",
                    description: Some(
                        "IRQRAW register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irqraw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "size",
                    description: Some(
                        "SIZE register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Size",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "address",
                    description: Some(
                        "ADDRESS register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Address",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lfsrval",
                    description: Some(
                        "LFSRVAL register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Lfsrval",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pageprot0",
                    description: Some(
                        "PAGEPROT0 register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pageprot0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pageprot1",
                    description: Some(
                        "PAGEPROT1 register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pageprot1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data0",
                    description: Some(
                        "DATA0 register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Data0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data1",
                    description: Some(
                        "DATA1 register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Data1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data2",
                    description: Some(
                        "DATA2 register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Data2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data3",
                    description: Some(
                        "DATA3 register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Data3",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Address",
            extends: None,
            description: Some(
                "ADDRESS register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "yaddr",
                    description: Some(
                        "Flash column address offset to be used with some COMMAND.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xaddr",
                    description: Some(
                        "Flash row address offset to be used with some COMMAND.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Command",
            extends: None,
            description: Some(
                "COMMAND register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "command",
                    description: Some(
                        "Macro commands for flash operations (may require DATA0...DATA3 to be set): - 0x11 : ERASE - 0x22 : MASSERASE - 0x33 : WRITE - 0x55 : MASSREAD - 0xAA : SLEEP - 0xBB : WAKEUP - 0xCC : BURSTWRITE - 0xEE : OTPWRITE - 0xFF : KEYWRITE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Config",
            extends: None,
            description: Some(
                "CONFIG register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "remap",
                    description: Some(
                        "Bit to redirect boot area on SRAM0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dis_group_write",
                    description: Some(
                        "Burst write Control: - 0 : burst write allowed - 1 : burst write forbidden.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wait_states",
                    description: Some(
                        "Number of wait states to be inserted on Flash read (AHB accesses).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Data0",
            extends: None,
            description: Some(
                "DATA0 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data0",
                    description: Some(
                        "Value to be used as DATA for any COMMAND of type WRITE and compare value for MASSREAD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Data1",
            extends: None,
            description: Some(
                "DATA1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data1",
                    description: Some(
                        "Value to be used as DATA for any COMMAND of type WRITE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Data2",
            extends: None,
            description: Some(
                "DATA2 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data2",
                    description: Some(
                        "Value to be used as DATA for any COMMAND of type WRITE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Data3",
            extends: None,
            description: Some(
                "DATA3 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data3",
                    description: Some(
                        "Value to be used as DATA for any COMMAND of type WRITE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irqmask",
            extends: None,
            description: Some(
                "IRQMASK register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmddonem",
                    description: Some(
                        "Command done mask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmdstartm",
                    description: Some(
                        "Command started mask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmderrm",
                    description: Some(
                        "Command error mask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "illcmdm",
                    description: Some(
                        "Illegal command mask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "readokm",
                    description: Some(
                        "Mass read OK mask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fnreadym",
                    description: Some(
                        "(1: mask, 0: inactive) FNREADY_MIS mask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irqraw",
            extends: None,
            description: Some(
                "IRQRAW register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmddone_ris",
                    description: Some(
                        "Command done raw/unmasked interrupt status. This it is set once the requested command execution is completed. Cleared by writing 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmdstart_ris",
                    description: Some(
                        "Command started raw/unmasked interrupt status. This bit is set once the requested command execution has started.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmderr_ris",
                    description: Some(
                        "Command error raw/unmasked interrupt status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "illcmd_ris",
                    description: Some(
                        "Illegal command raw/unmasked interrupt status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "readok_ris",
                    description: Some(
                        "Mass read OK raw/unmasked interrupt status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmdsleeperr_ris",
                    description: Some(
                        "(1: active, 0: inactive) COMMAND issued while flash in sleep-mode (SLM=1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irqstat",
            extends: None,
            description: Some(
                "IRQSTAT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmddone_mis",
                    description: Some(
                        "Command done masked interrupt status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmdstart_mis",
                    description: Some(
                        "Command started masked interrupt status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmderr_mis",
                    description: Some(
                        "Command error masked interrupt status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "illcmd_mis",
                    description: Some(
                        "Illegal command masked interrupt status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "readok_mis",
                    description: Some(
                        "Mass read OK masked interrupt status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fnready_mis",
                    description: Some(
                        "(1: clear, 0: inactive) FNREADY_MIS flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lfsrval",
            extends: None,
            description: Some(
                "LFSRVAL register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lfsrval",
                    description: Some(
                        "Flash read data CRC signature.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pageprot0",
            extends: None,
            description: Some(
                "PAGEPROT0 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seg0",
                    description: Some(
                        "First segment definition.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seg1",
                    description: Some(
                        "Second segment definition. See SEG0 description for details on SEG1[31:16] content.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pageprot1",
            extends: None,
            description: Some(
                "PAGEPROT1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seg2",
                    description: Some(
                        "Third segment definition. See PAGEPROT0 SEG0 description for details on SEG2[15:0] content.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seg3",
                    description: Some(
                        "Fourth segment definition. See PAGEPROT0 SEG0 description for details on SEG3[15:0] content.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Size",
            extends: None,
            description: Some(
                "SIZE register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flash_size",
                    description: Some(
                        "Maximum valid address for flash memory: - 00 : 0x0BFFF (192kb) - 01 : 0x0FFFF (256kb) - 10 : 0x17FFF (384kb) - 11 : 0x1FFFF (512kb).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ram_size",
                    description: Some(
                        "RAM memory size selection: - 00 : 32kb - 01 : 32kb - 10 : 48kb - 11 : 64kb.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flash_secure",
                    description: Some(
                        "Flash memory protection (0: no key present, 1: key present).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swd_disable",
                    description: Some(
                        "Flash+SWD protection: 0: No SWD protection (refer to FLASH_SECURE) 1: Flash and SWD protected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "package_size",
                    description: Some(
                        "Package selection: - 0- : CSP - 10 : 32pins - 11 : 48pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
