#![forbid(unsafe_code)] // pin_mut! is completely safe.

use core::pin::Pin;
use pin_utils::pin_mut;

#[test]
fn stack_pin() {
    struct Foo {}
    let x = Foo {};
    pin_mut!(x);
    let _: Pin<&mut Foo> = x;

    let y = Foo {};
    let z = Foo {};
    pin_mut!(y, z,);
    let _: Pin<&mut Foo> = y;
    let _: Pin<&mut Foo> = z;
}
