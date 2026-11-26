//! Checks that lifetime bounds are accepted.

use trait_set::trait_set;

trait_set! {
    pub(crate) trait Set = 'static + Send + Sync;
}

fn test_set<T: Set>(_arg: T) {}

fn main() {
    test_set([10u8, 20, 30].as_ref().iter().copied());
    test_set(b"abcde".iter().copied());
}
