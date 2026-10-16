
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "System configuration controller.",
            ),
            items: &[
                BlockItem {
                    name: "bootcr",
                    description: Some(
                        "SYSCFG boot pin control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bootcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cm55cr",
                    description: Some(
                        "SYSCFG Cortex-M55 control register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cm55cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cm55tcmcr",
                    description: Some(
                        "SYSCFG Cortex-M55 TCM control register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cm55tcmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cm55rwmcr",
                    description: Some(
                        "SYSCFG Cortex-CM55 memory RW margin register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cm55rwmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "initsvtorcr",
                    description: Some(
                        "SYSCFG Cortex-M55 SVTOR control register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Initsvtorcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "initnsvtorcr",
                    description: Some(
                        "SYSCFG Cortex-M55 NSVTOR control register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Initnsvtorcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cm55rstcr",
                    description: Some(
                        "SYSCFG Cortex-M55 reset type control register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cm55rstcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cm55pahbwpr",
                    description: Some(
                        "SYSCFG Cortex-M55 P-AHB write posting control register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cm55pahbwpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vencramcr",
                    description: Some(
                        "SYSCFG VENCRAM control register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vencramcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pottamprstcr",
                    description: Some(
                        "SYSCFG potential tamper reset register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pottamprstcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icnewrcr",
                    description: Some(
                        "SYSCFG AHB-AXI bridge early write response control register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icnewrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icncgcr",
                    description: Some(
                        "SYSCFG ICN clock gating control register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icncgcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icnbwrcr",
                    description: Some(
                        "SYSCFG ICN bandwidth regulator control register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icnbwrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iocr",
                    description: Some(
                        "SYSCFG /O control register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iocr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vddio1cccr",
                    description: Some(
                        "SYSCFG VDDIO1 compensation cell control register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vddio1cccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vddio1ccsr",
                    description: Some(
                        "SYSCFG VDDIO1 compensation cell status register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vddio1ccsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vddio2cccr",
                    description: Some(
                        "SYSCFG VDDIO2 compensation cell control register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vddio2cccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vddio2ccsr",
                    description: Some(
                        "SYSCFG VDDIO2 compensation cell status register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vddio2ccsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vddio3cccr",
                    description: Some(
                        "SYSCFG VDDIO3 compensation cell control register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vddio3cccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vddio3ccsr",
                    description: Some(
                        "SYSCFG VDDIO3 compensation cell status register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vddio3ccsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vddio4cccr",
                    description: Some(
                        "SYSCFG VDDIO4 compensation cell control register.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vddio4cccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vddio4ccsr",
                    description: Some(
                        "SYSCFG VDDIO4 compensation cell status register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vddio4ccsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vddiocccr",
                    description: Some(
                        "SYSCFG VDDIO compensation cell control register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vddiocccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vddioccsr",
                    description: Some(
                        "SYSCFG VDDIO compensation cell status register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vddioccsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cbr",
                    description: Some(
                        "SYSCFG control timer break register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cbr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sec_aidcr",
                    description: Some(
                        "SYSCFG DMA CID secure control register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SecAidcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fmc_retimecr",
                    description: Some(
                        "SYSCFG FMC retiming logic control register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FmcRetimecr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "npu_icncr",
                    description: Some(
                        "SYSCFG NPU RAM interleaving control register.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "NpuIcncr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bootsr",
                    description: Some(
                        "SYSCFG boot pin status register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bootsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahbwp_error_sr",
                    description: Some(
                        "SYSCFG AHB write posting address error register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AhbwpErrorSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smpshdpcr",
                    description: Some(
                        "SYSCFG SMPS observable signals through HDP selection configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smpshdpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nonsec_aidcr",
                    description: Some(
                        "SYSCFG DMA CID non-secure control register.",
                    ),
                    array: None,
                    byte_offset: 0x800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "NonsecAidcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AhbwpErrorSr",
            extends: None,
            description: Some(
                "SYSCFG AHB write posting address error register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pahb_error_addr",
                    description: Some(
                        "Reports address of the first error in P-AHB write-posting buffer.",
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
            name: "Bootcr",
            extends: None,
            description: Some(
                "SYSCFG boot pin control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boot0_pd",
                    description: Some(
                        "BOOT0 pin pull-down disable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Boot0Pd",
                    ),
                },
                Field {
                    name: "boot1_pd",
                    description: Some(
                        "BOOT1 pin pull-down disable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Boot1Pd",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Bootsr",
            extends: None,
            description: Some(
                "SYSCFG boot pin status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boot0",
                    description: Some(
                        "BOOT0 pin value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Boot0",
                    ),
                },
                Field {
                    name: "boot1",
                    description: Some(
                        "BOOT1 pin value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Boot1",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cbr",
            extends: None,
            description: Some(
                "SYSCFG control timer break register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cm55l",
                    description: Some(
                        "CM55 lockup lock enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cm55l",
                    ),
                },
                Field {
                    name: "pvdl_lock",
                    description: Some(
                        "PVD lock enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "PvdlLock",
                    ),
                },
                Field {
                    name: "bkpraml",
                    description: Some(
                        "Backup SRAM double ECC error lock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bkpraml",
                    ),
                },
                Field {
                    name: "cm55cachel",
                    description: Some(
                        "CM55 cache double ECC error lock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cm55cachel",
                    ),
                },
                Field {
                    name: "cm55tcml",
                    description: Some(
                        "CM55 TCM double ECC error lock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cm55tcml",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cm55cr",
            extends: None,
            description: Some(
                "SYSCFG Cortex-M55 control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fpu_it_en",
                    description: Some(
                        "Enable FPU exception.",
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
                    name: "locksvtaircr",
                    description: Some(
                        "Prevent changes to:.",
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
                    name: "locknsvtor",
                    description: Some(
                        "Prevent changes to the non-secure vector table base address.",
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
                    name: "locksmpu",
                    description: Some(
                        "Prevent changes to programmed secure MPU memory regions.",
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
                    name: "locknsmpu",
                    description: Some(
                        "Prevent changes to non-secure MPU memory regions already programmed.",
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
                    name: "locksau",
                    description: Some(
                        "Prevent changes to secure SAU memory regions already programmed.",
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
                    name: "lockdcaic",
                    description: Some(
                        "Disable access to the instruction cache direct cache access registers DCAICLR and DCAICRR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cm55pahbwpr",
            extends: None,
            description: Some(
                "SYSCFG Cortex-M55 P-AHB write posting control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pahb_error_ack",
                    description: Some(
                        "Error capture in write posting buffer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "PahbErrorAck",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cm55rstcr",
            extends: None,
            description: Some(
                "SYSCFG Cortex-M55 reset type control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "core_reset_type",
                    description: Some(
                        "Select reset to apply on core upon SYSRESETREQ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "CoreResetType",
                    ),
                },
                Field {
                    name: "lockup_rst_en",
                    description: Some(
                        "Select action to perform on a lockup state on the core.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "LockupRstEn",
                    ),
                },
                Field {
                    name: "lockup_nmi_en",
                    description: Some(
                        "Select action to perform on a lockup state on the core.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "LockupNmiEn",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cm55rwmcr",
            extends: None,
            description: Some(
                "SYSCFG Cortex-CM55 memory RW margin register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rme_tcm",
                    description: Some(
                        "RW margin enable input for TCM memories.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "RmeTcm",
                    ),
                },
                Field {
                    name: "rm_tcm",
                    description: Some(
                        "External RW margin inputs for TCM memories.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bc1_tcm",
                    description: Some(
                        "Biasing level adjust input recommended for Vnom.",
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
                Field {
                    name: "bc2_tcm",
                    description: Some(
                        "Biasing level adjust input recommended for Vnom + 10%.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rme_cache",
                    description: Some(
                        "RW margin enable input for caches memories.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "RmeCache",
                    ),
                },
                Field {
                    name: "rm_cache",
                    description: Some(
                        "External read/write (RW) margin inputs for caches memories.",
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
                    name: "bc1_cache",
                    description: Some(
                        "Biasing level adjust input recommended for Vnom.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bc2_cache",
                    description: Some(
                        "Biasing level adjust input recommended for Vnom + 10%.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cm55tcmcr",
            extends: None,
            description: Some(
                "SYSCFG Cortex-M55 TCM control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfgitcmsz",
                    description: Some(
                        "Select ITCM memory size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Cfgitcmsz",
                    ),
                },
                Field {
                    name: "cfgdtcmsz",
                    description: Some(
                        "Select DTCM memory size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Cfgdtcmsz",
                    ),
                },
                Field {
                    name: "locktcm",
                    description: Some(
                        "Disable writes to registers associated with the TCM region.",
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
                    name: "lockitgu",
                    description: Some(
                        "Disable writes to registers associated with the ITCM interface security gating.",
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
                    name: "lockdtgu",
                    description: Some(
                        "Disable writes to registers associated with the DTCM interface security gating.",
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
                    name: "itcmwsdisable",
                    description: Some(
                        "Disable wait-state applied by default on extended ITCM memory.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtcmwsdisable",
                    description: Some(
                        "Disable wait-state applied by default on extended DTCM memory.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "FmcRetimecr",
            extends: None,
            description: Some(
                "SYSCFG FMC retiming logic control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfg_retime_rx",
                    description: Some(
                        "Retiming on Rx path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "CfgRetimeRx",
                    ),
                },
                Field {
                    name: "cfg_retime_tx",
                    description: Some(
                        "Retiming on Tx path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "CfgRetimeTx",
                    ),
                },
                Field {
                    name: "sdfbclk_180",
                    description: Some(
                        "Delay on feedback clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdfbclk180",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Icnbwrcr",
            extends: None,
            description: Some(
                "SYSCFG ICN bandwidth regulator control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icnbwrcr",
                    description: Some(
                        "Bandwidth regulator control bits.",
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
            name: "Icncgcr",
            extends: None,
            description: Some(
                "SYSCFG ICN clock gating control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icncgcr",
                    description: Some(
                        "When bit[i] is set to 1, ICN clock gating[i] is OFF.",
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
            name: "Icnewrcr",
            extends: None,
            description: Some(
                "SYSCFG AHB-AXI bridge early write response control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdmmc1_early_wr_rsp_enable",
                    description: Some(
                        "None.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdmmc1EarlyWrRspEnable",
                    ),
                },
                Field {
                    name: "sdmmc2_early_wr_rsp_enable",
                    description: Some(
                        "None.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdmmc2EarlyWrRspEnable",
                    ),
                },
                Field {
                    name: "usb1_early_wr_rsp_enable",
                    description: Some(
                        "None.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usb1EarlyWrRspEnable",
                    ),
                },
                Field {
                    name: "usb2_early_wr_rsp_enable",
                    description: Some(
                        "None.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usb2EarlyWrRspEnable",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Initnsvtorcr",
            extends: None,
            description: Some(
                "SYSCFG Cortex-M55 NSVTOR control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nsvtor_addr",
                    description: Some(
                        "Non-secure vector table base address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Initsvtorcr",
            extends: None,
            description: Some(
                "SYSCFG Cortex-M55 SVTOR control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "svtor_addr",
                    description: Some(
                        "Secure vector table base address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iocr",
            extends: None,
            description: Some(
                "SYSCFG /O control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iocr",
                    description: Some(
                        "Digital or analog pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "Iocr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "NonsecAidcr",
            extends: None,
            description: Some(
                "SYSCFG DMA CID non-secure control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmacid_nonsec",
                    description: Some(
                        "Non-secure OS allocates specific CID to DMA channel through these bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "NpuIcncr",
            extends: None,
            description: Some(
                "SYSCFG NPU RAM interleaving control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "interleaving_active",
                    description: Some(
                        "Control interleaving on NPU RAMs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "InterleavingActive",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pottamprstcr",
            extends: None,
            description: Some(
                "SYSCFG potential tamper reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pottampersetmask",
                    description: Some(
                        "This bit can be set by software to mask PKA, SAES, CRYP1/2, and HASH reset, in case of potential tamper.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pottampersetmask",
                    ),
                },
            ],
        },
        FieldSet {
            name: "SecAidcr",
            extends: None,
            description: Some(
                "SYSCFG DMA CID secure control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmacid_sec",
                    description: Some(
                        "Secure OS allocates specific CID to DMA channel through these bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Smpshdpcr",
            extends: None,
            description: Some(
                "SYSCFG SMPS observable signals through HDP selection configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smpshdpsel",
                    description: Some(
                        "Others: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Smpshdpsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vddio1cccr",
            extends: None,
            description: Some(
                "SYSCFG VDDIO1 compensation cell control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ransrc",
                    description: Some(
                        "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1.",
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
                    name: "rapsrc",
                    description: Some(
                        "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1.",
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
                    name: "en",
                    description: Some(
                        "Enables the compensation cell of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio1cccrEn",
                    ),
                },
                Field {
                    name: "cs",
                    description: Some(
                        "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio1cccrCs",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vddio1ccsr",
            extends: None,
            description: Some(
                "SYSCFG VDDIO1 compensation cell status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ansrc",
                    description: Some(
                        "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors.",
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
                    name: "apsrc",
                    description: Some(
                        "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors.",
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
                    name: "ready",
                    description: Some(
                        "Provides the compensation cell status of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio1ccsrReady",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vddio2cccr",
            extends: None,
            description: Some(
                "SYSCFG VDDIO2 compensation cell control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ransrc",
                    description: Some(
                        "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1.",
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
                    name: "rapsrc",
                    description: Some(
                        "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1.",
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
                    name: "en",
                    description: Some(
                        "Enables the compensation cell of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio2cccrEn",
                    ),
                },
                Field {
                    name: "cs",
                    description: Some(
                        "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio2cccrCs",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vddio2ccsr",
            extends: None,
            description: Some(
                "SYSCFG VDDIO2 compensation cell status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ansrc",
                    description: Some(
                        "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors.",
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
                    name: "apsrc",
                    description: Some(
                        "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors.",
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
                    name: "ready",
                    description: Some(
                        "Provides the compensation cell status of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio2ccsrReady",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vddio3cccr",
            extends: None,
            description: Some(
                "SYSCFG VDDIO3 compensation cell control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ransrc",
                    description: Some(
                        "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1.",
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
                    name: "rapsrc",
                    description: Some(
                        "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1.",
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
                    name: "en",
                    description: Some(
                        "Enables the compensation cell of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio3cccrEn",
                    ),
                },
                Field {
                    name: "cs",
                    description: Some(
                        "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio3cccrCs",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vddio3ccsr",
            extends: None,
            description: Some(
                "SYSCFG VDDIO3 compensation cell status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ansrc",
                    description: Some(
                        "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors.",
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
                    name: "apsrc",
                    description: Some(
                        "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors.",
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
                    name: "ready",
                    description: Some(
                        "Provides the compensation cell status of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio3ccsrReady",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vddio4cccr",
            extends: None,
            description: Some(
                "SYSCFG VDDIO4 compensation cell control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ransrc",
                    description: Some(
                        "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when the CS = 1.",
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
                    name: "rapsrc",
                    description: Some(
                        "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1.",
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
                    name: "en",
                    description: Some(
                        "Enables the compensation cell of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio4cccrEn",
                    ),
                },
                Field {
                    name: "cs",
                    description: Some(
                        "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio4cccrCs",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vddio4ccsr",
            extends: None,
            description: Some(
                "SYSCFG VDDIO4 compensation cell status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ansrc",
                    description: Some(
                        "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors.",
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
                    name: "apsrc",
                    description: Some(
                        "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors.",
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
                    name: "ready",
                    description: Some(
                        "Provides the compensation cell status of I/Os supplied by VDDIOx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vddio4ccsrReady",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vddiocccr",
            extends: None,
            description: Some(
                "SYSCFG VDDIO compensation cell control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ransrc",
                    description: Some(
                        "These bits are written by software to define an I/O compensation code for NMOS transistors. This code is applied to the I/O compensation cell when CS = 1.",
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
                    name: "rapsrc",
                    description: Some(
                        "These bits are written by software to define an I/O compensation code for PMOS transistors. This code is applied to the I/O compensation cell when CS = 1.",
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
                    name: "en",
                    description: Some(
                        "Enables the compensation cell of I/Os supplied by VDDIO.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "VddiocccrEn",
                    ),
                },
                Field {
                    name: "cs",
                    description: Some(
                        "Selects the code to be applied for the compensation cell of I/Os supplied by VDDIO.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "VddiocccrCs",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vddioccsr",
            extends: None,
            description: Some(
                "SYSCFG VDDIO compensation cell status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ansrc",
                    description: Some(
                        "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for NMOS transistors.",
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
                    name: "apsrc",
                    description: Some(
                        "This value is provided by the cell, and can be used by the CPU to compute an I/O compensation cell code for PMOS transistors.",
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
                    name: "ready",
                    description: Some(
                        "Provides the compensation cell status of I/Os supplied by VDDIO.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "VddioccsrReady",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vencramcr",
            extends: None,
            description: Some(
                "SYSCFG VENCRAM control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vencram_en",
                    description: Some(
                        "VENCRAM allocation VENC if active, or to system (if VENC inactive).",
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
    ],
    enums: &[
        Enum {
            name: "Bkpraml",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Backup SRAM double ECC error signal disconnected from TIM1/8/15/16/17 break inputs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Backup SRAM double ECC error signal connected to TIM1/8/15/16/17 break inputs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Boot0",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "BOOT0 pin connected to VSS (or left open if BOOT0_PD = 0).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "BOOT0 pin connected to VDD.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Boot0Pd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Pull-down enabled. The BOOT0 pin can be left open and takes a value of 0 if open.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Pull-down disabled. The BOOT0 pin must not be left open.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Boot1",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "BOOT1 pin connected to VSS (or left open if BOOT1_PD = 0 in BOOTCR).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "BOOT1 pin connected to VDD.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Boot1Pd",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Pull-down enabled. The BOOT1 pin can be left open and takes a value of 0 if open.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Pull-down disabled. The BOOT1 pin must not be left open.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "CfgRetimeRx",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No retiming on Rx path.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Retiming on Rx path.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "CfgRetimeTx",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No retiming on Tx path.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Retiming on Tx path.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cfgdtcmsz",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "B_0X8",
                    description: Some(
                        "128 Kbytes (default value).",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "B_0X9",
                    description: Some(
                        "256 Kbytes.",
                    ),
                    value: 9,
                },
            ],
        },
        Enum {
            name: "Cfgitcmsz",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "B_0X7",
                    description: Some(
                        "64 KB (default value).",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "B_0X8",
                    description: Some(
                        "128 Kbytes.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "B_0X9",
                    description: Some(
                        "256 Kbytes.",
                    ),
                    value: 9,
                },
            ],
        },
        Enum {
            name: "Cm55cachel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Cortex-M55 cache double ECC error signal disconnected from TIM1/8/15/16/17 break inputs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Cortex-M55 cache double ECC error signal connected to TIM1/8/15/16/17 break inputs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cm55l",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Cortex-M55 lockup output disconnected from TIM1/8/15/16/17 break inputs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Cortex-M55 lockup output disconnected from TIM1/8/15/16/17 break inputs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cm55tcml",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Cortex-M55 TCM double ECC error signal disconnected from TIM1/8/15/16/17 break inputs.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Cortex-M55 TCM double ECC error signal connected to TIM1/8/15/16/17 break inputs.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "CoreResetType",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Warm reset (default value).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Power-on reset.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "InterleavingActive",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Interleaving disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Interleaving enabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Iocr",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "High-speed mode disabled, or use ADC ANA pin.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "High-speed mode enabled, or connect internal ADC ANA signal to GPIO.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "LockupNmiEn",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Lockup state must be recovered from NVIC interrupt (default value).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Lockup generates a NMI on the core.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "LockupRstEn",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Lockup state shall be recovered from interrupt (default value).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Lockup requests a warm reset to the RCC.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "PahbErrorAck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Error capture.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Clean error.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pottampersetmask",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "PKA, SAES, CRYP1/2, and HASH reset in case of potential tamper.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "PKA, SAES, CRYP1/2, and HASH not reset in case of potential tamper.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "PvdlLock",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "PVD interrupt disconnected from TIM1/8/15/16/17 break input. PVDE bits can be programmed by the application.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "PVD interrupt connected to TIM1/8/15/16/17 break input. PVDE and bits are read only.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "RmeCache",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Default RW margin settings.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Use external pin RW margin setting.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "RmeTcm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Default RW margin settings.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Use external pin RW margin setting.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sdfbclk180",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No delay on the feedback clock.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Half a cycle delay on the feedback clock.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sdmmc1EarlyWrRspEnable",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Early-write response disabled. The last AHB write data beat receives the AXI buffered response for the complete AHB transaction.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Early-write response enabled. AHB-Lite write data beats receive an automatic OK response from the AHB-to-AXI bridge, whatever the B-channel AXI response.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sdmmc2EarlyWrRspEnable",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Early-write response disabled. The last AHB write data beat receives the AXI buffered response for the complete AHB transaction.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Early-write response enabled. AHB-Lite write data beats receive an automatic OK response from the AHB-to-AXI bridge, whatever the B-channel AXI response.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Smpshdpsel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Standard run mode (no HDP).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X_C",
                    description: Some(
                        "Analyze fsm mode analysis.",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "B_0X_D",
                    description: Some(
                        "Analyze fsm mos analysis.",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "B_0X_E",
                    description: Some(
                        "Analyze fsm rampe analysis.",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "B_0X_F",
                    description: Some(
                        "Analyze fsm mode analysis.",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Usb1EarlyWrRspEnable",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Early-write response disabled. The last AHB write data beat receives the AXI buffered response for the complete AHB transaction.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Early-write response enabled. AHB-Lite write data beats receive an automatic OK response from the AHB-to-AXI bridge, whatever the B-channel AXI response.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usb2EarlyWrRspEnable",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Early-write response disabled. The last AHB write data beat receives the AXI buffered response for the complete AHB transaction.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Early-write response enabled. AHB-Lite write data beats receive an automatic OK response from the AHB-to-AXI bridge, whatever the B-channel AXI response.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio1cccrCs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O code from the cell (available in the VDDIOxCCSR).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O code from RANSRC[3:0] and RAPSRC[3:0] in this register.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio1cccrEn",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O compensation cell disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O compensation cell enabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio1ccsrReady",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O compensation cell not ready.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O compensation cell ready.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio2cccrCs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O code from the cell (available in the VDDIOxCCSR).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O code from RANSRC[3:0] and RAPSRC[3:0] in this register.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio2cccrEn",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O compensation cell disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O compensation cell enabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio2ccsrReady",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O compensation cell not ready.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O compensation cell ready.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio3cccrCs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O code from the cell (available in the VDDIOxCCSR).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O code from RANSRC[3:0] and RAPSRC[3:0] in this register.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio3cccrEn",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O compensation cell disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O compensation cell enabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio3ccsrReady",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O compensation cell not ready.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O compensation cell ready.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio4cccrCs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O code from the cell (available in the VDDIOxCCSR).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O code from RANSRC[3:0] and RAPSRC[3:0] in this register.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio4cccrEn",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O compensation cell disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O compensation cell enabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vddio4ccsrReady",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIOx I/O compensation cell not ready.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIOx I/O compensation cell ready.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "VddiocccrCs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIO I/O code from the cell (available in the VDDIOCCSR).",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIO I/O code from RANSRC[3:0] and RAPSRC[3:0].",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "VddiocccrEn",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIO I/O compensation cell disabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIO I/O compensation cell enabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "VddioccsrReady",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VDDIO I/O compensation cell not ready.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VDDIO I/O compensation cell ready.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
