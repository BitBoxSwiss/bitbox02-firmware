// SPDX-License-Identifier: Apache-2.0

use bitbox_lvgl::{
    self as lvgl, LabelExt, LvAlign, LvCanvas, LvLabel, LvLabelLongMode, LvObj, LvOpacityLevel,
    ObjExt,
};

/// Checkmark shown inside the badge circle on success (white glyph on transparent).
const SUCCESS_PNG: &[u8] = include_bytes!("../../icons/status_success.png");
/// Cross shown inside the badge circle on failure/cancel (white glyph on transparent).
const CANCEL_PNG: &[u8] = include_bytes!("../../icons/status_cancel.png");

/// Diameter of the round status badge, in pixels (mockup viewBox).
const BADGE_SIZE: i32 = 140;
/// Stroke width of the badge circle (mockup stroke).
const BADGE_BORDER_WIDTH: i32 = 5;

pub fn build_status_screen(title: &str, status_success: bool) -> LvObj {
    let screen = LvObj::new().unwrap();
    screen.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    screen.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_COLUMN);
    screen.set_style_bg_color(lvgl::color::black(), 0);
    screen.set_style_text_color(lvgl::color::white(), 0);
    screen.set_style_pad_top(40, 0);
    screen.set_style_pad_right(50, 0);
    screen.set_style_pad_bottom(40, 0);
    screen.set_style_pad_left(50, 0);
    screen.set_style_pad_row(40, 0);
    // Centre the badge + title block on the screen (equal top/bottom padding keeps it exact).
    screen.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    screen.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);

    let badge = LvObj::with_parent(&screen).unwrap();
    badge.set_size(BADGE_SIZE, BADGE_SIZE);
    badge.set_style_radius(lvgl::ffi::LV_RADIUS_CIRCLE as i32, 0);
    badge.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    badge.set_style_border_width(BADGE_BORDER_WIDTH, 0);
    badge.set_style_border_color(lvgl::color::white(), 0);

    // `png_decoder` returns ARGB8888 pixels as RGBA; LVGL expects BGRA in memory.
    let png = if status_success {
        SUCCESS_PNG
    } else {
        CANCEL_PNG
    };
    let (header, mut data) = png_decoder::decode(png).expect("valid status icon png");
    for px in data.iter_mut() {
        px.swap(0, 2);
    }
    let glyph =
        LvCanvas::new(&badge, data, header.width, header.height).expect("status icon canvas");
    glyph.align(LvAlign::LV_ALIGN_CENTER, 0, 0);

    let title_label = LvLabel::new(&screen).unwrap();
    title_label.set_width(380);
    title_label.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    title_label.set_text(title).unwrap();
    title_label.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    title_label.set_style_text_font(
        lvgl::fonts::INTER_REGULAR_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );

    screen
}
