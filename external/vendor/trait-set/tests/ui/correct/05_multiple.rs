//! Checks that multiple aliases can exist within one `trait_set` call.

use trait_set::trait_set;

trait_set! {
    pub(crate) trait TraitSet = Send + Sync;
    pub trait BytesIterator = Iterator<Item = u8>;
    trait GenericIterator<T> = Iterator<Item = T>;
}

fn test_set<T: TraitSet>(_arg: T) {}
fn test_iter<T: BytesIterator>(_arg: T) {}
fn test_generic_iter<T: GenericIterator<u8>>(_arg: T) {}

fn main() {
    test_set(10u8);
    test_iter([10u8, 20, 30].as_ref().iter().copied());
    test_generic_iter([10u8, 20, 30].as_ref().iter().copied());
}
