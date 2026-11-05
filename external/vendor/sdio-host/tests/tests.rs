use sdio_host::sd::{BusWidth, CID, CSD, CurrentConsumption, OCR, SD, SDSpecVersion, SDStatus, SCR};

struct TestCard {
    cid: [u32; 4],
    cidr: CidRes,
    csd: [u32; 4],
    csdr: CsdRes,
    ocr: u32,
    ocrr: OcrRes,
    status: [u32; 16],
    statusr: StatusRes,
    scr: [u32; 2],
    scrr: ScrRes,
}

struct CidRes {
    mid: u8,
    serial: u32,
    name: &'static str,
    oem: &'static str,
    revision: u8,
    m_month: u8,
    m_year: u16,
}

struct CsdRes {
    version: u8,
    transfer_rate: u8,
    blocks: u64,
    size_bytes: u64,
    read_current_minimum_vdd: CurrentConsumption,
    write_current_minimum_vdd: CurrentConsumption,
    read_current_maximum_vdd: CurrentConsumption,
    write_current_maximum_vdd: CurrentConsumption,
    erase_size_blocks: u32,
}

struct OcrRes {
    voltage_window_mv: (u16, u16),
    v18_allowed: bool,
    over_2tb: bool,
    uhs2_card_status: bool,
    high_capacity: bool,
    powered: bool,
}

struct StatusRes {
    bus_width: BusWidth,
    secure_mode: bool,
    sd_card_type: u16,
    protected_area_size: u32,
    speed_class: u8,
    video_speed_class: u8,
    app_perf_class: u8,
    move_performance: u8,
    allocation_unit_size: u8,
    erase_size: u16,
    erase_timeout: u8,
    discard_support: bool,
}

struct ScrRes {
    bus_widths: u8,

    version: SDSpecVersion,
}

static CARDS: &[TestCard] = &[
    // Panasonic 8 Gb Class 4
    TestCard {
        cid: [4093715758, 333095359, 808993095, 22036825],
        cidr: CidRes {
            mid: 1,
            serial: 3668033524,
            name: "Y08AG",
            oem: "PA",
            revision: 19,
            m_month: 5,
            m_year: 2018,
        },
        csd: [171966712, 968064896, 1532559360, 1074659378],
        csdr: CsdRes {
            version: 1,
            transfer_rate: 50,
            blocks: 15126528,
            size_bytes: 7744782336,
            read_current_minimum_vdd: CurrentConsumption::I_100mA,
            write_current_minimum_vdd: CurrentConsumption::I_1mA,
            read_current_maximum_vdd: CurrentConsumption::I_45mA,
            write_current_maximum_vdd: CurrentConsumption::I_35mA,
            erase_size_blocks: 1,
        },
        ocr: 3237969920,
        ocrr: OcrRes {
            voltage_window_mv: (2700, 3600),
            v18_allowed: false,
            over_2tb: false,
            uhs2_card_status: false,
            high_capacity: true,
            powered: true,
        },

        status: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134676480, 33722368, 50331648, 2147483648,
        ],
        statusr: StatusRes {
            bus_width: BusWidth::Four,
            secure_mode: false,
            sd_card_type: 0,
            protected_area_size: 50331648,
            speed_class: 2, // Class 4
            video_speed_class: 0,
            app_perf_class: 0,
            move_performance: 2, // MB/s
            allocation_unit_size: 9,
            erase_size: 8,
            erase_timeout: 1,
            discard_support: false,
        },
        scr: [16777216, 37060608],
        scrr: ScrRes {
            bus_widths: 5,
            version: SDSpecVersion::V3,
        },
    },
    // Sandisk 8 Gb Class 4
    TestCard {
        cid: [2197869198, 2149469225, 1429223495, 55788627],
        cidr: CidRes {
            mid: 3,
            serial: 508307843,
            name: "SU08G",
            oem: "SD",
            revision: 128,
            m_month: 2,
            m_year: 2013,
        },
        csd: [171983022, 993492864, 1532559360, 1074659378],
        csdr: CsdRes {
            version: 1,
            transfer_rate: 50,
            size_bytes: 7948206080,
            blocks: 15523840,
            read_current_minimum_vdd: CurrentConsumption::I_100mA,
            write_current_minimum_vdd: CurrentConsumption::I_10mA,
            read_current_maximum_vdd: CurrentConsumption::I_5mA,
            write_current_maximum_vdd: CurrentConsumption::I_45mA,
            erase_size_blocks: 1,
        },

        ocr: 3237969920,
        ocrr: OcrRes {
            voltage_window_mv: (2700, 3600),
            v18_allowed: false,
            over_2tb: false,
            uhs2_card_status: false,
            high_capacity: true,
            powered: true,
        },

        status: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 184877056, 33722368, 50331648, 2147483648,
        ],
        statusr: StatusRes {
            bus_width: BusWidth::Four,
            secure_mode: false,
            sd_card_type: 0,
            protected_area_size: 50331648,
            speed_class: 2, // Class 4
            video_speed_class: 0,
            app_perf_class: 0,
            move_performance: 2, // MB/s
            allocation_unit_size: 9,
            erase_size: 11,
            erase_timeout: 1,
            discard_support: false,
        },

        scr: [0, 37060609],
        scrr: ScrRes {
            bus_widths: 5,
            version: SDSpecVersion::V3,
        },
    },
    // Sandisk extreme 32Gb Class 10
    TestCard {
        cid: [0xc000e344, 0x80f1086b, 0x45333247, 0x03534453],
        cidr: CidRes {
            mid: 3,
            serial: 4043860928,
            name: "SE32G",
            oem: "SD",
            revision: 128,
            m_month: 3,
            m_year: 2014,
        },
        csd: [0x0a4040c2, 0xedc87f80, 0x5b590000, 0x400e0032],
        csdr: CsdRes {
            version: 1,
            transfer_rate: 50,
            size_bytes: 31914983424,
            blocks: 62333952,
            read_current_minimum_vdd: CurrentConsumption::I_35mA,
            write_current_minimum_vdd: CurrentConsumption::I_35mA,
            read_current_maximum_vdd: CurrentConsumption::I_80mA,
            write_current_maximum_vdd: CurrentConsumption::I_10mA,
            erase_size_blocks: 1,
        },

        ocr: 3254747136,
        ocrr: OcrRes {
            voltage_window_mv: (2700, 3600),
            v18_allowed: true,
            over_2tb: false,
            uhs2_card_status: false,
            high_capacity: true,
            powered: true,
        },

        status: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 251992576, 67145728, 83886080, 2147483648,
        ],
        statusr: StatusRes {
            bus_width: BusWidth::Four,
            secure_mode: false,
            sd_card_type: 0,
            protected_area_size: 83886080,
            speed_class: 4, // Class 10
            video_speed_class: 0,
            app_perf_class: 0,
            move_performance: 0, // Ignore for class 10
            allocation_unit_size: 9,
            erase_size: 15,
            erase_timeout: 1,
            discard_support: false,
        },

        scr: [0x00000000, 0x02358001],
        scrr: ScrRes {
            bus_widths: 5,
            version: SDSpecVersion::V3,
        },
    },
];

