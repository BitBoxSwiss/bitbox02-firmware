// SPDX-License-Identifier: Apache-2.0

//! On-screen QWERTY keyboard used for BIP39 passphrase entry.
//!
//! Four rows of outlined keys (a digit row and three character rows) above a function row with a
//! caps-lock toggle, a space bar and a symbols toggle. Pressing a character key "jets out" a
//! balloon-shaped enlarged preview of the key above the finger, so the user sees which key they
//! hit; the character is inserted on release. Sliding off the pressed key cancels: the preview
//! hides and nothing is typed (the buttonmatrix discards its selection when the pointer leaves
//! the pressed button). The symbols layout reuses BitBox02's special character set
//! (`_special_chars` in trinary_input_string.c), which — with space on its own key — fills the
//! three character rows exactly (3 × 10); the digit row stays in place.
//!
//! Each key row is a single `lv_buttonmatrix` rather than per-key buttons: LVGL draws matrix
//! buttons without allocating an object per key, which keeps the screen well inside LVGL's small
//! builtin memory pool (`LV_MEM_SIZE`).

use alloc::rc::Rc;
use alloc::vec;
use core::cell::{Cell, RefCell};

use bitbox_lvgl::{
    self as lvgl, ButtonmatrixExt, LabelExt, LvButton, LvButtonmatrix, LvButtonmatrixCtrl,
    LvButtonmatrixMapEntry, LvCanvas, LvEventCode, LvLabel, LvObj, LvOpacityLevel, LvPart, LvState,
    LvTextarea, ObjExt, TextareaExt,
};

use super::nav_button::{add_icon, enable_press_invert, style_outline_button};

/// Character-key size (mockup: 40×60 keys in a 45px-pitch grid with 5px gaps).
const KEY_WIDTH: i32 = 40;
const KEY_HEIGHT: i32 = 60;
const KEY_GAP: i32 = 5;
/// Horizontal distance between the left edges of adjacent keys.
const KEY_PITCH_X: i32 = KEY_WIDTH + KEY_GAP;
/// Vertical distance between the top edges of adjacent rows (60px key + 20px gap).
const ROW_PITCH_Y: i32 = 80;
/// Keyboard width: the widest (10-key) row.
pub const KEYBOARD_WIDTH: i32 = 10 * KEY_PITCH_X - KEY_GAP; // 445
/// The function row sits 29px (mockup) below the last character row.
const FUNCTION_ROW_Y: i32 = 3 * ROW_PITCH_Y + KEY_HEIGHT + 29;
/// Total component height (function row included).
pub const KEYBOARD_HEIGHT: i32 = FUNCTION_ROW_Y + KEY_HEIGHT; // 389
/// Caps-lock / symbols-toggle key width; the function row is inset from the grid edges and the
/// space bar fills the rest (mockup: 55-wide toggles, 15px gaps, 24px insets).
const FUNCTION_KEY_WIDTH: i32 = 55;
const FUNCTION_ROW_INSET: i32 = 24;
const FUNCTION_ROW_GAP: i32 = 15;
const SPACE_X: i32 = FUNCTION_ROW_INSET + FUNCTION_KEY_WIDTH + FUNCTION_ROW_GAP;
const SPACE_WIDTH: i32 = KEYBOARD_WIDTH - 2 * SPACE_X;

const KEY_RADIUS: i32 = 10;
const KEY_BORDER: i32 = 3;

/// The "jet out" preview: an enlarged balloon-shaped copy of the pressed key, drawn from a
/// pre-rendered bitmap (white outline, opaque black fill). Its stem covers the pressed key; the
/// head reaches into the row above (mockup "Frame 150 - on click").
const PREVIEW_PNG: &[u8] = include_bytes!("../../icons/key_preview.png");
const PREVIEW_WIDTH: i32 = 72;
const PREVIEW_HEIGHT: i32 = 132;
/// Preview position relative to the pressed key's top-left corner.
const PREVIEW_OFFSET_X: i32 = (KEY_WIDTH - PREVIEW_WIDTH) / 2;
const PREVIEW_OFFSET_Y: i32 = -69;
/// How far the preview reaches beyond the container bounds (balloon head above the top key row);
/// declared as the container's ext draw size so the overhang is not clipped.
const PREVIEW_OVERHANG: i32 = -PREVIEW_OFFSET_Y;
/// Vertical offset of the preview character label inside the balloon head.
const PREVIEW_LABEL_Y: i32 = 10;

