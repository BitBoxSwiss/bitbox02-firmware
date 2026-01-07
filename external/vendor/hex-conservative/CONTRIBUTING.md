# Contributing to hex-conservative

Thanks for the interest in contributing!
Before you do anything, please familiarize yourself with the stabilization strategy as described in the README, since it places some restrictions on how the code is developed.

## Contributing to the stable crate

Generally, you should try to make your contributions against the `master` branch which contains unstable code and thus it's much more forgiving when it comes to adding any new features.
However it may be sometimes impossible, in which case please follow these rules:

1. Breaking any API of the 1.0 crate is unacceptable unless it can be done with semver trick (e.g. feature modifictaions). Please do not even attempt to do this.
2. For new APIs, please gate all of them behing `#[cfg(hex-conservative-unstable)]` to ensure they won't get used by accident. The stabilization procedure is similar to that of Rust itself.
3. Once a new unstable API is released, try using it in a real project. (Ideally multiple different projects.)
4. If you want to get an unstable feature stabilized, write a stabilization report that contains at least one Open Source project that uses it and a summary of how well the feature works in practice.
5. Any new feature should be out for at least 6 months before stabilization to give projects enough chance to test it out. We may make exceptions for particularly simple and desirable features.

## General rules

First and formost, all code has to produce correct results and have API that models a domain accurately and makes it hard to do wrong things.
If any other rule is in conflict with the first rule, the first rule takes precedence.
Both encoding and decoding should be reasonably fast. Please avoid unneeded allocations or other kinds of code that cause slowness.
All code should be idiomatic, reasonably consistent and maintainable. This includes high importance of "single source of truth" because violating it is extremely prone to causing bugs.

## Githooks

To assist devs in catching errors _before_ running CI we provide some githooks. If you do not
already have locally configured githooks you can use the ones in this repository by running, in the
root directory of the repository:
```
git config --local core.hooksPath githooks/
```

Alternatively add symlinks in your `.git/hooks` directory to any of the githooks we provide.
