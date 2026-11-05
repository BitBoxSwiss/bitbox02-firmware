//! Tests the simplest possible combination of traits.

use trait_set::trait_set;

trait_set! {
    pub(crate) trait TraitSet = Send + Sync;
}

fn test_set<T: TraitSet>(_arg: T) {}

fn main() {
    test_set(10u8);
    test_set("hello");
}