#[test]
fn test_cid() {
    for card in CARDS {
        let cid: CID<SD> = card.cid.into();
        println!("{:?}", cid);

        assert_eq!(cid.serial(), card.cidr.serial);
        assert_eq!(cid.manufacturer_id(), card.cidr.mid);
        assert_eq!(cid.product_revision(), card.cidr.revision);

        assert_eq!(cid.product_name(), card.cidr.name);
        assert_eq!(cid.oem_id(), card.cidr.oem);

        assert_eq!(cid.manufacturing_date().0, card.cidr.m_month);
        assert_eq!(cid.manufacturing_date().1, card.cidr.m_year);
    }
}

#[test]
fn test_csd() {
    for card in CARDS {
        let csd: CSD<SD> = card.csd.into();
        println!("{:?}", csd);

        assert_eq!(csd.version(), card.csdr.version);
        assert_eq!(csd.transfer_rate(), card.csdr.transfer_rate);

        assert_eq!(csd.block_count(), card.csdr.blocks);
        assert_eq!(csd.card_size(), card.csdr.size_bytes);

        assert_eq!(
            csd.read_current_minimum_vdd(),
            card.csdr.read_current_minimum_vdd
        );
        assert_eq!(
            csd.write_current_minimum_vdd(),
            card.csdr.write_current_minimum_vdd
        );
        assert_eq!(
            csd.read_current_maximum_vdd(),
            card.csdr.read_current_maximum_vdd
        );
        assert_eq!(
            csd.write_current_maximum_vdd(),
            card.csdr.write_current_maximum_vdd
        );
        assert_eq!(csd.erase_size_blocks(), card.csdr.erase_size_blocks);
    }
}

#[test]
fn test_ocr() {
    for card in CARDS {
        let ocr: OCR<SD> = card.ocr.into();
        println!("{:?}", ocr);

        assert_eq!(
            ocr.voltage_window_mv().unwrap(),
            card.ocrr.voltage_window_mv
        );
        assert_eq!(ocr.v18_allowed(), card.ocrr.v18_allowed);
        assert_eq!(ocr.over_2tb(), card.ocrr.over_2tb);
        assert_eq!(ocr.uhs2_card_status(), card.ocrr.uhs2_card_status);
        assert_eq!(ocr.high_capacity(), card.ocrr.high_capacity);
        assert_eq!(ocr.is_busy(), !card.ocrr.powered);
    }
}

#[test]
fn test_sdstatus() {
    for card in CARDS {
        let status: SDStatus = card.status.into();
        println!("{:?}", status);

        let r = &card.statusr;
        assert_eq!(status.bus_width(), r.bus_width);
        assert_eq!(status.secure_mode(), r.secure_mode);
        assert_eq!(status.sd_memory_card_type(), r.sd_card_type);
        assert_eq!(status.protected_area_size(), r.protected_area_size);
        assert_eq!(status.speed_class(), r.speed_class);
        assert_eq!(status.video_speed_class(), r.video_speed_class);
        assert_eq!(status.app_perf_class(), r.app_perf_class);
        assert_eq!(status.move_performance(), r.move_performance);
        assert_eq!(status.allocation_unit_size(), r.allocation_unit_size);
        assert_eq!(status.erase_size(), r.erase_size);
        assert_eq!(status.erase_timeout(), r.erase_timeout);
        assert_eq!(status.discard_support(), r.discard_support);
    }
}

#[test]
fn test_scr() {
    for card in CARDS {
        let scr: SCR = card.scr.into();
        println!("{:?}", scr);

        let r = &card.scrr;
        assert_eq!(scr.bus_widths(), r.bus_widths);
        assert_eq!(scr.version(), r.version);
    }
}
