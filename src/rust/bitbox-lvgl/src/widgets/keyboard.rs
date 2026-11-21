// SPDX-License-Identifier: Apache-2.0

//! A Rust reimplementation of LVGL's keyboard widget using `lv_buttonmatrix`.
//!
//! Edits [`BitboxTextarea`] through Rust event handlers.

use alloc::{rc::Rc, vec::Vec};
use core::{cell::RefCell, ptr::NonNull};

use super::buttonmatrix::{ButtonmatrixExt, LvButtonmatrix, LvButtonmatrixMapEntry, validate_map};
use super::textarea::{BitboxTextarea, TextareaExt};
use crate::{LvButtonmatrixCtrl, LvHandle, LvObj, ObjExt, ZeroizingText, class, ffi};

pub type BitboxKeyboard = LvHandle<class::KeyboardTag>;
pub type BitboxKeyboardMapEntry = LvButtonmatrixMapEntry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitboxKeyboardMode {
    Number,
    User1,
    User2,
    User3,
    User4,
}

#[derive(Clone, Copy)]
struct Map {
    keys: &'static [BitboxKeyboardMapEntry],
    controls: &'static [LvButtonmatrixCtrl],
}

struct State {
    mode: BitboxKeyboardMode,
    maps: [Option<Map>; 5],
}

const fn symbol(bytes: &'static [u8]) -> BitboxKeyboardMapEntry {
    match core::ffi::CStr::from_bytes_with_nul(bytes) {
        Ok(text) => BitboxKeyboardMapEntry::new(text),
        Err(_) => panic!("invalid LVGL symbol"),
    }
}

const NUMBER_MAP: Map = Map {
    keys: &[
        BitboxKeyboardMapEntry::new(c"1"),
        BitboxKeyboardMapEntry::new(c"2"),
        BitboxKeyboardMapEntry::new(c"3"),
        symbol(ffi::LV_SYMBOL_KEYBOARD),
        BitboxKeyboardMapEntry::new(c"\n"),
        BitboxKeyboardMapEntry::new(c"4"),
        BitboxKeyboardMapEntry::new(c"5"),
        BitboxKeyboardMapEntry::new(c"6"),
        symbol(ffi::LV_SYMBOL_OK),
        BitboxKeyboardMapEntry::new(c"\n"),
        BitboxKeyboardMapEntry::new(c"7"),
        BitboxKeyboardMapEntry::new(c"8"),
        BitboxKeyboardMapEntry::new(c"9"),
        symbol(ffi::LV_SYMBOL_BACKSPACE),
        BitboxKeyboardMapEntry::new(c"\n"),
        BitboxKeyboardMapEntry::new(c"+/-"),
        BitboxKeyboardMapEntry::new(c"0"),
        BitboxKeyboardMapEntry::new(c"."),
        symbol(ffi::LV_SYMBOL_LEFT),
        symbol(ffi::LV_SYMBOL_RIGHT),
        BitboxKeyboardMapEntry::new(c""),
    ],
    controls: &number_controls(),
};

const fn number_controls() -> [LvButtonmatrixCtrl; 17] {
    let mut controls = [LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_WIDTH_1; 17];
    controls[3] = LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_WIDTH_2;
    controls[7] = LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_WIDTH_2;
    controls[11] = LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_WIDTH_2;
    controls
}

fn state(obj: *mut ffi::lv_obj_t) -> Rc<RefCell<State>> {
    unsafe { super::util::widget_state::<class::KeyboardTag, RefCell<State>>(obj) }
}

pub trait KeyboardExt: ButtonmatrixExt {
    fn set_mode(&self, mode: BitboxKeyboardMode) {
        let map = {
            let state = state(self.as_ptr());
            let mut state = state.borrow_mut();
            let map = state.maps[mode as usize].expect("keyboard mode has no map");
            state.mode = mode;
            map
        };
        ButtonmatrixExt::set_map(self, map.keys);
        self.set_ctrl_map(map.controls);
        if mode == BitboxKeyboardMode::Number {
            for id in [3, 7] {
                for ctrl in [
                    LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_NO_REPEAT,
                    LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_CLICK_TRIG,
                    LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_CHECKED,
                ] {
                    self.set_button_ctrl(id, ctrl);
                }
            }
        }
    }

