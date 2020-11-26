#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
pub(crate) mod avx2;

#[cfg(any(
    not(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    )),
    fuzzing,
    test,
))]
pub(crate) mod soft;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2",
))]
pub(crate) use avx2::State;

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2",
)))]
pub(crate) use soft::State;