const CAPSLOCK_PNG: &[u8] = include_bytes!("../../icons/capslock.png");

/// Number of character-key rows (digits + three letter/symbol rows).
const ROWS: usize = 4;

/// Child index of the caps-lock button inside the keyboard container (after the `ROWS` key-row
/// buttonmatrices). The `CHILD_INDEX_*` constants document the container's child order for tests
/// and dev tooling (render example).
pub const CHILD_INDEX_CAPSLOCK: i32 = ROWS as i32;
/// Child index of the space bar.
pub const CHILD_INDEX_SPACE: i32 = CHILD_INDEX_CAPSLOCK + 1;
/// Child index of the symbols toggle.
pub const CHILD_INDEX_SYMBOLS: i32 = CHILD_INDEX_CAPSLOCK + 2;
/// Child index of the (initially hidden) pressed-key preview balloon.
pub const CHILD_INDEX_PREVIEW: i32 = CHILD_INDEX_CAPSLOCK + 3;

/// Builds a `'static` single-row buttonmatrix map: the given keys plus the required terminator.
macro_rules! key_row_map {
    ($name:ident, $count:literal, [$($key:expr),+ $(,)?]) => {
        const $name: &[LvButtonmatrixMapEntry; $count + 1] = &[
            $(LvButtonmatrixMapEntry::new($key)),+,
            LvButtonmatrixMapEntry::new(c""),
        ];
    };
}

key_row_map!(
    MAP_DIGITS,
    10,
    [c"1", c"2", c"3", c"4", c"5", c"6", c"7", c"8", c"9", c"0"]
);
key_row_map!(
    MAP_LOWER_1,
    10,
    [c"q", c"w", c"e", c"r", c"t", c"y", c"u", c"i", c"o", c"p"]
);
key_row_map!(
    MAP_LOWER_2,
    9,
    [c"a", c"s", c"d", c"f", c"g", c"h", c"j", c"k", c"l"]
);
key_row_map!(MAP_LOWER_3, 7, [c"z", c"x", c"c", c"v", c"b", c"n", c"m"]);
key_row_map!(
    MAP_UPPER_1,
    10,
    [c"Q", c"W", c"E", c"R", c"T", c"Y", c"U", c"I", c"O", c"P"]
);
key_row_map!(
    MAP_UPPER_2,
    9,
    [c"A", c"S", c"D", c"F", c"G", c"H", c"J", c"K", c"L"]
);
key_row_map!(MAP_UPPER_3, 7, [c"Z", c"X", c"C", c"V", c"B", c"N", c"M"]);
// BitBox02's special characters (minus space, which has its own key), three rows of ten.
key_row_map!(
    MAP_SYMBOLS_1,
    10,
    [c"!", c"\"", c"#", c"$", c"%", c"&", c"'", c"(", c")", c"*"]
);
key_row_map!(
    MAP_SYMBOLS_2,
    10,
    [c"+", c",", c"-", c".", c"/", c":", c";", c"<", c"=", c">"]
);
key_row_map!(
    MAP_SYMBOLS_3,
    10,
    [c"?", c"^", c"[", c"\\", c"]", c"@", c"_", c"{", c"|", c"}"]
);

/// Selector shorthands for state-dependent styles.
const PRESSED: u32 = LvState::LV_STATE_PRESSED as u32;
const CHECKED: u32 = LvState::LV_STATE_CHECKED as u32;
const DISABLED: u32 = LvState::LV_STATE_DISABLED as u32;
const ITEMS: u32 = LvPart::LV_PART_ITEMS as u32;

