#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))] // for tests

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

use portable_atomic::{AtomicBool, Ordering};

/// Statically allocated, initialized at runtime cell.
///
/// It has two states: "empty" and "full". It is created "empty", and obtaining a reference
/// to the contents permanently changes it to "full". This allows that reference to be valid
/// forever.
///
/// If your value can be initialized as a `const` value, consider using [`ConstStaticCell`]
/// instead if you only need to take the value at runtime.
///
/// See the [crate-level docs](crate) for usage.
pub struct StaticCell<T> {
    used: AtomicBool,
    val: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T> Send for StaticCell<T> {}
unsafe impl<T> Sync for StaticCell<T> {}

impl<T> StaticCell<T> {
    /// Create a new, empty `StaticCell`.
    ///
    /// It can be initialized at runtime with [`StaticCell::init()`] or similar methods.
    #[inline]
    pub const fn new() -> Self {
        Self {
            used: AtomicBool::new(false),
            val: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    /// Initialize the `StaticCell` with a value, returning a mutable reference to it.
    ///
    /// Using this method, the compiler usually constructs `val` in the stack and then moves
    /// it into the `StaticCell`. If `T` is big, this is likely to cause stack overflows.
    /// Considering using [`StaticCell::init_with`] instead, which will construct it in-place inside the `StaticCell`.
    ///
    /// # Panics
    ///
    /// Panics if this `StaticCell` is already full.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn init(&'static self, val: T) -> &'static mut T {
        self.uninit().write(val)
    }

    /// Initialize the `StaticCell` with the closure's return value, returning a mutable reference to it.
    ///
    /// The advantage over [`StaticCell::init`] is that this method allows the closure to construct
    /// the `T` value in-place directly inside the `StaticCell`, saving stack space.
    ///
    /// # Panics
    ///
    /// Panics if this `StaticCell` is already full.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn init_with(&'static self, val: impl FnOnce() -> T) -> &'static mut T {
        self.uninit().write(val())
    }

    /// Return a mutable reference to the uninitialized memory owned by the `StaticCell`.
    ///
    /// Using this method directly is not recommended, but it can be used to construct `T` in-place directly
    /// in a guaranteed fashion.
    ///
    /// # Panics
    ///
    /// Panics if this `StaticCell` is already full.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn uninit(&'static self) -> &'static mut MaybeUninit<T> {
        if let Some(val) = self.try_uninit() {
            val
        } else {
            panic!("`StaticCell` is already full, it can't be initialized twice.");
        }
    }

    /// Try initializing the `StaticCell` with a value, returning a mutable reference to it.
    ///
    /// If this `StaticCell` is already full, it returns `None`.
    ///
    /// Using this method, the compiler usually constructs `val` in the stack and then moves
    /// it into the `StaticCell`. If `T` is big, this is likely to cause stack overflows.
    /// Considering using [`StaticCell::try_init_with`] instead, which will construct it in-place inside the `StaticCell`.
    ///
    /// Will only return a Some(&'static mut T) when the `StaticCell` was not yet initialized.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn try_init(&'static self, val: T) -> Option<&'static mut T> {
        Some(self.try_uninit()?.write(val))
    }

    /// Try initializing the `StaticCell` with the closure's return value, returning a mutable reference to it.
    ///
    /// If this `StaticCell` is already full, it returns `None`.
    ///
    /// The advantage over [`StaticCell::init`] is that this method allows the closure to construct
    /// the `T` value in-place directly inside the `StaticCell`, saving stack space.
    ///
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn try_init_with(&'static self, val: impl FnOnce() -> T) -> Option<&'static mut T> {
        Some(self.try_uninit()?.write(val()))
    }

    /// Try returning a mutable reference to the uninitialized memory owned by the `StaticCell`.
    ///
    /// If this `StaticCell` is already full, it returns `None`.
    ///
    /// Using this method directly is not recommended, but it can be used to construct `T` in-place directly
    /// in a guaranteed fashion.
    #[inline]
    pub fn try_uninit(&'static self) -> Option<&'static mut MaybeUninit<T>> {
        if self
            .used
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            // SAFETY: We just checked that the value is not yet taken and marked it as taken.
            let val = unsafe { &mut *self.val.get() };
            Some(val)
        } else {
            None
        }
    }
}

// ---

/// Statically allocated and initialized, taken at runtime cell.
///
/// It has two states: "untaken" and "taken". It is created "untaken", and obtaining a reference
/// to the contents permanently changes it to "taken". This allows that reference to be valid
/// forever.
///
/// If your value can be const defined, for example a large, zero filled buffer used for DMA
/// or other scratch memory usage, `ConstStaticCell` can be used to guarantee the initializer
/// will never take up stack memory.
///
/// If your values are all zero initialized, the resulting `ConstStaticCell` should be placed
/// in `.bss`, not taking flash space for initialization either.
///
/// See the [crate-level docs](crate) for usage.
pub struct ConstStaticCell<T> {
    taken: AtomicBool,
    val: UnsafeCell<T>,
}

