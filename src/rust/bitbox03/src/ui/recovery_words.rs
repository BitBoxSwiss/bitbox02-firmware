// SPDX-License-Identifier: Apache-2.0

//! The recovery-words review screen: every word of the mnemonic on a single screen, numbered, in
//! two equal columns (the first half of the words on the left, the second half on the right).
//! Numbers are gray and right-aligned in their own sub-column; the words themselves are white.
//!
//! Numbers and words use different font sizes (medium 20 vs medium 32), whose line heights
//! differ, so each column pairs two multi-line labels whose per-line advance is equalized via
//! `text_line_space` and whose first baselines are aligned via `translate_y` — see
//! [`build_recovery_words_screen`].

use alloc::string::String;
use core::fmt::Write as _;

use bitbox_hal::ui::UserAbort;
use bitbox_lvgl::{self as lvgl, LabelExt, LvFont, LvLabel, LvObj, LvOpacityLevel, ObjExt, fonts};
use util::futures::completion::Responder;

use super::keyboard::gray;
use super::menu::transparent_row;
use super::nav_button::{NavIcon, build_nav_button};

/// What the user chose on the recovery-words screen.
#[derive(Clone, Copy)]
pub enum RecoveryWordsAction {
    /// Advance to the next step of the workflow.
    Continue,
    /// Request to cancel the workflow (the caller asks for confirmation).
    Cancel,
}

/// This screen narrows the standard 50px side padding to 20px: two columns of numbered words need
/// the width (the width invariants are pinned by `test_widest_word_and_number_fit_their_columns`).
const SIDE_PAD: i32 = 20;
/// Content width: the 480px display minus the side padding.
const CONTENT_WIDTH: i32 = 440;
/// Width of each of the two columns; the 8px remainder separates them.
const COLUMN_WIDTH: i32 = 216;
/// Width of the number sub-column; fits "24" in the number font with room to spare.
const NUMBER_WIDTH: i32 = 40;
/// Horizontal gap between a number and its word.
const NUMBER_WORD_GAP: i32 = 10;
/// Vertical distance between successive rows (baseline to baseline). The word font's line height
/// (38px) plus breathing room; 12 rows of 47px fit comfortably above the nav row.
const ROW_ADVANCE: i32 = 47;

const NUMBER_FONT: LvFont = fonts::INTER_MEDIUM_20;
const WORD_FONT: LvFont = fonts::INTER_MEDIUM_32;

/// Distance from the top of a line box to the baseline.
fn baseline_from_top(font: LvFont) -> i32 {
    font.line_height() - font.base_line()
}

/// A borderless multi-line label for one sub-column, spacing its lines `ROW_ADVANCE` apart.
fn build_column_label(parent: &LvObj, text: &str, font: LvFont) -> LvLabel {
    let label = LvLabel::new(parent).unwrap();
    label.set_text(text).unwrap();
    label.set_style_text_font(font, lvgl::LvState::LV_STATE_DEFAULT as u32);
    label.set_style_text_line_space(ROW_ADVANCE - font.line_height(), 0);
    label
}

/// One column: gray right-aligned numbers `first_number..` next to their white words.
fn build_column(parent: &LvObj, words: &[&str], first_number: usize) {
    let column = LvObj::with_parent(parent).unwrap();
    column.set_width(COLUMN_WIDTH);
    column.set_height(lvgl::ffi::LV_SIZE_CONTENT as i32);
    column.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    column.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW);
    column.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_START, 0);
    column.set_style_pad_top(0, 0);
    column.set_style_pad_bottom(0, 0);
    column.set_style_pad_left(0, 0);
    column.set_style_pad_right(0, 0);
    column.set_style_pad_column(NUMBER_WORD_GAP, 0);
    column.set_style_border_width(0, 0);
    column.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);

    let mut numbers_text = String::new();
    let mut words_text = String::new();
    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            numbers_text.push('\n');
            words_text.push('\n');
        }
        write!(numbers_text, "{}", first_number + i).unwrap();
        words_text.push_str(word);
    }

    let numbers = build_column_label(&column, &numbers_text, NUMBER_FONT);
    numbers.set_width(NUMBER_WIDTH);
    numbers.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_RIGHT, 0);
    numbers.set_style_text_color(gray(), 0);
    // The number font's first baseline sits higher in its (shorter) line box than the word font's;
    // shift the whole label down so the two sub-columns share baselines on every row.
    numbers.set_style_translate_y(
        baseline_from_top(WORD_FONT) - baseline_from_top(NUMBER_FONT),
        0,
    );

    let word_label = build_column_label(&column, &words_text, WORD_FONT);
    word_label.set_style_text_color(lvgl::color::white(), 0);
    word_label.set_style_flex_grow(1, 0);
}

