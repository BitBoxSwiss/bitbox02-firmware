// SPDX-License-Identifier: Apache-2.0

use core::ptr::NonNull;

use crate::{LvHandle, LvObj, LvPointPrecise, class, ffi};

pub type LvLine = LvHandle<class::LineTag>;

impl LvHandle<class::LineTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_line_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    /// Sets the polyline points.
    ///
    /// LVGL only retains the pointer to the points (it does not copy them), so the points are
    /// must be statically allocated. The line's color, width and rounded caps are controlled with
    /// the `set_style_line_*` methods from [`crate::ObjExt`].
    pub fn set_points(&self, points: &'static [LvPointPrecise]) {
        let point_num = points.len() as u32;
        unsafe { ffi::lv_line_set_points(self.as_ptr(), points.as_ptr(), point_num) }
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ObjExt;

    static POINTS: &[LvPointPrecise] = &[
        LvPointPrecise { x: 0, y: 0 },
        LvPointPrecise { x: 10, y: 20 },
        LvPointPrecise { x: 30, y: 5 },
    ];

    static POINTS_REPLACEMENT: &[LvPointPrecise] = &[
        LvPointPrecise { x: 1, y: 2 },
        LvPointPrecise { x: 3, y: 4 },
        LvPointPrecise { x: 5, y: 6 },
    ];

    #[test]
    fn test_line_set_points_uses_static_points() {
        let _lock = crate::test_util::lock_and_init();

        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let line = LvLine::new(&screen).unwrap();
        line.set_points(POINTS);

        assert_eq!(
            unsafe { ffi::lv_line_get_points(line.as_ptr()) },
            POINTS.as_ptr()
        );
        assert_eq!(unsafe { ffi::lv_line_get_point_count(line.as_ptr()) }, 3);

        // Deleting the object must not free the statically allocated points.
        unsafe { line.delete() };
    }

    #[test]
    fn test_line_set_points_replaces_points() {
        let _lock = crate::test_util::lock_and_init();

        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let line = LvLine::new(&screen).unwrap();
        line.set_points(POINTS);
        let event_count = unsafe { ffi::lv_obj_get_event_count(line.as_ptr()) };

        line.set_points(POINTS_REPLACEMENT);

        assert_eq!(
            unsafe { ffi::lv_obj_get_event_count(line.as_ptr()) },
            event_count
        );
        assert_eq!(
            unsafe { ffi::lv_line_get_points(line.as_ptr()) },
            POINTS_REPLACEMENT.as_ptr()
        );
        assert_eq!(unsafe { ffi::lv_line_get_point_count(line.as_ptr()) }, 3);

        unsafe { line.delete() };
    }
}