/// Disabled/secondary gray, sampled from the mockup's inactive backspace button.
pub(super) fn gray() -> lvgl::LvColor {
    lvgl::color::hex(0x777777)
}

#[derive(Clone, Copy)]
struct Mode {
    caps: bool,
    symbols: bool,
}

/// The buttonmatrix map of `row` in `mode`. Row 0 (digits) is mode-independent.
fn row_map(mode: Mode, row: usize) -> &'static [LvButtonmatrixMapEntry] {
    match (row, mode.symbols, mode.caps) {
        (0, _, _) => MAP_DIGITS,
        (1, true, _) => MAP_SYMBOLS_1,
        (2, true, _) => MAP_SYMBOLS_2,
        (3, true, _) => MAP_SYMBOLS_3,
        (1, false, false) => MAP_LOWER_1,
        (2, false, false) => MAP_LOWER_2,
        (3, false, false) => MAP_LOWER_3,
        (1, false, true) => MAP_UPPER_1,
        (2, false, true) => MAP_UPPER_2,
        (3, false, true) => MAP_UPPER_3,
        _ => unreachable!("keyboard has four key rows"),
    }
}

/// Number of keys in `row` for `mode`.
fn row_count(mode: Mode, row: usize) -> usize {
    row_map(mode, row).len() - 1 // minus the map terminator
}

/// Width of a key row of `count` keys.
fn row_width(count: usize) -> i32 {
    count as i32 * KEY_PITCH_X - KEY_GAP
}

/// X position of a key row of `count` keys (shorter rows are centred).
fn row_x(count: usize) -> i32 {
    (KEYBOARD_WIDTH - row_width(count)) / 2
}

/// X position of key `col` in a row of `count` keys.
pub(super) fn key_x(count: usize, col: usize) -> i32 {
    row_x(count) + col as i32 * KEY_PITCH_X
}

pub(super) fn key_y(row: usize) -> i32 {
    row as i32 * ROW_PITCH_Y
}

/// Absolute-positioned centre of the caps-lock key, for tests.
#[cfg(test)]
pub(super) fn capslock_center() -> (i32, i32) {
    (
        FUNCTION_ROW_INSET + FUNCTION_KEY_WIDTH / 2,
        FUNCTION_ROW_Y + KEY_HEIGHT / 2,
    )
}

/// Absolute-positioned centre of the symbols-toggle key, for tests.
#[cfg(test)]
pub(super) fn symbols_center() -> (i32, i32) {
    (
        KEYBOARD_WIDTH - FUNCTION_ROW_INSET - FUNCTION_KEY_WIDTH / 2,
        FUNCTION_ROW_Y + KEY_HEIGHT / 2,
    )
}

/// Absolute-positioned centre of the space bar, for tests.
#[cfg(test)]
pub(super) fn space_center() -> (i32, i32) {
    (KEYBOARD_WIDTH / 2, FUNCTION_ROW_Y + KEY_HEIGHT / 2)
}

/// Centre of character key (`row`, `col`) in the given layout, for tests.
#[cfg(test)]
pub(super) fn char_key_center(
    mode_symbols: bool,
    mode_caps: bool,
    row: usize,
    col: usize,
) -> (i32, i32) {
    let mode = Mode {
        caps: mode_caps,
        symbols: mode_symbols,
    };
    (
        key_x(row_count(mode, row), col) + KEY_WIDTH / 2,
        key_y(row) + KEY_HEIGHT / 2,
    )
}

/// The widgets the mode toggles have to update.
struct Widgets {
    key_rows: [LvButtonmatrix; ROWS],
    capslock: LvButton,
    capslock_icon: LvObj,
    symbols_label: LvLabel,
}

