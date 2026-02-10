//! Const Init
//!
//! A trait that is like `Default`, but const

/// A trait that is like `Default`, but const
pub trait ConstInit {
    /// The constant default value
    const VAL: Self;
}

// Here's some impls that roughly match the default
// value of these types

macro_rules! impl_const_init_for {
    ($(($tyname:ty, $val:expr),)+) => {
        $(
            impl ConstInit for $tyname {
                const VAL: Self = $val;
            }
        )+
    };
}

impl_const_init_for! {
    (u8, 0),
    (u16, 0),
    (u32, 0),
    (u64, 0),
    (u128, 0),
    (i8, 0),
    (i16, 0),
    (i32, 0),
    (i64, 0),
    (i128, 0),
    (f32, 0.0),
    (f64, 0.0),
    (bool, false),
    ((), ()),
}

impl<T, const N: usize> ConstInit for [T; N]
where
    T: ConstInit,
{
    const VAL: Self = [T::VAL; N];
}

impl<T> ConstInit for Option<T> {
    const VAL: Self = None;
}
