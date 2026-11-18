// SPDX-License-Identifier: Apache-2.0

use alloc::{rc::Rc, string::String, vec};
use core::{cell::RefCell, ffi::CStr};
use zeroize::Zeroizing;

use super::label::{LvLabel, char_offset};
use crate::{LabelExt, LvHandle, LvObj, ObjExt, ZeroizingText, class, ffi};

pub type LvTextareaTextError = super::LvTextError;
/// A single-line text editor with Rust-owned, zeroizing text storage.
pub type LvTextarea = LvHandle<class::TextareaTag>;

struct State {
    deleted: bool,
    text: Rc<ZeroizingText>,
    cursor: usize,
    accepted: Option<&'static CStr>,
    max_length: usize,
    password: bool,
    bullet: Rc<ZeroizingText>,
    scroll_x: i32,
}

type TextareaState = RefCell<State>;

fn state(obj: *mut ffi::lv_obj_t) -> Rc<TextareaState> {
    unsafe { super::util::widget_state::<class::TextareaTag, TextareaState>(obj) }
}

fn handle(obj: *mut ffi::lv_obj_t) -> LvTextarea {
    LvHandle::from_ptr(core::ptr::NonNull::new(obj).unwrap())
}

fn refresh_cursor(textarea: &LvTextarea) {
    let Some(label) = textarea.get_label() else {
        return;
    };
    let Some(cursor) = textarea.child(1) else {
        return;
    };
    unsafe { ffi::lv_obj_update_layout(textarea.as_ptr()) };
    let state = state(textarea.as_ptr());
    let position = state.borrow().cursor;
    let point = label.letter_position(position);
    let width = unsafe { ffi::lv_obj_get_content_width(textarea.as_ptr()) }.max(1);
    let scroll_x = {
        let mut state = state.borrow_mut();
        state.scroll_x = state.scroll_x.min(point.x).max(point.x - width + 2).max(0);
        state.scroll_x
    };
    label.set_pos(-scroll_x, 0);
    cursor.set_pos(point.x - scroll_x, point.y);
    unsafe {
        let font = ffi::lv_obj_get_style_prop(
            label.as_ptr(),
            crate::LvPart::LV_PART_MAIN,
            ffi::_lv_style_id_t::LV_STYLE_TEXT_FONT as ffi::lv_style_prop_t,
        )
        .ptr
        .cast::<ffi::lv_font_t>();
        cursor.set_height((*font).line_height);
        let color = ffi::lv_obj_get_style_prop(
            label.as_ptr(),
            crate::LvPart::LV_PART_MAIN,
            ffi::_lv_style_id_t::LV_STYLE_TEXT_COLOR as ffi::lv_style_prop_t,
        )
        .color;
        cursor.set_style_bg_color(color, 0);
    }
}

fn refresh_text(textarea: &LvTextarea) {
    let Some(label) = textarea.get_label() else {
        return;
    };
    let (text, password, bullet) = {
        let state = state(textarea.as_ptr());
        let state = state.borrow();
        (
            Rc::clone(&state.text),
            state.password,
            Rc::clone(&state.bullet),
        )
    };
    let text = text.to_str().unwrap();
    if password {
        let bullet = bullet.to_str().unwrap();
        let count = text.chars().count();
        let mut masked = Zeroizing::new(String::with_capacity(count * bullet.len()));
        for _ in 0..count {
            masked.push_str(bullet);
        }
        label.set_text(&masked).unwrap();
    } else {
        label.set_text(text).unwrap();
    }
    refresh_cursor(textarea);
}

