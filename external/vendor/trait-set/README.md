# `trait-set`: trait aliases on stable Rust

**Status:**
[![CI](https://github.com/popzxc/trait-set/workflows/CI/badge.svg)](https://github.com/popzxc/trait-set/actions)

**Project info:**
[![Docs.rs](https://docs.rs/trait-set/badge.svg)](https://docs.rs/trait-set)
[![Latest Version](https://img.shields.io/crates/v/trait-set.svg)](https://crates.io/crates/trait-set)
[![License](https://img.shields.io/github/license/popzxc/trait-set.svg)](https://github.com/popzxc/trait-set)
![Rust 1.50+ required](https://img.shields.io/badge/rust-1.50+-blue.svg?label=Rust)

Support for trait aliases on stable Rust.

## Description

This crate provide support for [trait aliases][alias]: a feature
that is already supported by Rust compiler, but is [not stable][tracking_issue]
yet.

The idea is simple: combine group of traits under a single name. The simplest
example will be:

```rust
use trait_set::trait_set;

trait_set! {
    pub trait ThreadSafe = Send + Sync;
}
```

Macro [`trait_set`] displayed here is the main entity of the crate:
it allows declaring multiple trait aliases, each of them is represented
as

```text
[visibility] trait [AliasName][<generics>] = [Element1] + [Element2] + ... + [ElementN];
```

[`trait_set`]: https://docs.rs/trait-set/latest/trait_set/macro.trait_set.html
[alias]: https://doc.rust-lang.org/unstable-book/language-features/trait-alias.html
[tracking_issue]: https://github.com/rust-lang/rust/issues/41517

## Example

```rust
use trait_set::trait_set;

trait_set! {
    // Simple combination of two traits.
    /// Doc-comments are also supported btw.
    pub trait ThreadSafe = Send + Sync;

    // Generic alias that gets passed to the associated type.
    pub trait ThreadSafeIterator<T> = ThreadSafe + Iterator<Item = T>;

    // Specialized alias for a generic trait.
    pub trait ThreadSafeBytesIterator = ThreadSafeIterator<u8>;

    // Lifetime bounds.
    pub trait StaticDebug = 'static + std::fmt::Debug;

    // Higher-ranked trait bounds.
    pub trait Serde = Serialize + for<'de> Deserialize<'de>;

    // Lifetime as a generic parameter.
    pub trait SerdeLifetimeTemplate<'de> = Serialize + Deserialize<'de>;
    
    // Trait bounds on generic parameters for an alias.
    pub trait GenericIteratorSendableT<T: Send> = Iterator<Item = T>;
}
```

## Motivation

Rust is great, and it becomes even better through time. However, a time gap between proposing
a new feature and getting it stabilized is way too big.

Trait aliases is a great example: 20% of functionality will serve the needs of 80%.
So, until they are stabilized, this crate hopefully will allow some folks to write more readable code.

## Contributing

Feel free to submit a PR!

## LICENSE

`trait-set` library is licensed under the MIT License. See [LICENSE](LICENSE) for details.
