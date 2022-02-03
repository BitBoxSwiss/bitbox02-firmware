use crc::*;

const INIT: &[u8] = b"123456789";

#[test]
fn crc_8() {
    let algs = &[CRC_8_AUTOSAR, CRC_8_BLUETOOTH, CRC_8_SMBUS, CRC_8_DARC];
    for alg in algs {
        let crc = Crc::<u8>::new(alg);
        assert_eq!(alg.check, crc.checksum(INIT));
        let mut digest = crc.digest();
        digest.update(INIT);
        assert_eq!(alg.check, digest.finalize());
    }
}

#[test]
fn crc_16() {
    let algs = &[
        CRC_16_IBM_SDLC,
        CRC_16_USB,
        CRC_16_ARC,
        CRC_16_CDMA2000,
        CRC_16_IBM_3740,
        CRC_16_IBM_SDLC,
        CRC_16_KERMIT,
    ];
    for alg in algs {
        let crc = Crc::<u16>::new(alg);
        assert_eq!(alg.check, crc.checksum(INIT));
        let mut digest = crc.digest();
        digest.update(INIT);
        assert_eq!(alg.check, digest.finalize());
    }
}

#[test]
fn crc_32() {
    let algs = &[
        CRC_32_ISCSI,
        CRC_32_AUTOSAR,
        CRC_32_BZIP2,
        CRC_32_ISCSI,
        CRC_32_ISO_HDLC,
    ];
    for alg in algs {
        let crc = Crc::<u32>::new(alg);
        assert_eq!(alg.check, crc.checksum(INIT));
        let mut digest = crc.digest();
        digest.update(INIT);
        assert_eq!(alg.check, digest.finalize());
    }
}

#[test]
fn crc_64() {
    let algs = &[CRC_64_ECMA_182, CRC_64_GO_ISO, CRC_64_WE, CRC_64_XZ];
    for alg in algs {
        let crc = Crc::<u64>::new(alg);
        assert_eq!(alg.check, crc.checksum(INIT));
        let mut digest = crc.digest();
        digest.update(INIT);
        assert_eq!(alg.check, digest.finalize());
    }
}
