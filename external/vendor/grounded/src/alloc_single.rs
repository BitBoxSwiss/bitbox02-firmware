//! Utilities for allocating a single item, using a box-like smart pointer

use core::{
    ops::{Deref, DerefMut},
    sync::atomic::Ordering,
};
use portable_atomic::AtomicBool;

use crate::{const_init::ConstInit, uninit::GroundedCell};

/// AllocSingle is our one-element allocator pool
///
/// If your type implements [ConstInit], consider using
/// [AllocSingle::alloc_const_val] instead of [AllocSingle::alloc]
/// to avoid unnecessary stack usage.
///
/// This does require use of CAS atomics. You must enable the `cas`
/// feature, and if your target does not have native atomic CAS, you
/// must also enable the `critical-section` feature.
///
/// ```rust
/// use grounded::alloc_single::AllocSingle;
///
/// static SINGLE: AllocSingle<[u8; 256]> = AllocSingle::new();
///
/// // alloc a single item
/// let mut s1 = SINGLE.alloc([4; 256]).unwrap();
/// s1.iter().for_each(|b| assert_eq!(*b, 4));
///
/// // we can't alloc while `s1` is still live
/// assert!(SINGLE.alloc([5; 256]).is_none());
///
/// // now drop it
/// drop(s1);
///
/// // and we can alloc again
/// let mut s2 = SINGLE.alloc([7; 256]).unwrap();
/// s2.iter().for_each(|b| assert_eq!(*b, 7));
/// ```
pub struct AllocSingle<T> {
    taken: AtomicBool,
    storage: GroundedCell<T>,
}

impl<T> AllocSingle<T> {
    /// Create a new, uninitalized, single-element allocation pool
    pub const fn new() -> Self {
        Self {
            taken: AtomicBool::new(false),
            storage: GroundedCell::uninit(),
        }
    }

    /// Attempts to allocate a single item. Returns None and
    /// discards `t` if an allocation is already live.
    #[inline]
    pub fn alloc(&self, t: T) -> Option<SingleBox<'_, T>> {
        // Set taken, and if it was already taken before, we can't
        // allocate
        if self.taken.swap(true, Ordering::AcqRel) {
            // already taken
            return None;
        }
        let new = SingleBox { single: self };
        // Initialize by moving t into the storage
        unsafe {
            new.as_ptr().write(t);
        }
        Some(new)
    }
}

impl<T: ConstInit> AllocSingle<T> {
    /// Attempts to allocate a single item, using `ConstInit::VAL` as
    /// the initializer. Returns None if the item is already allocated
    pub fn alloc_const_val(&self) -> Option<SingleBox<'_, T>> {
        // Set taken, and if it was already taken before, we can't
        // allocate
        if self.taken.swap(true, Ordering::AcqRel) {
            // already taken
            return None;
        }
        let new = SingleBox { single: self };
        // Initialize by writing t into the storage
        unsafe {
            new.as_ptr().write(T::VAL);
        }
        Some(new)
    }
}

pub struct SingleBox<'a, T> {
    single: &'a AllocSingle<T>,
}

impl<'a, T> SingleBox<'a, T> {
    fn as_ptr(&self) -> *mut T {
        self.single.storage.get()
    }
}

impl<'a, T> Drop for SingleBox<'a, T> {
    fn drop(&mut self) {
        // When we drop the SingleBox, mark the AllocSingle as available again
        unsafe { self.as_ptr().drop_in_place() }
        self.single.taken.store(false, Ordering::Release);
    }
}

impl<'a, T> Deref for SingleBox<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.as_ptr() }
    }
}

impl<'a, T> DerefMut for SingleBox<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_ptr() }
    }
}

#[cfg(test)]
pub mod test {
    use super::AllocSingle;
    use crate::const_init::ConstInit;
    use core::ops::Deref;

    #[derive(Debug)]
    struct Demo([u8; 512]);

    impl ConstInit for Demo {
        const VAL: Self = Demo([44u8; 512]);
    }

    #[test]
    fn smoke() {
        static SINGLE: AllocSingle<[u8; 1024]> = AllocSingle::new();
        static SINGLE_DEMO: AllocSingle<Demo> = AllocSingle::new();

        {
            let buf = [0xAF; 1024];
            let mut bx = SINGLE.alloc(buf).unwrap();
            println!("{:?}", bx.as_slice());
            bx.iter_mut().for_each(|b| *b = 123);
            println!("{:?}", bx.as_slice());

            // Second alloc fails
            let buf2 = [0x01; 1024];
            assert!(SINGLE.alloc(buf2).is_none());
        }

        // bx is dropped because we left scope, which means we can
        // alloc again
        let buf3 = [0x42; 1024];
        let mut bx2 = SINGLE.alloc(buf3).unwrap();
        println!("{:?}", bx2.as_slice());
        bx2.iter_mut().for_each(|b| *b = 231);
        println!("{:?}", bx2.as_slice());

        // look ma no stack
        let bx3 = SINGLE_DEMO.alloc_const_val().unwrap();
        println!("{:?}", bx3.deref());
    }
}
