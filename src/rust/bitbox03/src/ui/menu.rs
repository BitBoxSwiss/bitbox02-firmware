// SPDX-License-Identifier: Apache-2.0

use alloc::format;

use bitbox_hal::ui::UserAbort;
use bitbox_lvgl::{
    self as lvgl, LabelExt, LvLabel, LvLabelLongMode, LvObj, LvOpacityLevel, ObjExt,
};
use util::futures::completion::Responder;

use super::nav_button::{NavIcon, build_close_button, build_nav_button};

#[derive(Clone, Copy)]
pub enum MenuAction {
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

pub fn build_menu_screen(
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
        let navigation = transparent_row(&screen, 82);
        // Keep Back on the left and Next on the right, whichever are present.
        navigation.set_style_flex_main_place(
            match (can_go_previous, can_go_next) {
                (true, true) => lvgl::LvFlexAlign::LV_FLEX_ALIGN_SPACE_BETWEEN,
                (false, true) => lvgl::LvFlexAlign::LV_FLEX_ALIGN_END,
                _ => lvgl::LvFlexAlign::LV_FLEX_ALIGN_START,
            },
            0,
        );
        if can_go_previous {
            let previous_responder = responder.clone();
            let back = build_nav_button(&navigation, NavIcon::Back);
            back.add_click_cb(move || {
                previous_responder.resolve(MenuAction::Previous);
            })
            .expect("failed to register previous callback");
        }
        if can_go_next {
            let next_responder = responder.clone();
            let next = build_nav_button(&navigation, NavIcon::Next);
            next.add_click_cb(move || {
                next_responder.resolve(MenuAction::Next);
            })
            .expect("failed to register next callback");
        }
    }

    // Cancel lives in the top-right corner so it doesn't crowd the bottom navigation.
    let cancel_responder = responder.clone();
    let close = build_close_button(&screen);
    close
        .add_click_cb(move || {
            cancel_responder.resolve(MenuAction::Cancel);
        })
        .expect("failed to register cancel callback");

    let show_continue = continue_on_last && index + 1 == words.len();
    if select_word || show_continue {
        let actions = transparent_row(&screen, 82);
        // Primary action sits on the right, under the Next button.
        actions.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_END, 0);
        if select_word {
            // Confirming the highlighted word.
            let select = build_nav_button(&actions, NavIcon::Confirm);
            select
                .add_click_cb(move || {
                    responder.resolve(MenuAction::Select);
                })
                .expect("failed to register select callback");
        } else {
            // Advancing to the next step of the workflow.
            let cont = build_nav_button(&actions, NavIcon::Next);
            cont.add_click_cb(move || {
                responder.resolve(MenuAction::Continue);
            })
            .expect("failed to register continue callback");
        }
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
