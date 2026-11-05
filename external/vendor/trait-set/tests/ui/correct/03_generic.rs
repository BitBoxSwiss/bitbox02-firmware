//! Checks that generic aliases are processed as expected.

use trait_set::trait_set;

pub trait GenericTrait<T> {
    fn new(t: T) -> Self;
}

impl GenericTrait<u8> for u8 {
    fn new(t: u8) -> u8 {
        t
    }
}

trait_set! {
    pub(crate) trait GenericIterator<T> = Iterator<Item = T>;
    pub(crate) trait GenericFoo<T> = GenericTrait<T>;
    pub(crate) trait SpecializedFoo = GenericTrait<u8>;
}

fn test_set<T: GenericIterator<u8>>(_arg: T) {}
fn test_generic<T: GenericFoo<u8>>(_arg: T) {}
fn test_specialized<T: SpecializedFoo>(_arg: T) {}

fn main() {
    test_set([10u8, 20, 30].as_ref().iter().copied());
    test_set(b"abcde".iter().copied());
    test_generic(10);
    test_specialized(10);
}
