// SPDX-License-Identifier: Apache-2.0

use alloc::format;

use bitbox_hal::ui::UserAbort;
use bitbox_lvgl::{
    self as lvgl, LabelExt, LvAlign, LvButton, LvLabel, LvLabelLongMode, LvObj, LvOpacityLevel,
    ObjExt,
};
use util::futures::completion::Responder;

#[derive(Clone, Copy)]
pub(super) enum MenuAction {
    Previous,
    Next,
    Select,
    Continue,
    Cancel,
}

pub(super) enum MenuResult {
    Selected(u8),
    Continue,
    Cancel(usize),
}

fn add_button<F>(parent: &LvObj, width: i32, height: i32, label: &str, primary: bool, cb: F)
where
    F: FnMut() + 'static,
{
    let button = LvButton::new(parent).unwrap();
    button.set_size(width, height);
    button.set_style_bg_color(
        if primary {
            lvgl::color::white()
        } else {
            lvgl::color::hex(0x30333a)
        },
        0,
    );
    button.set_style_bg_opa(LvOpacityLevel::LV_OPA_COVER as u8, 0);
    button.set_style_border_width(2, 0);
    button.set_style_border_color(
        if primary {
            lvgl::color::black()
        } else {
            lvgl::color::white()
        },
        0,
    );
    button
        .add_click_cb(cb)
        .expect("failed to register menu callback");

    let button_label = LvLabel::new(&button).unwrap();
    button_label.set_text(label).unwrap();
    button_label.set_style_text_font(
        lvgl::fonts::INTER_BOLD_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
    button_label.set_style_text_color(
        if primary {
            lvgl::color::black()
        } else {
            lvgl::color::white()
        },
        0,
    );
    button_label.align(LvAlign::LV_ALIGN_CENTER, 0, 0);
}

fn transparent_row(parent: &LvObj, height: i32) -> LvObj {
    let row = LvObj::with_parent(parent).unwrap();
    row.set_width(380);
    row.set_height(height);
    row.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    row.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW);
    row.set_style_pad_top(0, 0);
    row.set_style_pad_bottom(0, 0);
    row.set_style_pad_left(0, 0);
    row.set_style_pad_right(0, 0);
    row.set_style_pad_column(20, 0);
    row.set_style_border_width(0, 0);
    row.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    row
}

pub(super) fn build_menu_screen(
    words: &[&str],
    title: Option<&str>,
    index: usize,
    select_word: bool,
    continue_on_last: bool,
    responder: Responder<MenuAction>,
) -> LvObj {
    assert!(!words.is_empty(), "menu requires at least one word");
    assert!(index < words.len(), "menu index out of bounds");

    let screen = LvObj::new().unwrap();
    screen.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    screen.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_COLUMN);
    screen.set_style_bg_color(lvgl::color::black(), 0);
    screen.set_style_text_color(lvgl::color::white(), 0);
    screen.set_style_pad_top(40, 0);
    screen.set_style_pad_right(50, 0);
    screen.set_style_pad_bottom(40, 0);
    screen.set_style_pad_left(50, 0);
    screen.set_style_pad_row(24, 0);

    let title_text = title
        .map(alloc::string::ToString::to_string)
        .unwrap_or_else(|| format!("{:02}", index + 1));
    let title_label = LvLabel::new(&screen).unwrap();
    title_label.set_width(380);
    title_label.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    title_label.set_text(&title_text).unwrap();
    title_label.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    title_label.set_style_text_font(
        lvgl::fonts::INTER_BOLD_48,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );

    let word_label = LvLabel::new(&screen).unwrap();
    word_label.set_width(380);
    word_label.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    word_label.set_text(words[index]).unwrap();
    word_label.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    word_label.set_style_text_font(
        lvgl::fonts::INTER_BOLD_48,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
    word_label.set_style_flex_grow(1, 0);

    let can_go_previous = index > 0;
    let can_go_next = index + 1 < words.len();
    if can_go_previous || can_go_next {
        let navigation = transparent_row(&screen, 64);
        let navigation_button_width = if can_go_previous && can_go_next {
            180
        } else {
            380
        };
        if can_go_previous {
            let previous_responder = responder.clone();
            add_button(
                &navigation,
                navigation_button_width,
                64,
                "Back",
                false,
                move || {
                    previous_responder.resolve(MenuAction::Previous);
                },
            );
        }
        if can_go_next {
            let next_responder = responder.clone();
            add_button(
                &navigation,
                navigation_button_width,
                64,
                "Next",
                false,
                move || {
                    next_responder.resolve(MenuAction::Next);
                },
            );
        }
    }

    let actions = transparent_row(&screen, 72);
    let show_continue = continue_on_last && index + 1 == words.len();
    let show_primary = select_word || show_continue;
    let action_button_width = if show_primary { 180 } else { 380 };

    let cancel_responder = responder.clone();
    add_button(
        &actions,
        action_button_width,
        72,
        "Cancel",
        false,
        move || {
            cancel_responder.resolve(MenuAction::Cancel);
        },
    );

    if select_word {
        add_button(&actions, 180, 72, "Select", true, move || {
            responder.resolve(MenuAction::Select);
        });
    } else if show_continue {
        add_button(&actions, 180, 72, "Continue", true, move || {
            responder.resolve(MenuAction::Continue);
        });
    }

    screen
}

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
