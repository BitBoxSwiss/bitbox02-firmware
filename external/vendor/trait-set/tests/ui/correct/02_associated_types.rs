//! Checks that traits with associated types can be used in alias.

use trait_set::trait_set;

trait_set! {
    pub(crate) trait BytesIterator = Iterator<Item = u8>;
}

fn test_set<T: BytesIterator>(_arg: T) {}

fn main() {
    test_set([10u8, 20, 30].as_ref().iter().copied());
    test_set(b"abcde".iter().copied());
}
