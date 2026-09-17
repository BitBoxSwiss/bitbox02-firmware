// SPDX-License-Identifier: Apache-2.0

//! A Rust reimplementation of LVGL's label widget, built on `lv_obj`.
//!
//! Owns zeroizing text storage and renders through `lv_draw_label`.

use alloc::{boxed::Box, rc::Rc, string::String};
use core::{cell::RefCell, ffi::CStr, ops::Deref};
use zeroize::Zeroizing;

use crate::{LvHandle, LvObj, LvPoint, ObjExt, ZeroizingText, class, ffi};

pub type BitboxLabelTextError = super::LvTextError;
pub type BitboxLabel = LvHandle<class::LabelTag>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitboxLabelLongMode {
    Wrap,
    Clip,
}

#[derive(Clone)]
enum Text {
    Owned(Rc<ZeroizingText>),
    Static(&'static CStr),
}

impl Deref for Text {
    type Target = CStr;
    fn deref(&self) -> &CStr {
        match self {
            Self::Owned(text) => text,
            Self::Static(text) => text,
        }
    }
}

struct State {
    text: Text,
    mode: BitboxLabelLongMode,
}

type TextState = RefCell<State>;

fn text_state(obj: *mut ffi::lv_obj_t) -> Rc<TextState> {
    unsafe { super::util::widget_state::<class::LabelTag, TextState>(obj) }
}

fn draw_descriptor(
    obj: *mut ffi::lv_obj_t,
    mode: BitboxLabelLongMode,
    layer: *mut ffi::lv_layer_t,
) -> ffi::lv_draw_label_dsc_t {
    let mut dsc = ffi::lv_draw_label_dsc_t::default();
    unsafe {
        ffi::lv_draw_label_dsc_init(&mut dsc);
        dsc.base.layer = layer;
        ffi::lv_obj_init_draw_label_dsc(obj, crate::LvPart::LV_PART_MAIN, &mut dsc);
        // The draw initializer skips these properties when text opacity is zero.
        dsc.font = ffi::lv_obj_get_style_prop(
            obj,
            crate::LvPart::LV_PART_MAIN,
            ffi::_lv_style_id_t::LV_STYLE_TEXT_FONT as ffi::lv_style_prop_t,
        )
        .ptr
        .cast();
    }
    dsc.letter_space = style_num(obj, ffi::_lv_style_id_t::LV_STYLE_TEXT_LETTER_SPACE);
    dsc.line_space = style_num(obj, ffi::_lv_style_id_t::LV_STYLE_TEXT_LINE_SPACE);
    if mode == BitboxLabelLongMode::Clip {
        dsc.set_flag(ffi::lv_text_flag_t::LV_TEXT_FLAG_EXPAND);
    }
    dsc
}

fn style_num(obj: *mut ffi::lv_obj_t, property: ffi::_lv_style_id_t) -> i32 {
    unsafe {
        ffi::lv_obj_get_style_prop(
            obj,
            crate::LvPart::LV_PART_MAIN,
            property as ffi::lv_style_prop_t,
        )
        .num
    }
}

unsafe extern "C" fn release_draw_text(event: *mut ffi::lv_event_t) {
    let code = unsafe { ffi::lv_event_get_code(event) };
    if matches!(
        code,
        crate::LvEventCode::LV_EVENT_RENDER_READY | crate::LvEventCode::LV_EVENT_DELETE
    ) {
        unsafe {
            let display = ffi::lv_event_get_current_target(event).cast();
            let text = ffi::lv_event_get_user_data(event);
            ffi::lv_display_remove_event_cb_with_user_data(display, Some(release_draw_text), text);
            drop(Box::from_raw(text.cast::<Rc<ZeroizingText>>()));
        }
    }
}

fn retain_draw_text(obj: *mut ffi::lv_obj_t, text: &Text) {
    if let Text::Owned(text) = text {
        // Draw tasks can outlive this callback, including replacement of the label text.
        let text = Box::into_raw(Box::new(Rc::clone(text))).cast();
        unsafe {
            ffi::lv_display_add_event_cb(
                ffi::lv_obj_get_display(obj),
                Some(release_draw_text),
                crate::LvEventCode::LV_EVENT_ALL,
                text,
            );
        }
    }
}

fn refresh(obj: *mut ffi::lv_obj_t) {
    unsafe {
        ffi::lv_obj_refresh_self_size(obj);
        ffi::lv_obj_invalidate(obj);
    }
}

unsafe extern "C" fn label_event(event: *mut ffi::lv_event_t) {
    let obj = unsafe { ffi::lv_event_get_current_target(event) }.cast::<ffi::lv_obj_t>();
    let code = unsafe { ffi::lv_event_get_code(event) };
    if code == crate::LvEventCode::LV_EVENT_DELETE {
        text_state(obj).borrow_mut().text = Text::Static(c"");
        return;
    }
    if matches!(
        code,
        crate::LvEventCode::LV_EVENT_STYLE_CHANGED | crate::LvEventCode::LV_EVENT_SIZE_CHANGED
    ) {
        refresh(obj);
        return;
    }
    if !matches!(
        code,
        crate::LvEventCode::LV_EVENT_GET_SELF_SIZE
            | crate::LvEventCode::LV_EVENT_DRAW_MAIN
            | crate::LvEventCode::LV_EVENT_REFR_EXT_DRAW_SIZE
    ) {
        return;
    }
    let (text, mode) = {
        let state = text_state(obj);
        let state = state.borrow();
        (state.text.clone(), state.mode)
    };
    let layer = if code == crate::LvEventCode::LV_EVENT_DRAW_MAIN {
        unsafe { ffi::lv_event_get_layer(event) }
    } else {
        core::ptr::null_mut()
    };
    let mut dsc = draw_descriptor(obj, mode, layer);
    if code == crate::LvEventCode::LV_EVENT_REFR_EXT_DRAW_SIZE {
        unsafe { ffi::lv_event_set_ext_draw_size(event, (*dsc.font).line_height / 4) };
    } else if code == crate::LvEventCode::LV_EVENT_GET_SELF_SIZE {
        let width = style_num(obj, ffi::_lv_style_id_t::LV_STYLE_WIDTH);
        let grow = style_num(obj, ffi::_lv_style_id_t::LV_STYLE_FLEX_GROW);
        let max_width = if width == ffi::LV_SIZE_CONTENT as i32 && grow == 0 {
            ffi::LV_COORD_MAX as i32
        } else {
            unsafe { ffi::lv_obj_get_content_width(obj) }.max(0)
        }
        .min(style_num(obj, ffi::_lv_style_id_t::LV_STYLE_MAX_WIDTH));
        let mut size = LvPoint { x: 0, y: 0 };
        unsafe {
            ffi::lv_text_get_size(
                &mut size,
                text.as_ptr(),
                dsc.font,
                dsc.letter_space,
                dsc.line_space,
                max_width,
                dsc.flag(),
            );
            size.y = size
                .y
                .min(style_num(obj, ffi::_lv_style_id_t::LV_STYLE_MAX_HEIGHT));
            let result = &mut *ffi::lv_event_get_param(event).cast::<LvPoint>();
            result.x = result.x.max(size.x);
            result.y = result.y.max(size.y);
        }
    } else {
        let mut area = crate::LvArea {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        };
        unsafe {
            ffi::lv_obj_get_content_coords(obj, &mut area);
            dsc.text = text.as_ptr();
            dsc.set_text_static(u8::from(matches!(text, Text::Static(_))));
            let original_clip = (*layer)._clip_area;
            let clip = crate::LvArea {
                x1: area.x1.max(original_clip.x1),
                y1: area.y1.max(original_clip.y1),
                x2: area.x2.min(original_clip.x2),
                y2: area.y2.min(original_clip.y2),
            };
            if clip.x1 > clip.x2
                || clip.y1 > clip.y2
                || text.is_empty()
                || dsc.opa <= ffi::LV_OPA_MIN as u8
            {
                return;
            }
            retain_draw_text(obj, &text);
            if mode == BitboxLabelLongMode::Clip {
                (*layer)._clip_area = clip;
            } else {
                (*layer)._clip_area.y2 = clip.y2;
            }
            ffi::lv_draw_label(dsc.base.layer, &dsc, &area);
            (*layer)._clip_area = original_clip;
        }
    }
}

pub trait LabelExt: ObjExt {
    /// Replaces the zeroizing text buffer and sends `LV_EVENT_VALUE_CHANGED`.
    fn set_text(&self, txt: &str) -> Result<(), BitboxLabelTextError> {
        let text = Rc::new(ZeroizingText::new(txt)?);
        text_state(self.as_ptr()).borrow_mut().text = Text::Owned(text);
        refresh(self.as_ptr());
        unsafe {
            ffi::lv_obj_send_event(
                self.as_ptr(),
                crate::LvEventCode::LV_EVENT_VALUE_CHANGED,
                core::ptr::null_mut(),
            );
        }
        Ok(())
    }