    /// `map` must be a `'static` LVGL keyboard map terminated by an empty string.
    fn set_map(
        &self,
        mode: BitboxKeyboardMode,
        map: &'static [BitboxKeyboardMapEntry],
        ctrl_map: &'static [LvButtonmatrixCtrl],
    ) {
        let button_count = validate_map(map);
        assert_eq!(
            ctrl_map.len(),
            button_count,
            "LVGL keyboard control map length must match the number of buttons in the map"
        );
        state(self.as_ptr()).borrow_mut().maps[mode as usize] = Some(Map {
            keys: map,
            controls: ctrl_map,
        });
        if self.get_mode() == mode {
            self.set_mode(mode);
        }
    }

    fn get_mode(&self) -> BitboxKeyboardMode {
        state(self.as_ptr()).borrow().mode
    }

    fn get_map_array(&self) -> Vec<ZeroizingText> {
        self.get_map()
    }
}

impl LvHandle<class::KeyboardTag> {
    /// Adds input handling for a Rust textarea. Deleting the textarea disables the binding.
    pub fn bind_textarea(
        &self,
        textarea: &BitboxTextarea,
    ) -> Result<(), crate::LvEventRegistrationError> {
        let textarea = textarea.weak_handle();
        let keyboard = Self::from_ptr(NonNull::new(self.as_ptr()).unwrap());
        self.add_event_cb(crate::LvEventCode::LV_EVENT_VALUE_CHANGED, move || {
            let Some(text) = keyboard.get_button_text(keyboard.get_selected_button()) else {
                return;
            };
            let text = text.to_str().unwrap();
            let symbol = |bytes: &[u8]| {
                core::ffi::CStr::from_bytes_with_nul(bytes)
                    .unwrap()
                    .to_bytes()
                    == text.as_bytes()
            };
            let event = if text == "Enter"
                || symbol(ffi::LV_SYMBOL_NEW_LINE)
                || symbol(ffi::LV_SYMBOL_OK)
            {
                Some(crate::LvEventCode::LV_EVENT_READY)
            } else if symbol(ffi::LV_SYMBOL_CLOSE) || symbol(ffi::LV_SYMBOL_KEYBOARD) {
                Some(crate::LvEventCode::LV_EVENT_CANCEL)
            } else {
                None
            };
            if let Some(event) = event {
                if unsafe {
                    ffi::lv_obj_send_event(keyboard.as_ptr(), event, core::ptr::null_mut())
                } != ffi::lv_result_t::LV_RESULT_OK
                {
                    return;
                }
                if let Some(textarea) = textarea() {
                    unsafe {
                        ffi::lv_obj_send_event(textarea.as_ptr(), event, core::ptr::null_mut());
                    }
                }
                return;
            }
            let Some(textarea) = textarea() else { return };
            if symbol(ffi::LV_SYMBOL_LEFT) {
                textarea.cursor_left();
            } else if symbol(ffi::LV_SYMBOL_RIGHT) {
                textarea.cursor_right();
            } else if symbol(ffi::LV_SYMBOL_BACKSPACE) {
                textarea.delete_char();
            } else if text == "+/-" {
                textarea.toggle_sign();
            } else {
                textarea.add_text(text).unwrap();
            }
        })?;
        Ok(())
    }

    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        unsafe extern "C" fn event(_: *mut ffi::lv_event_t) {}
        let keyboard: Self = unsafe {
            super::util::create_widget(
                parent,
                Rc::new(RefCell::new(State {
                    mode: BitboxKeyboardMode::Number,
                    maps: [Some(NUMBER_MAP), None, None, None, None],
                })),
                event,
            )
        }?;
        keyboard.remove_flag(crate::LvObjFlag::LV_OBJ_FLAG_CLICK_FOCUSABLE);
        keyboard.set_mode(BitboxKeyboardMode::Number);
        Some(keyboard)
    }

    pub fn get_selected_button(&self) -> u32 {
        ButtonmatrixExt::get_selected_button(self)
    }

    pub fn get_button_text(&self, button_id: u32) -> Option<ZeroizingText> {
        ButtonmatrixExt::get_button_text(self, button_id)
    }

    pub fn to_buttonmatrix(self) -> LvButtonmatrix {
        self.cast()
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl KeyboardExt for LvHandle<class::KeyboardTag> {}

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use core::ptr::NonNull;

    use super::{BitboxKeyboard, BitboxKeyboardMapEntry, KeyboardExt, symbol};
    use crate::{BitboxKeyboardMode, BitboxTextarea, ObjExt, TextareaExt, ffi};
    use crate::{LvButtonmatrixCtrl, LvHandle, class};

    #[test]
    fn test_bind_textarea_input_and_deletion() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(320, 240).unwrap();
        let screen = display.screen_active().unwrap();
        let textarea = BitboxTextarea::new(&screen).unwrap();
        let keyboard = BitboxKeyboard::new(&screen).unwrap();
        keyboard.bind_textarea(&textarea).unwrap();
        const MAP: &[BitboxKeyboardMapEntry] = &[
            BitboxKeyboardMapEntry::new(c"ö"),
            symbol(ffi::LV_SYMBOL_LEFT),
            symbol(ffi::LV_SYMBOL_BACKSPACE),
            BitboxKeyboardMapEntry::new(c"+/-"),
            BitboxKeyboardMapEntry::new(c""),
        ];
        keyboard.set_map(
            BitboxKeyboardMode::User1,
            MAP,
            &[LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_WIDTH_1; 4],
        );
        keyboard.set_mode(BitboxKeyboardMode::User1);
        let press = |id| {
            crate::ButtonmatrixExt::set_selected_button(&keyboard, id);
            unsafe {
                ffi::lv_obj_send_event(
                    keyboard.as_ptr(),
                    crate::LvEventCode::LV_EVENT_VALUE_CHANGED,
                    core::ptr::null_mut(),
                );
            }
        };
        textarea.set_text("ab").unwrap();
        press(1);
        press(0);
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "aöb");
        press(2);
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "ab");
        press(3);
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "-ab");
        press(3);
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "+ab");
        unsafe { textarea.delete() };
        press(0);
        unsafe { keyboard.delete() };
    }

    fn dummy_keyboard() -> BitboxKeyboard {
        LvHandle::<class::KeyboardTag>::from_ptr(NonNull::dangling())
    }

    #[test]
    fn test_bind_textarea_number_mode_and_ready_deletion() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(320, 240).unwrap();
        let screen = display.screen_active().unwrap();
        let parent = crate::LvObj::with_parent(&screen).unwrap();
        let textarea = BitboxTextarea::new(&parent).unwrap();
        textarea.set_accepted_chars(Some(c"0123456789"));
        let keyboard = BitboxKeyboard::new(&parent).unwrap();
        keyboard.bind_textarea(&textarea).unwrap();
        let press = |id| {
            crate::ButtonmatrixExt::set_selected_button(&keyboard, id);
            unsafe {
                ffi::lv_obj_send_event(
                    keyboard.as_ptr(),
                    crate::LvEventCode::LV_EVENT_VALUE_CHANGED,
                    core::ptr::null_mut(),
                )
            };
        };
        press(0);
        press(4);
        press(12);
        press(14);
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "14");
        press(11);
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "1");
        let live = textarea.weak_handle();
        keyboard
            .add_event_cb(crate::LvEventCode::LV_EVENT_READY, move || {
                unsafe { ffi::lv_obj_delete(parent.as_ptr()) };
            })
            .unwrap();
        press(7);
        assert!(live().is_none());
    }

    #[test]
    #[should_panic(
        expected = "LVGL keyboard control map length must match the number of buttons in the map"
    )]
    fn test_set_map_rejects_incorrect_ctrl_map_length() {
        let keyboard = dummy_keyboard();
        let map: &'static [BitboxKeyboardMapEntry] = Box::leak(Box::new([
            BitboxKeyboardMapEntry::new(c"1"),
            BitboxKeyboardMapEntry::new(c"2"),
            BitboxKeyboardMapEntry::new(c""),
        ]));
        let ctrl_map: &'static [LvButtonmatrixCtrl] =
            Box::leak(Box::new([LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_WIDTH_1]));

        keyboard.set_map(BitboxKeyboardMode::User1, map, ctrl_map);
    }
}
