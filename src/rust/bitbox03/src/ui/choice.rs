// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use bitbox_hal::ui::TrinaryChoice;
use bitbox_lvgl::{
    self as lvgl, LabelExt, LvAlign, LvButton, LvLabel, LvLabelLongMode, LvObj, LvOpacityLevel,
    ObjExt,
};
use util::futures::completion::Responder;

fn add_button(
    parent: &LvObj,
    width: i32,
    label: &str,
    choice: TrinaryChoice,
    responder: Responder<TrinaryChoice>,
) {
    let button = LvButton::new(parent).unwrap();
    button.set_size(width, 72);
    button.set_style_bg_color(lvgl::color::white(), 0);
    button.set_style_bg_opa(LvOpacityLevel::LV_OPA_COVER as u8, 0);
    button.set_style_border_width(2, 0);
    button.set_style_border_color(lvgl::color::black(), 0);
    button
        .add_click_cb(move || responder.resolve(choice))
        .expect("failed to register choice callback");

    let button_label = LvLabel::new(&button).unwrap();
    button_label.set_text(label).unwrap();
    button_label.set_style_text_font(
        lvgl::fonts::INTER_BOLD_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
    button_label.set_style_text_color(lvgl::color::black(), 0);
    button_label.align(LvAlign::LV_ALIGN_CENTER, 0, 0);
}

pub(super) fn build_trinary_choice_screen(
    message: &str,
    label_left: Option<&str>,
    label_middle: Option<&str>,
    label_right: Option<&str>,
    responder: Responder<TrinaryChoice>,
) -> LvObj {
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

    let title = LvLabel::new(&screen).unwrap();
    title.set_width(380);
    title.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    title.set_text(message).unwrap();
    title.set_style_text_font(
        lvgl::fonts::INTER_BOLD_48,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
    title.set_style_flex_grow(1, 0);

    let actions = LvObj::with_parent(&screen).unwrap();
    actions.set_width(380);
    actions.set_height(72);
    actions.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    actions.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW);
    actions.set_style_pad_top(0, 0);
    actions.set_style_pad_bottom(0, 0);
    actions.set_style_pad_left(0, 0);
    actions.set_style_pad_right(0, 0);
    actions.set_style_pad_column(20, 0);
    actions.set_style_border_width(0, 0);
    actions.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);

    let choices: Vec<(&str, TrinaryChoice)> = [
        label_left.map(|label| (label, TrinaryChoice::Left)),
        label_middle.map(|label| (label, TrinaryChoice::Middle)),
        label_right.map(|label| (label, TrinaryChoice::Right)),
    ]
    .into_iter()
    .flatten()
    .collect();
    assert!(!choices.is_empty(), "trinary choice requires a button");
    let choice_count = choices.len();
    let width = match choice_count {
        1 => 380,
        2 => 180,
        3 => 113,
        _ => unreachable!("only three choices exist"),
    };

    for (label, choice) in choices {
        add_button(&actions, width, label, choice, responder.clone());
    }

    screen
}
