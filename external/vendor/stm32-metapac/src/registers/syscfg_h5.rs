
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "SBS register block",
            ),
            items: &[
                BlockItem {
                    name: "hdplcr",
                    description: Some(
                        "SBS temporal isolation control register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdplcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdplsr",
                    description: Some(
                        "SBS temporal isolation status register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdplsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nexthdplcr",
                    description: Some(
                        "SBS next HDPL control register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nexthdplcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbgcr",
                    description: Some(
                        "SBS debug control register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbgcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbglockr",
                    description: Some(
                        "SBS debug lock register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbglockr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsscmdr",
                    description: Some(
                        "SBS RSS command register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsscmdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "epochselcr",
                    description: Some(
                        "SBS EPOCH selection control register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Epochselcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "SBS security mode configuration control register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pmcr",
                    description: Some(
                        "SBS product mode and configuration register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fpuimr",
                    description: Some(
                        "SBS FPU interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fpuimr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mesr",
                    description: Some(
                        "SBS memory erase status register",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mesr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cccsr",
                    description: Some(
                        "SBS compensation cell for I/Os control and status register",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cccsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccvalr",
                    description: Some(
                        "SBS compensation cell for I/Os value register",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccvalr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccswcr",
                    description: Some(
                        "SBS compensation cell for I/Os software code register",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccswcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "SBS Class B register",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnslckr",
                    description: Some(
                        "SBS CPU non-secure lock register",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnslckr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cslckr",
                    description: Some(
                        "SBS CPU secure lock register",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cslckr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccnmir",
                    description: Some(
                        "SBS flift ECC NMI mask register",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccnmir",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cccsr",
            extends: None,
            description: Some(
                "SBS compensation cell for I/Os control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "enable compensation cell for VDDIO power rail\r This bit enables the I/O compensation cell.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "cs",
                    description: Some(
                        "code selection for VDDIO power rail (reset value set to 1)\r This bit selects the code to be applied for the I/O compensation cell.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Cs",
                    ),
                },
                Field {
                    name: "rdy",
                    description: Some(
                        "VDDIO compensation cell ready flag\r This bit provides the status of the compensation cell.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccswcr",
            extends: None,
            description: Some(
                "SBS compensation cell for I/Os software code register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw_ansrc1",
                    description: Some(
                        "NMOS compensation code for VDD power rails\r This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sw_apsrc1",
                    description: Some(
                        "PMOS compensation code for the VDD power rails\r This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS1 is set in SBS_CCSR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sw_ansrc2",
                    description: Some(
                        "NMOS compensation code for VDDIO power rails\r This bitfield is written by software to define an I/O compensation cell code for NMOS transistors of the VDD power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sw_apsrc2",
                    description: Some(
                        "PMOS compensation code for the VDDIO power rails\r This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccvalr",
            extends: None,
            description: Some(
                "SBS compensation cell for I/Os value register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ansrc1",
                    description: Some(
                        "compensation value for the NMOS transistor\r This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apsrc1",
                    description: Some(
                        "compensation value for the PMOS transistor\r This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ansrc2",
                    description: Some(
                        "Compensation value for the NMOS transistor\r This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apsrc2",
                    description: Some(
                        "compensation value for the PMOS transistor\r This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "SBS Class B register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cll",
                    description: Some(
                        "core lockup lock\r This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1/8/15/16/17 break inputs.",
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
                    name: "sel",
                    description: Some(
                        "SRAM ECC error lock\r This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1/8/15/16/17.",
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
                    name: "pvdl",
                    description: Some(
                        "PVD lock\r This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1/8/15/16/17 break inputs.",
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
                    name: "eccl",
                    description: Some(
                        "ECC lock\r This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1/8/15/6/17.",
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
            ],
        },
        FieldSet {
            name: "Cnslckr",
            extends: None,
            description: Some(
                "SBS CPU non-secure lock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "locknsvtor",
                    description: Some(
                        "VTOR_NS register lock\r This bit is set by software and cleared only by a system reset.",
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
                    name: "locknsmpu",
                    description: Some(
                        "non-secure MPU register lock\r This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.",
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
            ],
        },
        FieldSet {
            name: "Cslckr",
            extends: None,
            description: Some(
                "SBS CPU secure lock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "locksvtaircr",
                    description: Some(
                        "VTOR_S and AIRCR register lock\r This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register.",
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
                    name: "locksmpu",
                    description: Some(
                        "secure MPU registers lock\r This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers.",
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
                    name: "locksau",
                    description: Some(
                        "SAU registers lock\r This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers.",
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
            ],
        },
        FieldSet {
            name: "Dbgcr",
            extends: None,
            description: Some(
                "SBS debug control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ap_unlock",
                    description: Some(
                        "access port unlock\r Write 0xB4 to this bitfield to open the device access port.",
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
                Field {
                    name: "dbg_unlock",
                    description: Some(
                        "debug unlock when DBG_AUTH_HDPL is reached\r Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_auth_hdpl",
                    description: Some(
                        "authenticated debug temporal isolation level\r Writing to this bitfield defines at which HDPL the authenticated debug opens.\r Note: Writing any other values is ignored. Reading any other value means the debug never opens.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "DbgAuthHdpl",
                    ),
                },
                Field {
                    name: "dbg_auth_sec",
                    description: Some(
                        "control debug opening secure/non-secure\r Write 0xB4 to this bitfield to open debug for secure and non-secure.\r Writing any other values only open non-secure.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dbglockr",
            extends: None,
            description: Some(
                "SBS debug lock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbgcfg_lock",
                    description: Some(
                        "debug configuration lock\r Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4.\r 0xC3 is the recommended value to lock the debug configuration using this bitfield.\r Other: Writes to SBS_DBGCR ignored",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "DbgcfgLock",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Eccnmir",
            extends: None,
            description: Some(
                "SBS flift ECC NMI mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccnmi_mask_en",
                    description: Some(
                        "NMI behavior setup when a double ECC error occurs on flitf data part",
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
            ],
        },
        FieldSet {
            name: "Epochselcr",
            extends: None,
            description: Some(
                "SBS EPOCH selection control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "epoch_sel",
                    description: Some(
                        "select EPOCH value to be sent to the SAES\r 1x: EPOCH forced to zero (value used to retrieve PUF reference value at boot time)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "EpochSel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Fpuimr",
            extends: None,
            description: Some(
                "SBS FPU interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fpu_ie",
                    description: Some(
                        "FPU interrupt enable\r Set and cleared by software to enable the Cortex-M33 FPU interrupts\r FPU_IE[5]: inexact interrupt enable (interrupt disabled at reset)\r FPU_IE[4]: input abnormal interrupt enable\r FPU_IE[3]: overflow interrupt enable\r FPU_IE[2]: underflow interrupt enable\r FPU_IE[1]: divide-by-zero interrupt enable\r FPU_IE[0]: invalid operation interrupt enable",
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
            ],
        },
        FieldSet {
            name: "Hdplcr",
            extends: None,
            description: Some(
                "SBS temporal isolation control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "incr_hdpl",
                    description: Some(
                        "increment HDPL value\r Other: all other values allow a HDPL level increment.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "IncrHdpl",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Hdplsr",
            extends: None,
            description: Some(
                "SBS temporal isolation status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdpl",
                    description: Some(
                        "temporal isolation level\r This bitfield returns the current temporal isolation level.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "Hdpl",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Mesr",
            extends: None,
            description: Some(
                "SBS memory erase status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mclr",
                    description: Some(
                        "erase after reset status\r This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, DCACHE, ICACHE and PKA. It is set by hardware and reset by software",
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
                    name: "ipmee",
                    description: Some(
                        "end-of-erase status for ICACHE and PKA RAM\r This bit shows the status of the protection for ICACHE and PKA. It is set by hardware and reset by software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nexthdplcr",
            extends: None,
            description: Some(
                "SBS next HDPL control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nexthdpl",
                    description: Some(
                        "index to point to a higher HDPL than the current one\r Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pmcr",
            extends: None,
            description: Some(
                "SBS product mode and configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boosten",
                    description: Some(
                        "booster enable\r Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "boostvddsel",
                    description: Some(
                        "booster VDD selection\r Note: Booster must not be used when VDDA < 2.7 V, but VDD > 2.7 V (add current consumption).\r When both VDD < 2.7 V and VDDA < 2.7 V, booster is needed to get full AC performances from I/O analog switches.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb6_fmplus",
                    description: Some(
                        "Fast-mode Plus command on PB(6)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb7_fmplus",
                    description: Some(
                        "Fast-mode Plus command on PB(7)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb8_fmplus",
                    description: Some(
                        "Fast-mode Plus command on PB(8)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb9_fmplus",
                    description: Some(
                        "Fast-mode Plus command on PB(9)",
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
                    name: "eth_sel_phy",
                    description: Some(
                        "Ethernet PHY interface selection\r Other: reserved",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "EthSelPhy",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Rsscmdr",
            extends: None,
            description: Some(
                "SBS RSS command register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsscmd",
                    description: Some(
                        "RSS command\r The application can use this bitfield to pass on a command to the RSS, executed at the next reset.\r When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value.",
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
            ],
        },
        FieldSet {
            name: "Seccfgr",
            extends: None,
            description: Some(
                "SBS security mode configuration control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbssec",
                    description: Some(
                        "SBS clock control, memory-erase status register and compensation cell register security enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "classbsec",
                    description: Some(
                        "ClassB security enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "fpusec",
                    description: Some(
                        "FPU security enable\r Note: This bit can only be written through privilege transaction.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "sdce_sec_en",
                    description: Some(
                        "control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Cs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CELL",
                    description: Some(
                        "Code from the cell (available in the SBS_CCVR)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SOFTWARE",
                    description: Some(
                        "Code from SBS_CCCR",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "DbgAuthHdpl",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B_0X51",
                    description: Some(
                        "HDPL1",
                    ),
                    value: 81,
                },
                EnumVariant {
                    name: "B_0X6F",
                    description: Some(
                        "HDPL3",
                    ),
                    value: 111,
                },
                EnumVariant {
                    name: "B_0X8A",
                    description: Some(
                        "HDPL2",
                    ),
                    value: 138,
                },
            ],
        },
        Enum {
            name: "DbgcfgLock",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B_0X_B4",
                    description: Some(
                        "Writes to SBS_DBGCR allowed (default)",
                    ),
                    value: 180,
                },
            ],
        },
        Enum {
            name: "EpochSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "SEC_EPOCH counter input selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "NS_EPOCH (non-secure) input selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "EthSelPhy",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "MII_GMII",
                    description: Some(
                        "GMII or MII",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RESERVED_RGMII",
                    description: Some(
                        "reserved (RGMII)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RMII",
                    description: Some(
                        "RMII",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Hdpl",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B_0X51",
                    description: Some(
                        "HDPL1, iRoT",
                    ),
                    value: 81,
                },
                EnumVariant {
                    name: "B_0X6F",
                    description: Some(
                        "HDPL3, application (secure/non-secure)",
                    ),
                    value: 111,
                },
                EnumVariant {
                    name: "B_0X8A",
                    description: Some(
                        "HDPL2, uRoT",
                    ),
                    value: 138,
                },
                EnumVariant {
                    name: "B_0X_B4",
                    description: Some(
                        "HDPL0, RSS",
                    ),
                    value: 180,
                },
            ],
        },
        Enum {
            name: "IncrHdpl",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B_0X6A",
                    description: Some(
                        "recommended value to increment HDPL level by one",
                    ),
                    value: 106,
                },
                EnumVariant {
                    name: "B_0X_B4",
                    description: Some(
                        "no increment",
                    ),
                    value: 180,
                },
            ],
        },
        Enum {
            name: "Sec",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "SBS_CFGR2 register accessible through secure or non-secure transaction",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "SBS_CFGR2 register only accessible through secure transaction",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
