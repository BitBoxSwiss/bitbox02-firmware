// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::ffi::c_void;
use core::ptr::NonNull;

use crate::{LvHandle, class, ffi};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvTextError {
    ContainsNul,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvMapError {
    ContainsNul,
    EventRegistrationFailed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvEventRegistrationError {
    RegistrationFailed,
}

type EventCallback = RefCell<Box<dyn FnMut(*mut ffi::lv_event_t) + 'static>>;

#[repr(C)]
struct WidgetState<T> {
    drop: unsafe fn(*mut c_void),
    state: Rc<T>,
}

unsafe fn drop_widget_state<T>(ptr: *mut c_void) {
    drop(unsafe { Box::from_raw(ptr.cast::<WidgetState<T>>()) });
}

// LVGL runs the destructor after all deletion callbacks and child deletion.
#[unsafe(no_mangle)]
unsafe extern "C" fn bitbox_lv_widget_destructor(
    _class: *const ffi::lv_obj_class_t,
    obj: *mut ffi::lv_obj_t,
) {
    let ptr = unsafe { ffi::lv_obj_get_user_data(obj) };
    if !ptr.is_null() {
        unsafe {
            ffi::lv_obj_set_user_data(obj, core::ptr::null_mut());
            let drop = *ptr.cast::<unsafe fn(*mut c_void)>();
            drop(ptr);
        }
    }
}

// Only classes with bitbox_lv_widget_destructor may use this user-data layout.
pub(crate) unsafe fn create_widget<C: class::LvClass, T>(
    parent: &LvHandle<impl class::LvClass>,
    state: Rc<T>,
    callback: unsafe extern "C" fn(*mut ffi::lv_event_t),
) -> Option<LvHandle<C>> {
    let raw =
        NonNull::new(unsafe { ffi::lv_obj_class_create_obj(C::class_ptr(), parent.as_ptr()) })?;
    let obj = LvHandle::from_ptr(raw);
    let state = Box::into_raw(Box::new(WidgetState {
        drop: drop_widget_state::<T>,
        state,
    }))
    .cast();
    unsafe {
        ffi::lv_obj_set_user_data(obj.as_ptr(), state);
        if ffi::lv_obj_add_event_cb(
            obj.as_ptr(),
            Some(callback),
            ffi::lv_event_code_t::LV_EVENT_ALL,
            core::ptr::null_mut(),
        )
        .is_null()
        {
            ffi::lv_obj_delete(obj.as_ptr());
            return None;
        }
        ffi::lv_obj_class_init_obj(obj.as_ptr());
    }
    Some(obj)
}

// C and T must match the class/state pair passed to create_widget.
pub(crate) unsafe fn widget_state<C: class::LvClass, T>(obj: *mut ffi::lv_obj_t) -> Rc<T> {
    assert!(unsafe { ffi::lv_obj_has_class(obj, C::class_ptr()) });
    let ptr = unsafe { ffi::lv_obj_get_user_data(obj) }.cast::<WidgetState<T>>();
    assert!(!ptr.is_null(), "widget has been deleted");
    Rc::clone(unsafe { &(*ptr).state })
}

unsafe extern "C" fn on_delete_drop_attachment<T>(e: *mut ffi::lv_event_t) {
    if unsafe { ffi::lv_event_get_code(e) } != ffi::lv_event_code_t::LV_EVENT_DELETE {
        return;
    }

    let user_data = unsafe { ffi::lv_event_get_user_data(e) };
    if user_data.is_null() {
        return;
    }

    drop(unsafe { Box::from_raw(user_data as *mut T) });
}

unsafe extern "C" fn event_callback_trampoline(e: *mut ffi::lv_event_t) {
    let user_data = unsafe { ffi::lv_event_get_user_data(e) };
    if user_data.is_null() {
        return;
    }

    let callback_ptr = user_data.cast::<EventCallback>().cast_const();
    unsafe {
        Rc::increment_strong_count(callback_ptr);
    }
    let callback = unsafe { Rc::from_raw(callback_ptr) };
    let mut callback = callback.borrow_mut();
    callback.as_mut()(e);
}

unsafe extern "C" fn on_delete_drop_event_callback(e: *mut ffi::lv_event_t) {
    if unsafe { ffi::lv_event_get_code(e) } != ffi::lv_event_code_t::LV_EVENT_DELETE {
        return;
    }

    let user_data = unsafe { ffi::lv_event_get_user_data(e) };
    if user_data.is_null() {
        return;
    }

    drop(unsafe { Rc::from_raw(user_data.cast::<EventCallback>().cast_const()) });
}

pub(crate) fn attach_to_object<T, C: class::LvClass>(
    obj: &LvHandle<C>,
    attachment: T,
) -> Result<NonNull<T>, ()> {
    let attachment = Box::new(attachment);
    let attachment_ptr = Box::into_raw(attachment);
    let event = unsafe {
        ffi::lv_obj_add_event_cb(
            obj.as_ptr(),
            Some(on_delete_drop_attachment::<T>),
            ffi::lv_event_code_t::LV_EVENT_DELETE,
            attachment_ptr as *mut c_void,
        )
    };
    if event.is_null() {
        drop(unsafe { Box::from_raw(attachment_ptr) });
        return Err(());
    }
    Ok(unsafe { NonNull::new_unchecked(attachment_ptr) })
}

pub(crate) fn add_event_cb<F>(
    obj: *mut ffi::lv_obj_t,
    filter: ffi::lv_event_code_t,
    mut cb: F,
) -> Result<(), LvEventRegistrationError>
where
    F: FnMut() + 'static,
{
    add_event_cb_with_event(obj, filter, move |_| cb())
}

pub(crate) fn add_event_cb_with_event<F>(
    obj: *mut ffi::lv_obj_t,
    filter: ffi::lv_event_code_t,
    cb: F,
) -> Result<(), LvEventRegistrationError>
where
    F: FnMut(*mut ffi::lv_event_t) + 'static,
{
    let callback: Rc<EventCallback> = Rc::new(RefCell::new(Box::new(cb)));
    let callback_ptr = Rc::into_raw(callback);
    let user_data = callback_ptr.cast_mut().cast::<c_void>();

    let event_dsc = unsafe {
        ffi::lv_obj_add_event_cb(obj, Some(event_callback_trampoline), filter, user_data)
    };
    if event_dsc.is_null() {
        drop(unsafe { Rc::from_raw(callback_ptr) });
        return Err(LvEventRegistrationError::RegistrationFailed);
    }

    let cleanup_dsc = unsafe {
        ffi::lv_obj_add_event_cb(
            obj,
            Some(on_delete_drop_event_callback),
            ffi::lv_event_code_t::LV_EVENT_DELETE,
            user_data,
        )
    };
    if cleanup_dsc.is_null() {
        let removed = unsafe { ffi::lv_obj_remove_event_dsc(obj, event_dsc) };
        assert!(removed, "failed to roll back event registration");
        drop(unsafe { Rc::from_raw(callback_ptr) });
        return Err(LvEventRegistrationError::RegistrationFailed);
    }

    Ok(())
}