    fn copy_text_from(&self, source: &BitboxLabel) {
        // The setter can dispatch callbacks, so release the source borrow before calling it.
        let text = source.get_text().unwrap();
        self.set_text(text.to_str().unwrap()).unwrap();
    }

    fn set_text_static(&self, text: Option<&'static CStr>) {
        if let Some(text) = text {
            text.to_str().expect("label text must be UTF-8");
            text_state(self.as_ptr()).borrow_mut().text = Text::Static(text);
        }
        refresh(self.as_ptr());
    }

    fn set_long_mode(&self, mode: BitboxLabelLongMode) {
        text_state(self.as_ptr()).borrow_mut().mode = mode;
        refresh(self.as_ptr());
    }

    fn get_long_mode(&self) -> BitboxLabelLongMode {
        text_state(self.as_ptr()).borrow().mode
    }

    fn get_text(&self) -> Option<ZeroizingText> {
        Some(ZeroizingText::from_cstr(
            &text_state(self.as_ptr()).borrow().text,
        ))
    }

    fn ins_text(&self, pos: u32, txt: &str) -> Result<(), BitboxLabelTextError> {
        if txt.as_bytes().contains(&0) {
            return Err(BitboxLabelTextError::ContainsNul);
        }
        let current = self.get_text().unwrap();
        let current = current.to_str().unwrap();
        let offset = char_offset(current, pos as usize);
        let mut text = Zeroizing::new(String::with_capacity(current.len() + txt.len()));
        text.push_str(&current[..offset]);
        text.push_str(txt);
        text.push_str(&current[offset..]);
        self.set_text(&text)
    }