/// Builds the recovery-words review screen. The words are split in half between the two columns
/// and numbered starting at 1. The navigation buttons sit exactly where the confirm screen puts
/// them: the bottom-left Cancel button resolves [`RecoveryWordsAction::Cancel`], the
/// bottom-right Next button resolves [`RecoveryWordsAction::Continue`].
pub fn build_recovery_words_screen(
    words: &[&str],
    responder: Responder<RecoveryWordsAction>,
) -> LvObj {
    assert!(!words.is_empty(), "recovery words screen requires words");

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
    // The navigation row below is narrower than the word columns; centre children so it lands
    // where the standard 50px-padded screens put it.
    screen.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);

    let title = LvLabel::new(&screen).unwrap();
    title.set_width(380);
    title.set_text("Recovery words").unwrap();
    title.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    title.set_style_text_font(
        lvgl::fonts::INTER_REGULAR_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );

    // The two word columns, vertically centred in the space between the title and the
    // navigation row.
    let columns = LvObj::with_parent(&screen).unwrap();
    columns.set_width(CONTENT_WIDTH);
    columns.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    columns.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW);
    columns.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_SPACE_BETWEEN, 0);
    columns.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    // Centres the (single) track of columns vertically; `flex_cross_place` alone does not move
    // content along the cross axis of this grown container.
    columns.set_style_flex_track_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    columns.set_style_flex_grow(1, 0);
    // Nudge the word block 10px above the exact centre of the title/navigation gap.
    columns.set_style_translate_y(-10, 0);
    columns.set_style_pad_top(0, 0);
    columns.set_style_pad_bottom(0, 0);
    columns.set_style_pad_left(0, 0);
    columns.set_style_pad_right(0, 0);
    columns.set_style_border_width(0, 0);
    columns.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);

    let rows = words.len().div_ceil(2);
    let (left, right) = words.split_at(rows);
    build_column(&columns, left, 1);
    build_column(&columns, right, rows + 1);

    // Cancel and Next as full-size navigation buttons, in the exact positions the confirm
    // screen uses (cancel left, advance right, spread over the standard 380px content width —
    // narrower than this screen's word columns).
    let actions = transparent_row(&screen, 380, 82);
    actions.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_SPACE_BETWEEN, 0);

    let cancel_responder = responder.clone();
    let cancel = build_nav_button(&actions, NavIcon::Cancel);
    cancel
        .add_click_cb(move || {
            cancel_responder.resolve(RecoveryWordsAction::Cancel);
        })
        .expect("failed to register cancel callback");

    let next = build_nav_button(&actions, NavIcon::Next);
    next.add_click_cb(move || {
        responder.resolve(RecoveryWordsAction::Continue);
    })
    .expect("failed to register continue callback");

    screen
}

/// Asks the user to confirm abandoning the recovery-words workflow.
pub(super) async fn confirm_recovery_words_cancel(
    ui: &mut impl bitbox_hal::ui::Ui,
) -> Result<(), UserAbort> {
    ui.confirm(&bitbox_hal::ui::ConfirmParams {
        title: "Recovery\nwords",
        body: "Do you really\nwant to cancel?",
        ..Default::default()
    })
    .await
}

#[cfg(test)]
mod tests {
    extern crate std;

    use core::pin::Pin;
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    use alloc::ffi::CString;
    use alloc::format;
    use alloc::vec::Vec;
    use bitbox_lvgl::{LvColor, LvPart, class, ffi};
    use util::futures::completion;

    use super::super::test_util::{ScriptedTouch, coords, lock_and_init, pump_for};
    use super::*;