/// Applies `mode`: swaps the key-row maps and geometry, enables/disables caps lock (it has no
/// meaning on the symbols layout) and relabels the symbols toggle.
fn apply_mode(widgets: &Widgets, mode: Mode) {
    for (row, matrix) in widgets.key_rows.iter().enumerate() {
        let count = row_count(mode, row);
        matrix.set_map(row_map(mode, row));
        // Insert on release (click), not press: while pressed, the jet-out preview shows the key
        // under the finger, and sliding off the key aborts instead of typing.
        matrix.set_button_ctrl_all(LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_CLICK_TRIG);
        // CLICK_TRIG does not gate the long-press repeat path: without NO_REPEAT the
        // buttonmatrix fires VALUE_CHANGED every repeat period while a key is held (encouraged
        // by the preview balloon), silently duplicating characters in the masked input.
        matrix.set_button_ctrl_all(LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_NO_REPEAT);
        matrix.set_size(row_width(count), KEY_HEIGHT);
        matrix.set_pos(row_x(count), key_y(row));
    }

    if mode.symbols {
        widgets.capslock.add_state(LvState::LV_STATE_DISABLED);
        widgets.capslock_icon.add_state(LvState::LV_STATE_DISABLED);
        widgets
            .capslock
            .remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
    } else {
        widgets.capslock.remove_state(LvState::LV_STATE_DISABLED);
        widgets
            .capslock_icon
            .remove_state(LvState::LV_STATE_DISABLED);
        widgets
            .capslock
            .add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
    }

    // The toggle shows what pressing it switches to.
    if mode.symbols {
        widgets
            .symbols_label
            .set_style_text_font(lvgl::fonts::INTER_REGULAR_24, 0);
        widgets.symbols_label.set_text("abc").unwrap();
    } else {
        widgets
            .symbols_label
            .set_style_text_font(lvgl::fonts::INTER_BOLD_32, 0);
        widgets.symbols_label.set_text("!@").unwrap();
    }
}

/// The pressed-key preview balloon (a canvas with the balloon bitmap plus the character label).
struct Preview {
    root: LvObj,
    label: LvLabel,
    /// The (row, key) the preview currently shows, to skip redundant updates from the
    /// once-per-input-period `LV_EVENT_PRESSING` stream.
    shown: Cell<Option<(usize, u32)>>,
}

impl Preview {
    fn build(parent: &LvObj) -> Self {
        let root = LvObj::with_parent(parent).unwrap();
        root.set_size(PREVIEW_WIDTH, PREVIEW_HEIGHT);
        root.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
        root.set_style_border_width(0, 0);
        root.set_style_radius(0, 0);
        root.set_style_pad_top(0, 0);
        root.set_style_pad_bottom(0, 0);
        root.set_style_pad_left(0, 0);
        root.set_style_pad_right(0, 0);
        // The preview pops up underneath the finger; it must never grab input away from the keys.
        root.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
        root.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_SCROLLABLE);
        root.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);

        // `png_decoder` returns ARGB8888 pixels as RGBA; LVGL expects BGRA in memory. (Same
        // decode path as the nav-button icons, but without recolor: the bitmap's white outline
        // and black fill are used as-is.)
        let (header, mut data) = png_decoder::decode(PREVIEW_PNG).expect("valid key preview png");
        for px in data.iter_mut() {
            px.swap(0, 2);
        }
        let canvas =
            LvCanvas::new(&root, data, header.width, header.height).expect("key preview canvas");
        canvas.align(lvgl::LvAlign::LV_ALIGN_TOP_MID, 0, 0);

        let label = LvLabel::new(&root).unwrap();
        label.set_style_text_color(lvgl::color::white(), 0);
        label.set_style_text_font(lvgl::fonts::INTER_BOLD_48, 0);
        label.align(lvgl::LvAlign::LV_ALIGN_TOP_MID, 0, PREVIEW_LABEL_Y);
        label.set_text("").unwrap();

        Self {
            root,
            label,
            shown: Cell::new(None),
        }
    }

    /// Shows the balloon for the key `id` of `row` (labelled `text`), positioned over the key.
    fn show(&self, mode: Mode, row: usize, id: u32, text: &str) {
        if self.shown.get() == Some((row, id)) {
            return;
        }
        self.shown.set(Some((row, id)));
        self.label.set_text(text).expect("key text contains no NUL");
        self.root.set_pos(
            key_x(row_count(mode, row), id as usize) + PREVIEW_OFFSET_X,
            key_y(row) + PREVIEW_OFFSET_Y,
        );
        self.root.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
    }

    fn hide(&self) {
        self.shown.set(None);
        self.root.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
    }
}

