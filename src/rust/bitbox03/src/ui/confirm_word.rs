// SPDX-License-Identifier: Apache-2.0

//! The mnemonic-word confirmation (quiz) screen: the candidate words are shown as a wrapped row
//! of outline buttons under a "Select the correct recovery word" prompt. Tapping a candidate
//! lifts it out of the row into a larger preview box (leaving its slot in the row empty) and
//! enables the confirm button; tapping the preview or another candidate revises the choice.
//! A back button asks to go back to reviewing the recovery words, and the corner close button
//! requests cancelling the workflow.

use alloc::format;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::Cell;

use bitbox_lvgl::{
    self as lvgl, LabelExt, LvAlign, LvButton, LvLabel, LvLabelLongMode, LvObj, LvOpacityLevel,
    ObjExt, fonts,
};
use util::futures::completion::Responder;

use super::enter_string::{set_nav_button_enabled, style_nav_button_disabled};
use super::menu::transparent_row;
use super::nav_button::{
    NavIcon, build_close_button, build_nav_button, enable_press_invert, style_outline_button,
};

/// What the user chose on the word-confirmation screen.
#[derive(Clone, Copy)]
pub enum ConfirmWordAction {
    /// The user confirmed the candidate at this index.
    Selected(u8),
    /// Request to go back to reviewing the recovery words (the workflow asks for confirmation).
    Back,
    /// Request to cancel the workflow (the caller asks for confirmation). Carries the candidate
    /// selected at that moment, if any, so a declined cancel can restore the selection.
    Cancel(Option<u8>),
}

/// This screen narrows the standard 50px side padding: two of the widest candidate buttons must
/// fit side by side in the candidates container (see [`CANDIDATES_WIDTH`]).
const SIDE_PAD: i32 = 30;
/// Width of the candidates container. Wrapped rows start at its left edge; the container itself
/// is centred on the screen. Wide enough that any two candidate buttons share a row (pinned by
/// `test_two_widest_candidates_share_a_row`), so five candidates always fit in three rows.
const CANDIDATES_WIDTH: i32 = 480 - 2 * SIDE_PAD;
/// Height of one candidate button.
const CANDIDATE_HEIGHT: i32 = 56;
/// Border width of the candidate buttons and the preview box.
const BORDER_WIDTH: i32 = 2;
/// Corner radius of the candidate buttons and the preview box.
const RADIUS: i32 = 12;
/// Horizontal padding between a candidate button's border and its word.
const CANDIDATE_PAD: i32 = 12;
/// Horizontal gap between candidate buttons in a row.
const CANDIDATE_COLUMN_GAP: i32 = 20;
/// Vertical gap between candidate rows.
const CANDIDATE_ROW_GAP: i32 = 36;
/// Horizontal / vertical padding between the preview box's border and its word.
const PREVIEW_PAD_HOR: i32 = 12;
const PREVIEW_PAD_VER: i32 = 10;
/// Resting upward offset of the preview box above the exact centre of the subtitle/prompt gap,
/// per the mockup.
const PREVIEW_RAISE: i32 = -30;
/// Duration of the animation flying a word between its row slot and the preview box.
const FLY_MS: u32 = 180;
/// How far above the candidates container a word flies while returning from the preview box.
/// The container's ext draw size must cover it, or the flying word is clipped away (see the
/// overflow note on the container).
const FLY_OVERHANG: i32 = 300;

const SUBTITLE_FONT: lvgl::LvFont = fonts::INTER_REGULAR_24;
const PROMPT_FONT: lvgl::LvFont = fonts::INTER_REGULAR_24;
const CANDIDATE_FONT: lvgl::LvFont = fonts::INTER_MEDIUM_32;
const PREVIEW_FONT: lvgl::LvFont = fonts::INTER_MEDIUM_32;

/// The on-screen centre of a widget, from its current layout coordinates.
fn center(obj: &impl ObjExt) -> (i32, i32) {
    let (x1, y1, x2, y2) = obj.get_coords();
    ((x1 + x2 + 1) / 2, (y1 + y2 + 1) / 2)
}

/// The selection state and the widgets it toggles.
struct QuizState {
    /// Index of the currently selected candidate, if any.
    selected: Cell<Option<usize>>,
    candidates: Vec<LvButton>,
    /// The word labels inside [`Self::candidates`]; the preview reads the selected word from
    /// here, so no extra copies of the words are kept in the state.
    candidate_labels: Vec<LvLabel>,
    /// The empty flex area between the subtitle and the prompt; the (floating) preview box
    /// rests at its centre, [`PREVIEW_RAISE`] up.
    preview_area: LvObj,
    preview: LvButton,
    preview_label: LvLabel,
    confirm: LvButton,
    confirm_icon: LvObj,
}