    const WORDS_24: [&str; 24] = [
        "wisdom", "spoil", "tilt", "grocery", "acoustic", "shoot", "engage", "asset", "wave",
        "cinnamon", "provide", "sadness", "budget", "gravity", "vault", "boring", "sunset", "mule",
        "found", "auto", "sponsor", "salon", "faint", "patrol",
    ];

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
        result: completion::Result<RecoveryWordsAction>,
    }

    impl Harness {
        fn new(words: &[&str]) -> Self {
            let touch = ScriptedTouch::new();
            let (responder, result) = completion::completion();
            let screen = build_recovery_words_screen(words, responder);
            unsafe { ffi::lv_screen_load(screen.as_ptr()) };
            pump_for(60); // layout + first render
            Self {
                touch,
                screen,
                result,
            }
        }

        fn columns(&self) -> LvObj {
            self.screen.child(1).expect("columns container")
        }

        /// The numbers (`0`) or words (`1`) label of the left (`0`) or right (`1`) column.
        fn label_text(&self, column: usize, label: usize) -> String {
            let label = self
                .columns()
                .child(column as i32)
                .expect("column")
                .child(label as i32)
                .expect("column label")
                .try_downcast::<class::LabelTag>()
                .expect("column child is a label");
            String::from(label.get_text().unwrap().to_str().unwrap())
        }

        fn label_color(&self, column: usize, label: usize) -> LvColor {
            let label = self
                .columns()
                .child(column as i32)
                .expect("column")
                .child(label as i32)
                .expect("column label");
            let value = unsafe {
                ffi::lv_obj_get_style_prop(
                    label.as_ptr(),
                    LvPart::LV_PART_MAIN,
                    ffi::_lv_style_id_t::LV_STYLE_TEXT_COLOR as ffi::lv_style_prop_t,
                )
            };
            unsafe { value.color }
        }

        fn cancel_button(&self) -> LvObj {
            self.screen
                .child(2)
                .expect("actions row")
                .child(0)
                .expect("cancel button")
        }

        fn next_button(&self) -> LvObj {
            self.screen
                .child(2)
                .expect("actions row")
                .child(1)
                .expect("next button")
        }

        fn tap(&mut self, button: &LvObj) {
            let area = coords(button);
            self.touch
                .tap((area.x1 + area.x2) / 2, (area.y1 + area.y2) / 2);
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

    #[test]
    fn test_split_numbering_and_colors() {
        let _lock = lock_and_init();
        let harness = Harness::new(&WORDS_24);

        assert_eq!(
            harness.label_text(0, 0),
            (1..=12)
                .map(|n| format!("{n}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        assert_eq!(
            harness.label_text(1, 0),
            (13..=24)
                .map(|n| format!("{n}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        assert_eq!(harness.label_text(0, 1), WORDS_24[..12].join("\n"));
        assert_eq!(harness.label_text(1, 1), WORDS_24[12..].join("\n"));

        let expected_gray = gray();
        for column in 0..2 {
            let number_color = harness.label_color(column, 0);
            assert_eq!(
                (number_color.red, number_color.green, number_color.blue),
                (expected_gray.red, expected_gray.green, expected_gray.blue)
            );
            let word_color = harness.label_color(column, 1);
            assert_eq!(
                (word_color.red, word_color.green, word_color.blue),
                (0xff, 0xff, 0xff)
            );
        }
    }

    #[test]
    fn test_12_words_split_in_half() {
        let _lock = lock_and_init();
        let harness = Harness::new(&WORDS_24[..12]);

        assert_eq!(harness.label_text(0, 1), WORDS_24[..6].join("\n"));
        assert_eq!(harness.label_text(1, 1), WORDS_24[6..12].join("\n"));
        assert_eq!(harness.label_text(1, 0).lines().next(), Some("7"));
    }

    #[test]
    fn test_next_resolves_continue() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(&WORDS_24);

        assert!(poll_once(&mut harness.result).is_none());
        let next = harness.next_button();
        harness.tap(&next);
        assert!(matches!(
            poll_once(&mut harness.result).expect("next resolves"),
            RecoveryWordsAction::Continue
        ));
    }

    #[test]
    fn test_cancel_resolves_cancel() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(&WORDS_24);

        let cancel = harness.cancel_button();
        harness.tap(&cancel);
        assert!(matches!(
            poll_once(&mut harness.result).expect("cancel resolves"),
            RecoveryWordsAction::Cancel
        ));
    }

    /// Every row must stay on one line: a wrapped word would shift all later rows and let the
    /// user pair words with the wrong numbers. Check the whole BIP39 wordlist against the words
    /// sub-column width, and every possible number against the number sub-column width.
    #[test]
    fn test_widest_word_and_number_fit_their_columns() {
        let _lock = lock_and_init();

        let words_width = COLUMN_WIDTH - NUMBER_WIDTH - NUMBER_WORD_GAP;
        for word in bip39::Language::English.word_list() {
            assert!(
                text_width(word, WORD_FONT) <= words_width,
                "{word} does not fit the words column"
            );
        }
        for number in 1..=24 {
            assert!(text_width(&format!("{number}"), NUMBER_FONT) <= NUMBER_WIDTH);
        }
    }

    /// The 24-word layout must fit above the navigation row (no scrolling on a review screen).
    #[test]
    fn test_24_words_fit_above_navigation() {
        let _lock = lock_and_init();
        let harness = Harness::new(&WORDS_24);

        let words_bottom = (0..2)
            .map(|column| {
                let label = harness
                    .columns()
                    .child(column)
                    .expect("column")
                    .child(1)
                    .expect("words label");
                coords(&label).y2
            })
            .max()
            .unwrap();
        let actions = harness.screen.child(2).expect("actions row");
        assert!(words_bottom < coords(&actions).y1);
    }

    /// The laid-out height of a `rows`-line label whose per-row advance is `ROW_ADVANCE`.
    fn expected_label_height(rows: i32, font: LvFont) -> i32 {
        (rows - 1) * ROW_ADVANCE + font.line_height()
    }

    /// Rows must stay level across the two sub-columns of the BUILT screen, even with every slot
    /// holding the widest BIP39 word: both labels advance `ROW_ADVANCE` per row (no drift, and no
    /// wrapped line — wrapping would inflate a label's height by a whole extra line), and the
    /// numbers label sits exactly the baseline correction below the words label.
    #[test]
    fn test_rows_stay_level_at_widest_words() {
        let _lock = lock_and_init();
        let widest = ["mushroom"; 24];
        let harness = Harness::new(&widest);

        for column in 0..2 {
            let column = harness.columns().child(column).expect("column");
            let numbers = coords(&column.child(0).expect("numbers label"));
            let words = coords(&column.child(1).expect("words label"));
            assert_eq!(
                numbers.y2 - numbers.y1 + 1,
                expected_label_height(12, NUMBER_FONT)
            );
            assert_eq!(
                words.y2 - words.y1 + 1,
                expected_label_height(12, WORD_FONT)
            );
            assert_eq!(
                numbers.y1 - words.y1,
                baseline_from_top(WORD_FONT) - baseline_from_top(NUMBER_FONT)
            );
        }
    }

    /// The word block sits 10px above the vertical centre of the space between the title and the
    /// navigation row (most visible with 12 words, where over half the area is slack).
    #[test]
    fn test_block_sits_10px_above_centre() {
        let _lock = lock_and_init();
        let harness = Harness::new(&WORDS_24[..12]);

        let column = coords(&harness.columns().child(0).expect("left column"));
        let title = coords(&harness.screen.child(0).expect("title"));
        let actions = coords(&harness.screen.child(2).expect("actions row"));
        let above = column.y1 - title.y2;
        let below = actions.y1 - column.y2;
        assert!(above > 100, "12-word screen should have plenty of slack");
        assert!(
            (below - above - 20).abs() <= 1,
            "block should sit 10px above the centre (above {above}, below {below})"
        );
    }

    /// Numbers right-align in their sub-column so all right edges line up (Inter digits are not
    /// tabular: '1' is much narrower than '0').
    #[test]
    fn test_numbers_right_aligned() {
        let _lock = lock_and_init();
        let harness = Harness::new(&WORDS_24);

        for column in 0..2 {
            let numbers = harness
                .columns()
                .child(column)
                .expect("column")
                .child(0)
                .expect("numbers label");
            let value = unsafe {
                ffi::lv_obj_get_style_prop(
                    numbers.as_ptr(),
                    LvPart::LV_PART_MAIN,
                    ffi::_lv_style_id_t::LV_STYLE_TEXT_ALIGN as ffi::lv_style_prop_t,
                )
            };
            assert_eq!(
                unsafe { value.num },
                lvgl::LvTextAlign::LV_TEXT_ALIGN_RIGHT as i32
            );
        }
    }

    /// Cancel (bottom-left) and Next (bottom-right) sit exactly where the confirm screen puts
    /// its reject/accept buttons: spread over the standard 380px content width (50px from the
    /// display edges), flush above the standard 32px bottom padding.
    #[test]
    fn test_nav_buttons_match_confirm_screen_positions() {
        let _lock = lock_and_init();
        let harness = Harness::new(&WORDS_24);

        let cancel = coords(&harness.cancel_button());
        let next = coords(&harness.next_button());
        assert_eq!(cancel.x1, 50);
        assert_eq!(next.x2, 480 - 50 - 1);
        assert_eq!(cancel.y2, 800 - 32 - 1);
        assert_eq!(next.y2, 800 - 32 - 1);
    }
}