/// Styles a function-key frame: white rounded outline, no fill, and no default-theme press
/// effects (press feedback is the white fill from `enable_press_invert`).
fn style_function_key(button: &LvButton) {
    button.set_style_radius(KEY_RADIUS, 0);
    style_outline_button(button, KEY_BORDER);
    // Cancel the default theme's pressed-state grow and dim.
    button.set_style_recolor_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, PRESSED);
    button.set_style_transform_width(0, PRESSED);
    button.set_style_transform_height(0, PRESSED);
}

/// Styles a key-row buttonmatrix: transparent background, 5px key gaps, and each key drawn as a
/// white rounded outline with white bold text (unchanged while pressed — press feedback is the
/// jet-out preview covering the key).
fn style_key_row(matrix: &LvButtonmatrix) {
    matrix.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    matrix.set_style_border_width(0, 0);
    matrix.set_style_radius(0, 0);
    matrix.set_style_pad_top(0, 0);
    matrix.set_style_pad_bottom(0, 0);
    matrix.set_style_pad_left(0, 0);
    matrix.set_style_pad_right(0, 0);
    matrix.set_style_pad_column(KEY_GAP, 0);

    for selector in [ITEMS, ITEMS | PRESSED] {
        matrix.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, selector);
        matrix.set_style_border_width(KEY_BORDER, selector);
        matrix.set_style_border_color(lvgl::color::white(), selector);
        matrix.set_style_radius(KEY_RADIUS, selector);
        matrix.set_style_shadow_width(0, selector);
        matrix.set_style_text_color(lvgl::color::white(), selector);
        matrix.set_style_text_font(lvgl::fonts::INTER_BOLD_32, selector);
    }
}

/// A second handle to a key-row matrix, for use inside its own event callbacks.
fn matrix_handle(container: &LvObj, row: usize) -> LvButtonmatrix {
    container
        .child(row as i32)
        .expect("key row")
        .try_downcast()
        .expect("key row is a buttonmatrix")
}

