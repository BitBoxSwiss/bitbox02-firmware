// SPDX-License-Identifier: Apache-2.0

use alloc::{rc::Rc, string::String, vec::Vec};

use bitbox_hal::ui::{CanCancel, EnterStringParams, UserAbort, WordlistEntryAbort};
use bitbox_lvgl::{
    self as lvgl, KeyboardExt, LabelExt, LvAlign, LvButton, LvButtonmatrixCtrl, LvKeyboard,
    LvKeyboardMapEntry, LvLabel, LvLabelLongMode, LvObj, LvOpacityLevel, LvPart, LvTextarea,
    ObjExt, TextareaExt, class,
};
use util::futures::completion::Responder;

use super::keyboard::build_keyboard;
use super::keypad::build_keypad;
use super::nav_button::{NavIcon, build_close_button, build_nav_button};
use super::slide_to_confirm::build_slide_to_confirm;

/// Snapshots the (possibly secret) textarea content into a zeroized-on-drop string. Reads LVGL's
/// buffer in place — `TextareaExt::get_text` would copy the content into an intermediate
/// `CString` that is dropped without zeroizing. The single `push_str` into the empty string
/// allocates exactly once, so no reallocation leaves an unzeroized copy behind either.
fn snapshot_text(textarea: &LvTextarea) -> zeroize::Zeroizing<String> {
    let mut snapshot = zeroize::Zeroizing::new(String::new());
    let text = unsafe { lvgl::ffi::lv_textarea_get_text(textarea.as_ptr()) };
    if !text.is_null() {
        let text = unsafe { core::ffi::CStr::from_ptr(text) };
        snapshot.push_str(text.to_str().expect("textarea content must be valid UTF-8"));
    }
    snapshot
}

/// Whether the textarea is empty, read in place from LVGL's buffer — unlike [`snapshot_text`]
/// this does not copy the (possibly secret) content to the heap.
pub(super) fn textarea_is_empty(textarea: &LvTextarea) -> bool {
    let text = unsafe { lvgl::ffi::lv_textarea_get_text(textarea.as_ptr()) };
    text.is_null() || unsafe { *text == 0 }
}

/// The textarea's length in bytes and its last byte, read in place from LVGL's buffer without
/// copying the (possibly secret) content to the heap. The passphrase keyboard only enters ASCII,
/// so bytes are characters.
fn textarea_len_and_last(textarea: &LvTextarea) -> (usize, Option<u8>) {
    let text = unsafe { lvgl::ffi::lv_textarea_get_text(textarea.as_ptr()) };
    if text.is_null() {
        return (0, None);
    }
    let mut len = 0usize;
    let mut last = 0u8;
    loop {
        let byte = unsafe { *text.add(len) } as u8;
        if byte == 0 {
            break;
        }
        last = byte;
        len += 1;
    }
    (len, (len > 0).then_some(last))
}

/// Diameter of a circle masking one entered passphrase character.
const MASK_DOT_SIZE: i32 = 24;
/// Gap between masking circles.
const MASK_DOT_GAP: i32 = 10;
/// Masking circles shown at most (what fits the display row). The count saturates here: the
/// circles are identical, and real passphrases are far shorter than the 149-character cap.
const MASK_DOT_COUNT_MAX: usize = 10;

#[derive(Clone, Copy)]
enum KeyboardMode {
    LowerCase,
    UpperCase,
    Digits,
    SpecialChars,
}

impl KeyboardMode {
    fn next(self, special_chars: bool) -> Self {
        match self {
            Self::LowerCase => Self::UpperCase,
            Self::UpperCase => Self::Digits,
            Self::Digits => {
                if special_chars {
                    Self::SpecialChars
                } else {
                    Self::LowerCase
                }
            }
            Self::SpecialChars => Self::LowerCase,
        }
    }

    fn switch_label(self, special_chars: bool) -> &'static str {
        match self {
            Self::LowerCase => "ABC",
            Self::UpperCase => "123",
            Self::Digits => {
                if special_chars {
                    "&?+"
                } else {
                    "abc"
                }
            }
            Self::SpecialChars => "abc",
        }
    }

    fn lvgl_mode(self) -> lvgl::LvKeyboardMode {
        match self {
            Self::LowerCase => lvgl::LvKeyboardMode::LV_KEYBOARD_MODE_USER_1,
            Self::UpperCase => lvgl::LvKeyboardMode::LV_KEYBOARD_MODE_USER_2,
            Self::Digits => lvgl::LvKeyboardMode::LV_KEYBOARD_MODE_USER_3,
            Self::SpecialChars => lvgl::LvKeyboardMode::LV_KEYBOARD_MODE_USER_4,
        }
    }
}

const KEY_WIDTH_1: LvButtonmatrixCtrl = LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_WIDTH_1;

const LOWERCASE_MAP: &[LvKeyboardMapEntry] = &[
    LvKeyboardMapEntry::new(c"a"),
    LvKeyboardMapEntry::new(c"b"),
    LvKeyboardMapEntry::new(c"c"),
    LvKeyboardMapEntry::new(c"d"),
    LvKeyboardMapEntry::new(c"e"),
    LvKeyboardMapEntry::new(c"f"),
    LvKeyboardMapEntry::new(c"g"),
    LvKeyboardMapEntry::new(c"h"),
    LvKeyboardMapEntry::new(c"i"),
    LvKeyboardMapEntry::new(c"\n"),
    LvKeyboardMapEntry::new(c"j"),
    LvKeyboardMapEntry::new(c"k"),
    LvKeyboardMapEntry::new(c"l"),
    LvKeyboardMapEntry::new(c"m"),
    LvKeyboardMapEntry::new(c"n"),
    LvKeyboardMapEntry::new(c"o"),
    LvKeyboardMapEntry::new(c"p"),
    LvKeyboardMapEntry::new(c"q"),
    LvKeyboardMapEntry::new(c"r"),
    LvKeyboardMapEntry::new(c"\n"),
    LvKeyboardMapEntry::new(c"s"),
    LvKeyboardMapEntry::new(c"t"),
    LvKeyboardMapEntry::new(c"u"),
    LvKeyboardMapEntry::new(c"v"),
    LvKeyboardMapEntry::new(c"w"),
    LvKeyboardMapEntry::new(c"x"),
    LvKeyboardMapEntry::new(c"y"),
    LvKeyboardMapEntry::new(c"z"),
    LvKeyboardMapEntry::new(c""),
];

const UPPERCASE_MAP: &[LvKeyboardMapEntry] = &[
    LvKeyboardMapEntry::new(c"A"),
    LvKeyboardMapEntry::new(c"B"),
    LvKeyboardMapEntry::new(c"C"),
    LvKeyboardMapEntry::new(c"D"),
    LvKeyboardMapEntry::new(c"E"),
    LvKeyboardMapEntry::new(c"F"),
    LvKeyboardMapEntry::new(c"G"),
    LvKeyboardMapEntry::new(c"H"),
    LvKeyboardMapEntry::new(c"I"),
    LvKeyboardMapEntry::new(c"\n"),
    LvKeyboardMapEntry::new(c"J"),
    LvKeyboardMapEntry::new(c"K"),
    LvKeyboardMapEntry::new(c"L"),
    LvKeyboardMapEntry::new(c"M"),
    LvKeyboardMapEntry::new(c"N"),
    LvKeyboardMapEntry::new(c"O"),
    LvKeyboardMapEntry::new(c"P"),
    LvKeyboardMapEntry::new(c"Q"),
    LvKeyboardMapEntry::new(c"R"),
    LvKeyboardMapEntry::new(c"\n"),
    LvKeyboardMapEntry::new(c"S"),
    LvKeyboardMapEntry::new(c"T"),
    LvKeyboardMapEntry::new(c"U"),
    LvKeyboardMapEntry::new(c"V"),
    LvKeyboardMapEntry::new(c"W"),
    LvKeyboardMapEntry::new(c"X"),
    LvKeyboardMapEntry::new(c"Y"),
    LvKeyboardMapEntry::new(c"Z"),
    LvKeyboardMapEntry::new(c""),
];

const DIGITS_MAP: &[LvKeyboardMapEntry] = &[
    LvKeyboardMapEntry::new(c"1"),
    LvKeyboardMapEntry::new(c"2"),
    LvKeyboardMapEntry::new(c"3"),
    LvKeyboardMapEntry::new(c"4"),
    LvKeyboardMapEntry::new(c"5"),
    LvKeyboardMapEntry::new(c"\n"),
    LvKeyboardMapEntry::new(c"6"),
    LvKeyboardMapEntry::new(c"7"),
    LvKeyboardMapEntry::new(c"8"),
    LvKeyboardMapEntry::new(c"9"),
    LvKeyboardMapEntry::new(c"0"),
    LvKeyboardMapEntry::new(c""),
];

const SPECIAL_CHARS_MAP: &[LvKeyboardMapEntry] = &[
    LvKeyboardMapEntry::new(c" "),
    LvKeyboardMapEntry::new(c"!"),
    LvKeyboardMapEntry::new(c"\""),
    LvKeyboardMapEntry::new(c"#"),
    LvKeyboardMapEntry::new(c"$"),
    LvKeyboardMapEntry::new(c"%"),
    LvKeyboardMapEntry::new(c"&"),
    LvKeyboardMapEntry::new(c"'"),
    LvKeyboardMapEntry::new(c"\n"),
    LvKeyboardMapEntry::new(c"("),
    LvKeyboardMapEntry::new(c")"),
    LvKeyboardMapEntry::new(c"*"),
    LvKeyboardMapEntry::new(c"+"),
    LvKeyboardMapEntry::new(c","),
    LvKeyboardMapEntry::new(c"-"),
    LvKeyboardMapEntry::new(c"."),
    LvKeyboardMapEntry::new(c"/"),
    LvKeyboardMapEntry::new(c"\n"),
    LvKeyboardMapEntry::new(c":"),
    LvKeyboardMapEntry::new(c";"),
    LvKeyboardMapEntry::new(c"<"),
    LvKeyboardMapEntry::new(c"="),
    LvKeyboardMapEntry::new(c">"),
    LvKeyboardMapEntry::new(c"?"),
    LvKeyboardMapEntry::new(c"^"),
    LvKeyboardMapEntry::new(c"["),
    LvKeyboardMapEntry::new(c"\n"),
    LvKeyboardMapEntry::new(c"\\"),
    LvKeyboardMapEntry::new(c"]"),
    LvKeyboardMapEntry::new(c"@"),
    LvKeyboardMapEntry::new(c"_"),
    LvKeyboardMapEntry::new(c"{"),
    LvKeyboardMapEntry::new(c"|"),
    LvKeyboardMapEntry::new(c"}"),
    LvKeyboardMapEntry::new(c""),
];

const LOWERCASE_CTRL_MAP: &[LvButtonmatrixCtrl] = &[KEY_WIDTH_1; 26];
const UPPERCASE_CTRL_MAP: &[LvButtonmatrixCtrl] = &[KEY_WIDTH_1; 26];
const DIGITS_CTRL_MAP: &[LvButtonmatrixCtrl] = &[KEY_WIDTH_1; 10];
const SPECIAL_CHARS_CTRL_MAP: &[LvButtonmatrixCtrl] = &[KEY_WIDTH_1; 31];