impl QuizState {
    /// Shows or hides a candidate button. A hidden candidate keeps its slot in the wrapped row
    /// (the row must not reflow while its word sits in the preview box), so it is made fully
    /// transparent instead of layout-hidden, and unclickable.
    fn set_candidate_visible(&self, index: usize, visible: bool) {
        let button = &self.candidates[index];
        if visible {
            button.set_style_opa(LvOpacityLevel::LV_OPA_COVER as u8, 0);
            button.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
        } else {
            button.set_style_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
            button.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
        }
    }

    /// Returns a candidate to its slot in the row. When animating, its word flies back from the
    /// preview box's current position; the translate decays to zero, so no completion callback
    /// is needed for the button to end up exactly in its slot.
    fn restore_candidate(&self, index: usize, animate: bool) {
        let button = &self.candidates[index];
        self.set_candidate_visible(index, true);
        if animate {
            button.update_layout();
            let (preview_x, preview_y) = center(&self.preview);
            let (slot_x, slot_y) = center(button);
            let slot_x = slot_x - button.get_style_translate_x(lvgl::LvPart::LV_PART_MAIN);
            let slot_y = slot_y - button.get_style_translate_y(lvgl::LvPart::LV_PART_MAIN);
            lvgl::anim::animate_translate(
                button,
                (preview_x - slot_x, preview_y - slot_y),
                (0, 0),
                FLY_MS,
            );
        } else {
            button.set_style_translate_x(0, 0);
            button.set_style_translate_y(0, 0);
        }
    }

    /// Selects the candidate at `index`: its word moves from the row to the preview box and the
    /// confirm button becomes available. When animating, the word visibly flies from its slot
    /// to the preview position (and a previously selected word flies back to its slot).
    fn select(&self, index: usize, animate: bool) {
        if let Some(previous) = self.selected.replace(Some(index)) {
            self.restore_candidate(previous, animate);
        }
        self.set_candidate_visible(index, false);
        self.preview_label
            .copy_text_from(&self.candidate_labels[index]);
        self.preview
            .remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
        set_nav_button_enabled(&self.confirm, &self.confirm_icon, true);

        // The preview floats centred on the screen; lay the (just shown, just re-worded) box
        // out there, then translate it to its resting spot at the preview area's centre,
        // `PREVIEW_RAISE` up — flying in from the tapped candidate's slot when animating.
        self.preview.set_style_translate_x(0, 0);
        self.preview.set_style_translate_y(0, 0);
        self.preview.update_layout();
        let (aligned_x, aligned_y) = center(&self.preview);
        let (area_x, area_y) = center(&self.preview_area);
        let rest = (area_x - aligned_x, area_y + PREVIEW_RAISE - aligned_y);
        if animate {
            let (slot_x, slot_y) = center(&self.candidates[index]);
            lvgl::anim::animate_translate(
                &self.preview,
                (slot_x - aligned_x, slot_y - aligned_y),
                rest,
                FLY_MS,
            );
        } else {
            self.preview.set_style_translate_x(rest.0, 0);
            self.preview.set_style_translate_y(rest.1, 0);
        }
    }

    /// Clears the selection: the previewed word flies back to its slot in the row and the
    /// confirm button grays out again.
    fn deselect(&self) {
        if let Some(previous) = self.selected.take() {
            self.restore_candidate(previous, true);
        }
        self.preview.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
        set_nav_button_enabled(&self.confirm, &self.confirm_icon, false);
    }
}

/// A centred label of `font`, sized to its content.
fn add_centered_label(parent: &LvObj, text: &str, font: lvgl::LvFont) -> LvLabel {
    let label = LvLabel::new(parent).unwrap();
    label.set_text(text).unwrap();
    label.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    label.set_style_text_font(font, lvgl::LvState::LV_STATE_DEFAULT as u32);
    label
}

/// An outline text button (candidate or preview): white border and word, inverting on press.
fn build_word_button(parent: &LvObj, word: &str, font: lvgl::LvFont) -> (LvButton, LvLabel) {
    let button = LvButton::new(parent).unwrap();
    button.set_style_radius(RADIUS, 0);
    style_outline_button(&button, BORDER_WIDTH);
    // Unlike the (transparent) outline buttons, word buttons fly over other content — an opaque
    // black fill looks identical on the black screen but occludes the text they pass over.
    button.set_style_bg_color(lvgl::color::black(), 0);
    button.set_style_bg_opa(LvOpacityLevel::LV_OPA_COVER as u8, 0);

    let label = LvLabel::new(&button).unwrap();
    label.set_text(word).unwrap();
    label.set_style_text_font(font, lvgl::LvState::LV_STATE_DEFAULT as u32);
    label.set_style_text_color(lvgl::color::white(), 0);
    label.set_style_text_color(lvgl::color::black(), lvgl::LvState::LV_STATE_PRESSED as u32);
    label.align(LvAlign::LV_ALIGN_CENTER, 0, 0);

    let label_part = button.child(0).expect("word button label");
    enable_press_invert(&button, vec![label_part]);

    (button, label)
}