/// Builds the QWERTY keyboard as a `KEYBOARD_WIDTH`×`KEYBOARD_HEIGHT` container and appends it to
/// `parent`. Typed characters are inserted into `textarea`.
///
/// Child order (see the `CHILD_INDEX_*` constants): `ROWS` key-row buttonmatrices, caps lock,
/// space bar, symbols toggle, preview balloon.
pub fn build_keyboard(parent: &LvObj, textarea: Rc<LvTextarea>) -> LvObj {
    let container = LvObj::with_parent(parent).unwrap();
    container.set_size(KEYBOARD_WIDTH, KEYBOARD_HEIGHT);
    container.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    container.set_style_border_width(0, 0);
    container.set_style_radius(0, 0);
    container.set_style_pad_top(0, 0);
    container.set_style_pad_bottom(0, 0);
    container.set_style_pad_left(0, 0);
    container.set_style_pad_right(0, 0);
    container.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_SCROLLABLE);
    // The preview balloon overhangs the top row (by `PREVIEW_OVERHANG`) and the outermost
    // columns (by 16px). OVERFLOW_VISIBLE alone is not enough: it only widens the children clip
    // rect to the container's own ext draw size, which is 0 for this plain container — so the
    // overhang must also be declared as ext draw size or the balloon head gets clipped away.
    container.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_OVERFLOW_VISIBLE);
    unsafe extern "C" fn refresh_ext_draw_size_cb(event: *mut lvgl::ffi::lv_event_t) {
        unsafe { lvgl::ffi::lv_event_set_ext_draw_size(event, PREVIEW_OVERHANG) };
    }
    unsafe {
        lvgl::ffi::lv_obj_add_event_cb(
            container.as_ptr(),
            Some(refresh_ext_draw_size_cb),
            lvgl::LvEventCode::LV_EVENT_REFR_EXT_DRAW_SIZE,
            core::ptr::null_mut(),
        );
        lvgl::ffi::lv_obj_refresh_ext_draw_size(container.as_ptr());
    }

    let mode = Rc::new(RefCell::new(Mode {
        caps: false,
        symbols: false,
    }));

    let key_rows: [LvButtonmatrix; ROWS] = core::array::from_fn(|_| {
        let matrix = LvButtonmatrix::new(&container).unwrap();
        style_key_row(&matrix);
        matrix
    });

    // Caps lock: outline arrow icon; toggled = white fill with the icon inverted to black
    // (`LV_STATE_CHECKED`, managed manually in the click handler); disabled (gray) on the symbols
    // layout.
    let capslock = LvButton::new(&container).unwrap();
    capslock.set_size(FUNCTION_KEY_WIDTH, KEY_HEIGHT);
    capslock.set_pos(FUNCTION_ROW_INSET, FUNCTION_ROW_Y);
    style_function_key(&capslock);
    capslock.set_style_bg_color(lvgl::color::white(), CHECKED);
    capslock.set_style_bg_opa(LvOpacityLevel::LV_OPA_COVER as u8, CHECKED);
    capslock.set_style_border_color(gray(), DISABLED);
    let icon = add_icon(&capslock, CAPSLOCK_PNG);
    enable_press_invert(&capslock, vec![icon]);
    // A second handle to the icon canvas, for the checked/disabled recolors.
    let capslock_icon = capslock.child(0).expect("caps lock icon");
    capslock_icon.set_style_image_recolor(lvgl::color::black(), CHECKED);
    capslock_icon.set_style_image_recolor(gray(), DISABLED);

    // Space bar (no label; press feedback is the white fill).
    let space = LvButton::new(&container).unwrap();
    space.set_size(SPACE_WIDTH, KEY_HEIGHT);
    space.set_pos(SPACE_X, FUNCTION_ROW_Y);
    style_function_key(&space);
    enable_press_invert(&space, vec![]);
    {
        let textarea = Rc::clone(&textarea);
        space
            .add_click_cb(move || textarea.add_char(u32::from(b' ')))
            .expect("failed to register space callback");
    }

    // Symbols toggle ("!@" on the character layouts, "abc" on the symbols layout).
    let symbols = LvButton::new(&container).unwrap();
    symbols.set_size(FUNCTION_KEY_WIDTH, KEY_HEIGHT);
    symbols.set_pos(
        KEYBOARD_WIDTH - FUNCTION_ROW_INSET - FUNCTION_KEY_WIDTH,
        FUNCTION_ROW_Y,
    );
    style_function_key(&symbols);
    let symbols_label = LvLabel::new(&symbols).unwrap();
    symbols_label.set_style_text_color(lvgl::color::white(), 0);
    symbols_label.set_style_text_color(lvgl::color::black(), PRESSED);
    symbols_label.align(lvgl::LvAlign::LV_ALIGN_CENTER, 0, 0);
    symbols_label.set_text("").unwrap();
    let symbols_label_part = symbols.child(0).expect("symbols toggle label");
    enable_press_invert(&symbols, vec![symbols_label_part]);

    // The preview balloon is created last so it draws above every key.
    let preview = Rc::new(Preview::build(&container));

    // Key-row behaviour: preview over the pressed key while pressed (hidden again if the finger
    // slides off it, which also discards the selection); insert the selected key's character on
    // release over it (`CLICK_TRIG`, set in `apply_mode`).
    for (row, matrix) in key_rows.iter().enumerate() {
        {
            let matrix_cb = matrix_handle(&container, row);
            let mode = Rc::clone(&mode);
            let textarea = Rc::clone(&textarea);
            matrix
                .add_event_cb(LvEventCode::LV_EVENT_VALUE_CHANGED, move || {
                    let id = matrix_cb.get_selected_button();
                    if (id as usize) < row_count(*mode.borrow(), row)
                        && let Some(text) = matrix_cb.get_button_text(id)
                    {
                        let _ = textarea.add_text(text.to_str().expect("key text is ASCII"));
                    }
                })
                .expect("failed to register key callback");
        }
        for code in [
            LvEventCode::LV_EVENT_PRESSED,
            LvEventCode::LV_EVENT_PRESSING,
        ] {
            let matrix_cb = matrix_handle(&container, row);
            let mode = Rc::clone(&mode);
            let preview = Rc::clone(&preview);
            matrix
                .add_event_cb(code, move || {
                    let mode = *mode.borrow();
                    let id = matrix_cb.get_selected_button();
                    if (id as usize) < row_count(mode, row) {
                        // PRESSING fires every input period; skip the button-text lookup (which
                        // allocates) while the preview already shows this key.
                        if preview.shown.get() == Some((row, id)) {
                            return;
                        }
                        match matrix_cb.get_button_text(id) {
                            Some(text) => preview.show(
                                mode,
                                row,
                                id,
                                text.to_str().expect("key text is ASCII"),
                            ),
                            None => preview.hide(),
                        }
                    } else {
                        // The finger slid off the keys (e.g. into a row gap).
                        preview.hide();
                    }
                })
                .expect("failed to register key press callback");
        }
        for code in [
            LvEventCode::LV_EVENT_RELEASED,
            LvEventCode::LV_EVENT_PRESS_LOST,
        ] {
            let matrix_cb = matrix_handle(&container, row);
            let preview = Rc::clone(&preview);
            matrix
                .add_event_cb(code, move || {
                    preview.hide();
                    // Discard the selection once the interaction ends (this callback runs after
                    // the class handler has fired VALUE_CHANGED for a legitimate click). LVGL
                    // keeps the lastly clicked key selected forever, and a press sliding in from
                    // a neighbouring key reaches the matrix without a PRESSED event (which is
                    // what re-derives the selection) but still gets RELEASED — a stale selection
                    // would type that key again.
                    matrix_cb.set_selected_button(lvgl::ffi::LV_BUTTONMATRIX_BUTTON_NONE);
                })
                .expect("failed to register key release callback");
        }
    }

    let widgets = Rc::new(Widgets {
        key_rows,
        capslock,
        capslock_icon,
        symbols_label,
    });

    {
        let mode = Rc::clone(&mode);
        let widgets_cb = Rc::clone(&widgets);
        widgets
            .capslock
            .add_click_cb(move || {
                let new_mode = {
                    let mut mode = mode.borrow_mut();
                    mode.caps = !mode.caps;
                    *mode
                };
                if new_mode.caps {
                    widgets_cb.capslock.add_state(LvState::LV_STATE_CHECKED);
                    widgets_cb
                        .capslock_icon
                        .add_state(LvState::LV_STATE_CHECKED);
                } else {
                    widgets_cb.capslock.remove_state(LvState::LV_STATE_CHECKED);
                    widgets_cb
                        .capslock_icon
                        .remove_state(LvState::LV_STATE_CHECKED);
                }
                apply_mode(&widgets_cb, new_mode);
            })
            .expect("failed to register caps lock callback");
    }

    {
        let mode = Rc::clone(&mode);
        let widgets_cb = Rc::clone(&widgets);
        symbols
            .add_click_cb(move || {
                let new_mode = {
                    let mut mode = mode.borrow_mut();
                    mode.symbols = !mode.symbols;
                    // Caps lock has no meaning for symbols; start over in lowercase.
                    mode.caps = false;
                    *mode
                };
                widgets_cb.capslock.remove_state(LvState::LV_STATE_CHECKED);
                widgets_cb
                    .capslock_icon
                    .remove_state(LvState::LV_STATE_CHECKED);
                apply_mode(&widgets_cb, new_mode);
            })
            .expect("failed to register symbols toggle callback");
    }

    apply_mode(&widgets, *mode.borrow());

    container
}