fn configure_keyboard_maps(keyboard: &LvKeyboard) {
    keyboard.set_map(
        lvgl::LvKeyboardMode::LV_KEYBOARD_MODE_USER_1,
        LOWERCASE_MAP,
        LOWERCASE_CTRL_MAP,
    );
    keyboard.set_map(
        lvgl::LvKeyboardMode::LV_KEYBOARD_MODE_USER_2,
        UPPERCASE_MAP,
        UPPERCASE_CTRL_MAP,
    );
    keyboard.set_map(
        lvgl::LvKeyboardMode::LV_KEYBOARD_MODE_USER_3,
        DIGITS_MAP,
        DIGITS_CTRL_MAP,
    );
    keyboard.set_map(
        lvgl::LvKeyboardMode::LV_KEYBOARD_MODE_USER_4,
        SPECIAL_CHARS_MAP,
        SPECIAL_CHARS_CTRL_MAP,
    );
}

fn add_button<F>(parent: &LvObj, width: i32, height: i32, label: &str, cb: F)
where
    F: FnMut() + 'static,
{
    let button = LvButton::new(parent).unwrap();
    button.set_size(width, height);
    button.set_style_bg_color(lvgl::color::hex(0x30333a), 0);
    button.set_style_bg_opa(LvOpacityLevel::LV_OPA_COVER as u8, 0);
    button.set_style_border_width(2, 0);
    button.set_style_border_color(lvgl::color::white(), 0);
    button
        .add_click_cb(cb)
        .expect("failed to register click callback");

    let button_label = LvLabel::new(&button).unwrap();
    button_label.set_text(label).unwrap();
    button_label.set_style_text_font(
        lvgl::fonts::INTER_BOLD_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
    button_label.set_style_text_color(lvgl::color::white(), 0);
    button_label.align(LvAlign::LV_ALIGN_CENTER, 0, 0);
}

/// Adds the standard entry-screen title label (32px regular, standard content width, wrapping).
fn add_title(screen: &LvObj, text: &str) {
    let title = LvLabel::new(screen).unwrap();
    title.set_width(380);
    title.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    title.set_text(text).unwrap();
    title.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    title.set_style_text_font(
        lvgl::fonts::INTER_REGULAR_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
}

/// Adds the standard entry text field (380×72; masked with `*` bullets when `hide`).
fn add_textarea(screen: &LvObj, preset: &str, hide: bool) -> LvTextarea {
    let textarea = LvTextarea::new(screen).unwrap();
    textarea.set_size(380, 72);
    textarea.set_one_line(true);
    textarea
        .set_text(preset)
        .expect("preset must not contain NUL");
    textarea.set_style_bg_color(lvgl::color::hex(0x111317), 0);
    textarea.set_style_bg_opa(LvOpacityLevel::LV_OPA_COVER as u8, 0);
    textarea.set_style_border_width(2, 0);
    textarea.set_style_border_color(lvgl::color::white(), 0);
    textarea.set_style_radius(0, 0);
    textarea.set_style_pad_left(20, 0);
    textarea.set_style_pad_right(20, 0);
    textarea.set_style_pad_top(16, 0);
    textarea.set_style_pad_bottom(16, 0);
    textarea.set_style_text_color(lvgl::color::white(), 0);
    textarea.set_style_text_font(
        lvgl::fonts::INTER_REGULAR_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
    if hide {
        textarea.set_password_mode(true);
        textarea
            .set_password_bullet("*")
            .expect("valid password bullet");
        textarea.set_password_show_time(0);
    }
    textarea
}

/// Adds the bottom actions row: standard content width, nav-button height, children spread to
/// the edges.
fn add_actions_row(screen: &LvObj) -> LvObj {
    let actions = LvObj::with_parent(screen).unwrap();
    actions.set_width(380);
    actions.set_height(82);
    actions.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    actions.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW);
    actions.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_SPACE_BETWEEN, 0);
    actions.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    actions.set_style_pad_top(0, 0);
    actions.set_style_pad_bottom(0, 0);
    actions.set_style_pad_left(0, 0);
    actions.set_style_pad_right(0, 0);
    actions.set_style_border_width(0, 0);
    actions.set_style_bg_opa(
        LvOpacityLevel::LV_OPA_TRANSP as u8,
        LvPart::LV_PART_MAIN as u32,
    );
    actions
}

/// The BIP39 passphrase entry screen: title, a masked entry display (LVGL-drawn circles plus the
/// last entered character in plaintext), the full QWERTY keyboard component and a
/// backspace/confirm navigation row.
///
/// Unlike the generic entry screen, accepting is a plain tap on the checkmark even though the
/// passphrase params request `longtouch` (the workflow visually confirms the passphrase on a
/// separate screen right after). Backspace lives in the navigation row (grayed out while the
/// input is empty); with `CanCancel::Yes` (not used by the passphrase workflow, which cannot be
/// cancelled) a corner close button rejects.
pub fn build_passphrase_screen(
    params: &EnterStringParams<'_>,
    can_cancel: CanCancel,
    preset: &str,
    responder: Responder<Result<zeroize::Zeroizing<String>, UserAbort>>,
) -> LvObj {
    let screen = build_entry_screen_frame();

    add_title(&screen, params.title);

    let textarea = add_masked_display(&screen, preset);

    // The keyboard and the navigation row are anchored to the bottom of the screen (taken out
    // of the flex flow), so the Back/Confirm buttons sit exactly where the other workflows put
    // them — flush above the standard 32px bottom padding — with the keyboard right above,
    // independent of how many lines the title wraps to.
    let keyboard = build_keyboard(&screen, Rc::clone(&textarea));
    keyboard.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_FLOATING);
    keyboard.align(LvAlign::LV_ALIGN_BOTTOM_MID, 0, -(82 + 20));

    let actions = add_actions_row(&screen);
    actions.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_FLOATING);
    actions.align(LvAlign::LV_ALIGN_BOTTOM_MID, 0, 0);

    // The flex-flow stand-in for the floating keyboard and navigation row: it makes the growing
    // entry display above end a standard gap over the keyboard.
    add_bottom_region_spacer(&screen, super::keyboard::KEYBOARD_HEIGHT + 20 + 82);

    // Backspace (the mockup's left chevron): deletes the last character; gray and inert while
    // the input is empty.
    let backspace = build_nav_button(&actions, NavIcon::Back);
    let backspace_icon = style_nav_button_disabled(&backspace);
    let delete_textarea = Rc::clone(&textarea);
    backspace
        .add_click_cb(move || delete_textarea.delete_char())
        .expect("failed to register backspace callback");

    let refresh_textarea = Rc::clone(&textarea);
    let refresh_backspace = Rc::new(move || {
        set_nav_button_enabled(
            &backspace,
            &backspace_icon,
            !textarea_is_empty(refresh_textarea.as_ref()),
        );
    });
    refresh_backspace();
    let refresh_backspace_cb = Rc::clone(&refresh_backspace);
    textarea
        .add_event_cb(lvgl::LvEventCode::LV_EVENT_VALUE_CHANGED, move || {
            refresh_backspace_cb()
        })
        .expect("failed to register textarea change callback");

    if matches!(can_cancel, CanCancel::Yes) {
        let reject_responder = responder.clone();
        let close = build_close_button(&screen);
        // This screen has no side padding; re-anchor the corner button ~12px from the edges.
        close.align(lvgl::LvAlign::LV_ALIGN_TOP_RIGHT, -12, -28);
        close
            .add_click_cb(move || reject_responder.resolve(Err(UserAbort)))
            .expect("failed to register cancel callback");
    }

    let accept = build_nav_button(&actions, NavIcon::Confirm);
    accept
        .add_click_cb(move || {
            responder.resolve(Ok(snapshot_text(textarea.as_ref())));
        })
        .expect("failed to register confirm callback");

    screen
}

/// The gray disabled look of a navigation icon button (border and icon); returns the icon for
/// state toggling. Pairs with [`set_nav_button_enabled`].
fn style_nav_button_disabled(button: &LvButton) -> LvObj {
    const DISABLED: u32 = lvgl::LvState::LV_STATE_DISABLED as u32;
    let icon = button.child(0).expect("nav button icon");
    button.set_style_border_color(super::keyboard::gray(), DISABLED);
    icon.set_style_image_recolor(super::keyboard::gray(), DISABLED);
    icon
}

/// Enables/disables a navigation icon button: disabled renders the gray look from
/// [`style_nav_button_disabled`] and makes the button inert.
fn set_nav_button_enabled(button: &LvButton, icon: &LvObj, enabled: bool) {
    if enabled {
        button.remove_state(lvgl::LvState::LV_STATE_DISABLED);
        icon.remove_state(lvgl::LvState::LV_STATE_DISABLED);
        button.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
    } else {
        button.add_state(lvgl::LvState::LV_STATE_DISABLED);
        icon.add_state(lvgl::LvState::LV_STATE_DISABLED);
        button.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
    }
}

/// Runs `f` on the textarea's content, borrowed in place from LVGL's buffer — unlike
/// [`snapshot_text`] this makes no copy of the (possibly secret) content.
fn with_textarea_text<R>(textarea: &LvTextarea, f: impl FnOnce(&str) -> R) -> R {
    let text = unsafe { lvgl::ffi::lv_textarea_get_text(textarea.as_ptr()) };
    if text.is_null() {
        return f("");
    }
    let text = unsafe { core::ffi::CStr::from_ptr(text) };
    f(text.to_str().expect("textarea content must be valid UTF-8"))
}

/// The BIP39 English word at `idx`, or `None` if out of range. The wordlist is public
/// compiled-in data, so the returned `&'static str` needs no zeroizing.
fn bip39_word(idx: u16) -> Option<&'static str> {
    bip39::Language::English
        .word_list()
        .get(idx as usize)
        .copied()
}

/// Word-entry state derived from `wordlist` (BIP39 word indices; out-of-range indices are
/// skipped) for the entered `prefix`.
struct WordlistMatch {
    /// The letters that can follow `prefix` towards one of the words.
    next_letters: super::keyboard::LetterSet,
    /// `prefix` is itself one of the words.
    complete: bool,
    /// The one word starting with `prefix`, if exactly one does (BitBox02 parity: a duplicated
    /// index counts as two matches).
    unique: Option<&'static str>,
}

fn wordlist_matches(wordlist: &[u16], prefix: &str) -> WordlistMatch {
    let mut next_letters = super::keyboard::LetterSet::EMPTY;
    let mut complete = false;
    let mut last_match = None;
    let mut match_count = 0usize;
    for &idx in wordlist {
        let Some(word) = bip39_word(idx) else {
            continue;
        };
        if let Some(rest) = word.strip_prefix(prefix) {
            match_count += 1;
            last_match = Some(word);
            match rest.as_bytes().first() {
                Some(&letter) => next_letters.insert(letter),
                None => complete = true,
            }
        }
    }
    WordlistMatch {
        next_letters,
        complete,
        unique: if match_count == 1 { last_match } else { None },
    }
}