    fn cut_text(&self, pos: u32, count: u32) {
        let current = self.get_text().unwrap();
        let current = current.to_str().unwrap();
        let start = char_offset(current, pos as usize);
        let end = start + char_offset(&current[start..], count as usize);
        let mut text = Zeroizing::new(String::with_capacity(current.len() - (end - start)));
        text.push_str(&current[..start]);
        text.push_str(&current[end..]);
        self.set_text(&text).unwrap();
    }
}

impl LvHandle<class::LabelTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        let label: Self = unsafe {
            super::util::create_widget(
                parent,
                Rc::new(RefCell::new(State {
                    text: Text::Static(c""),
                    mode: BitboxLabelLongMode::Wrap,
                })),
                label_event,
            )
        }?;
        label.remove_flag(crate::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
        label.remove_flag(crate::LvObjFlag::LV_OBJ_FLAG_SCROLLABLE);
        Some(label)
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }

    fn prefix_width(&self, text: &CStr, offset: usize) -> i32 {
        let dsc = draw_descriptor(
            self.as_ptr(),
            BitboxLabelLongMode::Clip,
            core::ptr::null_mut(),
        );
        let attrs = ffi::lv_text_attributes_t {
            letter_space: dsc.letter_space,
            line_space: dsc.line_space,
            max_width: ffi::LV_COORD_MAX as i32,
            text_flags: ffi::lv_text_flag_t::LV_TEXT_FLAG_EXPAND,
        };
        let width = unsafe {
            ffi::lv_text_get_width(text.as_ptr(), offset.try_into().unwrap(), dsc.font, &attrs)
        };
        width + if offset > 0 { dsc.letter_space } else { 0 }
    }