unsafe extern "C" fn textarea_event(event: *mut ffi::lv_event_t) {
    let obj = unsafe { ffi::lv_event_get_current_target(event) }.cast::<ffi::lv_obj_t>();
    let textarea = handle(obj);
    match unsafe { ffi::lv_event_get_code(event) } {
        crate::LvEventCode::LV_EVENT_DELETE => {
            // Child labels wipe their own displayed text when LVGL deletes them next.
            let state = state(obj);
            state.borrow_mut().deleted = true;
            state.borrow_mut().text = Rc::new(ZeroizingText::new("").unwrap());
        }
        crate::LvEventCode::LV_EVENT_SIZE_CHANGED | crate::LvEventCode::LV_EVENT_STYLE_CHANGED => {
            refresh_cursor(&textarea);
        }
        crate::LvEventCode::LV_EVENT_PRESSED => {
            let indev = unsafe { ffi::lv_indev_active() };
            if indev.is_null() {
                return;
            }
            let Some(label) = textarea.get_label() else {
                return;
            };
            let mut point = crate::LvPoint { x: 0, y: 0 };
            let mut area = crate::LvArea {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
            };
            unsafe {
                ffi::lv_indev_get_point(indev, &mut point);
                ffi::lv_obj_get_coords(label.as_ptr(), &mut area);
            }
            point.x -= area.x1;
            point.y -= area.y1;
            textarea.set_cursor_pos(label.letter_at(point.x));
        }
        crate::LvEventCode::LV_EVENT_FOCUSED => {
            if let Some(cursor) = textarea.child(1) {
                cursor.remove_flag(crate::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
            }
        }
        crate::LvEventCode::LV_EVENT_DEFOCUSED => {
            if let Some(cursor) = textarea.child(1) {
                cursor.add_flag(crate::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
            }
        }
        _ => {}
    }
}

fn replace_text(textarea: &LvTextarea, text: &str, cursor: usize) {
    let text = Rc::new(ZeroizingText::new(text).unwrap());
    {
        let state = state(textarea.as_ptr());
        let mut state = state.borrow_mut();
        state.text = text;
        state.cursor = cursor;
    }
    refresh_text(textarea);
    unsafe {
        ffi::lv_obj_send_event(
            textarea.as_ptr(),
            crate::LvEventCode::LV_EVENT_VALUE_CHANGED,
            core::ptr::null_mut(),
        );
    }
}

fn insert_text(
    textarea: &LvTextarea,
    text: &str,
    replace: bool,
) -> Result<(), LvTextareaTextError> {
    if text.as_bytes().contains(&0) {
        return Err(LvTextareaTextError::ContainsNul);
    }
    let (current, cursor, accepted, max_length) = {
        let state = state(textarea.as_ptr());
        let state = state.borrow();
        (
            Rc::clone(&state.text),
            state.cursor,
            state.accepted,
            state.max_length,
        )
    };
    let current = if replace {
        ""
    } else {
        current.to_str().unwrap()
    };
    let cursor = if replace { 0 } else { cursor };
    let offset = char_offset(current, cursor);
    let available = max_length.saturating_sub(current.chars().count());
    let accepted = accepted.map(|chars| chars.to_str().expect("accepted characters must be UTF-8"));
    let mut result = Zeroizing::new(String::with_capacity(current.len() + text.len()));
    result.push_str(&current[..offset]);
    let mut added = 0;
    for (offset, ch) in text.char_indices() {
        if added == available {
            break;
        }
        if matches!(ch, '\n' | '\r') || accepted.is_some_and(|chars| !chars.contains(ch)) {
            continue;
        }
        result.push_str(&text[offset..offset + ch.len_utf8()]);
        added += 1;
    }
    result.push_str(&current[offset..]);
    if replace || added > 0 {
        replace_text(textarea, &result, cursor + added);
    }
    Ok(())
}

pub trait TextareaExt: ObjExt {
    fn add_char(&self, ch: char) {
        let mut bytes = Zeroizing::new(vec![0; ch.len_utf8()].into_boxed_slice());
        let text = ch.encode_utf8(&mut bytes);
        self.add_text(text).unwrap();
    }

    fn add_text(&self, text: &str) -> Result<(), LvTextareaTextError> {
        insert_text(&handle(self.as_ptr()), text, false)
    }

    fn set_text(&self, text: &str) -> Result<(), LvTextareaTextError> {
        insert_text(&handle(self.as_ptr()), text, true)
    }

    fn delete_char(&self) {
        let (text, cursor) = {
            let state = state(self.as_ptr());
            let state = state.borrow();
            (Rc::clone(&state.text), state.cursor)
        };
        if cursor == 0 {
            return;
        }
        let text = text.to_str().unwrap();
        let start = char_offset(text, cursor - 1);
        let end = char_offset(text, cursor);
        let mut result = Zeroizing::new(String::with_capacity(text.len() - (end - start)));
        result.push_str(&text[..start]);
        result.push_str(&text[end..]);
        replace_text(&handle(self.as_ptr()), &result, cursor - 1);
    }

    fn delete_char_forward(&self) {
        let cursor = self.get_cursor_pos();
        if cursor < self.with_text(|text| text.chars().count()) {
            self.set_cursor_pos(cursor + 1);
            self.delete_char();
        }
    }

    fn set_cursor_pos(&self, position: usize) {
        {
            let state = state(self.as_ptr());
            let mut state = state.borrow_mut();
            state.cursor = position.min(state.text.to_str().unwrap().chars().count());
        }
        refresh_cursor(&handle(self.as_ptr()));
    }

    fn get_cursor_pos(&self) -> usize {
        state(self.as_ptr()).borrow().cursor
    }

    fn cursor_left(&self) {
        self.set_cursor_pos(self.get_cursor_pos().saturating_sub(1));
    }

    fn cursor_right(&self) {
        self.set_cursor_pos(self.get_cursor_pos() + 1);
    }

    fn set_password_mode(&self, enable: bool) {
        state(self.as_ptr()).borrow_mut().password = enable;
        refresh_text(&handle(self.as_ptr()));
    }

    fn set_password_bullet(&self, bullet: &str) -> Result<(), LvTextareaTextError> {
        let text = ZeroizingText::new(bullet)?;
        assert_eq!(
            bullet.chars().count(),
            1,
            "password bullet must be one character"
        );
        state(self.as_ptr()).borrow_mut().bullet = Rc::new(text);
        refresh_text(&handle(self.as_ptr()));
        Ok(())
    }

    fn set_accepted_chars(&self, accepted: Option<&'static CStr>) {
        if let Some(chars) = accepted {
            chars.to_str().expect("accepted characters must be UTF-8");
        }
        state(self.as_ptr()).borrow_mut().accepted = accepted;
    }

    fn set_max_length(&self, max_length: usize) {
        state(self.as_ptr()).borrow_mut().max_length = if max_length == 0 {
            usize::MAX
        } else {
            max_length
        };
    }

    fn get_text(&self) -> Option<ZeroizingText> {
        Some(ZeroizingText::from_cstr(
            &state(self.as_ptr()).borrow().text,
        ))
    }

    /// Reads through an owned snapshot, so the callback can also update the textarea.
    fn with_text<R>(&self, f: impl FnOnce(&str) -> R) -> R {
        let text = self.get_text().unwrap();
        f(text.to_str().unwrap())
    }

    fn get_label(&self) -> Option<LvLabel> {
        self.child(0)?.try_downcast::<class::LabelTag>().ok()
    }
}

impl LvHandle<class::TextareaTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        let state = Rc::new(RefCell::new(State {
            deleted: false,
            text: Rc::new(ZeroizingText::new("").unwrap()),
            cursor: 0,
            accepted: None,
            max_length: usize::MAX,
            password: false,
            bullet: Rc::new(ZeroizingText::new("*").unwrap()),
            scroll_x: 0,
        }));
        let textarea: Self = unsafe { super::util::create_widget(parent, state, textarea_event) }?;
        textarea.remove_flag(crate::LvObjFlag::LV_OBJ_FLAG_SCROLLABLE);
        let Some(label) = LvLabel::new(&textarea) else {
            unsafe { textarea.delete() };
            return None;
        };
        label.set_long_mode(crate::LvLabelLongMode::Clip);
        let Some(cursor) = LvObj::with_parent(&textarea) else {
            unsafe { textarea.delete() };
            return None;
        };
        cursor.set_width(2);
        cursor.set_style_border_width(0, 0);
        cursor.set_style_radius(0, 0);
        cursor.set_style_bg_opa(crate::LvOpacityLevel::LV_OPA_COVER as u8, 0);
        cursor.remove_flag(crate::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
        cursor.remove_flag(crate::LvObjFlag::LV_OBJ_FLAG_SCROLLABLE);
        cursor.add_flag(crate::LvObjFlag::LV_OBJ_FLAG_IGNORE_LAYOUT);
        refresh_cursor(&textarea);
        Some(textarea)
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }

    pub(crate) fn toggle_sign(&self) {
        let (text, cursor, accepted, max_length) = {
            let state = state(self.as_ptr());
            let state = state.borrow();
            (
                Rc::clone(&state.text),
                state.cursor,
                state.accepted,
                state.max_length,
            )
        };
        let text = text.to_str().unwrap();
        let sign = if text.starts_with('-') { "+" } else { "-" };
        if accepted.is_some_and(|chars| !chars.to_str().unwrap().contains(sign)) {
            return;
        }
        let has_sign = text.starts_with(['-', '+']);
        if !has_sign && text.chars().count() >= max_length {
            return;
        }
        let mut result = Zeroizing::new(String::with_capacity(text.len() + usize::from(!has_sign)));
        result.push_str(sign);
        result.push_str(if has_sign { &text[1..] } else { text });
        replace_text(self, &result, cursor + usize::from(!has_sign));
    }

    pub(crate) fn weak_handle(&self) -> impl Fn() -> Option<Self> + 'static {
        let state = Rc::downgrade(&state(self.as_ptr()));
        let obj = self.as_ptr();
        move || {
            let state = state.upgrade()?;
            if state.borrow().deleted {
                None
            } else {
                Some(handle(obj))
            }
        }
    }
}

