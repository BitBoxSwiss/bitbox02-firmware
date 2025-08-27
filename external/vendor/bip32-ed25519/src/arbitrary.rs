#[derive(Debug, Clone)]
pub struct Arbitrary32(pub [u8; 32]);
impl quickcheck::Arbitrary for Arbitrary32 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Arbitrary32 {
        let mut out = [0u8; 32];
        for e in out.iter_mut() {
            *e = u8::arbitrary(g)
        }
        Arbitrary32(out)
    }
}

#[derive(Debug, Clone)]
pub struct Arbitrary64(pub [u8; 64]);
impl quickcheck::Arbitrary for Arbitrary64 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Arbitrary64 {
        let mut out = [0u8; 64];
        for e in out.iter_mut() {
            *e = u8::arbitrary(g)
        }
        Arbitrary64(out)
    }
}