// ConstStaticCell<T> essentially lets you construct a value of T, pass &ConstStaticCell<T> to
// another thread, and then in that thread obtain a &'static mut T, which is effectively sending
// the value between threads, therefore T must implement Send.
unsafe impl<T: Send> Send for ConstStaticCell<T> {}
unsafe impl<T: Send> Sync for ConstStaticCell<T> {}

impl<T> ConstStaticCell<T> {
    /// Create a new, empty `ConstStaticCell`.
    ///
    /// It can be taken at runtime with [`ConstStaticCell::take()`] or similar methods.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            taken: AtomicBool::new(false),
            val: UnsafeCell::new(value),
        }
    }

    /// Take the `ConstStaticCell`, returning a mutable reference to it.
    ///
    /// # Panics
    ///
    /// Panics if this `ConstStaticCell` was already taken.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn take(&'static self) -> &'static mut T {
        if let Some(val) = self.try_take() {
            val
        } else {
            panic!("`ConstStaticCell` is already taken, it can't be taken twice")
        }
    }

    /// Try to take the `ConstStaticCell`, returning None if it was already taken
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn try_take(&'static self) -> Option<&'static mut T> {
        if self
            .taken
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            // SAFETY: We just checked that the value is not yet taken and marked it as taken.
            let val = unsafe { &mut *self.val.get() };
            Some(val)
        } else {
            None
        }
    }
}

/// Convert a `T` to a `&'static mut T`.
///
/// The macro declares a `static StaticCell` and then initializes it when run, returning the `&'static mut`.
/// Therefore, each instance can only be run once. Next runs will panic. The `static` can additionally be
/// decorated with attributes, such as `#[link_section]`, `#[used]`, et al.
///
/// This macro is nightly-only. It requires `#![feature(type_alias_impl_trait)]` in the crate using it.
///
/// # Examples
///
/// ```
/// # #![feature(type_alias_impl_trait)]
/// use static_cell::make_static;
///
/// # fn main() {
/// let x: &'static mut u32 = make_static!(42);
///
/// // This attribute instructs the linker to allocate it in the external RAM's BSS segment.
/// // This specific example is for ESP32S3 with PSRAM support.
/// let buf = make_static!([0u8; 4096], #[link_section = ".ext_ram.bss.buf"]);
///
/// // Multiple attributes can be supplied.
/// let s = make_static!(0usize, #[used] #[export_name = "exported_symbol_name"]);
/// # }
/// ```
#[cfg(feature = "nightly")]
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
#[macro_export]
macro_rules! make_static {
    ($val:expr) => ($crate::make_static!($val, ));
    ($val:expr, $(#[$m:meta])*) => {{
        type T = impl ::core::marker::Sized;
        $(#[$m])*
        static STATIC_CELL: $crate::StaticCell<T> = $crate::StaticCell::new();
        #[deny(unused_attributes)]
        let (x,) = STATIC_CELL.uninit().write(($val,));
        x
    }};
}

#[cfg(test)]
mod tests {
    use crate::StaticCell;

    #[test]
    fn test_static_cell() {
        static CELL: StaticCell<u32> = StaticCell::new();
        let val: &'static u32 = CELL.init(42u32);
        assert_eq!(*val, 42);
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn test_make_static() {
        let val: &'static u32 = make_static!(42u32);
        assert_eq!(*val, 42);
    }
}

#[cfg(doctest)]
mod compile_fail_tests {
    /// ```compile_fail
    /// use std::rc::Rc;
    ///
    /// use static_cell::ConstStaticCell;
    ///
    /// fn main() {
    ///     let rc = Rc::new(0);
    ///     let sc = ConstStaticCell::new(rc.clone());
    ///     let t = std::thread::spawn(move || {
    ///         let sc = Box::leak(Box::new(sc));
    ///         let rc: &mut Rc<i32> = sc.take();
    ///         _ = rc.clone(); // data race!
    ///     });
    ///     _ = rc.clone();
    ///     _ = t.join();
    /// }
    /// ```
    fn send_requires_send() {}

    /// ```compile_fail
    /// use std::rc::Rc;
    ///
    /// use static_cell::ConstStaticCell;
    ///
    /// fn main() {
    ///     let rc = Rc::new(0);
    ///     let sc = Box::leak(Box::new(ConstStaticCell::new(rc.clone())));
    ///     let t = std::thread::spawn(|| {
    ///         let rc: &mut Rc<i32> = sc.take();
    ///         _ = rc.clone(); // data race!
    ///     });
    ///     _ = rc.clone();
    ///     _ = t.join();
    /// }
    /// ```
    fn sync_requires_send() {}
}
