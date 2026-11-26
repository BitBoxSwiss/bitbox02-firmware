//! Checks that aliases can be combined between each other into a new alias.

use trait_set::trait_set;

trait_set! {
    pub trait ThreadSafe = Send + Sync;
    pub trait BytesIterator = Iterator<Item = u8>;
    pub trait ThreadSafeBytesIterator = ThreadSafe + BytesIterator;
}

fn test_set<T: ThreadSafeBytesIterator>(_arg: T) {}

fn main() {
    test_set([10u8, 20, 30].as_ref().iter().copied());
}
