
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Flash",
            extends: None,
            description: Some(
                "FLASH address block description",
            ),
            items: &[
                BlockItem {
                    name: "acr",
                    description: Some(
                        "FLASH access control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Acr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nskeyr",
                    description: Some(
                        "FLASH non-secure key register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "seckeyr",
                    description: Some(
                        "FLASH secure key register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "optkeyr",
                    description: Some(
                        "FLASH option key register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "nsobkkeyr",
                    description: Some(
                        "FLASH non-secure OBK key register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "secobkkeyr",
                    description: Some(
                        "FLASH secure OBK key register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "opsr",
                    description: Some(
                        "FLASH operation status register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Opsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optcr",
                    description: Some(
                        "FLASH option control register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nssr",
                    description: Some(
                        "FLASH non-secure status register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nssr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secsr",
                    description: Some(
                        "FLASH secure status register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nscr",
                    description: Some(
                        "FLASH non-secure control register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccr",
                    description: Some(
                        "FLASH secure control register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nsccr",
                    description: Some(
                        "FLASH non-secure clear control register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nsccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secccr",
                    description: Some(
                        "FLASH secure clear control register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfgr",
                    description: Some(
                        "FLASH privilege configuration register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nsobkcfgr",
                    description: Some(
                        "FLASH non-secure OBK configuration register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nsobkcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secobkcfgr",
                    description: Some(
                        "FLASH secure OBK configuration register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secobkcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdpextr",
                    description: Some(
                        "FLASH HDP extension register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdpextr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optsr_cur",
                    description: Some(
                        "FLASH option status register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optsr_prg",
                    description: Some(
                        "FLASH option status register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nsepochr_cur",
                    description: Some(
                        "FLASH non-secure EPOCH register",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nsepochr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secepochr_cur",
                    description: Some(
                        "FLASH secure EPOCH register",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secepochr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optsr2_cur",
                    description: Some(
                        "FLASH option status register 2",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optsr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "optsr2_prg",
                    description: Some(
                        "FLASH option status register 2",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Optsr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nsbootr_cur",
                    description: Some(
                        "FLASH non-secure boot register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nsbootr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nsbootr_prg",
                    description: Some(
                        "FLASH non-secure boot register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nsbootr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secbootr_cur",
                    description: Some(
                        "FLASH secure boot register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secbootr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bootr_prg",
                    description: Some(
                        "FLASH secure boot register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bootr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otpblr_cur",
                    description: Some(
                        "FLASH non-secure OTP block lock",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otpblr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otpblr_prg",
                    description: Some(
                        "FLASH non-secure OTP block lock",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otpblr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secbb1r1",
                    description: Some(
                        "FLASH secure block based register for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secbb1r2",
                    description: Some(
                        "FLASH secure block based register for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secbb1r3",
                    description: Some(
                        "FLASH secure block based register for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secbb1r4",
                    description: Some(
                        "FLASH secure block based register for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privbb1r1",
                    description: Some(
                        "FLASH privilege block based register for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privbb1r2",
                    description: Some(
                        "FLASH privilege block based register for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privbb1r3",
                    description: Some(
                        "FLASH privilege block based register for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privbb1r4",
                    description: Some(
                        "FLASH privilege block based register for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secwm1r_cur",
                    description: Some(
                        "FLASH security watermark for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secwm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secwm1r_prg",
                    description: Some(
                        "FLASH security watermark for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secwm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp1r_cur",
                    description: Some(
                        "FLASH write sector group protection for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp1r_prg",
                    description: Some(
                        "FLASH write sector group protection for Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edata1r_cur",
                    description: Some(
                        "FLASH data sector configuration Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Edata1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edata1r_prg",
                    description: Some(
                        "FLASH data sector configuration Bank 1",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Edata1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdp1r_cur",
                    description: Some(
                        "FLASH HDP Bank 1 configuration",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdp1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdp1r_prg",
                    description: Some(
                        "FLASH HDP Bank 1 configuration",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdp1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecccorr",
                    description: Some(
                        "FLASH ECC correction register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecccorr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccdetr",
                    description: Some(
                        "FLASH ECC detection register",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccdetr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccdr",
                    description: Some(
                        "FLASH ECC data",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secbb2r1",
                    description: Some(
                        "FLASH secure block-based register for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secbb2r2",
                    description: Some(
                        "FLASH secure block-based register for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secbb2r3",
                    description: Some(
                        "FLASH secure block-based register for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secbb2r4",
                    description: Some(
                        "FLASH secure block-based register for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privbb2r1",
                    description: Some(
                        "FLASH privilege block-based register for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privbb2r2",
                    description: Some(
                        "FLASH privilege block-based register for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privbb2r3",
                    description: Some(
                        "FLASH privilege block-based register for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privbb2r4",
                    description: Some(
                        "FLASH privilege block-based register for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privbb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secwm2r_cur",
                    description: Some(
                        "FLASH security watermark for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secwm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secwm2r_prg",
                    description: Some(
                        "FLASH security watermark for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secwm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp2r_cur",
                    description: Some(
                        "FLASH write sector group protection for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrp2r_prg",
                    description: Some(
                        "FLASH write sector group protection for Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edata2r_cur",
                    description: Some(
                        "FLASH data sectors configuration Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Edata2r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edata2r_prg",
                    description: Some(
                        "FLASH data sector configuration Bank 2",
                    ),
                    array: None,
                    byte_offset: 0x1f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Edata2r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdp2r_cur",
                    description: Some(
                        "FLASH HDP Bank 2 configuration",
                    ),
                    array: None,
                    byte_offset: 0x1f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdp2r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hdp2r_prg",
                    description: Some(
                        "FLASH HDP Bank 2 configuration",
                    ),
                    array: None,
                    byte_offset: 0x1fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hdp2r",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Acr",
            extends: None,
            description: Some(
                "FLASH access control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "latency",
                    description: Some(
                        "Read latency\r These bits are used to control the number of wait states used during read operations on both nonvolatile memory banks. The application software has to program them to the correct value depending on the embedded flash memory interface frequency and voltage conditions.\r ...\r Note: No check is performed by hardware to verify that the configuration is correct.",
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
                    name: "wrhighfreq",
                    description: Some(
                        "Flash signal delay \r These bits are used to control the delay between nonvolatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded flash memory interface frequency. Please refer to Table�44 for details.\r Note: No check is performed to verify that the configuration is correct. \r Note: Two WRHIGHFREQ values can be selected for some frequencies.",
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
                Field {
                    name: "prften",
                    description: Some(
                        "Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account.\r Bits used to control the prefetch.",
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
            ],
        },
        FieldSet {
            name: "Bootr",
            extends: None,
            description: Some(
                "FLASH secure boot register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secboot_lock",
                    description: Some(
                        "A field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "BootrSecbootLock",
                    ),
                },
                Field {
                    name: "secbootadd",
                    description: Some(
                        "Secure unique boot entry address.\r These bits allow configuring the secure UBE address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ecccorr",
            extends: None,
            description: Some(
                "FLASH ECC correction register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr_ecc",
                    description: Some(
                        "ECC error address\r When an ECC error occurs (for single correction) during a read operation, the ADDR_ECC contains the address that generated the error. \r ADDR_ECC is reset when the flag error is reset. \r The flash interface programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved.\r The address in ADDR_ECC is relative to the flash memory area where the error occurred (user flash memory, system flash memory, data area, read-only/OTP area).",
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
                    name: "obk_ecc",
                    description: Some(
                        "Single ECC error corrected in flash OB Keys storage area. It indicates the OBK storage concerned by ECC error.",
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
                    name: "edata_ecc",
                    description: Some(
                        "ECC fail for corrected ECC error in flash high-cycle data area\r It indicates if flash high-cycle data area is concerned by ECC error.",
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
                Field {
                    name: "bk_ecc",
                    description: Some(
                        "ECC fail bank for corrected ECC error\r It indicates which bank is concerned by ECC error",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sysf_ecc",
                    description: Some(
                        "ECC fail for corrected ECC error in system flash memory\r It indicates if system flash memory is concerned by ECC error.",
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
                    name: "otp_ecc",
                    description: Some(
                        "OTP ECC error bit\r This bit is set to 1 when one single ECC correction occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield.",
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
                Field {
                    name: "ecccie",
                    description: Some(
                        "ECC single correction error interrupt enable bit\r When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccc",
                    description: Some(
                        "ECC correction set by hardware when single ECC error has been detected and corrected.\r Cleared by writing 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eccdetr",
            extends: None,
            description: Some(
                "FLASH ECC detection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr_ecc",
                    description: Some(
                        "ECC error address\r When an ECC error occurs (double detection) during a read operation, the ADDR_ECC contains the address that generated the error. \r ADDR_ECC is reset when the flag error is reset. \r The flash interface programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an double ECC error is saved.\r The address in ADDR_ECC is relative to the flash memory area where the error occurred (user flash memory, system flash memory, data area, read-only/OTP area).",
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
                    name: "obk_ecc",
                    description: Some(
                        "ECC fail double ECC error in flash OB Keys storage area. It indicates the OBK storage concerned by ECC error.",
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
                    name: "edata_ecc",
                    description: Some(
                        "ECC fail double ECC error in flash high-cycle data area\r It indicates if flash high-cycle data area is concerned by ECC error.",
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
                Field {
                    name: "bk_ecc",
                    description: Some(
                        "ECC fail bank for double ECC error\r It indicates which bank is concerned by ECC error",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sysf_ecc",
                    description: Some(
                        "ECC fail for double ECC error in system flash memory\r It indicates if system flash memory is concerned by ECC error.",
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
                    name: "otp_ecc",
                    description: Some(
                        "OTP ECC error bit\r This bit is set to 1 when double ECC detection occurred during the last read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield.",
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
                Field {
                    name: "eccd",
                    description: Some(
                        "ECC detection\r Set by hardware when two ECC error has been detected.\r When this bit is set, a NMI is generated.\r Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors.",
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
        FieldSet {
            name: "Eccdr",
            extends: None,
            description: Some(
                "FLASH ECC data",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_ecc",
                    description: Some(
                        "ECC error data\r When an double detection ECC error occurs on special areas with 6-bit ECC on 16-bit data (data area, read-only/OTP area), the failing data is read to this register.\r By checking if it is possible to determine whether the failure was on a real data, or due to access to uninitialized memory.",
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
            name: "Edata1r",
            extends: None,
            description: Some(
                "FLASH data sector configuration Bank 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "edata1_strt",
                    description: Some(
                        "EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher.\r ...\r Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data",
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
                    name: "edata1_en",
                    description: Some(
                        "Bank 1 flash high-cycle data enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Edata2r",
            extends: None,
            description: Some(
                "FLASH data sector configuration Bank 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "edata2_strt",
                    description: Some(
                        "EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher.\r ...\r Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data.",
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
                    name: "edata2_en",
                    description: Some(
                        "Bank 2 flash high-cycle data enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hdp1r",
            extends: None,
            description: Some(
                "FLASH HDP Bank 1 configuration",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp1_strt",
                    description: Some(
                        "HDPL barrier start set in number of 8-Kbyte sectors",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hdp1_end",
                    description: Some(
                        "HDPL barrier end set in number of 8-Kbyte sectors",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hdp2r",
            extends: None,
            description: Some(
                "FLASH HDP Bank 2 configuration",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp2_strt",
                    description: Some(
                        "HDPL barrier start set in number of 8-Kbyte sectors",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hdp2_end",
                    description: Some(
                        "HDPL barrier end set in number of 8-Kbyte sectors",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hdpextr",
            extends: None,
            description: Some(
                "FLASH HDP extension register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hdp1_ext",
                    description: Some(
                        "HDP area extension in 8�Kbytes sectors in Bank1. Extension is added after the HDP1_END sector (included).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hdp2_ext",
                    description: Some(
                        "HDP area extension in 8�Kbytes sectors in bank 2. Extension is added after the HDP2_END sector (included).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nsbootr",
            extends: None,
            description: Some(
                "FLASH non-secure boot register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nsboot_lock",
                    description: Some(
                        "A field locking the values of SWAP_ BANK, and NSBOOTADD settings.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "NsbootrNsbootLock",
                    ),
                },
                Field {
                    name: "nsbootadd",
                    description: Some(
                        "Non secure unique boot entry address \r These bits allow configuring the Non secure BOOT address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nsccr",
            extends: None,
            description: Some(
                "FLASH non-secure clear control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clr_eop",
                    description: Some(
                        "EOP flag clear bit\r Setting this bit to 1 resets to 0 EOP flag in FLASH_NSSR register.",
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
                    name: "clr_wrperr",
                    description: Some(
                        "WRPERR flag clear bit\r Setting this bit to 1 resets to 0 WRPERR flag in FLASH_NSSR register.",
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
                    name: "clr_pgserr",
                    description: Some(
                        "PGSERR flag clear bit\r Setting this bit to 1 resets to 0 PGSERR flag in FLASH_NSSR register.",
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
                    name: "clr_strberr",
                    description: Some(
                        "STRBERR flag clear bit\r Setting this bit to 1 resets to 0 STRBERR flag in FLASH_NSSR register.",
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
                    name: "clr_incerr",
                    description: Some(
                        "INCERR flag clear bit\r Setting this bit to 1 resets to 0 INCERR flag in FLASH_NSSR register.",
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
                    name: "clr_obkerr",
                    description: Some(
                        "OBKERR flag clear bit.\r Setting this bit to 1 resets to 0 OBKERR flag in FLASH_NSSR register.",
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
                Field {
                    name: "clr_obkwerr",
                    description: Some(
                        "OBKWERR flag clear bit.\r Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_NSSR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clr_optchangeerr",
                    description: Some(
                        "Clear the flag corresponding flag in FLASH_NSSR by writing this bit.",
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
            ],
        },
        FieldSet {
            name: "Nscr",
            extends: None,
            description: Some(
                "FLASH non-secure control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock",
                    description: Some(
                        "configuration lock bit\r This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. \r LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change.",
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
                    name: "pg",
                    description: Some(
                        "programming control bit\r PG can be programmed only when LOCK is cleared to 0. \r PG allows programming in Bank1 and Bank2.",
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
                    name: "ser",
                    description: Some(
                        "sector erase request\r Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. \r If MER and SER are also set, a PGSERR is raised.",
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
                    name: "ber",
                    description: Some(
                        "erase request\r Setting BER bit to 1 requests a bank erase operation (user flash memory only). BER can be programmed only when LOCK is cleared to 0. \r If MER and SER are also set, a PGSERR is raised.\r Note: Write protection error is triggered when a bank erase is required and some sectors are protected.",
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
                    name: "fw",
                    description: Some(
                        "write forcing control bit\r FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. \r The embedded flash memory resets FW when the corresponding operation has been acknowledged. \r Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error.\r Write forcing is effective only if the write buffer is not empty and was filled by non-secure access (in particular, FW does not start several write operations when the force-write operations are performed consecutively). \r Since there is just one write buffer, FW can force a write in bank1 or bank2.",
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
                    name: "strt",
                    description: Some(
                        "erase start control bit\r STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. \r STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software.",
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
                    name: "snb",
                    description: Some(
                        "sector erase selection number\r These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0.\r ..",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mer",
                    description: Some(
                        "Mass erase request\r Setting MER bit to 1 requests a mass erase operation (user flash memory only). MER can be programmed only when LOCK is cleared to 0. \r If BER or SER are both set, a PGSERR is raised.\r Error is triggered when a mass erase is required and some sectors are protected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eopie",
                    description: Some(
                        "end of operation interrupt control bit\r Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0.",
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
                    name: "wrperrie",
                    description: Some(
                        "write protection error interrupt enable bit\r When this bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0.",
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
                    name: "pgserrie",
                    description: Some(
                        "programming sequence error interrupt enable bit\r When this bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0.",
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
                    name: "strberrie",
                    description: Some(
                        "strobe error interrupt enable bit\r When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0.",
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
                    name: "incerrie",
                    description: Some(
                        "inconsistency error interrupt enable bit\r When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0.",
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
                    name: "obkerrie",
                    description: Some(
                        "OBK general error interrupt enable bit\r OBKERRIE enables generating an interrupt in case of OBK specific access error. This bit can be programmed only when LOCK bit is cleared to 0.",
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
                Field {
                    name: "obkwerrie",
                    description: Some(
                        "OBK write error interrupt enable bit\r OBKWERRIE enables generation of interrupt in case of OBK specific write error. This bit can be programmed only when LOCK bit is cleared to 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optchangeerrie",
                    description: Some(
                        "Option byte change error interrupt enable bit\r This bit controls if an interrupt must be generated when an error occurs during an option byte change. It can be programmed only when LOCK bit is cleared to 0.",
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
                    name: "bksel",
                    description: Some(
                        "Bank selector bit\r BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "NscrBksel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Nsepochr",
            extends: None,
            description: Some(
                "FLASH non-secure EPOCH register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ns_epoch",
                    description: Some(
                        "Non-volatile non-secure EPOCH counter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nsobkcfgr",
            extends: None,
            description: Some(
                "FLASH non-secure OBK configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock",
                    description: Some(
                        "OBKCFGR lock option configuration bit\r This bit locks the FLASH_NSOBKCFGR register. The correct write sequence to FLASH_NSOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change.",
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
                    name: "swap_sect_req",
                    description: Some(
                        "OBK swap sector request bit\r When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one.\r The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPL\u{a0}≠\u{a0}0. Minimum value is 16 for OBK-HDPL\u{a0}=\u{a0}1, 144 for OBK-HDPL\u{a0}=\u{a0}2 and\u{a0}192 for OBK-HDPL\u{a0}=\u{a0}3.",
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
                    name: "alt_sect",
                    description: Some(
                        "alternate sector bit\r This bit must not change while filling the write buffer, otherwise an error (OBKERR) is generated",
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
                    name: "alt_sect_erase",
                    description: Some(
                        "alternate sector erase bit\r When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as BUSY bit.",
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
                    name: "swap_offset",
                    description: Some(
                        "Key index (offset /16 bits) pointing for next swap.\r 0x01 means that only the first OBK data (128 bits) is copied from current to alternate OBK sector\r 0x02 means that the two first OBK data is copied …\r …",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nssr",
            extends: None,
            description: Some(
                "FLASH non-secure status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bsy",
                    description: Some(
                        "busy flag\r BSY flag indicates that a flash memory is busy by an operation (write, erase, option byte change, OBK operation). It is set at the beginning of a flash memory operation and cleared when the operation finishes, or an error occurs.",
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
                    name: "wbne",
                    description: Some(
                        "write buffer not empty flag \r WBNE flag is set when the flash interface is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below:\r the application software forces the write operation using FW bit in FLASH_NSCR\r the embedded flash memory detects an error that involves data loss\r This bit cannot be reset by software writing 0 directly. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data.",
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
                    name: "dbne",
                    description: Some(
                        "data buffer not empty flag \r DBNE flag is set when the flash interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free.",
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
                    name: "eop",
                    description: Some(
                        "end of operation flag\r EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to 1. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_NSCCR register.",
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
                    name: "wrperr",
                    description: Some(
                        "write protection error flag\r WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_NSCCR register clears WRPERR.",
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
                    name: "pgserr",
                    description: Some(
                        "programming sequence error flag\r PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_NSCCR register clears PGSERR.",
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
                    name: "strberr",
                    description: Some(
                        "strobe error flag \r STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_NSCCR register clears STRBERR.",
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
                    name: "incerr",
                    description: Some(
                        "inconsistency error flag\r NSINCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_NSCCR register clears NSINCERR.",
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
                    name: "obkerr",
                    description: Some(
                        "OBK general error flag\r OBKERR flag is raised when the OBK-HDPL signal from the SBS does not match the HDPL value associated with the key slot during access to the key location. Alternatively also when the ALT_SECT is unexpectedly changed while the write buffer is being filled.",
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
                Field {
                    name: "obkwerr",
                    description: Some(
                        "OBK write error flag\r OBKWERR flag is raised when the address is not virgin on a write access to the OBK storage. Alternatively also when the OBK selector in the alternate sector is not virgin during a swap operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "optchangeerr",
                    description: Some(
                        "Option byte change error flag \r OPTCHANGEERR flag indicates that an error occurred during an option byte change operation. When OPTCHANGEERR is set to 1, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTCHANGEERRIE bit of FLASH_NSCR register is set to 1. \r Writing 1 to CLR_OPTCHANGEERR of register FLASH_NSCCR clears OPTCHANGEERR.\r Note: The OPTSTRT bit in FLASH_OPTCR cannot be set while OPTCHANGEERR is set.",
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
            ],
        },
        FieldSet {
            name: "Opsr",
            extends: None,
            description: Some(
                "FLASH operation status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr_op",
                    description: Some(
                        "Interrupted operation address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "data_op",
                    description: Some(
                        "Flash high-cycle data area operation interrupted\r It indicates if flash high-cycle data area is concerned by operation.",
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
                Field {
                    name: "bk_op",
                    description: Some(
                        "Interrupted operation bank\r It indicates which bank was concerned by operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sysf_op",
                    description: Some(
                        "Operation in system flash memory interrupted \r Indicates that reset interrupted an ongoing operation in system flash.",
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
                    name: "otp_op",
                    description: Some(
                        "OTP operation interrupted\r Indicates that reset interrupted an ongoing operation in OTP area (or OBKeys area).",
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
                Field {
                    name: "code_op",
                    description: Some(
                        "Flash memory operation code",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "CodeOp",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Optcr",
            extends: None,
            description: Some(
                "FLASH option control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "optlock",
                    description: Some(
                        "FLASH_OPTCR lock option configuration bit\r The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. \r It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change.",
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
                    name: "optstrt",
                    description: Some(
                        "Option byte start change option configuration bit\r OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It is set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It is reseted at the same time as BSY bit.\r The user application cannot modify any FLASH_XXX_PRG flash interface register until the option change operation has been completed.\r Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in nonvolatile memory.",
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
                    name: "swap_bank",
                    description: Some(
                        "Bank swapping option configuration bit\r SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptcrSwapBank",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Optsr",
            extends: None,
            description: Some(
                "FLASH option status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bor_lev",
                    description: Some(
                        "Brownout level option status bit\r These bits reflects the power level that generates a system reset.\r 00 or 11: BOR Level 1, the threshold level is low (around 2.1�V)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "OptsrBorLev",
                    ),
                },
                Field {
                    name: "borh_en",
                    description: Some(
                        "Brownout high enable",
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
                    name: "iwdg_sw",
                    description: Some(
                        "IWDG control mode option status bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrIwdgSw",
                    ),
                },
                Field {
                    name: "wwdg_sw",
                    description: Some(
                        "WWDG control mode option status bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrWwdgSw",
                    ),
                },
                Field {
                    name: "nrst_stop",
                    description: Some(
                        "Core domain Stop entry reset option status bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrNrstStop",
                    ),
                },
                Field {
                    name: "nrst_stdby",
                    description: Some(
                        "Core domain Standby entry reset option status bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrNrstStdby",
                    ),
                },
                Field {
                    name: "product_state",
                    description: Some(
                        "Life state code (based on Hamming 8,4). More information in Section�7.6.11: Product state transitions.",
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
                    name: "io_vdd_hslv",
                    description: Some(
                        "High-speed IO at low V<sub>DD</sub> voltage configuration bit.\r This bit can be set only with V<sub>DD</sub> below 2.7�V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrIoVddHslv",
                    ),
                },
                Field {
                    name: "io_vddio2_hslv",
                    description: Some(
                        "High-speed IO at low V<sub>DDIO2</sub> voltage configuration bit.\r This bit can be set only with V<sub>DDIO2</sub> below 2.7�V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrIoVddioHslv",
                    ),
                },
                Field {
                    name: "iwdg_stop",
                    description: Some(
                        "IWDG Stop mode freeze option status bit\r When set the independent watchdog IWDG is in system Stop mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrIwdgStop",
                    ),
                },
                Field {
                    name: "iwdg_stdby",
                    description: Some(
                        "IWDG Standby mode freeze option status bit\r When set the independent watchdog IWDG is frozen in system Standby mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrIwdgStdby",
                    ),
                },
                Field {
                    name: "boot_ube",
                    description: Some(
                        "Available only on cryptography enabled devices.\r Unique boot entry control, selects either ST or OEM iRoT for secure boot.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "OptsrBootUbe",
                    ),
                },
                Field {
                    name: "swap_bank",
                    description: Some(
                        "Bank swapping option status bit\r SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. \r SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrSwapBank",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Optsr2",
            extends: None,
            description: Some(
                "FLASH option status register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sram13_rst",
                    description: Some(
                        "SRAM1 and SRAM3 erase upon system reset",
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
                    name: "sram2_rst",
                    description: Some(
                        "SRAM2 erase when system reset",
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
                    name: "bkpram_ecc",
                    description: Some(
                        "Backup RAM ECC detection and correction disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrBkpramEcc",
                    ),
                },
                Field {
                    name: "sram3_ecc",
                    description: Some(
                        "SRAM3 ECC detection and correction disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrSramEcc",
                    ),
                },
                Field {
                    name: "sram2_ecc",
                    description: Some(
                        "SRAM2 ECC detection and correction disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "OptsrSramEcc",
                    ),
                },
                Field {
                    name: "usbpd_dis",
                    description: Some(
                        "USB power delivery configuration option bit",
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
                    name: "tzen",
                    description: Some(
                        "TrustZone enable configuration bits\r This bit enables the device is in TrustZone mode during an option byte change.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "OptsrTzen",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Otpblr",
            extends: None,
            description: Some(
                "FLASH non-secure OTP block lock",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lockbl",
                    description: Some(
                        "OTP block lock \r Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31.\r LOCKBL[n] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR.\r LOCKBL[n] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked.\r When one block is locked, it’s not possible to remove the write protection.\r Also if not locked, it is not possible to erase OTP words.",
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
            name: "Privbb",
            extends: None,
            description: Some(
                "FLASH privilege block-based register for Bank 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "privbb",
                    description: Some(
                        "Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "PrivbbrPrivbb",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Privcfgr",
            extends: None,
            description: Some(
                "FLASH privilege configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spriv",
                    description: Some(
                        "privilege attribute for secure registers",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Spriv",
                    ),
                },
                Field {
                    name: "nspriv",
                    description: Some(
                        "privilege attribute for non secure registers",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Nspriv",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Secbb",
            extends: None,
            description: Some(
                "FLASH secure block-based register for Bank 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secbb",
                    description: Some(
                        "Secure/non-secure flash Bank 2 sector attribute",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: Some(
                        "SecbbrSecbb",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Secbootr",
            extends: None,
            description: Some(
                "FLASH secure boot register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secboot_lock",
                    description: Some(
                        "A field locking the values of UBE, SWAP_BANK, and SECBOOTADD settings.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "SecbootrSecbootLock",
                    ),
                },
                Field {
                    name: "secbootadd",
                    description: Some(
                        "Unique boot entry secure address \r These bits reflect the Secure UBE address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Secccr",
            extends: None,
            description: Some(
                "FLASH secure clear control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clr_eop",
                    description: Some(
                        "EOP flag clear bit\r Setting this bit to 1 resets to 0 EOP flag in FLASH_SECSR register.",
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
                    name: "clr_wrperr",
                    description: Some(
                        "WRPERR flag clear bit\r Setting this bit to 1 resets to 0 WRPERR flag in FLASH_SECSR register.",
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
                    name: "clr_pgserr",
                    description: Some(
                        "PGSERR flag clear bit\r Setting this bit to 1 resets to 0 PGSERR flag in FLASH_SECSR register.",
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
                    name: "clr_strberr",
                    description: Some(
                        "STRBERR flag clear bit\r Setting this bit to 1 resets to 0 STRBERR flag in FLASH_SECSR register.",
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
                    name: "clr_incerr",
                    description: Some(
                        "INCERR flag clear bit\r Setting this bit to 1 resets to 0 INCERR flag in FLASH_SECSR register.",
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
                    name: "clr_obkerr",
                    description: Some(
                        "OBKWERR flag clear bit\r Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_SECSR register.",
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
                Field {
                    name: "clr_obkwerr",
                    description: Some(
                        "OBKWERR flag clear bit\r Setting this bit to 1 resets to 0 OBKWERR flag in FLASH_SECSR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Seccr",
            extends: None,
            description: Some(
                "FLASH secure control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock",
                    description: Some(
                        "configuration lock bit \r This bit locks the FLASH_SECCR register. The correct write sequence to FLASH_SECKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. \r LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_SECCR register do not change.",
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
                    name: "pg",
                    description: Some(
                        "programming control bit\r PG can be programmed only when LOCK is cleared to 0. \r PG allows programming in Bank1 and Bank2.",
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
                    name: "ser",
                    description: Some(
                        "sector erase request\r Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. \r If BER and MER are also set, a PGSERR is raised.",
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
                    name: "ber",
                    description: Some(
                        "erase request \r Setting BER bit to 1 requests a bank erase operation (user flash memory only). BER can be programmed only when LOCK is cleared to 0. \r If MER and SER are also set, a PGSERR is raised.\r Note: Write protection error is triggered when a bank erase is required and some sectors are protected.",
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
                    name: "fw",
                    description: Some(
                        "write forcing control bit\r FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. \r The embedded flash memory resets FW when the corresponding operation has been acknowledged. \r Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error.\r Write forcing is effective only if the write buffer is not empty and was filled by secure access (in particular, FW does not start several write operations when the force-write operations are performed consecutively). \r Since there is just one write buffer, FW can force a write in bank1 or bank2.",
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
                    name: "strt",
                    description: Some(
                        "erase start control bit\r STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. \r STRT is reseted at the end of the operation or when an error occurs. It cannot be reset by software.",
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
                    name: "snb",
                    description: Some(
                        "sector erase selection number\r These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0.\r ..",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mer",
                    description: Some(
                        "mass erase request\r Setting MER bit to 1 requests a mass erase operation (user flash memory only). MER can be programmed only when LOCK is cleared to 0. \r If BER or SER are also set, a PGSERR is raised.\r Error is triggered when a mass erase is required and some sectors are protected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eopie",
                    description: Some(
                        "end of operation interrupt control bit\r Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program/erase operation. EOPIE can be programmed only when LOCK is cleared to 0.",
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
                    name: "wrperrie",
                    description: Some(
                        "write protection error interrupt enable bit\r When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0.",
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
                    name: "pgserrie",
                    description: Some(
                        "programming sequence error interrupt enable bit\r When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0.",
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
                    name: "strberrie",
                    description: Some(
                        "strobe error interrupt enable bit\r When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0.",
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
                    name: "incerrie",
                    description: Some(
                        "inconsistency error interrupt enable bit\r When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0.",
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
                    name: "obkerrie",
                    description: Some(
                        "OBK general error interrupt enable bit\r OBKERRIE enables generating an interrupt in case of OBK specific access error. OBKERRIE can be programmed only when LOCK is cleared to 0.",
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
                Field {
                    name: "obkwerrie",
                    description: Some(
                        "OBK write error interrupt enable bit\r OBKWERRIE enables generation of interrupt in case of OBK specific write error. OBKWERRIE can be programmed only when LOCK is cleared to 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inv",
                    description: Some(
                        "Flash memory security state invert.\r This bit inverts the flash memory security state.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bksel",
                    description: Some(
                        "bank selector bit\r BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "SeccrBksel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Secepochr",
            extends: None,
            description: Some(
                "FLASH secure EPOCH register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sec_epoch",
                    description: Some(
                        "Non-volatile secure EPOCH counter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Secobkcfgr",
            extends: None,
            description: Some(
                "FLASH secure OBK configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock",
                    description: Some(
                        "OBKCFGR lock option configuration bit\r This bit locks the FLASH_OBKCFGR register. The correct write sequence to FLASH_SECOBKKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_SECOBKKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change.",
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
                    name: "swap_sect_req",
                    description: Some(
                        "OBK swap sector request bit\r When set, all the OBKs which have not been updated in the alternate sector is copied from current sector to alternate one.\r The SWAP_OFFSET value must be a certain minimum value in order for the swap to be launched in OBK-HDPL\u{a0}≠\u{a0}0. Minimum value is 16 for OBK-HDPL\u{a0}=\u{a0}1, 144 for OBK-HDPL\u{a0}=\u{a0}2 and 192 for OBK-HDPL\u{a0}=\u{a0}3.",
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
                    name: "alt_sect",
                    description: Some(
                        "alternate sector bit\r This bit must not change while filling the write buffer, otherwise an error is generated",
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
                    name: "alt_sect_erase",
                    description: Some(
                        "alternate sector erase bit\r When ALT_SECT bit is set, use this bit to generate an erase command for the OBK alternate sector. It is set only by Software and cleared when the OBK swap operation is completed or an error occurs (PGSERR). It is reseted at the same time as the BUSY bit.",
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
                    name: "swap_offset",
                    description: Some(
                        "key index (offset /16 bits) pointing for next swap.\r …",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Secsr",
            extends: None,
            description: Some(
                "FLASH secure status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bsy",
                    description: Some(
                        "busy flag\r BSY flag indicates that a FLASH memory is busy (write, erase, option byte change, OBK operations). It is set at the beginning of a flash memory operation and cleared when the operation finishes or an error occurs.",
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
                    name: "wbne",
                    description: Some(
                        "write buffer not empty flag \r WBNE flag is set when the flash interface is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below:\r the application software forces the write operation using FW bit in FLASH_SECCR\r the flash interface detects an error that involves data loss\r This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data.",
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
                    name: "dbne",
                    description: Some(
                        "data buffer not empty flag \r DBNE flag is set when the embedded flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free.",
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
                    name: "eop",
                    description: Some(
                        "end of operation flag\r EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register.",
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
                    name: "wrperr",
                    description: Some(
                        "write protection error flag\r WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR.",
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
                    name: "pgserr",
                    description: Some(
                        "programming sequence error flag\r PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR.",
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
                    name: "strberr",
                    description: Some(
                        "strobe error flag \r STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR.",
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
                    name: "incerr",
                    description: Some(
                        "inconsistency error flag\r INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR.",
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
                    name: "obkerr",
                    description: Some(
                        "OBK general error flag\r OBKERR flag is raised when the OBK-HDPL signal from the SBS does not match the HDPL value associated with the key slot during access to the key location. Alternatively also when the ALT_SECT is unexpectedly changed while the write buffer is being filled.",
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
                Field {
                    name: "obkwerr",
                    description: Some(
                        "OBK write error flag\r OBKWERR flag is raised when the address is not virgin on a write access to the OBK storage. Alternatively also when the OBK selector in the alternate sector is not virgin during a swap operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Secwm",
            extends: None,
            description: Some(
                "FLASH security watermark for Bank 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secwmstrt",
                    description: Some(
                        "Bank2 security WM area start sector",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "secwmend",
                    description: Some(
                        "Bank2 security WM end sector",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrp",
            extends: None,
            description: Some(
                "FLASH write sector group protection for Bank 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrpsg",
                    description: Some(
                        "Bank1 sector group protection option status byte\r Setting WRPSG1 bits to 0 write protects the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected)\r Bit 0: Group embedding sectors 0 to 3\r Bit 1: Group embedding sectors 4 to 7\r Bit N: Group embedding sectors 4 x N to 4 x N + 3\r Bit 31: Group embedding sectors 124 to 127",
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
    ],
    enums: &[
        Enum {
            name: "BootrSecbootLock",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B_0X_B4",
                    description: Some(
                        "The BOOT_UBE and SECBOOTADD are frozen. SWAP_BANK can only be modified with TZEN set to 0xC3 (disabled).",
                    ),
                    value: 180,
                },
                EnumVariant {
                    name: "B_0X_C3",
                    description: Some(
                        "The BOOT_UBE, SWAP_ BANK and SECBOOTADD can still be modified following their individual rules.",
                    ),
                    value: 195,
                },
            ],
        },
        Enum {
            name: "CodeOp",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No flash operation on going during previous reset",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Single write operation interrupted",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "OBK alternate sector erase",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "Sector erase operation interrupted",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "Bank erase operation interrupted",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "Mass erase operation interrupted",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "B_0X6",
                    description: Some(
                        "Option change operation interrupted",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "B_0X7",
                    description: Some(
                        "OBK swap sector request",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "NsbootrNsbootLock",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B_0X_B4",
                    description: Some(
                        "The NSBOOTADD is frozen. SWAP_ BANK can only be modified with TZEN set to 0xB4 (enabled).",
                    ),
                    value: 180,
                },
                EnumVariant {
                    name: "B_0X_C3",
                    description: Some(
                        "The SWAP_ BANK and NSBOOTADD can still be modified following their individual rules.",
                    ),
                    value: 195,
                },
            ],
        },
        Enum {
            name: "NscrBksel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bank1 is selected for Bank erase / sector erase / interrupt enable",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bank2 is selected for BER / SER",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Nspriv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "access to non secure registers is always granted",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "access to non secure registers is denied in case of unprivileged access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptcrSwapBank",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bank1 and Bank2 not swapped",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bank1 and Bank2 swapped",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrBkpramEcc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "BKPRAM ECC check enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "BKPRAM ECC check disabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrBootUbe",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B_0X_B4",
                    description: Some(
                        "OEM-iRoT (user flash) selected. In Open PRODUCT_STATE this value selects bootloader. Defaut value.",
                    ),
                    value: 180,
                },
                EnumVariant {
                    name: "B_0X_C3",
                    description: Some(
                        "ST-iRoT (system flash) selected",
                    ),
                    value: 195,
                },
            ],
        },
        Enum {
            name: "OptsrBorLev",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "BOR Level 2, the threshold level is medium (around 2.4 V)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "BOR Level 3, the threshold level is high (around 2.7 V)",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "OptsrIoVddHslv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "High-speed IO at low V<sub>DD</sub> voltage feature disabled (V<sub>DD</sub> can exceed 2.7�V)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "High-speed IO at low V<sub>DD</sub> voltage feature enabled (V<sub>DD</sub> remains below 2.7�V)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrIoVddioHslv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "High-speed IO at low V<sub>DDIO2</sub> voltage feature disabled (V<sub>DDIO2</sub> can exceed 2.7�V)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "High-speed IO at low V<sub>DDIO2</sub> voltage feature enabled (V<sub>DDIO2</sub> remains below 2.7�V)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrIwdgStdby",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Independent watchdog frozen in Standby mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Independent watchdog keep running in Standby mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrIwdgStop",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Independent watchdog frozen in system Stop mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Independent watchdog keep running in system Stop mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrIwdgSw",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "IWDG watchdog is controlled by hardware",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "IWDG watchdog is controlled by software",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrNrstStdby",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "a reset is generated when entering Standby mode on core domain",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "no reset generated when entering Standby mode on core domain.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrNrstStop",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "a reset is generated when entering Stop mode on core domain",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "no reset generated when entering Stop mode on core domain.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrSramEcc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "SRAM2 ECC check enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "SRAM2 ECC check disabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrSwapBank",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bank1 and Bank2 not swapped",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bank1 and Bank2 swapped",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "OptsrTzen",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B_0X_B4",
                    description: Some(
                        "TrustZone enabled.",
                    ),
                    value: 180,
                },
                EnumVariant {
                    name: "B_0X_C3",
                    description: Some(
                        "TrustZone disabled",
                    ),
                    value: 195,
                },
            ],
        },
        Enum {
            name: "OptsrWwdgSw",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "WWDG watchdog is controlled by hardware",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "WWDG watchdog is controlled by software",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "PrivbbrPrivbb",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "sectors (32 * (x-1) + y) in bank 2 is unprivileged",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "sector (32 * (x-1) + y) in bank 2 is privileged",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "SecbbrSecbb",
            description: None,
            bit_size: 32,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "sectors (32 * (x-1) + y) in bank 2 is non secure",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "sector (32 * (x-1) + y) in bank 2 is secure",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "SecbootrSecbootLock",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "B_0X_B4",
                    description: Some(
                        "The BOOT_UBE and SECBOOTADD are frozen. SWAP_ BANK can only be modified with TZEN set to 0xC3 (disabled).",
                    ),
                    value: 180,
                },
                EnumVariant {
                    name: "B_0X_C3",
                    description: Some(
                        "The BOOT_UBE, SWAP_BANK and SECBOOTADD can still be modified following their individual rules.",
                    ),
                    value: 195,
                },
            ],
        },
        Enum {
            name: "SeccrBksel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Bank1 is selected for Bank erase / sector erase / interrupt enable",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Bank2 is selected for BER / SER",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spriv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "access to secure registers is always granted",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "access to secure registers is denied in case of unprivileged access.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
