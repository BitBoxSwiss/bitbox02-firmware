// SPDX-License-Identifier: Apache-2.0

use alloc::{rc::Rc, vec, vec::Vec};
use core::cell::RefCell;
use zeroize::{Zeroize, Zeroizing};

use crate::{LabelExt, LvEventCode, LvHandle, LvLabel, LvText, LvTextError, ObjExt, class, ffi};

/// A label with a fixed-capacity Rust text buffer, wiped on replacement and deletion.
///
/// LVGL borrows the buffer via its static-text API, so it never allocates an unwiped copy.
/// The deletion callback keeps the buffer alive even if this handle is dropped first.
pub struct LvZeroizingLabel {
    label: LvLabel,
    buffer: Rc<RefCell<Zeroizing<Vec<u8>>>>,
}

impl LvZeroizingLabel {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>, max_bytes: usize) -> Option<Self> {
        let capacity = max_bytes.checked_add(1)?;
        let buffer = Rc::new(RefCell::new(Zeroizing::new(vec![0; capacity])));
        let label = LvLabel::new(parent)?;
        let obj = label.as_ptr();
        let buffer_on_delete = Rc::clone(&buffer);
        if label
            .add_event_cb(LvEventCode::LV_EVENT_DELETE, move || {
                // Detach before wiping, so subsequent deletion callbacks cannot read old text.
                unsafe { ffi::lv_label_set_text_static(obj, c"".as_ptr()) };
                buffer_on_delete.borrow_mut().as_mut_slice().zeroize();
            })
            .is_err()
        {
            unsafe { label.delete() };
            return None;
        }
        let text = buffer.borrow_mut().as_mut_ptr().cast();
        unsafe { ffi::lv_label_set_text_static(obj, text) };
        Some(Self { label, buffer })
    }

    /// Replaces the text and sends `LV_EVENT_VALUE_CHANGED`.
    pub fn set_text(&self, text: &str) -> Result<(), LvTextError> {
        if text.as_bytes().contains(&0) {
            return Err(LvTextError::ContainsNul);
        }
        let ptr = {
            let mut buffer = self.buffer.borrow_mut();
            if text.len() >= buffer.len() {
                return Err(LvTextError::TooLong);
            }
            // Wipe the full buffer, including any suffix left by shortening the text.
            buffer.as_mut_slice().zeroize();
            buffer[..text.len()].copy_from_slice(text.as_bytes());
            buffer.as_mut_ptr().cast()
        };
        unsafe {
            ffi::lv_label_set_text_static(self.as_ptr(), ptr);
            ffi::lv_obj_send_event(
                self.as_ptr(),
                LvEventCode::LV_EVENT_VALUE_CHANGED,
                core::ptr::null_mut(),
            );
        }
        Ok(())
    }

    pub fn get_text(&self) -> Option<LvText> {
        self.label.get_text()
    }
}

impl ObjExt for LvZeroizingLabel {
    fn as_ptr(&self) -> *mut ffi::lv_obj_t {
        self.label.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_text_wipes_replaced_text_and_deletion() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let label = LvZeroizingLabel::new(&screen, 8).unwrap();
        label.set_text("abstract").unwrap();
        let buffer = Rc::clone(&label.buffer);
        let ptr = buffer.borrow().as_ptr();
        assert_eq!(
            unsafe { ffi::lv_label_get_text(label.as_ptr()) }.cast::<u8>(),
            ptr.cast_mut()
        );
        assert_eq!(label.set_text("too long!"), Err(LvTextError::TooLong));
        assert_eq!(label.set_text("a\0b"), Err(LvTextError::ContainsNul));
        assert_eq!(label.get_text().unwrap().to_str().unwrap(), "abstract");
        label.set_text("act").unwrap();
        assert_eq!(buffer.borrow().as_slice(), b"act\0\0\0\0\0\0");
        assert_eq!(buffer.borrow().as_ptr(), ptr);
        // Dropping the Rust handle must leave the visible label's buffer alive.
        let obj = label.as_ptr();
        drop(label);
        unsafe { ffi::lv_obj_delete(obj) };
        assert!(buffer.borrow().iter().all(|byte| *byte == 0));
    }
}