/// Builds the word-confirmation screen for word `word_idx` (0-based) of `num_words`. Selecting a
/// candidate and confirming resolves [`ConfirmWordAction::Selected`] with the candidate's index;
/// the back button resolves [`ConfirmWordAction::Back`]; the corner close button resolves
/// [`ConfirmWordAction::Cancel`]. A `preselected` candidate starts out selected (without the
/// fly-in animation), so a rebuilt screen — e.g. after a declined cancel — restores the user's
/// selection.
pub fn build_confirm_word_screen(
    choices: &[&str],
    word_idx: usize,
    num_words: usize,
    preselected: Option<u8>,
    responder: Responder<ConfirmWordAction>,
) -> LvObj {
    assert!(!choices.is_empty(), "confirm word screen requires choices");
    assert!(choices.len() <= 256, "at most 256 choices are supported");

    let screen = LvObj::new().unwrap();
    screen.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    screen.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_COLUMN);
    screen.set_style_bg_color(lvgl::color::black(), 0);
    screen.set_style_text_color(lvgl::color::white(), 0);
    screen.set_style_pad_top(40, 0);
    screen.set_style_pad_right(SIDE_PAD, 0);
    // Standard bottom padding (32px), so the navigation buttons sit at the same height as on
    // the other workflow screens.
    screen.set_style_pad_bottom(32, 0);
    screen.set_style_pad_left(SIDE_PAD, 0);
    screen.set_style_pad_row(24, 0);
    // The navigation row and title are narrower than the candidates container; centre all
    // children so they land where the standard 50px-padded screens put them.
    screen.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);

    let title = LvLabel::new(&screen).unwrap();
    title.set_width(380);
    title.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    title.set_text("Confirm recovery words").unwrap();
    title.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    title.set_style_text_font(
        fonts::INTER_REGULAR_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );

    add_centered_label(
        &screen,
        &format!("{} of {}", word_idx + 1, num_words),
        SUBTITLE_FONT,
    );

    // The empty area between the subtitle and the prompt, absorbing the column's slack. The
    // preview box rests at its centre but is created floating at the end of the screen (so a
    // flying word draws over — and is not clipped by — the widgets it passes).
    let preview_area = LvObj::with_parent(&screen).unwrap();
    preview_area.set_width(380);
    preview_area.set_style_flex_grow(1, 0);
    preview_area.set_style_pad_top(0, 0);
    preview_area.set_style_pad_bottom(0, 0);
    preview_area.set_style_pad_left(0, 0);
    preview_area.set_style_pad_right(0, 0);
    preview_area.set_style_border_width(0, 0);
    preview_area.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    preview_area.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);

    add_centered_label(&screen, "Select the correct recovery word", PROMPT_FONT);

    // The candidate words, as a left-aligned wrapped row.
    let candidates_container = LvObj::with_parent(&screen).unwrap();
    candidates_container.set_width(CANDIDATES_WIDTH);
    candidates_container.set_height(lvgl::SIZE_CONTENT);
    candidates_container.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    candidates_container.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW_WRAP);
    candidates_container.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_START, 0);
    candidates_container.set_style_pad_top(0, 0);
    candidates_container.set_style_pad_bottom(0, 0);
    candidates_container.set_style_pad_left(0, 0);
    candidates_container.set_style_pad_right(0, 0);
    candidates_container.set_style_pad_column(CANDIDATE_COLUMN_GAP, 0);
    candidates_container.set_style_pad_row(CANDIDATE_ROW_GAP, 0);
    candidates_container.set_style_border_width(0, 0);
    candidates_container.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    // Set the prompt and the navigation row a bit further apart than the standard row gap.
    candidates_container.set_style_margin_top(16, 0);
    candidates_container.set_style_margin_bottom(48, 0);
    // A deselected word flies from the preview box back into its slot, drawn far above the
    // container on the way. OVERFLOW_VISIBLE alone is not enough: it only widens the children
    // clip rect by the container's own ext draw size, so that must cover the flight path too.
    candidates_container.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_OVERFLOW_VISIBLE);
    candidates_container
        .request_ext_draw_size(FLY_OVERHANG)
        .expect("failed to register draw size callback");

    let mut candidates: Vec<LvButton> = Vec::with_capacity(choices.len());
    let mut candidate_labels: Vec<LvLabel> = Vec::with_capacity(choices.len());
    for word in choices {
        let (button, label) = build_word_button(&candidates_container, word, CANDIDATE_FONT);
        button.set_height(CANDIDATE_HEIGHT);
        button.set_style_pad_left(CANDIDATE_PAD, 0);
        button.set_style_pad_right(CANDIDATE_PAD, 0);
        candidates.push(button);
        candidate_labels.push(label);
    }

    let actions = transparent_row(&screen, 380, 82);
    // Back on the left, Confirm on the right; the click callbacks are wired below, once the
    // selection state they report exists.
    actions.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_SPACE_BETWEEN, 0);
    let back = build_nav_button(&actions, NavIcon::Back);
    let confirm = build_nav_button(&actions, NavIcon::Confirm);
    let confirm_icon = style_nav_button_disabled(&confirm);
    set_nav_button_enabled(&confirm, &confirm_icon, false);

    // Cancel lives in the top-right corner so it doesn't crowd the bottom navigation.
    let close = build_close_button(&screen);
    // Re-align for this screen's narrowed side padding, keeping the button ~12px from the
    // display corner like on the 50px-padded screens.
    close.align(LvAlign::LV_ALIGN_TOP_RIGHT, SIDE_PAD - 12, -28);

    // The preview box: the last (topmost) child, floating so the screen's flex column ignores
    // it, hidden until a candidate is selected. [`QuizState::select`] translates it from this
    // aligned position to its resting spot over the preview area.
    let (preview, preview_label) = build_word_button(&screen, "", PREVIEW_FONT);
    preview.set_style_pad_left(PREVIEW_PAD_HOR, 0);
    preview.set_style_pad_right(PREVIEW_PAD_HOR, 0);
    preview.set_style_pad_top(PREVIEW_PAD_VER, 0);
    preview.set_style_pad_bottom(PREVIEW_PAD_VER, 0);
    preview.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_FLOATING);
    preview.align(LvAlign::LV_ALIGN_CENTER, 0, 0);
    preview.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);

    let state = Rc::new(QuizState {
        selected: Cell::new(None),
        candidates,
        candidate_labels,
        preview_area,
        preview,
        preview_label,
        confirm,
        confirm_icon,
    });

    let cancel_responder = responder.clone();
    let cancel_state = Rc::clone(&state);
    close
        .add_click_cb(move || {
            let selected = cancel_state.selected.get().map(|index| index as u8);
            cancel_responder.resolve(ConfirmWordAction::Cancel(selected));
        })
        .expect("failed to register cancel callback");

    let back_responder = responder.clone();
    back.add_click_cb(move || {
        back_responder.resolve(ConfirmWordAction::Back);
    })
    .expect("failed to register back callback");

    for (index, candidate) in state.candidates.iter().enumerate() {
        let select_state = Rc::clone(&state);
        candidate
            .add_click_cb(move || select_state.select(index, true))
            .expect("failed to register candidate callback");
    }

    let deselect_state = Rc::clone(&state);
    state
        .preview
        .add_click_cb(move || deselect_state.deselect())
        .expect("failed to register preview callback");

    let confirm_state = Rc::clone(&state);
    state
        .confirm
        .add_click_cb(move || {
            // The confirm button is unclickable without a selection; the check is belt and
            // braces.
            if let Some(index) = confirm_state.selected.get() {
                responder.resolve(ConfirmWordAction::Selected(index as u8));
            }
        })
        .expect("failed to register confirm callback");

    if let Some(index) = preselected {
        let index = usize::from(index);
        if index < state.candidates.len() {
            state.select(index, false);
        }
    }

    screen
}

