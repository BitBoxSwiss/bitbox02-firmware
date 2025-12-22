# vex-sdk

This project contains raw C-level bindings to VEX's system APIs.

You can use this crate by adding it to your project's Cargo.toml with `cargo add vex-sdk`.

## Linkage

> [!NOTE]
> If you are using a higher-level library such as [vexide](https://crates.io/crates/vexide), this is likely already handled for you!

`vex-sdk` provides declarations for functions included in VEX's platform SDKs, but doesn't
link to any runtime library itself. Your project will need an implementation of these functions
to compile properly.

One option is to depend on [`vex-sdk-jumptable`](https://github.com/vexide/vex-sdk/tree/main/packages/vex-sdk-jumptable), an open-source reimplementation of VEX's runtime library, then bring it into scope in your project:

```rs
// Bring runtime library into scope
use ::vex_sdk_jumptable as *;

fn main() { /* ... */ }
```

Alternatively, you could use a build script to add one of VEX's official SDKs into your link search path, then link to `libv5rt.a` or the equivalent for your platform. (See [libv5rt-meson](https://github.com/ZestCommunity/libv5rt-meson) for an example of how to do this.)