impl TextareaExt for LvHandle<class::TextareaTag> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_text_filters_limits_and_events() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let textarea = LvTextarea::new(&screen).unwrap();
        let alias = screen
            .child(0)
            .unwrap()
            .try_downcast::<class::TextareaTag>()
            .unwrap();
        let changes = Rc::new(core::cell::Cell::new(0));
        let changes_cb = Rc::clone(&changes);
        textarea
            .add_event_cb(crate::LvEventCode::LV_EVENT_VALUE_CHANGED, move || {
                changes_cb.set(changes_cb.get() + 1);
                assert!(alias.with_text(|text| text.chars().all(|ch| ch.is_ascii_digit())));
                alias.set_cursor_pos(alias.get_cursor_pos());
            })
            .unwrap();
        textarea.set_accepted_chars(Some(c"0123456789"));
        textarea.set_max_length(3);
        textarea.add_text("a12\n3z4").unwrap();
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "123");
        assert_eq!(changes.get(), 1);
        textarea.add_text("5").unwrap();
        assert_eq!(changes.get(), 1);
        assert_eq!(
            textarea.set_text("9\0x"),
            Err(LvTextareaTextError::ContainsNul)
        );
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "123");
        textarea.set_cursor_pos(1);
        textarea.delete_char();
        textarea.add_text("7").unwrap();
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "723");
        assert_eq!(textarea.get_cursor_pos(), 1);
        assert_eq!(changes.get(), 3);
        unsafe { textarea.delete() };
    }

    #[test]
    fn test_delete_char_and_mask_utf8() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let textarea = LvTextarea::new(&screen).unwrap();
        textarea.set_max_length(4);
        textarea.set_text("aö🙂z").unwrap();
        textarea.set_password_mode(true);
        textarea.set_password_bullet("#").unwrap();
        assert_eq!(
            textarea
                .get_label()
                .unwrap()
                .get_text()
                .unwrap()
                .to_str()
                .unwrap(),
            "####"
        );
        textarea.set_cursor_pos(2);
        textarea.delete_char_forward();
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "aöz");
        textarea.delete_char();
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "az");
        textarea.add_text("ä🙂extra").unwrap();
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "aä🙂z");
        textarea.set_password_mode(false);
        assert_eq!(
            textarea
                .get_label()
                .unwrap()
                .get_text()
                .unwrap()
                .to_str()
                .unwrap(),
            "aä🙂z"
        );
        textarea.set_cursor_pos(0);
        textarea.delete_char();
        assert_eq!(textarea.get_text().unwrap().to_str().unwrap(), "aä🙂z");
        unsafe { textarea.delete() };
    }

    #[test]
    fn test_delete_releases_text_and_invalidates_weak_handles() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let parent = LvObj::with_parent(&screen).unwrap();
        let textarea = LvTextarea::new(&parent).unwrap();
        textarea.set_text("sensitive input").unwrap();
        let state = state(textarea.as_ptr());
        let text = Rc::downgrade(&state.borrow().text);
        let weak_handle = textarea.weak_handle();
        let snapshot = textarea.get_text().unwrap();
        assert!(weak_handle().is_some());
        unsafe { parent.delete() };
        assert!(weak_handle().is_none());
        assert!(text.upgrade().is_none());
        assert_eq!(Rc::strong_count(&state), 1);
        assert_eq!(snapshot.to_str().unwrap(), "sensitive input");
    }

    #[test]
    fn test_with_text_allows_replacement() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let textarea = LvTextarea::new(&screen).unwrap();
        assert!(textarea.with_text(str::is_empty));
        textarea.set_text("öäü").unwrap();
        let len = textarea.with_text(|text| {
            textarea.set_text("replacement").unwrap();
            assert_eq!(text, "öäü");
            text.len()
        });
        assert_eq!(len, 6);
        textarea.with_text(|text| assert_eq!(text, "replacement"));
        unsafe { textarea.delete() };
    }
}
