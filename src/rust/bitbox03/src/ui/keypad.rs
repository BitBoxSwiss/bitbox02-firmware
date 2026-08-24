// SPDX-License-Identifier: Apache-2.0

//! Numeric keypad for PIN entry: a 3×4 grid of navigation-button-sized keys — digits 1–9, then a
//! bottom row of backspace, 0 and confirm ("PIN entry mockup"). Digits are inserted into the
//! textarea on release with the standard press-invert feedback; sliding off a key aborts the tap
//! (see `ObjExt::add_click_cb`); backspace is grayed out and inert while the input is empty.

use alloc::rc::Rc;
use alloc::vec;

use bitbox_lvgl::{self as lvgl, LabelExt, LvLabel, LvObj, LvTextarea, ObjExt, TextareaExt};

use super::keyboard::gray;
use super::nav_button::{NavIcon, build_nav_button, enable_press_invert, style_outline_button};

/// Key side length; matches the navigation buttons.
const KEY_SIZE: i32 = 82;
/// Gap between keys (mockup: 50px both ways).
const KEY_GAP: i32 = 50;
const KEY_PITCH: i32 = KEY_SIZE + KEY_GAP;

pub const KEYPAD_WIDTH: i32 = 3 * KEY_PITCH - KEY_GAP; // 346
pub const KEYPAD_HEIGHT: i32 = 4 * KEY_PITCH - KEY_GAP; // 478

/// Selector shorthand for the pressed state.
const PRESSED: u32 = lvgl::LvState::LV_STATE_PRESSED as u32;

/// Centre of the key at (`row`, `col`) in keypad-container coordinates, for tests.
#[cfg(test)]
pub(super) fn key_center(row: usize, col: usize) -> (i32, i32) {
    (
        col as i32 * KEY_PITCH + KEY_SIZE / 2,
        row as i32 * KEY_PITCH + KEY_SIZE / 2,
    )
}

/// Adds one digit key: the navigation-button frame with the digit as its label; inserts the
/// digit on release.
fn add_digit_key(container: &LvObj, textarea: &Rc<LvTextarea>, digit: u8, x: i32, y: i32) {
    let key = lvgl::LvButton::new(container).unwrap();
    key.set_size(KEY_SIZE, KEY_SIZE);
    key.set_pos(x, y);
    key.set_style_radius(19, 0); // navigation-button corner radius
    style_outline_button(&key, 3);
    let label = LvLabel::new(&key).unwrap();
    label.set_style_text_color(lvgl::color::white(), 0);
    label.set_style_text_color(lvgl::color::black(), PRESSED);
    label.set_style_text_font(
        lvgl::fonts::INTER_REGULAR_48,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
    let text = [digit];
    label
        .set_text(core::str::from_utf8(&text).expect("keypad digits are ASCII"))
        .expect("keypad digits contain no NUL");
    label.align(lvgl::LvAlign::LV_ALIGN_CENTER, 0, 0);
    let label_part = key.child(0).expect("digit label");
    enable_press_invert(&key, vec![label_part]);
    let textarea = Rc::clone(textarea);
    key.add_click_cb(move || textarea.add_char(u32::from(digit)))
        .expect("failed to register digit callback");
}

/// Builds the PIN keypad as a `KEYPAD_WIDTH`×`KEYPAD_HEIGHT` container appended to `parent`.
/// Digit keys insert into `textarea`; the checkmark key calls `on_confirm`.
///
/// Child order is row-major over the grid: digits 1–9 (0..=8), backspace (9), 0 (10),
/// confirm (11).
pub fn build_keypad<F>(parent: &LvObj, textarea: Rc<LvTextarea>, on_confirm: F) -> LvObj
where
    F: FnMut() + 'static,
{
    let container = LvObj::with_parent(parent).unwrap();
    container.set_size(KEYPAD_WIDTH, KEYPAD_HEIGHT);
    container.set_style_bg_opa(lvgl::LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    container.set_style_border_width(0, 0);
    container.set_style_radius(0, 0);
    container.set_style_pad_top(0, 0);
    container.set_style_pad_bottom(0, 0);
    container.set_style_pad_left(0, 0);
    container.set_style_pad_right(0, 0);
    container.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_SCROLLABLE);

    for (i, digit) in (b'1'..=b'9').enumerate() {
        let (row, col) = (i / 3, i % 3);
        add_digit_key(
            &container,
            &textarea,
            digit,
            col as i32 * KEY_PITCH,
            row as i32 * KEY_PITCH,
        );
    }

    let bottom = 3 * KEY_PITCH;

    // Backspace: the standard back icon button, grayed out and inert while the input is empty.
    let backspace = build_nav_button(&container, NavIcon::Back);
    backspace.set_pos(0, bottom);
    let backspace_icon = backspace.child(0).expect("backspace icon");
    backspace.set_style_border_color(gray(), lvgl::LvState::LV_STATE_DISABLED as u32);
    backspace_icon.set_style_image_recolor(gray(), lvgl::LvState::LV_STATE_DISABLED as u32);
    let delete_textarea = Rc::clone(&textarea);
    backspace
        .add_click_cb(move || delete_textarea.delete_char())
        .expect("failed to register backspace callback");
    let refresh_textarea = Rc::clone(&textarea);
    let refresh_backspace = move || {
        if super::enter_string::textarea_is_empty(refresh_textarea.as_ref()) {
            backspace.add_state(lvgl::LvState::LV_STATE_DISABLED);
            backspace_icon.add_state(lvgl::LvState::LV_STATE_DISABLED);
            backspace.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
        } else {
            backspace.remove_state(lvgl::LvState::LV_STATE_DISABLED);
            backspace_icon.remove_state(lvgl::LvState::LV_STATE_DISABLED);
            backspace.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
        }
    };
    refresh_backspace();
    textarea
        .add_event_cb(lvgl::LvEventCode::LV_EVENT_VALUE_CHANGED, refresh_backspace)
        .expect("failed to register backspace state callback");

    add_digit_key(&container, &textarea, b'0', KEY_PITCH, bottom);

    let confirm = build_nav_button(&container, NavIcon::Confirm);
    confirm.set_pos(2 * KEY_PITCH, bottom);
    confirm
        .add_click_cb(on_confirm)
        .expect("failed to register confirm callback");

    container
}
