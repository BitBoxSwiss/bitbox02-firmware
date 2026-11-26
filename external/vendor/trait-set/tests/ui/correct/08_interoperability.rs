//! Checks that aliases for the same set are interoperable between
//! each other and with plain trait combination.

use trait_set::trait_set;

trait_set! {
    pub(crate) trait TraitSet1 = Send + Sync;
    pub(crate) trait TraitSet2 = Send + Sync;
}

fn test_set1<T: TraitSet1>(arg: T) {
    test_set2(arg)
}
fn test_set2<T: TraitSet2>(arg: T) {
    test_set3(arg)
}
fn test_set3<T: Send + Sync>(_arg: T) {}

fn main() {
    test_set1(10u8);
    test_set1("hello");
}