/// Adds the visible word entry display: the standard entry field, centred (via a `flex_grow`
/// wrapper, like the passphrase screen's masked display) in the space the screen's flex flow
/// leaves between the title and the bottom-anchored keyboard region.
///
/// Returns the textarea (the display row's child 0) that the input widgets operate on.
fn add_word_display(screen: &LvObj, preset: &str) -> Rc<LvTextarea> {
    let display = LvObj::with_parent(screen).unwrap();
    display.set_size(380, 72);
    display.set_style_flex_grow(1, 0);
    display.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    display.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW);
    display.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    display.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    // Centre the flex track too: cross-place only centres items within their (72px-tall) track,
    // and the track itself defaults to the container's top.
    display.set_style_flex_track_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    display.set_style_pad_top(0, 0);
    display.set_style_pad_bottom(0, 0);
    display.set_style_pad_left(0, 0);
    display.set_style_pad_right(0, 0);
    display.set_style_border_width(0, 0);
    display.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    display.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_SCROLLABLE);
    display.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);

    let textarea = add_textarea(&display, preset, false);
    // The field is display-only: a tap must not move the insertion cursor into the middle of
    // the word (letters always append; backspace always deletes the last one).
    textarea.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
    // No entry box on this screen: the letters sit directly on the background, centred, in the
    // same bold 48px the mnemonic review screen shows the words in. The standard field's 72px
    // box (16px paddings) cannot hold that line height, so the field sizes to its content.
    textarea.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    textarea.set_style_border_width(0, 0);
    textarea.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    textarea.set_style_text_font(
        lvgl::fonts::INTER_BOLD_48,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
    textarea.set_style_pad_top(0, 0);
    textarea.set_style_pad_bottom(0, 0);
    textarea.set_height(lvgl::ffi::LV_SIZE_CONTENT as i32);
    Rc::new(textarea)
}

/// The BIP39 recovery-word entry screen: title ("x of y"), the word being typed in the standard
/// entry field (recovery words are shown in plaintext — reading the word back is the point of
/// the entry), the letters-only keyboard and a backspace/confirm navigation row.
///
/// The keyboard only ever offers letters that extend the entry towards one of the
/// `params.wordlist` words (BIP39 word indices; must be set); all other keys are gray and
/// inert, so no invalid word can be composed. A typed letter that leaves exactly one candidate
/// autocompletes the whole word (BitBox02 parity; deleting never re-autocompletes, so backspace
/// can undo it). Confirm is likewise gray until the entry exactly matches a wordlist word, so
/// the workflow's "Invalid word" retry loop can never trigger. Back deletes the last letter; on
/// an empty entry it rejects with [`WordlistEntryAbort::Back`] — the mnemonic workflow goes
/// straight back to the previous word. With `CanCancel::Yes` (always set by that workflow) a
/// corner close button rejects with [`WordlistEntryAbort::Cancel`] at any time — the workflow
/// asks "Cancel restore?"; with `CanCancel::No` there is no close button and Back grays out on
/// an empty entry.
pub fn build_wordlist_screen(
    params: &EnterStringParams<'_>,
    can_cancel: CanCancel,
    preset: &str,
    responder: Responder<Result<zeroize::Zeroizing<String>, WordlistEntryAbort>>,
) -> LvObj {
    let wordlist = params
        .wordlist
        .expect("wordlist entry requires params.wordlist");
    let screen = build_entry_screen_frame();

    add_title(&screen, params.title);

    let textarea = add_word_display(&screen, preset);
    // Wordlist words are lowercase ASCII, at most 8 letters; refuse anything else at the widget
    // level too (the key filtering already guarantees it for touch input).
    textarea.set_accepted_chars(Some(c"abcdefghijklmnopqrstuvwxyz"));
    textarea.set_max_length(8);

    let wordlist: Rc<[u16]> = wordlist.into();

    // BitBox02 parity: a typed letter that leaves exactly one candidate autocompletes the whole
    // word. This runs from the keyboard's insert path, not the textarea change callback: only
    // insertions may autocomplete (completing after a deletion would trap backspace in an
    // undo-redo loop right after it deletes an autocompleted letter), and the textarea callback
    // cannot mutate the textarea anyway (that would re-enter it). The `set_text` here fires the
    // change callback below, which then relays the completed state to the keys and buttons.
    let autocomplete_textarea = Rc::clone(&textarea);
    let autocomplete_wordlist = Rc::clone(&wordlist);
    let autocomplete: Rc<dyn Fn()> = Rc::new(move || {
        let unique = with_textarea_text(autocomplete_textarea.as_ref(), |prefix| {
            wordlist_matches(&autocomplete_wordlist, prefix)
                .unique
                .filter(|word| word.len() > prefix.len())
        });
        if let Some(word) = unique {
            autocomplete_textarea
                .set_text(word)
                .expect("wordlist words contain no NUL");
        }
    });

    // Keyboard and navigation row are bottom-anchored like on the passphrase screen (see
    // `build_passphrase_screen`), with the keyboard sitting 50px higher over the navigation
    // row here.
    let keyboard =
        super::keyboard::build_wordlist_keyboard(&screen, Rc::clone(&textarea), autocomplete);
    keyboard.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_FLOATING);
    keyboard.align(LvAlign::LV_ALIGN_BOTTOM_MID, 0, -(82 + 20 + 50));

    let actions = add_actions_row(&screen);
    actions.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_FLOATING);
    actions.align(LvAlign::LV_ALIGN_BOTTOM_MID, 0, 0);

    add_bottom_region_spacer(
        &screen,
        super::keyboard::WORDLIST_KEYBOARD_HEIGHT + 20 + 50 + 82,
    );

    // Back: deletes the last letter; on an empty entry it goes back to the previous word
    // instead (the mnemonic workflow re-opens that word directly). With `CanCancel::No` (not
    // used by the mnemonic workflow) there is nowhere to go back to, so the button grays out
    // on an empty entry.
    let allow_back_out = matches!(can_cancel, CanCancel::Yes);
    let backspace = build_nav_button(&actions, NavIcon::Back);
    let backspace_icon = style_nav_button_disabled(&backspace);
    let delete_textarea = Rc::clone(&textarea);
    let back_responder = responder.clone();
    backspace
        .add_click_cb(move || {
            if !textarea_is_empty(delete_textarea.as_ref()) {
                delete_textarea.delete_char();
            } else if allow_back_out {
                back_responder.resolve(Err(WordlistEntryAbort::Back));
            }
        })
        .expect("failed to register backspace callback");

    // The corner close button requests cancelling the whole restore; the workflow asks for
    // confirmation before acting on it.
    if matches!(can_cancel, CanCancel::Yes) {
        let reject_responder = responder.clone();
        let close = build_close_button(&screen);
        // This screen has no side padding; re-anchor the corner button ~12px from the edges.
        close.align(lvgl::LvAlign::LV_ALIGN_TOP_RIGHT, -12, -28);
        close
            .add_click_cb(move || reject_responder.resolve(Err(WordlistEntryAbort::Cancel)))
            .expect("failed to register cancel callback");
    }

    // Confirm: gray and inert until the entry is a complete wordlist word.
    let accept = build_nav_button(&actions, NavIcon::Confirm);
    let accept_icon = style_nav_button_disabled(&accept);
    {
        let textarea = Rc::clone(&textarea);
        accept
            .add_click_cb(move || {
                responder.resolve(Ok(snapshot_text(textarea.as_ref())));
            })
            .expect("failed to register confirm callback");
    }

    // Relay the word-entry state — which letters may come next and whether the entry is a
    // complete word — from the wordlist to the keys and buttons on every content change.
    let refresh_textarea = Rc::clone(&textarea);
    let refresh = Rc::new(move || {
        let (len, matches) = with_textarea_text(refresh_textarea.as_ref(), |prefix| {
            (prefix.len(), wordlist_matches(&wordlist, prefix))
        });
        super::keyboard::set_enabled_letters(&keyboard, matches.next_letters);
        set_nav_button_enabled(&backspace, &backspace_icon, len > 0 || allow_back_out);
        set_nav_button_enabled(&accept, &accept_icon, matches.complete);
    });
    refresh();
    let refresh_cb = Rc::clone(&refresh);
    textarea
        .add_event_cb(lvgl::LvEventCode::LV_EVENT_VALUE_CHANGED, move || {
            refresh_cb()
        })
        .expect("failed to register wordlist refresh callback");

    screen
}

/// Builds a screen skeleton shared by the passphrase and PIN screens: black background, flex
/// column with both cross alignments centred (content is wider than the standard 380px on the
/// passphrase screen), standard outer padding, no scrolling.
fn build_entry_screen_frame() -> LvObj {
    let screen = LvObj::new().unwrap();
    screen.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    screen.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_COLUMN);
    screen.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_SCROLLABLE);
    screen.set_style_bg_color(lvgl::color::black(), 0);
    screen.set_style_text_color(lvgl::color::white(), 0);
    // Centring needs both alignments: CROSS centres items within their flex track (which is
    // only as wide as the widest child), TRACK centres that track on the screen.
    screen.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    screen.set_style_flex_track_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    screen.set_style_pad_top(40, 0);
    screen.set_style_pad_right(0, 0);
    screen.set_style_pad_bottom(32, 0);
    screen.set_style_pad_left(0, 0);
    screen.set_style_pad_row(20, 0);
    screen
}

/// Adds the invisible flex-flow stand-in for a bottom-anchored (floating) input widget region,
/// so the growing entry display above it ends a standard gap over that region.
fn add_bottom_region_spacer(screen: &LvObj, height: i32) {
    let spacer = LvObj::with_parent(screen).unwrap();
    spacer.set_size(0, height);
    spacer.set_style_border_width(0, 0);
    spacer.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    spacer.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
}

/// The device PIN entry screen ("PIN entry mockup"): title, the masked entry display and a
/// numeric 3×4 keypad whose bottom row carries backspace, 0 and a tap confirm.
///
/// The keypad is bottom-anchored so its bottom row sits exactly where the other workflows put
/// their navigation buttons. The BitBox03's device unlock secret is a numeric PIN, so titles
/// show "PIN" where the (BitBox02-shared) workflow strings say "password", and the input
/// accepts digits only. As on the passphrase screen, accepting is a plain tap despite the
/// params' `longtouch`; with `CanCancel::Yes` (set/repeat PIN) a corner close button rejects.
pub fn build_pin_screen(
    params: &EnterStringParams<'_>,
    can_cancel: CanCancel,
    preset: &str,
    responder: Responder<Result<zeroize::Zeroizing<String>, UserAbort>>,
) -> LvObj {
    let screen = build_entry_screen_frame();

    add_title(&screen, &params.title.replace("password", "PIN"));

    let textarea = add_masked_display(&screen, preset);
    textarea.set_accepted_chars(Some(c"0123456789"));

    let confirm_textarea = Rc::clone(&textarea);
    let confirm_responder = responder.clone();
    let keypad = build_keypad(&screen, Rc::clone(&textarea), move || {
        confirm_responder.resolve(Ok(snapshot_text(confirm_textarea.as_ref())));
    });
    keypad.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_FLOATING);
    keypad.align(LvAlign::LV_ALIGN_BOTTOM_MID, 0, 0);
    add_bottom_region_spacer(&screen, super::keypad::KEYPAD_HEIGHT);

    if matches!(can_cancel, CanCancel::Yes) {
        let reject_responder = responder.clone();
        let close = build_close_button(&screen);
        // This screen has no side padding; re-anchor the corner button ~12px from the edges.
        close.align(lvgl::LvAlign::LV_ALIGN_TOP_RIGHT, -12, -28);
        close
            .add_click_cb(move || reject_responder.resolve(Err(UserAbort)))
            .expect("failed to register cancel callback");
    }

    screen
}