#[cfg(test)]
mod tests {
    extern crate std;

    use core::pin::Pin;
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    use alloc::ffi::CString;
    use alloc::string::String;
    use bitbox_lvgl::{LvFont, LvPart, LvState, class, ffi};
    use util::futures::completion;

    use super::super::test_util::{ScriptedTouch, coords, lock_and_init, pump_for};
    use super::*;

    /// The candidate words of the design mockup.
    const MOCKUP_WORDS: [&str; 5] = ["kite", "rude", "outdoor", "nut", "gown"];

    /// Polls a completion future once with a no-op waker.
    fn poll_once<T>(result: &mut completion::Result<T>) -> Option<T> {
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(core::ptr::null(), &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
        let waker = unsafe { Waker::from_raw(RawWaker::new(core::ptr::null(), &VTABLE)) };
        let mut cx = Context::from_waker(&waker);
        match Pin::new(result).poll(&mut cx) {
            Poll::Ready(value) => Some(value),
            Poll::Pending => None,
        }
    }

    struct Harness {
        touch: ScriptedTouch,
        screen: LvObj,
        result: completion::Result<ConfirmWordAction>,
    }

    impl Harness {
        fn new(choices: &[&str], word_idx: usize, num_words: usize) -> Self {
            Self::new_preselected(choices, word_idx, num_words, None)
        }

        fn new_preselected(
            choices: &[&str],
            word_idx: usize,
            num_words: usize,
            preselected: Option<u8>,
        ) -> Self {
            let touch = ScriptedTouch::new();
            let (responder, result) = completion::completion();
            let screen =
                build_confirm_word_screen(choices, word_idx, num_words, preselected, responder);
            unsafe { ffi::lv_screen_load(screen.as_ptr()) };
            pump_for(60); // layout + first render
            Self {
                touch,
                screen,
                result,
            }
        }

        fn label_text(label: LvObj) -> String {
            let label = label
                .try_downcast::<class::LabelTag>()
                .expect("child is a label");
            String::from(label.get_text().unwrap().to_str().unwrap())
        }

        fn title(&self) -> LvObj {
            self.screen.child(0).expect("title")
        }

        fn subtitle(&self) -> LvObj {
            self.screen.child(1).expect("subtitle")
        }

        fn preview_area(&self) -> LvObj {
            self.screen.child(2).expect("preview area")
        }

        fn preview(&self) -> LvObj {
            // The preview floats as the last (topmost) child, after the close button.
            self.screen.child(7).expect("preview")
        }

        /// Where the preview rests: the preview area's centre, `PREVIEW_RAISE` up.
        fn preview_rest(&self) -> (i32, i32) {
            let (x, y) = center(&self.preview_area());
            (x, y + PREVIEW_RAISE)
        }

        fn preview_word(&self) -> String {
            Self::label_text(self.preview().child(0).expect("preview label"))
        }

        fn candidates(&self) -> LvObj {
            self.screen.child(4).expect("candidates container")
        }

        fn candidate(&self, index: usize) -> LvObj {
            self.candidates()
                .child(index as i32)
                .expect("candidate button")
        }

        fn candidate_word(&self, index: usize) -> String {
            Self::label_text(self.candidate(index).child(0).expect("candidate label"))
        }

        fn actions(&self) -> LvObj {
            self.screen.child(5).expect("actions row")
        }

        fn confirm_button(&self) -> LvObj {
            // The confirm button is the last action; a back button may precede it.
            let actions = self.actions();
            let mut last = actions.child(0).expect("confirm button");
            if let Some(button) = actions.child(1) {
                last = button;
            }
            last
        }

        fn close_button(&self) -> LvObj {
            self.screen.child(6).expect("close button")
        }

        fn is_hidden(obj: &LvObj) -> bool {
            unsafe { ffi::lv_obj_has_flag(obj.as_ptr(), lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN) }
        }

        fn is_clickable(obj: &LvObj) -> bool {
            unsafe { ffi::lv_obj_has_flag(obj.as_ptr(), lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE) }
        }

        fn is_disabled(obj: &LvObj) -> bool {
            unsafe { ffi::lv_obj_has_state(obj.as_ptr(), LvState::LV_STATE_DISABLED) }
        }

        /// The resolved whole-object opacity (`LV_OPA_TRANSP` hides a candidate in place).
        fn opa(obj: &LvObj) -> u8 {
            let value = unsafe {
                ffi::lv_obj_get_style_prop(
                    obj.as_ptr(),
                    LvPart::LV_PART_MAIN,
                    ffi::_lv_style_id_t::LV_STYLE_OPA as ffi::lv_style_prop_t,
                )
            };
            unsafe { value.num as u8 }
        }

        /// The resolved translate style (nonzero while a word is flying in or out).
        fn translate(obj: &LvObj) -> (i32, i32) {
            let read = |prop: ffi::_lv_style_id_t| {
                let value = unsafe {
                    ffi::lv_obj_get_style_prop(
                        obj.as_ptr(),
                        LvPart::LV_PART_MAIN,
                        prop as ffi::lv_style_prop_t,
                    )
                };
                unsafe { value.num }
            };
            (
                read(ffi::_lv_style_id_t::LV_STYLE_TRANSLATE_X),
                read(ffi::_lv_style_id_t::LV_STYLE_TRANSLATE_Y),
            )
        }

        fn tap(&mut self, obj: &LvObj) {
            let area = coords(obj);
            self.touch
                .tap((area.x1 + area.x2) / 2, (area.y1 + area.y2) / 2);
            pump_for(60);
        }
    }

    impl Drop for Harness {
        fn drop(&mut self) {
            // Swap in a fresh empty screen so the tested screen can be deleted.
            let blank = LvObj::new().unwrap();
            unsafe {
                ffi::lv_screen_load(blank.as_ptr());
            }
            pump_for(40);
            unsafe { core::ptr::read(&self.screen).delete() };
        }
    }

    /// The rendered width of `text` in `font`, in pixels.
    fn text_width(text: &str, font: LvFont) -> i32 {
        let text = CString::new(text).unwrap();
        let mut size = ffi::lv_point_t { x: 0, y: 0 };
        unsafe {
            ffi::lv_text_get_size(
                &mut size,
                text.as_ptr(),
                font.as_ptr(),
                0,
                0,
                10_000, // effectively unlimited: measure without wrapping
                ffi::lv_text_flag_t::LV_TEXT_FLAG_NONE,
            );
        }
        size.x
    }

    /// The widest BIP39 word in the candidate font.
    fn widest_word() -> &'static str {
        bip39::Language::English
            .word_list()
            .iter()
            .max_by_key(|word| text_width(word, CANDIDATE_FONT))
            .unwrap()
    }

