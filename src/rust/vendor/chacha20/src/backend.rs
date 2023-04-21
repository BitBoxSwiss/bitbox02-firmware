//! Backends providing the ChaCha20 core function.
//!
//! Defined in RFC 8439 Section 2.3:
//! <https://tools.ietf.org/html/rfc8439#section-2.3>

use cfg_if::cfg_if;

pub(crate) mod soft;

cfg_if! {
    if #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2",
        not(feature = "force-soft")
    ))] {
        pub(crate) mod autodetect;
        pub(crate) mod avx2;
        pub(crate) mod sse2;

        pub(crate) use self::autodetect::BUFFER_SIZE;
        pub use self::autodetect::Core;
    } else if #[cfg(all(
        feature = "neon",
        target_arch = "aarch64",
        target_feature = "neon",
        not(feature = "force-soft")
    ))] {
        pub(crate) mod neon;
        pub(crate) use self::neon::BUFFER_SIZE;
        pub use self::neon::Core;
    } else {
        pub(crate) use self::soft::BUFFER_SIZE;
        pub use self::soft::Core;
    }
}
