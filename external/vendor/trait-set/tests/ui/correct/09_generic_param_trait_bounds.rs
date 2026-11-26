//! Checks that trait bounds can be applied to the generic arguments
//! of an alias.

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
    pub(crate) trait GenericIteratorSendableT<T: Send> = Iterator<Item = T>;
}

fn test_set<T: GenericIteratorSendableT<u8>>(_arg: T) {}

fn main() {
    test_set([10u8, 20, 30].as_ref().iter().copied());
    test_set(b"abcde".iter().copied());
}