    #[test]
    fn test_texts() {
        let _lock = lock_and_init();
        let harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        assert_eq!(
            Harness::label_text(harness.title()),
            "Confirm recovery words"
        );
        assert_eq!(Harness::label_text(harness.subtitle()), "1 of 24");
        assert_eq!(
            Harness::label_text(harness.screen.child(3).expect("prompt")),
            "Select the correct recovery word"
        );
        for (index, word) in MOCKUP_WORDS.iter().enumerate() {
            assert_eq!(harness.candidate_word(index), *word);
        }
    }

    #[test]
    fn test_initial_state() {
        let _lock = lock_and_init();
        let harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        assert!(Harness::is_hidden(&harness.preview()));
        let confirm = harness.confirm_button();
        assert!(Harness::is_disabled(&confirm));
        assert!(!Harness::is_clickable(&confirm));
        // Exactly two actions: back and confirm.
        assert!(harness.actions().child(1).is_some());
        assert!(harness.actions().child(2).is_none());
        for index in 0..MOCKUP_WORDS.len() {
            let candidate = harness.candidate(index);
            assert_eq!(Harness::opa(&candidate), 0xff);
            assert!(Harness::is_clickable(&candidate));
        }
    }

    #[test]
    fn test_select_shows_preview_and_enables_confirm() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let before: alloc::vec::Vec<_> = (0..MOCKUP_WORDS.len())
            .map(|index| coords(&harness.candidate(index)))
            .collect();

