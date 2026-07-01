// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use core::ptr::NonNull;

use super::util;
use crate::{LvHandle, LvObj, LvPointPrecise, class, ffi};

pub type LvLine = LvHandle<class::LineTag>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvLineError {
    PointAttachmentFailed,
}

impl LvHandle<class::LineTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_line_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    /// Sets the polyline points.
    ///
    /// LVGL only retains the pointer to the points (it does not copy them), so the points are
    /// stored alongside the object and freed together with it. The line's color, width and rounded
    /// caps are controlled with the `set_style_line_*` methods from [`crate::ObjExt`].
    pub fn set_points(&self, points: Vec<LvPointPrecise>) -> Result<(), LvLineError> {
        let point_num = points.len() as u32;
        let attachment =
            util::attach_to_object(self, points).map_err(|_| LvLineError::PointAttachmentFailed)?;
        unsafe {
            ffi::lv_line_set_points(self.as_ptr(), (*attachment.as_ptr()).as_ptr(), point_num)
        }
        Ok(())
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ObjExt;

    #[test]
    fn test_line_set_points_keeps_points_alive() {
        let _lock = crate::test_util::lock_and_init();

        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let line = LvLine::new(&screen).unwrap();
        line.set_points(alloc::vec![
            LvPointPrecise { x: 0, y: 0 },
            LvPointPrecise { x: 10, y: 20 },
            LvPointPrecise { x: 30, y: 5 },
        ])
        .unwrap();

        // Deleting the object frees the attached points without leaking or double-freeing.
        unsafe { line.delete() };
    }
}
