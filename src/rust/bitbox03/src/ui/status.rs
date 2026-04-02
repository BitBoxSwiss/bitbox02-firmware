// SPDX-License-Identifier: Apache-2.0

use bitbox_lvgl::{
    self as lvgl, LabelExt, LvAlign, LvLabel, LvLabelLongMode, LvObj, LvOpacityLevel, ObjExt,
};

pub(super) fn build_status_screen(title: &str, status_success: bool) -> LvObj {
    let screen = LvObj::new().unwrap();
    screen.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    screen.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_COLUMN);
    screen.set_style_bg_color(lvgl::color::black(), 0);
    screen.set_style_text_color(lvgl::color::white(), 0);
    screen.set_style_pad_top(96, 0);
    screen.set_style_pad_right(50, 0);
    screen.set_style_pad_bottom(40, 0);
    screen.set_style_pad_left(50, 0);
    screen.set_style_pad_row(40, 0);
    screen.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);

    let badge = LvObj::with_parent(&screen).unwrap();
    badge.set_size(112, 112);
    badge.set_style_radius(56, 0);
    badge.set_style_bg_color(
        if status_success {
            lvgl::color::hex(0x0d8f4b)
        } else {
            lvgl::color::hex(0xb3261e)
        },
        0,
    );
    badge.set_style_bg_opa(LvOpacityLevel::LV_OPA_COVER as u8, 0);
    badge.set_style_border_width(0, 0);

    let badge_label = LvLabel::new(&badge).unwrap();
    badge_label
        .set_text(if status_success { "OK" } else { "ERR" })
        .unwrap();
    badge_label.set_style_text_font(
        lvgl::fonts::INTER_BOLD_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );
    badge_label.align(LvAlign::LV_ALIGN_CENTER, 0, 0);

    let title_label = LvLabel::new(&screen).unwrap();
    title_label.set_width(380);
    title_label.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    title_label.set_text(title).unwrap();
    title_label.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    title_label.set_style_text_font(
        lvgl::fonts::INTER_BOLD_48,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );

    screen
}