        let rude = harness.candidate(1);
        harness.tap(&rude);

        assert!(!Harness::is_hidden(&harness.preview()));
        assert_eq!(harness.preview_word(), "rude");
        assert_eq!(Harness::opa(&rude), 0);
        assert!(!Harness::is_clickable(&rude));
        let confirm = harness.confirm_button();
        assert!(!Harness::is_disabled(&confirm));
        assert!(Harness::is_clickable(&confirm));

        // The selected candidate keeps its slot: no other candidate moves, and neither does the
        // (invisible) selected one.
        for (index, area) in before.iter().enumerate() {
            let after = coords(&harness.candidate(index));
            assert_eq!(
                (area.x1, area.y1, area.x2, area.y2),
                (after.x1, after.y1, after.x2, after.y2),
                "candidate {index} moved on selection"
            );
        }
        for index in [0, 2, 3, 4] {
            assert_eq!(Harness::opa(&harness.candidate(index)), 0xff);
        }
    }

    #[test]
    fn test_switching_selection_restores_previous_candidate() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let rude = harness.candidate(1);
        harness.tap(&rude);
        let gown = harness.candidate(4);
        harness.tap(&gown);

        assert_eq!(harness.preview_word(), "gown");
        assert_eq!(Harness::opa(&rude), 0xff);
        assert!(Harness::is_clickable(&rude));
        assert_eq!(Harness::opa(&gown), 0);
        assert!(!Harness::is_clickable(&gown));
    }

    #[test]
    fn test_tapping_preview_deselects() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let rude = harness.candidate(1);
        harness.tap(&rude);
        let preview = harness.preview();
        harness.tap(&preview);

        assert!(Harness::is_hidden(&harness.preview()));
        assert_eq!(Harness::opa(&rude), 0xff);
        assert!(Harness::is_clickable(&rude));
        let confirm = harness.confirm_button();
        assert!(Harness::is_disabled(&confirm));
        assert!(!Harness::is_clickable(&confirm));

        // Nothing resolved: deselecting is not a confirmation.
        assert!(poll_once(&mut harness.result).is_none());
    }

    #[test]
    fn test_tapping_hidden_candidate_does_nothing() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let rude = harness.candidate(1);
        harness.tap(&rude);
        harness.tap(&rude); // now invisible and unclickable

        assert!(!Harness::is_hidden(&harness.preview()));
        assert_eq!(harness.preview_word(), "rude");
        assert_eq!(Harness::opa(&rude), 0);
    }

    #[test]
    fn test_confirm_without_selection_does_not_resolve() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let confirm = harness.confirm_button();
        harness.tap(&confirm);
        assert!(poll_once(&mut harness.result).is_none());
    }

    #[test]
    fn test_confirm_resolves_selected_index() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let outdoor = harness.candidate(2);
        harness.tap(&outdoor);
        let confirm = harness.confirm_button();
        harness.tap(&confirm);

        assert!(matches!(
            poll_once(&mut harness.result).expect("confirm resolves"),
            ConfirmWordAction::Selected(2)
        ));
    }

    #[test]
    fn test_back_resolves_back() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let back = harness.actions().child(0).expect("back button");
        harness.tap(&back);
        assert!(matches!(
            poll_once(&mut harness.result).expect("back resolves"),
            ConfirmWordAction::Back
        ));
    }

    #[test]
    fn test_close_resolves_cancel() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let close = harness.close_button();
        harness.tap(&close);
        assert!(matches!(
            poll_once(&mut harness.result).expect("close resolves"),
            ConfirmWordAction::Cancel(None)
        ));
    }

    /// Cancelling with a word selected reports that selection, so a declined cancel can restore
    /// it on the rebuilt screen.
    #[test]
    fn test_close_reports_current_selection() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let outdoor = harness.candidate(2);
        harness.tap(&outdoor);
        let close = harness.close_button();
        harness.tap(&close);
        assert!(matches!(
            poll_once(&mut harness.result).expect("close resolves"),
            ConfirmWordAction::Cancel(Some(2))
        ));
    }

    /// A preselected candidate (a rebuilt screen after a declined cancel) starts out selected,
    /// with the preview already at its resting position (no fly-in animation).
    #[test]
    fn test_preselected_candidate_starts_selected() {
        let _lock = lock_and_init();
        let harness = Harness::new_preselected(&MOCKUP_WORDS, 0, 24, Some(1));

        assert!(!Harness::is_hidden(&harness.preview()));
        assert_eq!(harness.preview_word(), "rude");
        assert_eq!(Harness::opa(&harness.candidate(1)), 0);
        let confirm = harness.confirm_button();
        assert!(!Harness::is_disabled(&confirm));
        assert_eq!(
            center(&harness.preview()),
            harness.preview_rest(),
            "no fly-in on a rebuilt screen: the preview starts at its resting spot"
        );
    }

    /// Synthesizes a click, running the button's callback synchronously — unlike a scripted
    /// touch there is no input/animation timing involved, so the flight's start state can be
    /// asserted deterministically right after (the callback itself sets the start translate).
    fn click(obj: &LvObj) {
        unsafe {
            ffi::lv_obj_send_event(
                obj.as_ptr(),
                lvgl::LvEventCode::LV_EVENT_CLICKED as ffi::lv_event_code_t,
                core::ptr::null_mut(),
            );
        }
    }

    /// Selecting a candidate flies its word from the row slot to the preview position: the
    /// flight starts down at the slot ("kite" sits left of and below the preview's spot) and
    /// settles at the resting position.
    #[test]
    fn test_selection_flies_word_into_preview() {
        let _lock = lock_and_init();
        let harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        click(&harness.candidate(0));
        let (start_x, start_y) = Harness::translate(&harness.preview());
        assert!(
            start_x < 0,
            "flight starts at the slot, left of the aligned centre"
        );
        assert!(
            start_y > 0,
            "flight starts at the slot, below the aligned centre"
        );

        pump_for(400); // let the 180ms flight finish, with generous slack
        assert_eq!(center(&harness.preview()), harness.preview_rest());
    }

    /// Deselecting flies the word back: the restored candidate starts translated up at the
    /// preview position and settles into its slot.
    #[test]
    fn test_deselection_flies_word_back_to_slot() {
        let _lock = lock_and_init();
        let harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        click(&harness.candidate(0));
        pump_for(400); // let the fly-in finish

        click(&harness.preview());
        let (start_x, start_y) = Harness::translate(&harness.candidate(0));
        assert!(
            start_x > 0,
            "fly-back starts at the preview, right of kite's slot"
        );
        assert!(
            start_y < 0,
            "fly-back starts at the preview, above kite's slot"
        );

        pump_for(400);
        assert_eq!(Harness::translate(&harness.candidate(0)), (0, 0));
        assert!(Harness::is_hidden(&harness.preview()));
    }

    #[test]
    fn test_deselection_restarts_from_current_preview() {
        let _lock = lock_and_init();
        let harness = Harness::new_preselected(&MOCKUP_WORDS, 0, 24, Some(1));
        let candidate = harness.candidate(1);
        let preview = harness.preview();
        let slot = center(&candidate);

        click(&preview);
        harness.screen.update_layout();
        assert_ne!(Harness::translate(&candidate), (0, 0));

        click(&candidate);
        harness.screen.update_layout();
        assert!(!Harness::is_hidden(&preview));
        assert_ne!(center(&candidate), slot);
        let preview_center = center(&preview);

        click(&preview);
        harness.screen.update_layout();
        assert!(Harness::is_hidden(&preview));
        assert_eq!(center(&candidate), preview_center);

        pump_for(400);
        assert_eq!(center(&candidate), slot);
        assert_eq!(Harness::translate(&candidate), (0, 0));
    }

    /// Back (bottom-left) and Confirm (bottom-right) sit exactly where the confirm screen puts
    /// its reject/accept buttons: spread over the standard 380px content width (50px from the
    /// display edges), flush above the standard 32px bottom padding.
    #[test]
    fn test_nav_buttons_match_confirm_screen_positions() {
        let _lock = lock_and_init();
        let harness = Harness::new(&MOCKUP_WORDS, 1, 24);

        assert_eq!(Harness::label_text(harness.subtitle()), "2 of 24");
        let back = coords(&harness.actions().child(0).expect("back button"));
        let confirm = coords(&harness.confirm_button());
        assert_eq!(back.x1, 50);
        assert_eq!(confirm.x2, 480 - 50 - 1);
        assert_eq!(back.y2, 800 - 32 - 1);
        assert_eq!(confirm.y2, 800 - 32 - 1);
    }

    /// A flying word passes over other content (e.g. the prompt), so word buttons must have an
    /// opaque black fill — a transparent one shows both texts on top of each other mid-flight.
    #[test]
    fn test_word_buttons_have_opaque_backgrounds() {
        let _lock = lock_and_init();
        let harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        for button in [harness.preview(), harness.candidate(0)] {
            let opa = unsafe {
                ffi::lv_obj_get_style_prop(
                    button.as_ptr(),
                    LvPart::LV_PART_MAIN,
                    ffi::_lv_style_id_t::LV_STYLE_BG_OPA as ffi::lv_style_prop_t,
                )
            };
            assert_eq!(unsafe { opa.num } as u8, 0xff);
            let color = unsafe {
                ffi::lv_obj_get_style_prop(
                    button.as_ptr(),
                    LvPart::LV_PART_MAIN,
                    ffi::_lv_style_id_t::LV_STYLE_BG_COLOR as ffi::lv_style_prop_t,
                )
            };
            let color = unsafe { color.color };
            assert_eq!((color.red, color.green, color.blue), (0, 0, 0));
        }
    }

    /// Despite this screen's narrowed side padding, the corner close button hugs the display
    /// corner exactly like on the 50px-padded screens (~12px from the edges).
    #[test]
    fn test_close_button_hugs_the_corner() {
        let _lock = lock_and_init();
        let harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let close = coords(&harness.close_button());
        assert_eq!(close.x2, 480 - 12 - 1);
        assert_eq!(close.y1, 12);
    }

    /// The title must not wrap: it stays a single line of the standard title font.
    #[test]
    fn test_title_fits_one_line() {
        let _lock = lock_and_init();
        let harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let title = coords(&harness.title());
        assert_eq!(
            title.y2 - title.y1 + 1,
            fonts::INTER_REGULAR_32.line_height()
        );
    }

    /// Any two candidate buttons must fit side by side in the candidates container, so five
    /// candidates always wrap into at most three rows: check the whole BIP39 wordlist.
    #[test]
    fn test_two_widest_candidates_share_a_row() {
        let _lock = lock_and_init();

        let widest = text_width(widest_word(), CANDIDATE_FONT);
        let widest_button = widest + 2 * (CANDIDATE_PAD + BORDER_WIDTH);
        assert!(
            2 * widest_button + CANDIDATE_COLUMN_GAP <= CANDIDATES_WIDTH,
            "two {}px '{}' buttons plus the {}px gap exceed the {}px container",
            widest_button,
            widest_word(),
            CANDIDATE_COLUMN_GAP,
            CANDIDATES_WIDTH
        );
    }

    /// Even five widest-possible candidates (three rows) fit between the prompt and the
    /// navigation buttons, and the preview box stays clear of the subtitle.
    #[test]
    fn test_worst_case_words_fit() {
        let _lock = lock_and_init();
        let widest = widest_word();
        let words = [widest; 5];
        let mut harness = Harness::new(&words, 0, 24);

        // Three rows: two pairs and a remainder.
        assert_eq!(
            coords(&harness.candidate(0)).y1,
            coords(&harness.candidate(1)).y1
        );
        assert_eq!(
            coords(&harness.candidate(2)).y1,
            coords(&harness.candidate(3)).y1
        );
        assert!(coords(&harness.candidate(2)).y1 > coords(&harness.candidate(1)).y1);
        assert!(coords(&harness.candidate(4)).y1 > coords(&harness.candidate(3)).y1);

        let candidates = coords(&harness.candidates());
        let actions = coords(&harness.actions());
        assert!(candidates.y2 < actions.y1);

        let candidate = harness.candidate(0);
        harness.tap(&candidate);
        pump_for(300); // let the fly-in animation settle at the resting position
        let preview = coords(&harness.preview());
        let subtitle = coords(&harness.subtitle());
        assert!(preview.y1 > subtitle.y2);
        let prompt = coords(&harness.screen.child(3).expect("prompt"));
        assert!(preview.y2 < prompt.y1);
    }

    /// The mockup's words wrap exactly as designed: kite/rude/outdoor in the first row,
    /// nut/gown in the second, one candidate-height plus one row gap further down.
    #[test]
    fn test_mockup_words_wrap_like_the_mockup() {
        let _lock = lock_and_init();
        let harness = Harness::new(&MOCKUP_WORDS, 0, 24);

        let first_row = coords(&harness.candidate(0)).y1;
        assert_eq!(coords(&harness.candidate(1)).y1, first_row);
        assert_eq!(coords(&harness.candidate(2)).y1, first_row);
        let second_row = coords(&harness.candidate(3)).y1;
        assert_eq!(coords(&harness.candidate(4)).y1, second_row);
        assert_eq!(second_row, first_row + CANDIDATE_HEIGHT + CANDIDATE_ROW_GAP);
    }
}