/// Adds the masked entry display: a bare centred row of one filled circle per masked character —
/// drawn as LVGL objects, since the ASCII-only fonts have no bullet glyph — with the last
/// entered character in plaintext until the next keystroke (deleting re-masks everything). The
/// row has `flex_grow`, so it fills and centres within the space the screen's flex flow leaves
/// between the title and whatever follows.
///
/// Returns the invisible storage/event textarea (the display row's child 0) that the input
/// widgets operate on.
fn add_masked_display(screen: &LvObj, preset: &str) -> Rc<LvTextarea> {
    let display = LvObj::with_parent(screen).unwrap();
    display.set_size(380, 72);
    // Fill the whole area between the title and the (bottom-anchored) keyboard, so the centred
    // circle row sits in the middle of it; a spacer below reserves the keyboard/nav region,
    // which the flex flow cannot see (those widgets are floating).
    display.set_style_flex_grow(1, 0);
    display.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    display.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW);
    display.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    display.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    // Centre the flex track too: its height is that of the tallest visible child, so without
    // this the circles shift vertically whenever the last-character label appears or hides.
    display.set_style_flex_track_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    display.set_style_pad_top(0, 0);
    display.set_style_pad_bottom(0, 0);
    display.set_style_pad_left(0, 0);
    display.set_style_pad_right(0, 0);
    display.set_style_pad_column(MASK_DOT_GAP, 0);
    display.set_style_border_width(0, 0);
    display.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    display.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_SCROLLABLE);
    display.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);

    let textarea = add_textarea(&display, preset, false);
    textarea.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
    // BitBox02 parity: its entry buffer caps input at 149 characters (INPUT_STRING_MAX_SIZE).
    // A longer passphrase entered here could never be retyped on a BitBox02, so apply the same
    // limit.
    textarea.set_max_length(149);
    let textarea = Rc::new(textarea);

    // The circle pool plus the plaintext label for the last entered character. The circle count
    // saturates at what fits the row: the dots are identical, so a saturated display is
    // indistinguishable from a scrolled one.
    let mut dots = Vec::with_capacity(MASK_DOT_COUNT_MAX);
    for _ in 0..MASK_DOT_COUNT_MAX {
        let dot = LvObj::with_parent(&display).unwrap();
        dot.set_size(MASK_DOT_SIZE, MASK_DOT_SIZE);
        dot.set_style_radius(lvgl::ffi::LV_RADIUS_CIRCLE as i32, 0);
        dot.set_style_border_width(0, 0);
        dot.set_style_bg_color(lvgl::color::white(), 0);
        dot.set_style_bg_opa(LvOpacityLevel::LV_OPA_COVER as u8, 0);
        dot.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
        // The theme's default padding makes the empty object "overflow", drawing scrollbar
        // stubs into the circle.
        unsafe {
            lvgl::ffi::lv_obj_set_scrollbar_mode(
                dot.as_ptr(),
                lvgl::ffi::lv_scrollbar_mode_t::LV_SCROLLBAR_MODE_OFF,
            );
        }
        dot.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
        dots.push(dot);
    }
    let last_char_label = LvLabel::new(&display).unwrap();
    last_char_label.set_style_text_color(lvgl::color::white(), 0);
    last_char_label.set_style_text_font(
        lvgl::fonts::INTER_REGULAR_48,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
    // Fixed box height with the same (even) parity as the circles: the flex track is as tall as
    // its tallest visible child, and centring an odd-height track floors differently from an
    // even one — the font's natural 59px line height would nudge the circles by 1px whenever
    // the label appears or hides.
    last_char_label.set_height(60);
    last_char_label.set_text("").unwrap();
    last_char_label.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);

    let display_textarea = Rc::clone(&textarea);
    // Starts at MAX so the initial refresh never reveals a preset's last character.
    let prev_len = core::cell::Cell::new(usize::MAX);
    let refresh_display = Rc::new(move || {
        let (len, last) = textarea_len_and_last(display_textarea.as_ref());
        // Only a just-entered character is readable; any other change (deletion) re-masks.
        let reveal = len > 0 && prev_len.get() < len;
        prev_len.set(len);
        let shown = core::cmp::min(if reveal { len - 1 } else { len }, MASK_DOT_COUNT_MAX);
        for (i, dot) in dots.iter().enumerate() {
            if i < shown {
                dot.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
            } else {
                dot.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
            }
        }
        match last.filter(|_| reveal) {
            Some(ch) => {
                let text = [ch];
                last_char_label
                    .set_text(core::str::from_utf8(&text).expect("entered text is ASCII"))
                    .expect("entered text contains no NUL");
                last_char_label.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
            }
            None => last_char_label.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN),
        }
    });
    refresh_display();
    let refresh_display_cb = Rc::clone(&refresh_display);
    textarea
        .add_event_cb(lvgl::LvEventCode::LV_EVENT_VALUE_CHANGED, move || {
            refresh_display_cb()
        })
        .expect("failed to register entry display callback");

    textarea
}

pub fn build_enter_string_screen(
    params: &EnterStringParams<'_>,
    can_cancel: CanCancel,
    preset: &str,
    responder: Responder<Result<zeroize::Zeroizing<String>, UserAbort>>,
) -> LvObj {
    // Recovery-word entry goes through `build_wordlist_screen` (via the HAL's
    // `enter_wordlist_word`), whose result distinguishes going back from cancelling.
    debug_assert!(
        params.wordlist.is_none(),
        "wordlist entry must use build_wordlist_screen"
    );
    if params.pin {
        return build_pin_screen(params, can_cancel, preset, responder);
    }
    if params.passphrase && !params.number_input {
        return build_passphrase_screen(params, can_cancel, preset, responder);
    }

    let screen = LvObj::new().unwrap();
    screen.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    screen.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_COLUMN);
    // All content fits (the keyboard shrinks via flex_grow); scrolling must stay off so a
    // vertical wobble during the slide-to-confirm drag cannot turn into a scroll-steal.
    screen.remove_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_SCROLLABLE);
    screen.set_style_bg_color(lvgl::color::black(), 0);
    screen.set_style_text_color(lvgl::color::white(), 0);
    screen.set_style_pad_top(40, 0);
    screen.set_style_pad_right(50, 0);
    screen.set_style_pad_bottom(32, 0);
    screen.set_style_pad_left(50, 0);
    screen.set_style_pad_row(20, 0);

    add_title(&screen, params.title);

    let textarea = add_textarea(&screen, preset, params.hide);
    if params.number_input {
        textarea.set_accepted_chars(Some(c"0123456789"));
    }

    let textarea = Rc::new(textarea);

    let keyboard = LvKeyboard::new(&screen).unwrap();
    keyboard.set_width(380);
    keyboard.set_height(260);
    keyboard.set_style_flex_grow(1, 0);
    // No extra margin: LVGL's flex sizing does not subtract margins when distributing
    // flex_grow space, so a margin here overflows the screen's bottom padding.
    keyboard.set_popovers(false);
    let show_keyboard_switch = !params.number_input;
    let initial_keyboard_mode = if params.number_input {
        None
    } else if params.default_to_digits {
        Some(KeyboardMode::Digits)
    } else {
        Some(KeyboardMode::LowerCase)
    };
    if let Some(mode) = initial_keyboard_mode {
        configure_keyboard_maps(&keyboard);
        keyboard.set_mode(mode.lvgl_mode());
    } else {
        keyboard.set_mode(lvgl::LvKeyboardMode::LV_KEYBOARD_MODE_NUMBER);
    }
    // Safe because the textarea and keyboard are siblings on the same screen and remain alive
    // until the whole screen is popped.
    unsafe { keyboard.set_textarea(Some(textarea.as_ref())) };
    // Discard the key selection once an interaction ends (after the class handler has processed
    // a legitimate click): LVGL keeps the lastly clicked key selected forever, and a press
    // sliding in from the Delete/switch buttons below reaches the keyboard without a PRESSED
    // event (which is what re-derives the selection) — a stale selection would retype that key
    // via the long-press repeat path.
    for code in [
        lvgl::LvEventCode::LV_EVENT_RELEASED,
        lvgl::LvEventCode::LV_EVENT_PRESS_LOST,
    ] {
        // A second handle to the keyboard, for the `'static` callback (the keyboard is screen
        // child 2, after the title and the textarea).
        let keyboard_cb = screen
            .child(2)
            .expect("keyboard")
            .try_downcast::<class::KeyboardTag>()
            .expect("screen child 2 is the keyboard");
        keyboard
            .add_event_cb(code, move || {
                // Fully qualified: importing `ButtonmatrixExt` would make the keyboard's
                // `set_map` calls above ambiguous with `KeyboardExt::set_map`.
                lvgl::ButtonmatrixExt::set_selected_button(
                    &keyboard_cb,
                    lvgl::ffi::LV_BUTTONMATRIX_BUTTON_NONE,
                );
            })
            .expect("failed to register keyboard release callback");
    }

    let input_controls = LvObj::with_parent(&screen).unwrap();
    input_controls.set_width(380);
    input_controls.set_height(56);
    input_controls.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    input_controls.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW);
    input_controls.set_style_pad_top(0, 0);
    input_controls.set_style_pad_bottom(0, 0);
    input_controls.set_style_pad_left(0, 0);
    input_controls.set_style_pad_right(0, 0);
    input_controls.set_style_pad_column(12, 0);
    input_controls.set_style_border_width(0, 0);
    input_controls.set_style_bg_opa(
        LvOpacityLevel::LV_OPA_TRANSP as u8,
        LvPart::LV_PART_MAIN as u32,
    );

    if show_keyboard_switch {
        let mut keyboard_mode = initial_keyboard_mode.expect("keyboard switch requires mode");
        let allow_special_chars = params.special_chars;
        let switch_button = LvButton::new(&input_controls).unwrap();
        switch_button.set_size(184, 56);
        switch_button.set_style_bg_color(lvgl::color::hex(0x30333a), 0);
        switch_button.set_style_bg_opa(LvOpacityLevel::LV_OPA_COVER as u8, 0);
        switch_button.set_style_border_width(2, 0);
        switch_button.set_style_border_color(lvgl::color::white(), 0);
        let switch_button_label = LvLabel::new(&switch_button).unwrap();
        switch_button_label
            .set_text(keyboard_mode.switch_label(allow_special_chars))
            .unwrap();
        switch_button_label.set_style_text_font(
            lvgl::fonts::INTER_BOLD_32,
            lvgl::LvState::LV_STATE_DEFAULT as u32,
        );
        switch_button_label.set_style_text_color(lvgl::color::white(), 0);
        switch_button_label.align(LvAlign::LV_ALIGN_CENTER, 0, 0);
        switch_button
            .add_click_cb(move || {
                keyboard_mode = keyboard_mode.next(allow_special_chars);
                keyboard.set_mode(keyboard_mode.lvgl_mode());
                switch_button_label
                    .set_text(keyboard_mode.switch_label(allow_special_chars))
                    .expect("valid switch label");
            })
            .expect("failed to register keyboard switch callback");
    }

    let delete_textarea = Rc::clone(&textarea);
    add_button(
        &input_controls,
        if show_keyboard_switch { 184 } else { 380 },
        56,
        "Delete",
        move || delete_textarea.delete_char(),
    );

    let cancel_present = matches!(can_cancel, CanCancel::Yes);

    if params.longtouch {
        // High-stakes confirmation: accept is the slide gesture instead of a tap, and cancel
        // moves to the corner close button (the slide track needs the full content width).
        if cancel_present {
            let reject_responder = responder.clone();
            let close = build_close_button(&screen);
            close
                .add_click_cb(move || reject_responder.resolve(Err(UserAbort)))
                .expect("failed to register cancel callback");
        }
        let slide = build_slide_to_confirm(&screen, move || {
            responder.resolve(Ok(snapshot_text(textarea.as_ref())));
        });
        slide.set_style_margin_top(8, 0);
        return screen;
    }

    let actions = add_actions_row(&screen);
    actions.set_style_pad_column(20, 0);
    actions.set_style_margin_top(8, 0);

    if cancel_present {
        // Cancel / Back is always a tap action -> icon button.
        let icon = if params.cancel_is_backbutton {
            NavIcon::Back
        } else {
            NavIcon::Cancel
        };
        let reject_responder = responder.clone();
        let cancel = build_nav_button(&actions, icon);
        cancel
            .add_click_cb(move || {
                reject_responder.resolve(Err(UserAbort));
            })
            .expect("failed to register cancel callback");
    }

    // Plain tap confirm -> icon button.
    let accept = build_nav_button(&actions, NavIcon::Confirm);
    accept
        .add_click_cb(move || {
            responder.resolve(Ok(snapshot_text(textarea.as_ref())));
        })
        .expect("failed to register confirm callback");

    screen
}

