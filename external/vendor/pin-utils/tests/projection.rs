#![allow(deprecated)]

use pin_utils::{pin_mut, unsafe_pinned, unsafe_unpinned};
use std::pin::Pin;

struct Foo<T1, T2> {
    field1: T1,
    field2: T2,
}

impl<T1, T2> Foo<T1, T2> {
    unsafe_pinned!(field1: T1);
    unsafe_unpinned!(field2: T2);
}

impl<T1: Unpin, T2> Unpin for Foo<T1, T2> {} // Conditional Unpin impl

#[test]
fn projection() {
    let x = Foo {
        field1: 1,
        field2: 2,
    };
    pin_mut!(x);

    let x1: Pin<&mut i32> = x.as_mut().field1();
    assert_eq!(*x1, 1);

    let x2: &mut i32 = x.as_mut().field2();
    assert_eq!(*x2, 2);
}
