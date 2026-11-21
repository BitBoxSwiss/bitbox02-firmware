// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, vec};
use core::ffi::CStr;
use core::ops::Deref;
use zeroize::Zeroizing;

use crate::LvTextError;

/// An owned text snapshot that wipes its allocation when dropped.
///
/// Temporary strings and snapshots crossing the LVGL boundary use this type so their
/// Rust-owned storage is wiped automatically. Native widget text copies need separate
/// protection; use [`crate::LvZeroizingLabel`] for labels with zeroizing backing storage.
pub struct ZeroizingText(Zeroizing<Box<[u8]>>);

impl ZeroizingText {
    pub(crate) fn new(text: &str) -> Result<Self, LvTextError> {
        // Validate before copying, including on the error path.
        if text.as_bytes().contains(&0) {
            return Err(LvTextError::ContainsNul);
        }
        // Finish allocating the final buffer before copying sensitive text.
        let mut bytes = Zeroizing::new(vec![0; text.len() + 1].into_boxed_slice());
        bytes[..text.len()].copy_from_slice(text.as_bytes());
        Ok(Self(bytes))
    }

    pub(crate) fn from_cstr(text: &CStr) -> Self {
        Self(Zeroizing::new(Box::from(text.to_bytes_with_nul())))
    }
}

impl Deref for ZeroizingText {
    type Target = CStr;

    fn deref(&self) -> &CStr {
        // Both constructors store exactly one trailing NUL, with no interior NULs.
        unsafe { CStr::from_bytes_with_nul_unchecked(self.0.as_ref()) }
    }
}

impl AsRef<CStr> for ZeroizingText {
    fn as_ref(&self) -> &CStr {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        for value in ["", "recovery words", "öäü"] {
            let text = ZeroizingText::new(value).unwrap();
            assert_eq!(text.to_str().unwrap(), value);
            assert_eq!(text.to_bytes_with_nul().len(), value.len() + 1);
        }
        assert!(matches!(
            ZeroizingText::new("secret\0suffix"),
            Err(LvTextError::ContainsNul)
        ));
    }
}
