//! Tests that adding doc-comment doesn't break the build.

use trait_set::trait_set;

trait_set! {
    /// This is a doc-comment!
    ///
    /// It has multiple lines!
    ///
    #[doc = "And it's mixed with different flavors of comments..."]
    /** Even block-comments,  */
    pub(crate) trait TraitSet = Send + Sync;
}

fn test_set<T: TraitSet>(_arg: T) {}

fn main() {
    test_set(10u8);
    test_set("hello");
}
