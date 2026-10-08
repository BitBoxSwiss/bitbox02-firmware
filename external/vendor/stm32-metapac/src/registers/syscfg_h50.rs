
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "System configuration, boot and security",
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
                        "SBS CPU lock register",
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
                        "PMOS compensation code for the V<sub>DDIO</sub> power rails\r This bitfield is written by software to define an I/O compensation cell code for PMOS transistors of the VDDIO power rail. This code is applied to the I/O when CS2 is set in SBS_CCSR.",
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
                        "compensation value for the PMOS transistor \r This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range.",
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
                        "Compensation value for the NMOS transistor \r This value is provided by the cell and must be interpreted by the processor to compensate the slew rate in the functional range.",
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
                        "core lockup lock\r This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs.",
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
                        "SRAM ECC error lock\r This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1.",
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
                        "PVD lock\r This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs.",
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
                        "ECC lock\r This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1.",
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
                "SBS CPU lock register",
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
                        "MPU register lock \r This bit is set by software and cleared only by a system reset. When set, this bit disables write access to MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.",
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
                        "erase after reset status\r This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software",
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
                        "end-of-erase status for ICACHE\r This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software.",
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
                        "booster V<sub>DD</sub> selection\r Note: Booster must not be used when V<sub>DDA</sub> < 2.7 V, but V<sub>DD</sub> > 2.7 V (add current consumption).\r Note: When both V<sub>DD</sub> < 2.7 V and V<sub>DDA</sub> < 2.7 V, booster is needed to get full AC performances from I/O analog switches.",
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
                        "Code from the cell (available in SBS_CCVR)",
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
                        "HDPL3, application",
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
    ],
};
