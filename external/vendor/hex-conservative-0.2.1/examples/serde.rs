//! Demonstrate how to use the serde module with struct fields.

#![allow(clippy::disallowed_names)] // Foo is a valid name.

use hex_conservative as hex;
use serde::{Deserialize, Serialize};

/// Abstracts over foo.
#[derive(Debug, Serialize, Deserialize)]
pub struct Foo {
    // serialized as a hexadecimal string.
    #[serde(with = "hex::serde")]
    pub u: Vec<u8>,
    // serialized as an array of decimal integers.
    pub v: Vec<u8>,
}

fn main() {
    let v = vec![0xde, 0xad, 0xbe, 0xef];

    let foo = Foo { u: v.clone(), v };
    let ser = serde_json::to_string(&foo).expect("failed to serialize foo");

    // Prints:
    //
    //  foo: {"u":"deadbeef","v":[222,173,190,239]}
    //
    println!("\nfoo: {}", ser);
}