#[cfg(test)]
mod tests {
    extern crate std;

    use core::pin::Pin;
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    use bitbox_lvgl::{class, ffi};
    use util::futures::completion;

    use super::super::keyboard;
    use super::super::test_util::{ScriptedTouch, coords, lock_and_init, pump_for};
    use super::*;

    fn passphrase_params() -> EnterStringParams<'static> {
        EnterStringParams {
            title: "Optional passphrase",
            hide: true,
            special_chars: true,
            // The passphrase screen deliberately uses a tap confirm despite `longtouch`.
            longtouch: true,
            passphrase: true,
            ..Default::default()
        }
    }

    /// The device password params as `password::enter` builds them (the PIN screen renders the
    /// title with "password" replaced by "PIN").
    fn pin_params() -> EnterStringParams<'static> {
        EnterStringParams {
            title: "Enter password",
            hide: true,
            longtouch: true,
            pin: true,
            ..Default::default()
        }
    }

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

    /// The wordlist screen resolves with a richer abort than the other entry screens.
    type WordlistResult = Result<zeroize::Zeroizing<String>, WordlistEntryAbort>;

    struct Harness<R = Result<zeroize::Zeroizing<String>, UserAbort>> {
        touch: ScriptedTouch,
        screen: LvObj,
        result: completion::Result<R>,
    }

    impl Harness {
        fn with_params(
            params: &EnterStringParams<'_>,
            can_cancel: CanCancel,
            preset: &str,
        ) -> Self {
            let touch = ScriptedTouch::new();
            let (responder, result) = completion::completion();
            let screen = build_enter_string_screen(params, can_cancel, preset, responder);
            unsafe { ffi::lv_screen_load(screen.as_ptr()) };
            pump_for(60); // layout + first render
            Self {
                touch,
                screen,
                result,
            }
        }

        fn new(can_cancel: CanCancel) -> Self {
            Self::with_params(&passphrase_params(), can_cancel, "")
        }

        fn new_pin(can_cancel: CanCancel) -> Self {
            Self::with_params(&pin_params(), can_cancel, "")
        }

        fn new_wordlist(
            wordlist: &[u16],
            can_cancel: CanCancel,
            preset: &str,
        ) -> Harness<WordlistResult> {
            let params = EnterStringParams {
                title: "1 of 24",
                wordlist: Some(wordlist),
                ..Default::default()
            };
            let touch = ScriptedTouch::new();
            let (responder, result) = completion::completion();
            let screen = build_wordlist_screen(&params, can_cancel, preset, responder);
            unsafe { ffi::lv_screen_load(screen.as_ptr()) };
            pump_for(60); // layout + first render
            Harness {
                touch,
                screen,
                result,
            }
        }
    }

    impl<R> Harness<R> {
        /// The PIN keypad container (screen child 2 on the PIN screen).
        fn keypad(&self) -> LvObj {
            self.screen.child(2).expect("keypad container")
        }

        /// Taps the PIN keypad key at grid position (`row`, `col`).
        fn tap_pin_key(&mut self, row: usize, col: usize) {
            let area = coords(&self.keypad());
            let (x, y) = super::super::keypad::key_center(row, col);
            self.touch.tap(area.x1 + x, area.y1 + y);
        }

        /// The title label's current text.
        fn title_text(&self) -> String {
            let title = self
                .screen
                .child(0)
                .expect("title")
                .try_downcast::<class::LabelTag>()
                .expect("child 0 is the title label");
            String::from(title.get_text().unwrap().to_str().unwrap())
        }

        /// The entry display row (screen child 1): hidden textarea, `MASK_DOT_COUNT_MAX`
        /// circles, last-character label.
        fn entry_display(&self) -> LvObj {
            self.screen.child(1).expect("entry display")
        }

        fn textarea(&self) -> LvTextarea {
            self.entry_display()
                .child(0)
                .expect("textarea")
                .try_downcast::<class::TextareaTag>()
                .expect("display child 0 is the textarea")
        }

        /// The number of masking circles currently shown.
        fn shown_dots(&self) -> usize {
            let display = self.entry_display();
            (0..MASK_DOT_COUNT_MAX)
                .filter(|i| {
                    let dot = display.child(1 + *i as i32).expect("masking dot");
                    !Harness::hidden(&dot)
                })
                .count()
        }

        /// The last-character label's text, or `None` while it is hidden.
        fn revealed_char(&self) -> Option<String> {
            let label = self
                .entry_display()
                .child(1 + MASK_DOT_COUNT_MAX as i32)
                .expect("last-character label");
            if Harness::hidden(&label) {
                return None;
            }
            let label = label
                .try_downcast::<class::LabelTag>()
                .expect("last child is the label");
            Some(String::from(label.get_text().unwrap().to_str().unwrap()))
        }

        fn text(&self) -> zeroize::Zeroizing<String> {
            snapshot_text(&self.textarea())
        }

        fn keyboard(&self) -> LvObj {
            self.screen.child(2).expect("keyboard container")
        }

        /// Absolute screen coordinates for a point given in keyboard-container coordinates.
        fn on_keyboard(&self, (x, y): (i32, i32)) -> (i32, i32) {
            let area = coords(&self.keyboard());
            (area.x1 + x, area.y1 + y)
        }

        fn tap_char_key(&mut self, symbols: bool, caps: bool, row: usize, col: usize) {
            let (x, y) = self.on_keyboard(keyboard::char_key_center(symbols, caps, row, col));
            self.touch.tap(x, y);
        }

        fn tap_capslock(&mut self) {
            let (x, y) = self.on_keyboard(keyboard::capslock_center());
            self.touch.tap(x, y);
        }

        fn tap_symbols(&mut self) {
            let (x, y) = self.on_keyboard(keyboard::symbols_center());
            self.touch.tap(x, y);
        }

        fn tap_space(&mut self) {
            let (x, y) = self.on_keyboard(keyboard::space_center());
            self.touch.tap(x, y);
        }

        fn actions(&self) -> LvObj {
            self.screen.child(3).expect("actions row")
        }

        fn backspace(&self) -> LvObj {
            self.actions().child(0).expect("backspace button")
        }

        fn confirm(&self) -> LvObj {
            self.actions().child(1).expect("confirm button")
        }

        fn tap_button(&mut self, button: &LvObj) {
            let area = coords(button);
            self.touch
                .tap((area.x1 + area.x2) / 2, (area.y1 + area.y2) / 2);
        }

        fn preview(&self) -> LvObj {
            self.keyboard()
                .child(keyboard::CHILD_INDEX_PREVIEW)
                .expect("preview balloon")
        }

        /// The preview balloon of the (letters-only) wordlist keyboard.
        fn wordlist_preview(&self) -> LvObj {
            self.keyboard()
                .child(keyboard::WORDLIST_CHILD_INDEX_PREVIEW)
                .expect("preview balloon")
        }

        /// The wordlist-keyboard key-row buttonmatrix `row`.
        fn wordlist_row(&self, row: usize) -> bitbox_lvgl::LvButtonmatrix {
            self.keyboard()
                .child(row as i32)
                .expect("key row")
                .try_downcast::<class::ButtonmatrixTag>()
                .expect("key row is a buttonmatrix")
        }

        /// Taps the wordlist-keyboard key for `letter`.
        fn tap_wordlist_letter(&mut self, letter: u8) {
            let (row, col) = keyboard::wordlist_letter_pos(letter);
            let (x, y) = self.on_keyboard(keyboard::wordlist_key_center(row, col));
            self.touch.tap(x, y);
        }

        /// Whether the wordlist-keyboard key for `letter` is disabled.
        fn wordlist_letter_disabled(&self, letter: u8) -> bool {
            use bitbox_lvgl::ButtonmatrixExt;
            let (row, col) = keyboard::wordlist_letter_pos(letter);
            self.wordlist_row(row).has_button_ctrl(
                col as u32,
                bitbox_lvgl::LvButtonmatrixCtrl::LV_BUTTONMATRIX_CTRL_DISABLED,
            )
        }

        /// Asserts that exactly the letters in `expected` are enabled on the wordlist keyboard.
        #[track_caller]
        fn assert_enabled_letters(&self, expected: &[u8]) {
            for letter in b'a'..=b'z' {
                assert_eq!(
                    !self.wordlist_letter_disabled(letter),
                    expected.contains(&letter),
                    "letter '{}' enabled state",
                    letter as char
                );
            }
        }
    }

    // Widget-state helpers: associated functions of the non-generic harness so call sites can
    // stay `Harness::...` without turbofishing the (irrelevant) result type.
    impl Harness {
        fn hidden(obj: &LvObj) -> bool {
            unsafe { ffi::lv_obj_has_flag(obj.as_ptr(), lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN) }
        }

        fn disabled(obj: &LvObj) -> bool {
            unsafe { ffi::lv_obj_has_state(obj.as_ptr(), lvgl::LvState::LV_STATE_DISABLED) }
        }

        fn pressed(obj: &LvObj) -> bool {
            unsafe { ffi::lv_obj_has_state(obj.as_ptr(), lvgl::LvState::LV_STATE_PRESSED) }
        }

        /// The resolved whole-widget recolor opacity (`LV_STYLE_RECOLOR_OPA`) in the object's
        /// current state.
        fn recolor_opa(obj: &LvObj) -> u8 {
            let value = unsafe {
                ffi::lv_obj_get_style_prop(
                    obj.as_ptr(),
                    LvPart::LV_PART_MAIN,
                    ffi::_lv_style_id_t::LV_STYLE_RECOLOR_OPA as ffi::lv_style_prop_t,
                )
            };
            unsafe { value.num as u8 }
        }
    }

    impl<R> Drop for Harness<R> {
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

    #[test]
    fn test_types_lowercase_and_digits() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        harness.tap_char_key(false, false, 1, 0); // q
        harness.tap_char_key(false, false, 2, 0); // a
        harness.tap_char_key(false, false, 3, 6); // m
        harness.tap_char_key(false, false, 0, 9); // 0
        harness.tap_space();

        assert_eq!(harness.text().as_str(), "qam0 ");
    }

    #[test]
    fn test_capslock_toggles_case() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        harness.tap_capslock();
        harness.tap_char_key(false, true, 1, 0); // Q
        harness.tap_char_key(false, true, 0, 0); // digits are unaffected by caps
        harness.tap_capslock();
        harness.tap_char_key(false, false, 1, 0); // q

        assert_eq!(harness.text().as_str(), "Q1q");
    }

    #[test]
    fn test_symbols_layout() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        harness.tap_symbols();
        harness.tap_char_key(true, false, 1, 0); // !
        harness.tap_char_key(true, false, 2, 1); // ,
        harness.tap_char_key(true, false, 3, 9); // }
        harness.tap_char_key(true, false, 0, 0); // the digit row stays on the symbols layout
        // Caps lock is inert on the symbols layout.
        assert!(Harness::disabled(
            &harness
                .keyboard()
                .child(keyboard::CHILD_INDEX_CAPSLOCK)
                .unwrap()
        ));
        harness.tap_capslock();
        harness.tap_char_key(true, false, 1, 1); // still ", not W
        harness.tap_symbols();
        harness.tap_char_key(false, false, 1, 1); // w: back to (lowercase) letters

        assert_eq!(harness.text().as_str(), "!,}1\"w");
    }

    #[test]
    fn test_backspace_deletes_and_disables_when_empty() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        assert!(Harness::disabled(&harness.backspace()));
        harness.tap_char_key(false, false, 1, 0); // q
        harness.tap_char_key(false, false, 1, 1); // w
        assert!(!Harness::disabled(&harness.backspace()));

        let backspace = harness.backspace();
        harness.tap_button(&backspace);
        assert_eq!(harness.text().as_str(), "q");
        harness.tap_button(&backspace);
        assert_eq!(harness.text().as_str(), "");
        assert!(Harness::disabled(&harness.backspace()));

        // Tapping the disabled button is inert.
        harness.tap_button(&backspace);
        assert_eq!(harness.text().as_str(), "");
    }

    /// Enabling/disabling a button must swap its whole look in a single style update. The
    /// default theme dims disabled widgets with a 50% grey whole-widget recolor and animates
    /// `RECOLOR`/`RECOLOR_OPA` on state changes with a delay — left in place, that overlay lands
    /// ~150ms after the gray border/icon colors snap in, so the button visibly flickers
    /// (regression test for the `style_outline_button` disabled-state override).
    #[test]
    fn test_disable_enable_is_not_animated() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        // Canary for the assumption the zero-assertions below rest on: the default theme dims a
        // disabled widget with a (delayed) 50% whole-widget recolor. An unstyled button must
        // show that overlay once the transition has settled — if an LVGL bump changes the
        // mechanism, fail loudly here instead of letting the assertions below pass vacuously.
        let canary = LvButton::new(&harness.screen).unwrap().to_obj();
        canary.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_HIDDEN);
        canary.add_state(lvgl::LvState::LV_STATE_DISABLED);
        pump_for(300);
        assert_eq!(
            Harness::recolor_opa(&canary),
            LvOpacityLevel::LV_OPA_50 as u8
        );

        // Backspace: enabled by typing, disabled again by deleting the only character. The tap
        // pumps past the theme transition's 70ms delay, so with the transition in effect the
        // overlay would already be fading in here; after a further 300ms it would be fully on.
        let backspace = harness.backspace();
        harness.tap_char_key(false, false, 1, 0); // q
        harness.tap_button(&backspace);
        assert!(Harness::disabled(&backspace));
        assert_eq!(Harness::recolor_opa(&backspace), 0);
        pump_for(300);
        assert_eq!(Harness::recolor_opa(&backspace), 0);

        // Caps lock: disabled by switching to the symbols layout.
        let capslock = harness
            .keyboard()
            .child(keyboard::CHILD_INDEX_CAPSLOCK)
            .expect("caps lock");
        harness.tap_symbols();
        assert!(Harness::disabled(&capslock));
        assert_eq!(Harness::recolor_opa(&capslock), 0);
        pump_for(300);
        assert_eq!(Harness::recolor_opa(&capslock), 0);

        // Re-enabling must not fade the overlay back out either.
        harness.tap_symbols();
        assert!(!Harness::disabled(&capslock));
        assert_eq!(Harness::recolor_opa(&capslock), 0);
        pump_for(300);
        assert_eq!(Harness::recolor_opa(&capslock), 0);
    }

    #[test]
    fn test_confirm_resolves_with_text() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        harness.tap_char_key(false, false, 1, 0); // q
        harness.tap_capslock();
        harness.tap_char_key(false, true, 1, 1); // W
        assert!(poll_once(&mut harness.result).is_none());

        let confirm = harness.confirm();
        harness.tap_button(&confirm);
        let result = poll_once(&mut harness.result).expect("confirm resolves");
        let Ok(text) = result else {
            panic!("unexpected abort")
        };
        assert_eq!(text.as_str(), "qW");
    }

    #[test]
    fn test_confirm_empty_passphrase_allowed() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        let confirm = harness.confirm();
        harness.tap_button(&confirm);
        let result = poll_once(&mut harness.result).expect("confirm resolves");
        let Ok(text) = result else {
            panic!("unexpected abort")
        };
        assert_eq!(text.as_str(), "");
    }

    #[test]
    fn test_press_preview_jets_out() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        let preview = harness.preview();
        assert!(Harness::hidden(&preview));

        // Press and hold 'w' (row 1, col 1): the preview balloon pops up over the key.
        let (x, y) = harness.on_keyboard(keyboard::char_key_center(false, false, 1, 1));
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, true);
        pump_for(120);
        assert!(!Harness::hidden(&preview));
        // The balloon straddles the pressed key: horizontally centred on it, its head reaching
        // into the row above and its stem covering the key.
        let (key_center_x, key_center_y) = (x, y); // the tap targeted the key centre
        let balloon = coords(&preview);
        // `x2` is inclusive (x1 + width - 1), so round the centre up.
        assert_eq!(
            (balloon.x1 + balloon.x2 + 1) / 2,
            key_center_x,
            "balloon not centred on the key"
        );
        let key_top = key_center_y - 30;
        let key_bottom = key_center_y + 30;
        assert!(balloon.y1 < key_top - 60, "balloon head not above the key");
        assert!(
            balloon.y2 >= key_bottom,
            "balloon stem does not cover the key"
        );
        // The preview shows the pressed character.
        let label = preview
            .child(1)
            .expect("preview label")
            .try_downcast::<class::LabelTag>()
            .expect("preview label class");
        assert_eq!(label.get_text().unwrap().to_str().unwrap(), "w");

        harness.touch.push(x, y, false);
        pump_for(120);
        assert!(Harness::hidden(&preview));
        assert_eq!(harness.text().as_str(), "w");
    }

    #[test]
    fn test_masking_shows_last_character() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        // Every character but the most recently entered one is masked by a circle; the last one
        // stays readable until the next keystroke.
        assert_eq!(harness.shown_dots(), 0);
        assert_eq!(harness.revealed_char(), None);
        harness.tap_char_key(false, false, 1, 0); // q
        assert_eq!(harness.shown_dots(), 0);
        assert_eq!(harness.revealed_char().as_deref(), Some("q"));
        harness.tap_char_key(false, false, 1, 1); // w
        assert_eq!(harness.shown_dots(), 1);
        assert_eq!(harness.revealed_char().as_deref(), Some("w"));
        let first_dot = harness.entry_display().child(1).expect("first dot");
        let dot_before = coords(&first_dot);
        harness.tap_char_key(false, false, 0, 0); // 1
        assert_eq!(harness.shown_dots(), 2);
        assert_eq!(harness.revealed_char().as_deref(), Some("1"));
        assert_eq!(harness.text().as_str(), "qw1");

        // Deleting re-masks everything (the deleted character was the last one entered).
        let backspace = harness.backspace();
        harness.tap_button(&backspace);
        assert_eq!(harness.shown_dots(), 2);
        assert_eq!(harness.revealed_char(), None);
        assert_eq!(harness.text().as_str(), "qw");

        // The circles must not shift vertically when the last-character label hides (the flex
        // track shrinks to the tallest visible child; the track must stay centred).
        let dot_after = coords(&first_dot);
        assert_eq!(
            (dot_before.y1, dot_before.y2),
            (dot_after.y1, dot_after.y2),
            "masking circles moved vertically"
        );
    }

    #[test]
    fn test_holding_a_key_types_once() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        // Hold 'w' well past LVGL's long-press threshold (400ms) and repeat period (100ms): the
        // NO_REPEAT ctrl bit must keep the buttonmatrix from auto-repeating into the masked
        // input; exactly one character is inserted, on release.
        let (x, y) = harness.on_keyboard(keyboard::char_key_center(false, false, 1, 1));
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, true);
        pump_for(700);
        assert_eq!(harness.text().as_str(), "");
        harness.touch.push(x, y, false);
        pump_for(120);
        assert_eq!(harness.text().as_str(), "w");
    }

    #[test]
    fn test_close_button_rejects_when_cancellable() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::Yes);

        // With CanCancel::Yes the screen carries a corner close button (screen child after the
        // actions row and the keyboard-region spacer) that rejects with UserAbort.
        let close = harness.screen.child(5).expect("corner close button");
        harness.tap_button(&close);
        let result = poll_once(&mut harness.result).expect("close resolves");
        assert!(result.is_err(), "close button must reject");
    }

    #[test]
    fn test_preview_overhang_not_clipped() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        // Press '5' on the digit row: the balloon head reaches above the keyboard container.
        let (x, y) = harness.on_keyboard(keyboard::char_key_center(false, false, 0, 4));
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, true);
        pump_for(120);
        let preview = harness.preview();
        assert!(!Harness::hidden(&preview));
        let container = coords(&harness.keyboard());
        let balloon = coords(&preview);
        let overhang = container.y1 - balloon.y1;
        assert!(
            overhang > 0,
            "digit-row preview must overhang the container"
        );
        // OVERFLOW_VISIBLE only widens the children clip rect by the container's ext draw size,
        // so the overhang must be declared there or the balloon head is clipped away. Query it
        // the way LVGL does: fire REFR_EXT_DRAW_SIZE with an i32 param the handlers max() into.
        let mut ext_draw_size: i32 = 0;
        unsafe {
            ffi::lv_obj_send_event(
                harness.keyboard().as_ptr(),
                lvgl::LvEventCode::LV_EVENT_REFR_EXT_DRAW_SIZE,
                (&mut ext_draw_size as *mut i32).cast(),
            );
        }
        assert!(
            ext_draw_size >= overhang,
            "container ext draw size {ext_draw_size} does not cover the preview overhang {overhang}"
        );
    }

    #[test]
    fn test_slide_off_key_cancels() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        // Press 'w', slide off it (onto 'e'), release: the buttonmatrix discards its selection
        // when the pointer leaves the pressed key, so the preview hides and nothing is typed.
        let preview = harness.preview();
        let (x, y) = harness.on_keyboard(keyboard::char_key_center(false, false, 1, 1));
        let (x_next, _) = harness.on_keyboard(keyboard::char_key_center(false, false, 1, 2));
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, true);
        pump_for(120);
        assert!(!Harness::hidden(&preview));

        harness.touch.push(x_next, y, true);
        harness.touch.push(x_next, y, true);
        pump_for(120);
        assert!(Harness::hidden(&preview));

        harness.touch.push(x_next, y, false);
        pump_for(120);
        assert_eq!(harness.text().as_str(), "");
    }

    #[test]
    fn test_slide_from_space_onto_previous_key_does_not_type() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        // Type 'x' with a normal tap; the buttonmatrix must not keep it armed afterwards.
        harness.tap_char_key(false, false, 3, 1);
        assert_eq!(harness.text().as_str(), "x");

        // Press the space bar, slide along it until under 'x', then up into the key row
        // directly over 'x', and release there. The press migrates onto the row without a
        // PRESSED event, so a stale selection from the earlier tap must not fire: neither
        // space nor 'x' may be typed.
        let (space_x, space_y) = harness.on_keyboard(keyboard::space_center());
        let (key_x, key_y) = harness.on_keyboard(keyboard::char_key_center(false, false, 3, 1));
        harness.touch.push(space_x, space_y, true);
        harness.touch.push(space_x, space_y, true);
        harness.touch.push(key_x, space_y, true); // still on the space bar, under 'x'
        harness.touch.push(key_x, key_y, true); // entered the key row directly over 'x'
        harness.touch.push(key_x, key_y, true);
        harness.touch.push(key_x, key_y, false);
        pump_for(280);
        assert_eq!(harness.text().as_str(), "x");
    }

    #[test]
    fn test_preview_tracks_modes() {
        let _lock = lock_and_init();
        let mut harness = Harness::new(CanCancel::No);

        harness.tap_capslock();
        let (x, y) = harness.on_keyboard(keyboard::char_key_center(false, true, 1, 1));
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, true);
        pump_for(120);
        let label = harness
            .preview()
            .child(1)
            .expect("preview label")
            .try_downcast::<class::LabelTag>()
            .expect("preview label class");
        assert_eq!(label.get_text().unwrap().to_str().unwrap(), "W");
        harness.touch.push(x, y, false);
        pump_for(120);

        harness.tap_capslock();
        harness.tap_symbols();
        let (x, y) = harness.on_keyboard(keyboard::char_key_center(true, false, 3, 0));
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, true);
        pump_for(120);
        assert_eq!(label.get_text().unwrap().to_str().unwrap(), "?");
    }

    #[test]
    fn test_pin_types_digits_and_confirms() {
        let _lock = lock_and_init();
        let mut harness = Harness::new_pin(CanCancel::No);

        // Workflow titles say "password"; the PIN screen renders them with "PIN".
        assert_eq!(harness.title_text(), "Enter PIN");

        harness.tap_pin_key(0, 0); // 1
        harness.tap_pin_key(1, 1); // 5
        harness.tap_pin_key(2, 2); // 9
        harness.tap_pin_key(3, 1); // 0
        assert_eq!(harness.text().as_str(), "1590");
        // The masked display shows circles plus the last entered digit.
        assert_eq!(harness.shown_dots(), 3);
        assert_eq!(harness.revealed_char().as_deref(), Some("0"));

        assert!(poll_once(&mut harness.result).is_none());
        harness.tap_pin_key(3, 2); // confirm
        let result = poll_once(&mut harness.result).expect("confirm resolves");
        let Ok(text) = result else {
            panic!("unexpected abort")
        };
        assert_eq!(text.as_str(), "1590");
    }

    #[test]
    fn test_pin_backspace_deletes_and_disables_when_empty() {
        let _lock = lock_and_init();
        let mut harness = Harness::new_pin(CanCancel::No);

        let backspace = harness.keypad().child(9).expect("backspace key");
        assert!(Harness::disabled(&backspace));

        harness.tap_pin_key(0, 1); // 2
        harness.tap_pin_key(0, 2); // 3
        assert!(!Harness::disabled(&backspace));

        harness.tap_pin_key(3, 0); // backspace
        assert_eq!(harness.text().as_str(), "2");
        harness.tap_pin_key(3, 0);
        assert_eq!(harness.text().as_str(), "");
        assert!(Harness::disabled(&backspace));

        // Tapping the disabled key is inert.
        harness.tap_pin_key(3, 0);
        assert_eq!(harness.text().as_str(), "");
    }

    #[test]
    fn test_pin_key_slide_off_does_not_type() {
        let _lock = lock_and_init();
        let mut harness = Harness::new_pin(CanCancel::No);

        // Press '5', slide off the key into the grid gap, release: leaving the key clears its
        // pressed state (LV_EVENT_PRESS_LOST) and the release must not type.
        let area = coords(&harness.keypad());
        let (x, y) = super::super::keypad::key_center(1, 1);
        let (x, y) = (area.x1 + x, area.y1 + y);
        let key = harness.keypad().child(4).expect("digit key 5");
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, true);
        pump_for(120);
        assert!(Harness::pressed(&key));

        // Two steps to the right: past the key edge (41px half-width), into the 50px gap.
        harness.touch.push(x + 30, y, true);
        harness.touch.push(x + 60, y, true);
        pump_for(120);
        assert!(!Harness::pressed(&key));

        harness.touch.push(x + 60, y, false);
        pump_for(120);
        assert_eq!(harness.text().as_str(), "");

        // Sliding back onto the key before releasing does not re-arm it either: once the press
        // left the key, that tap is abandoned for good (same as the keyboard's character keys).
        harness.touch.push(x, y, true);
        harness.touch.push(x + 60, y, true);
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, false);
        pump_for(200);
        assert_eq!(harness.text().as_str(), "");

        // A regular tap still types.
        harness.tap_pin_key(1, 1);
        assert_eq!(harness.text().as_str(), "5");
    }

    #[test]
    fn test_pin_confirm_slide_off_does_not_resolve() {
        let _lock = lock_and_init();
        let mut harness = Harness::new_pin(CanCancel::No);

        harness.tap_pin_key(0, 0); // 1

        // Press the confirm key, slide off it, release: the workflow must not resolve.
        let area = coords(&harness.keypad());
        let (x, y) = super::super::keypad::key_center(3, 2);
        let (x, y) = (area.x1 + x, area.y1 + y);
        let key = harness.keypad().child(11).expect("confirm key");
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, true);
        pump_for(120);
        assert!(Harness::pressed(&key));
        harness.touch.push(x + 30, y, true);
        harness.touch.push(x + 60, y, true);
        harness.touch.push(x + 60, y, false);
        pump_for(240);
        assert!(poll_once(&mut harness.result).is_none());

        // A regular tap on confirm still resolves.
        harness.tap_pin_key(3, 2);
        let result = poll_once(&mut harness.result).expect("confirm resolves");
        let Ok(text) = result else {
            panic!("unexpected abort")
        };
        assert_eq!(text.as_str(), "1");
    }

    #[test]
    fn test_pin_close_button_rejects() {
        let _lock = lock_and_init();
        let mut harness = Harness::new_pin(CanCancel::Yes);

        // Children on the PIN screen: title, display, keypad, spacer, corner close button.
        let close = harness.screen.child(4).expect("corner close button");
        let area = coords(&close);
        harness
            .touch
            .tap((area.x1 + area.x2) / 2, (area.y1 + area.y2) / 2);
        let result = poll_once(&mut harness.result).expect("close resolves");
        assert!(result.is_err(), "close must reject with UserAbort");
    }

    /// The index of `word` in the BIP39 English wordlist.
    fn word_idx(word: &str) -> u16 {
        bip39::Language::English
            .word_list()
            .iter()
            .position(|w| *w == word)
            .unwrap_or_else(|| panic!("'{word}' is not a BIP39 word")) as u16
    }

    fn word_indices(words: &[&str]) -> Vec<u16> {
        words.iter().map(|word| word_idx(word)).collect()
    }

    #[test]
    fn test_wordlist_keyboard_layout() {
        use bitbox_lvgl::ButtonmatrixExt;

        let _lock = lock_and_init();
        let wordlist = word_indices(&["abandon"]);
        let harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        // The keyboard carries exactly the three letter-row matrices plus the preview balloon —
        // no digit row and no caps-lock/space/symbols function row.
        let keyboard = harness.keyboard();
        assert!(
            keyboard
                .child(keyboard::WORDLIST_CHILD_INDEX_PREVIEW)
                .is_some()
        );
        assert!(
            keyboard
                .child(keyboard::WORDLIST_CHILD_INDEX_PREVIEW + 1)
                .is_none(),
            "unexpected extra child on the wordlist keyboard"
        );
        let area = coords(&keyboard);
        assert_eq!(area.y2 - area.y1 + 1, keyboard::WORDLIST_KEYBOARD_HEIGHT);

        // Each key's text matches the letter table the enable/disable logic works from.
        for row in 0..3 {
            let matrix = harness.wordlist_row(row);
            let letters = keyboard::wordlist_row_letters(row);
            for (id, letter) in letters.iter().enumerate() {
                let text = matrix.get_button_text(id as u32).expect("key text");
                assert_eq!(text.to_bytes(), &[*letter]);
            }
            assert!(
                matrix.get_button_text(letters.len() as u32).is_none(),
                "row {row} has more keys than letters"
            );
        }
    }

    #[test]
    fn test_wordlist_typing_filters_keys() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["action", "actor", "zoo"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        // Empty entry: only the words' first letters are enabled.
        harness.assert_enabled_letters(b"az");
        harness.tap_wordlist_letter(b'b'); // disabled: types nothing
        assert_eq!(harness.text().as_str(), "");

        harness.tap_wordlist_letter(b'a');
        assert_eq!(harness.text().as_str(), "a");
        harness.assert_enabled_letters(b"c");
        harness.tap_wordlist_letter(b'z'); // was valid before, no longer
        assert_eq!(harness.text().as_str(), "a");

        harness.tap_wordlist_letter(b'c');
        harness.tap_wordlist_letter(b't');
        assert_eq!(harness.text().as_str(), "act");
        harness.assert_enabled_letters(b"io"); // action | actor

        // 'o' leaves "actor" as the only candidate: the word autocompletes, every letter is
        // disabled, confirm is enabled.
        harness.tap_wordlist_letter(b'o');
        assert_eq!(harness.text().as_str(), "actor");
        harness.assert_enabled_letters(b"");
        assert!(!Harness::disabled(&harness.confirm()));

        let confirm = harness.confirm();
        harness.tap_button(&confirm);
        let result = poll_once(&mut harness.result).expect("confirm resolves");
        let Ok(text) = result else {
            panic!("unexpected abort")
        };
        assert_eq!(text.as_str(), "actor");
    }

    #[test]
    fn test_wordlist_confirm_requires_complete_word() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["act", "action"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        // Confirm is gray and inert while the entry is no complete word.
        assert!(Harness::disabled(&harness.confirm()));
        let confirm = harness.confirm();
        harness.tap_button(&confirm);
        assert!(poll_once(&mut harness.result).is_none());

        harness.tap_wordlist_letter(b'a');
        harness.tap_wordlist_letter(b'c');
        assert!(Harness::disabled(&harness.confirm()));
        harness.tap_wordlist_letter(b't');

        // "act" is a word itself AND a prefix of "action": confirm and 'i' are both live.
        assert!(!Harness::disabled(&harness.confirm()));
        harness.assert_enabled_letters(b"i");

        harness.tap_button(&confirm);
        let result = poll_once(&mut harness.result).expect("confirm resolves");
        let Ok(text) = result else {
            panic!("unexpected abort")
        };
        assert_eq!(text.as_str(), "act");
    }

    #[test]
    fn test_wordlist_backspace_updates_filter() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["zoo", "zone", "zebra"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        assert!(Harness::disabled(&harness.backspace()));
        harness.assert_enabled_letters(b"z");
        harness.tap_wordlist_letter(b'z');
        harness.tap_wordlist_letter(b'o');
        harness.assert_enabled_letters(b"no"); // zone | zoo
        harness.tap_wordlist_letter(b'o');
        assert_eq!(harness.text().as_str(), "zoo");
        assert!(!Harness::disabled(&harness.confirm()));

        // Deleting reverts both the letter filter and the confirm gating.
        let backspace = harness.backspace();
        harness.tap_button(&backspace);
        assert_eq!(harness.text().as_str(), "zo");
        harness.assert_enabled_letters(b"no");
        assert!(Harness::disabled(&harness.confirm()));

        harness.tap_button(&backspace);
        harness.tap_button(&backspace);
        assert_eq!(harness.text().as_str(), "");
        assert!(Harness::disabled(&harness.backspace()));
        harness.assert_enabled_letters(b"z");
    }

    #[test]
    fn test_wordlist_back_on_empty_goes_back() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["zoo", "zone"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::Yes, "");

        // With a cancellable screen the Back button is live even on an empty entry.
        assert!(!Harness::disabled(&harness.backspace()));

        // While the entry has letters, Back is backspace: it deletes and does not resolve.
        harness.tap_wordlist_letter(b'z');
        let backspace = harness.backspace();
        harness.tap_button(&backspace);
        assert_eq!(harness.text().as_str(), "");
        assert!(poll_once(&mut harness.result).is_none());

        // On the now-empty entry, Back goes back to the previous word.
        harness.tap_button(&backspace);
        let result = poll_once(&mut harness.result).expect("back resolves");
        assert!(
            matches!(result, Err(WordlistEntryAbort::Back)),
            "back on an empty entry must reject with Back"
        );
    }

    #[test]
    fn test_wordlist_back_on_empty_inert_when_not_cancellable() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["zoo", "zone"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        // Without a cancel path there is nowhere to go back to: the button is gray and a tap
        // neither types nor resolves.
        assert!(Harness::disabled(&harness.backspace()));
        let backspace = harness.backspace();
        harness.tap_button(&backspace);
        assert_eq!(harness.text().as_str(), "");
        assert!(poll_once(&mut harness.result).is_none());
    }

    #[test]
    fn test_wordlist_preset_starts_complete() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["zoo", "zone"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::Yes, "zoo");

        // A preset word (re-editing a previously entered word) starts out confirmable.
        assert_eq!(harness.text().as_str(), "zoo");
        assert!(!Harness::disabled(&harness.confirm()));
        assert!(!Harness::disabled(&harness.backspace()));
        harness.assert_enabled_letters(b"");

        let confirm = harness.confirm();
        harness.tap_button(&confirm);
        let result = poll_once(&mut harness.result).expect("confirm resolves");
        let Ok(text) = result else {
            panic!("unexpected abort")
        };
        assert_eq!(text.as_str(), "zoo");
    }

    #[test]
    fn test_wordlist_disabled_key_no_preview_no_type() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["zoo", "zone"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        // Press and hold the disabled 'q': the preview must not pop up, and releasing must not
        // type (LVGL never selects a disabled matrix button).
        assert!(harness.wordlist_letter_disabled(b'q'));
        let preview = harness.wordlist_preview();
        let (row, col) = keyboard::wordlist_letter_pos(b'q');
        let (x, y) = harness.on_keyboard(keyboard::wordlist_key_center(row, col));
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, true);
        pump_for(120);
        assert!(Harness::hidden(&preview));
        harness.touch.push(x, y, false);
        pump_for(120);
        assert_eq!(harness.text().as_str(), "");

        // The enabled 'z' still previews and types.
        harness.tap_wordlist_letter(b'z');
        assert_eq!(harness.text().as_str(), "z");
    }

    #[test]
    fn test_wordlist_autocompletes_unique_match() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["zebra", "zoo"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        // Two candidates left after 'z': no autocomplete yet.
        harness.tap_wordlist_letter(b'z');
        assert_eq!(harness.text().as_str(), "z");
        assert!(Harness::disabled(&harness.confirm()));

        // 'e' leaves only "zebra": the rest of the word fills in by itself.
        harness.tap_wordlist_letter(b'e');
        assert_eq!(harness.text().as_str(), "zebra");
        harness.assert_enabled_letters(b"");
        assert!(!Harness::disabled(&harness.confirm()));
        assert!(!Harness::disabled(&harness.backspace()));

        let confirm = harness.confirm();
        harness.tap_button(&confirm);
        let result = poll_once(&mut harness.result).expect("confirm resolves");
        let Ok(text) = result else {
            panic!("unexpected abort")
        };
        assert_eq!(text.as_str(), "zebra");
    }

    #[test]
    fn test_wordlist_autocompletes_longest_word() {
        let _lock = lock_and_init();
        // "abstract" has 8 letters — exactly the entry's max length; the autocompleting
        // `set_text` must not be truncated by it.
        let wordlist = word_indices(&["abstract", "abandon"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        harness.tap_wordlist_letter(b'a');
        harness.tap_wordlist_letter(b'b');
        harness.tap_wordlist_letter(b's');
        assert_eq!(harness.text().as_str(), "abstract");
        assert!(!Harness::disabled(&harness.confirm()));
    }

    #[test]
    fn test_wordlist_autocompletes_past_shorter_word() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["act", "action"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        // "act" matches two candidates ("act", "action"): no autocomplete, even though the
        // entry is itself a confirmable word.
        harness.tap_wordlist_letter(b'a');
        harness.tap_wordlist_letter(b'c');
        harness.tap_wordlist_letter(b't');
        assert_eq!(harness.text().as_str(), "act");
        assert!(!Harness::disabled(&harness.confirm()));

        // 'i' leaves only "action": autocompletes.
        harness.tap_wordlist_letter(b'i');
        assert_eq!(harness.text().as_str(), "action");
        assert!(!Harness::disabled(&harness.confirm()));
    }

    #[test]
    fn test_wordlist_backspace_undoes_autocomplete() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["zebra", "zoo"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        harness.tap_wordlist_letter(b'z');
        harness.tap_wordlist_letter(b'e');
        assert_eq!(harness.text().as_str(), "zebra");

        // Deleting never re-autocompletes (else backspace could not get past the completed
        // part): each press removes exactly one letter, with the filter and confirm gating
        // tracking along.
        let backspace = harness.backspace();
        harness.tap_button(&backspace);
        assert_eq!(harness.text().as_str(), "zebr");
        assert!(Harness::disabled(&harness.confirm()));
        harness.assert_enabled_letters(b"a");
        harness.tap_button(&backspace);
        assert_eq!(harness.text().as_str(), "zeb");

        // Typing again re-arms the autocomplete.
        harness.tap_wordlist_letter(b'r');
        assert_eq!(harness.text().as_str(), "zebra");
    }

    #[test]
    fn test_wordlist_slide_off_key_cancels() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["zoo", "zone"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        // Press the enabled 'z', slide off it (onto the disabled 'x'), release: the buttonmatrix
        // discards its selection when the pointer leaves the pressed key, so the preview hides
        // and nothing is typed.
        let preview = harness.wordlist_preview();
        let (row, col) = keyboard::wordlist_letter_pos(b'z');
        let (x, y) = harness.on_keyboard(keyboard::wordlist_key_center(row, col));
        let (x_next, _) = harness.on_keyboard(keyboard::wordlist_key_center(row, col + 1));
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, true);
        pump_for(120);
        assert!(!Harness::hidden(&preview));

        harness.touch.push(x_next, y, true);
        harness.touch.push(x_next, y, true);
        pump_for(120);
        assert!(Harness::hidden(&preview));

        harness.touch.push(x_next, y, false);
        pump_for(120);
        assert_eq!(harness.text().as_str(), "");
    }

    #[test]
    fn test_wordlist_holding_key_types_once() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["zoo", "zone"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        // Hold 'z' well past LVGL's long-press threshold (400ms) and repeat period (100ms): the
        // NO_REPEAT ctrl bit must keep the buttonmatrix from auto-repeating; exactly one letter
        // is inserted, on release.
        let (row, col) = keyboard::wordlist_letter_pos(b'z');
        let (x, y) = harness.on_keyboard(keyboard::wordlist_key_center(row, col));
        harness.touch.push(x, y, true);
        harness.touch.push(x, y, true);
        pump_for(700);
        assert_eq!(harness.text().as_str(), "");
        harness.touch.push(x, y, false);
        pump_for(120);
        assert_eq!(harness.text().as_str(), "z");
    }

    #[test]
    fn test_wordlist_preview_on_centred_rows() {
        let _lock = lock_and_init();
        // The full wordlist, so every row's first letters are enabled.
        let wordlist: Vec<u16> = (0..2048).collect();
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::No, "");

        // The 9-key and 7-key rows are centred, so the preview position depends on the row's
        // key count: press-hold a key on each and check the balloon straddles it.
        let preview = harness.wordlist_preview();
        for (letter, expected_label) in [(b's', "s"), (b'c', "c")] {
            let (row, col) = keyboard::wordlist_letter_pos(letter);
            let (x, y) = harness.on_keyboard(keyboard::wordlist_key_center(row, col));
            harness.touch.push(x, y, true);
            harness.touch.push(x, y, true);
            pump_for(120);
            assert!(!Harness::hidden(&preview));
            let balloon = coords(&preview);
            // `x2` is inclusive (x1 + width - 1), so round the centre up.
            assert_eq!(
                (balloon.x1 + balloon.x2 + 1) / 2,
                x,
                "balloon not centred on '{}'",
                letter as char
            );
            assert!(
                balloon.y1 < y - 30 - 60,
                "balloon head not above '{}'",
                letter as char
            );
            let label = preview
                .child(1)
                .expect("preview label")
                .try_downcast::<class::LabelTag>()
                .expect("preview label class");
            assert_eq!(label.get_text().unwrap().to_str().unwrap(), expected_label);
            harness.touch.push(x, y, false);
            pump_for(120);
            assert!(Harness::hidden(&preview));
        }
        assert_eq!(harness.text().as_str(), "sc");
    }

    #[test]
    fn test_wordlist_close_button_rejects() {
        let _lock = lock_and_init();
        let wordlist = word_indices(&["zoo"]);
        let mut harness = Harness::new_wordlist(&wordlist, CanCancel::Yes, "");

        // Same child order as the passphrase screen: title, display, keyboard, actions, spacer,
        // corner close button.
        let close = harness.screen.child(5).expect("corner close button");
        harness.tap_button(&close);
        let result = poll_once(&mut harness.result).expect("close resolves");
        assert!(
            matches!(result, Err(WordlistEntryAbort::Cancel)),
            "close button must reject with Cancel"
        );
    }
}
