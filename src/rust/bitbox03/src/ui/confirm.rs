// SPDX-License-Identifier: Apache-2.0

use bitbox_hal::ui::{ConfirmParams, UserAbort};
use bitbox_lvgl::{
    self as lvgl, LabelExt, LvLabel, LvLabelLongMode, LvObj, LvOpacityLevel, ObjExt,
};
use util::futures::completion::Responder;

use super::nav_button::{NavIcon, build_nav_button};

pub fn build_confirm_screen(
    params: &ConfirmParams<'_>,
    responder: Responder<Result<(), UserAbort>>,
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
    title.set_text(params.title).unwrap();
    title.set_style_text_font(
        lvgl::fonts::INTER_BOLD_48,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );

    let body = LvLabel::new(&screen).unwrap();
    body.set_width(380);
    body.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    body.set_text(params.body).unwrap();
    body.set_style_text_font(
        lvgl::fonts::INTER_REGULAR_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
    body.set_style_flex_grow(1, 0);

    let actions = LvObj::with_parent(&screen).unwrap();
    actions.set_width(380);
    actions.set_height(82);
    actions.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    actions.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW);
    actions.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_SPACE_BETWEEN, 0);
    actions.set_style_pad_top(0, 0);
    actions.set_style_pad_bottom(0, 0);
    actions.set_style_pad_left(0, 0);
    actions.set_style_pad_right(0, 0);
    actions.set_style_margin_top(16, 0);
    actions.set_style_border_width(0, 0);
    actions.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);

    let reject_responder = responder.clone();
    let reject = build_nav_button(&actions, NavIcon::Cancel);
    reject
        .add_click_cb(move || reject_responder.resolve(Err(UserAbort)))
        .expect("failed to register reject callback");

    let accept = build_nav_button(&actions, NavIcon::Confirm);
    accept
        .add_click_cb(move || responder.resolve(Ok(())))
        .expect("failed to register accept callback");

    screen
}
