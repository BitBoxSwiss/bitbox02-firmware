
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "System configuration controller",
            ),
            items: &[
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "secure configuration register",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    name: "cfgr1",
                    description: Some(
                        "configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fpuimr",
                    description: Some(
                        "FPU interrupt mask register",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "cnslckr",
                    description: Some(
                        "CPU non-secure lock register",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                    name: "cslockr",
                    description: Some(
                        "CPU secure lock register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cslockr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x14,
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
                    name: "mesr",
                    description: Some(
                        "memory erase status register",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                        "compensation cell control/status register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
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
                    name: "ccvr",
                    description: Some(
                        "compensation cell value register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccvr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cccr",
                    description: Some(
                        "compensation cell code register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsscmdr",
                    description: Some(
                        "RSS command register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
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
                    name: "otghsphycr",
                    description: Some(
                        "OTG_HS PHY register",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otghsphycr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otghsphytuner2",
                    description: Some(
                        "OTG_HS PHY tune register 2",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otghsphytuner2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cccr",
            extends: None,
            description: Some(
                "compensation cell code register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ncc1",
                    description: Some(
                        "NMOS compensation code of the I/Os supplied by V<sub>DD</sub>\r These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is set.",
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
                    name: "pcc1",
                    description: Some(
                        "PMOS compensation code of the I/Os supplied by V<sub>DD</sub>\r These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is set.",
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
                    name: "ncc2",
                    description: Some(
                        "NMOS compensation code of the I/Os supplied by V<sub>DDIO2</sub>\r These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is set.",
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
                    name: "pcc2",
                    description: Some(
                        "PMOS compensation code of the I/Os supplied by V<sub>DDIO2</sub>\r These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is set.",
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
            name: "Cccsr",
            extends: None,
            description: Some(
                "compensation cell control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en1",
                    description: Some(
                        "VDD I/Os compensation cell enable\r This bit enables the compensation cell of the I/Os supplied by V<sub>DD</sub>.",
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
                    name: "cs1",
                    description: Some(
                        "VDD I/Os code selection\r This bit selects the code to be applied for the compensation cell of the I/Os supplied by V<sub>DD</sub>.",
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
                    name: "en2",
                    description: Some(
                        "VDDIO2 I/Os compensation cell enable\r This bit enables the compensation cell of the I/Os supplied by V<sub>DDIO2</sub>.",
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
                    name: "cs2",
                    description: Some(
                        "VDDIO2 I/Os code selection\r This bit selects the code to be applied for the compensation cell of the I/Os supplied by V<sub>DDIO2</sub>.",
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
                    name: "rdy1",
                    description: Some(
                        "VDD I/Os compensation cell ready flag\r This bit provides the compensation cell status of the I/Os supplied by V<sub>DD</sub>.\r Note: The HSI clock is required for the compensation cell to work properly. The compensation cell ready bit (RDY1) is not set if the HSI clock is not enabled (HSION).",
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
                    name: "rdy2",
                    description: Some(
                        "VDDIO2 I/Os compensation cell ready flag\r This bit provides the compensation cell status of the I/Os supplied by V<sub>DDIO2</sub>.\r Note: The HSI clock is required for the compensation cell to work properly. The compensation cell ready bit (RDY2) is not set if the HSI clock is not enabled (HSION).",
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
            ],
        },
        FieldSet {
            name: "Ccvr",
            extends: None,
            description: Some(
                "compensation cell value register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ncv1",
                    description: Some(
                        "NMOS compensation value of the I/Os supplied by V<sub>DD</sub>\r This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is reset.",
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
                    name: "pcv1",
                    description: Some(
                        "PMOS compensation value of the I/Os supplied by V<sub>DD</sub>\r This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is reset.",
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
                    name: "ncv2",
                    description: Some(
                        "NMOS compensation value of the I/Os supplied by V<sub>DDIO2</sub>\r This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is reset.",
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
                    name: "pcv2",
                    description: Some(
                        "PMOS compensation value of the I/Os supplied by V<sub>DDIO2</sub>\r This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS2 bit of the CCCSR is reset.",
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
            name: "Cfgr1",
            extends: None,
            description: Some(
                "configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boosten",
                    description: Some(
                        "I/O analog switch voltage booster enable\r Access can be protected by GTZC_TZSC ADC4SEC.\r Note: Refer to Table�121 for setting.",
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
                    name: "anaswvdd",
                    description: Some(
                        "GPIO analog switch control voltage selection\r Access can be protected by GTZC_TZSC ADC4SEC.\r Note: Refer to Table�121 for setting.",
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
                    name: "pa6_fmp",
                    description: Some(
                        "Fast-mode Plus drive capability activation on PA6\r This bit can be read and written only with secure access if PA6 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA6 when PA6 is not used by I2C peripheral. This can be used to dive a LED for instance.\r Access can be protected by GPIOA SEC6.",
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
                    name: "pa7_fmp",
                    description: Some(
                        "Fast-mode Plus drive capability activation on PA7\r This bit can be read and written only with secure access if PA7 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA7 when PA7 is not used by I2C peripheral. This can be used to dive a LED for instance.\r Access can be protected by GPIOA SEC7.",
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
                    name: "pa15_fmp",
                    description: Some(
                        "Fast-mode Plus drive capability activation on PA15\r This bit can be read and written only with secure access if PA15 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA15 when PA15 is not used by I2C peripheral. This can be used to dive a LED for instance.\r Access can be protected by GPIOA SEC15.",
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
                    name: "pb3_fmp",
                    description: Some(
                        "Fast-mode Plus drive capability activation on PB3\r This bit can be read and written only with secure access if PB3 is secure in GPIOB. This bit enables the Fast-mode Plus drive mode for PB3 when PB3 is not used by I2C peripheral. This can be used to dive a LED for instance.\r Access can be protected by GPIOB SEC3.",
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
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cll",
                    description: Some(
                        "Cortex-M33 LOCKUP (hardfault) output enable\r This bit is set by software and cleared only by a system reset. It can be used to enable and lock the connection of Cortex-M33 LOCKUP (hardfault) output to TIM1/16/17 break input.",
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
                    name: "spl",
                    description: Some(
                        "SRAM2 parity lock bit\r This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/16/17 break inputs.",
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
                        "PVD lock enable bit\r This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection to TIM1/16/17 break input, as well as the PVDE and PVDLS[2:0] in the PWR register.",
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
                        "ECC lock\r This bit is set by software and cleared only by a system reset. It can be used to enable and lock the Flash ECC double error signal connection to TIM1/16/17 break input.",
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
                "CPU non-secure lock register",
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
                        "Non-secure MPU registers lock\r This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.",
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
            name: "Cslockr",
            extends: None,
            description: Some(
                "CPU secure lock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "locksvtaircr",
                    description: Some(
                        "VTOR_S register and AIRCR register bits lock\r This bit is set by software and cleared only by a system reset. When set, it disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register.",
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
                        "Secure MPU registers lock\r This bit is set by software and cleared only by a system reset. When set, it disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers.",
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
                        "SAU registers lock\r This bit is set by software and cleared only by a system reset. When set, it disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers.",
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
            name: "Fpuimr",
            extends: None,
            description: Some(
                "FPU interrupt mask register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fpu_ie",
                    description: Some(
                        "Floating point unit interrupts enable bits\r FPU_IE[5]: Inexact interrupt enable (interrupt disable at reset)\r FPU_IE[4]: Input abnormal interrupt enable\r FPU_IE[3]: Overflow interrupt enable\r FPU_IE[2]: Underflow interrupt enable\r FPU_IE[1]: Divide-by-zero interrupt enable\r FPU_IE[0]: Invalid operation Interrupt enable",
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
            name: "Mesr",
            extends: None,
            description: Some(
                "memory erase status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mclr",
                    description: Some(
                        "Device memories erase status\r This bit is set by hardware when SRAM2, ICACHE, PKA SRAM erase is completed after power-on reset or tamper detection (refer to Section�75: Tamper and backup registers (TAMP) for more details). This bit is not reset by system reset and is cleared by software by writing 1 to it.",
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
                        "ICACHE and PKA SRAM erase status\r This bit is set by hardware when ICACHE and PKA SRAM erase is completed after potential tamper detection (refer to Section�75: Tamper and backup registers (TAMP) for more details). This bit is cleared by software by writing 1 to it.",
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
            name: "Otghsphycr",
            extends: None,
            description: Some(
                "OTG_HS PHY register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "PHY Enable",
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
                    name: "pdctrl",
                    description: Some(
                        "Common block power-down control",
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
                    name: "clksel",
                    description: Some(
                        "Reference clock frequency selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Usbrefcksel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Otghsphytuner2",
            extends: None,
            description: Some(
                "OTG_HS tune register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "compdistune",
                    description: Some(
                        "Disconnect threshold adjustment",
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
                Field {
                    name: "sqrxtune",
                    description: Some(
                        "Squelch threshold adjustment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txpreempamptune",
                    description: Some(
                        "HS transmitter preemphasis current control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsscmdr",
            extends: None,
            description: Some(
                "RSS command register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsscmd",
                    description: Some(
                        "RSS commands\r This field defines a command to be executed by the RSS.",
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
                "secure configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgsec",
                    description: Some(
                        "clock control, memory erase status and compensation cell registers security",
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
                    name: "classbsec",
                    description: Some(
                        "Class B security",
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
                    name: "fpusec",
                    description: Some(
                        "FPU security",
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
    ],
    enums: &[
        Enum {
            name: "Usbrefcksel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "MHZ16",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 16 MHz.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "MHZ19_2",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 19.2 MHz.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "MHZ20",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 20MHz.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "MHZ24",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 24 MHz (default after reset).",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "MHZ32",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 32 MHz.",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "MHZ26",
                    description: Some(
                        "The kernel clock frequency provided to the OTG_HS PHY is 26 MHz.",
                    ),
                    value: 14,
                },
            ],
        },
    ],
};