    pub(crate) fn letter_position(&self, index: usize) -> LvPoint {
        assert_eq!(self.get_long_mode(), BitboxLabelLongMode::Clip);
        let text = text_state(self.as_ptr()).borrow().text.clone();
        let offset = char_offset(text.to_str().unwrap(), index);
        LvPoint {
            x: self.prefix_width(&text, offset),
            y: 0,
        }
    }

    pub(crate) fn letter_at(&self, x: i32) -> usize {
        assert_eq!(self.get_long_mode(), BitboxLabelLongMode::Clip);
        let text = text_state(self.as_ptr()).borrow().text.clone();
        let value = text.to_str().unwrap();
        let mut previous = 0;
        let mut index = 0;
        for offset in value
            .char_indices()
            .skip(1)
            .map(|(offset, _)| offset)
            .chain(core::iter::once(value.len()))
        {
            let next = self.prefix_width(&text, offset);
            if x < (previous + next) / 2 {
                return index;
            }
            previous = next;
            index += 1;
        }
        index.min(value.chars().count())
    }
}

impl LabelExt for LvHandle<class::LabelTag> {}

pub(crate) fn char_offset(text: &str, index: usize) -> usize {
    text.char_indices()
        .nth(index)
        .map_or(text.len(), |(offset, _)| offset)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_text_shares_storage_and_releases_it_on_deletion() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let label = BitboxLabel::new(&screen).unwrap();
        let state = text_state(label.as_ptr());
        label.set_text("abstract").unwrap();
        let original = {
            let state = state.borrow();
            let Text::Owned(text) = &state.text else {
                panic!("expected owned text")
            };
            Rc::downgrade(text)
        };
        let alias = screen
            .child(0)
            .unwrap()
            .try_downcast::<class::LabelTag>()
            .unwrap();
        alias.set_text("a longer recovery word display").unwrap();
        assert!(original.upgrade().is_none());
        assert_eq!(
            label.get_text().unwrap().to_str().unwrap(),
            "a longer recovery word display"
        );
        alias.set_text("act").unwrap();
        let on_delete = state.clone();
        label
            .add_event_cb(crate::LvEventCode::LV_EVENT_DELETE, move || {
                assert!(on_delete.borrow().text.is_empty());
            })
            .unwrap();
        unsafe { alias.delete() };
        assert!(state.borrow().text.is_empty());
        assert_eq!(Rc::strong_count(&state), 1);
    }

    #[test]
    fn test_ins_text_and_cut_text_utf8() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let label = BitboxLabel::new(&screen).unwrap();
        label.set_text_static(Some(c"aö🙂z"));
        label.ins_text(2, "ä").unwrap();
        assert_eq!(label.get_text().unwrap().to_str().unwrap(), "aöä🙂z");
        label.cut_text(1, 3);
        assert_eq!(label.get_text().unwrap().to_str().unwrap(), "az");
        label.ins_text(u32::MAX, "ü").unwrap();
        assert_eq!(label.get_text().unwrap().to_str().unwrap(), "azü");
        label.cut_text(1, u32::MAX);
        assert_eq!(label.get_text().unwrap().to_str().unwrap(), "a");
        label.set_text_static(Some(c"static"));
        assert!(matches!(
            text_state(label.as_ptr()).borrow().text,
            Text::Static(_)
        ));
        label.set_text_static(None);
        assert_eq!(label.get_text().unwrap().to_str().unwrap(), "static");
        unsafe { label.delete() };
    }

    #[test]
    fn test_set_text_snapshot_survives_replacement() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let label = BitboxLabel::new(&screen).unwrap();
        label.set_text("recovery words").unwrap();
        let snapshot = label.get_text().unwrap();
        assert_eq!(
            label.set_text("invalid\0text"),
            Err(BitboxLabelTextError::ContainsNul)
        );
        assert_eq!(
            label.get_text().unwrap().to_str().unwrap(),
            "recovery words"
        );
        label.set_text("replacement").unwrap();
        unsafe { label.delete() };
        assert_eq!(snapshot.to_str().unwrap(), "recovery words");
    }

    #[test]
    fn test_retain_draw_text_until_render_ready() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(64, 64).unwrap();
        let screen = display.screen_active().unwrap();
        let label = BitboxLabel::new(&screen).unwrap();
        label.set_text("pending draw").unwrap();
        let display = unsafe { ffi::lv_obj_get_display(label.as_ptr()) };
        let text = text_state(label.as_ptr()).borrow().text.clone();
        let Text::Owned(owned) = &text else {
            panic!("expected owned text")
        };
        let weak = Rc::downgrade(owned);
        retain_draw_text(label.as_ptr(), &text);
        retain_draw_text(label.as_ptr(), &text);
        drop(text);
        label.set_text("").unwrap();
        unsafe { label.delete() };
        assert!(weak.upgrade().is_some());
        unsafe {
            ffi::lv_display_send_event(
                display,
                crate::LvEventCode::LV_EVENT_RENDER_READY,
                core::ptr::null_mut(),
            )
        };
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_layout_wrapping_clipping_and_transparent_text() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(320, 240).unwrap();
        let screen = display.screen_active().unwrap();
        let label = BitboxLabel::new(&screen).unwrap();
        label.set_style_text_font(crate::fonts::INTER_REGULAR_32, 0);
        label.set_text("one two three four").unwrap();
        label.set_width(100);
        unsafe { ffi::lv_obj_update_layout(label.as_ptr()) };
        let wrapped = unsafe { ffi::lv_obj_get_height(label.as_ptr()) };
        label.set_long_mode(BitboxLabelLongMode::Clip);
        unsafe { ffi::lv_obj_update_layout(label.as_ptr()) };
        let clipped = unsafe { ffi::lv_obj_get_height(label.as_ptr()) };
        assert!(wrapped > clipped);
        label.set_style_text_opa(0, 0);
        unsafe { ffi::lv_obj_update_layout(label.as_ptr()) };
        assert_eq!(unsafe { ffi::lv_obj_get_height(label.as_ptr()) }, clipped);
        label.set_text("aöyz").unwrap();
        for index in 0..=4 {
            assert_eq!(label.letter_at(label.letter_position(index).x), index);
        }
        unsafe { label.delete() };
    }

    #[test]
    fn test_copy_text_from_independent_storage() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(16, 16).unwrap();
        let screen = LvObj::new().unwrap();
        let source = BitboxLabel::new(&screen).unwrap();
        let destination = BitboxLabel::new(&screen).unwrap();

        source.set_text("abandon").unwrap();
        destination.set_text("old").unwrap();
        destination.copy_text_from(&source);
        source.set_text("zoo").unwrap();
        assert_eq!(destination.get_text().unwrap().to_bytes(), b"abandon");

        unsafe { source.delete() };
        assert_eq!(destination.get_text().unwrap().to_bytes(), b"abandon");

        unsafe {
            screen.delete();
            ffi::lv_display_delete(display.as_ptr());
        }
    }

    #[test]
    fn test_copy_text_from_self() {
        let _lock = crate::test_util::lock_and_init();
        let display = crate::LvDisplay::new(16, 16).unwrap();
        let screen = LvObj::new().unwrap();
        let label = BitboxLabel::new(&screen).unwrap();

        for text in ["abandon", "zoo", ""] {
            label.set_text(text).unwrap();
            label.copy_text_from(&label);
            assert_eq!(label.get_text().unwrap().to_bytes(), text.as_bytes());
        }

        unsafe {
            screen.delete();
            ffi::lv_display_delete(display.as_ptr());
        }
    }
}
