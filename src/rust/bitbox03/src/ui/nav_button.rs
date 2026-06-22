// SPDX-License-Identifier: Apache-2.0

//! Square navigation icon buttons (Back / Next / Confirm / Cancel) plus a circular corner close
//! button. The frame is a drawn LVGL outline (rounded square / circle); the glyph inside is a
//! bitmap (PNG decoded to an `LvCanvas`). The bitmap is recoloured white normally and black when
//! the button is pressed (the frame fills white on press), so the icon stays visible. The caller
//! wires the behaviour, e.g. `add_click_cb`.

use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;

use bitbox_lvgl::{
    self as lvgl, LvButton, LvCanvas, LvEventCode, LvObj, LvOpacityLevel, LvState,
    LvStyleTransition, ObjExt, style::prop,
};

/// Style selector for the pressed state.
const PRESSED_SELECTOR: u32 = LvState::LV_STATE_PRESSED as u32;

/// Properties that change between the normal and pressed look (the button's white fill).
const PRESS_TRANSITION_PROPS: [u8; 3] = [prop::BG_OPA, prop::BG_COLOR, prop::INV];
/// Effectively instant (1ms) transition, overriding the default theme's ~80ms fade so the press
/// highlight appears/clears immediately even on a very short tap.
static PRESS_TRANSITION: LvStyleTransition = LvStyleTransition::new(&PRESS_TRANSITION_PROPS, 1, 0);

/// Which navigation icon to show inside the button frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavIcon {
    /// Left-pointing chevron — go back a step in a workflow.
    Back,
    /// Right-pointing chevron — advance to the next step in a workflow.
    Next,
    /// Checkmark — confirm at the end of a workflow.
    Confirm,
    /// Cross — cancel / abort.
    Cancel,
}

impl NavIcon {
    /// The icon bitmap (white glyph on a transparent background).
    fn png(self) -> &'static [u8] {
        match self {
            NavIcon::Back => include_bytes!("../../icons/back.png"),
            NavIcon::Next => include_bytes!("../../icons/next.png"),
            NavIcon::Confirm => include_bytes!("../../icons/confirm.png"),
            NavIcon::Cancel => include_bytes!("../../icons/cancel.png"),
        }
    }
}

/// Side length of the (square) navigation button, in pixels. Matches the 82×82 mockup viewBox.
const SIZE: i32 = 82;
/// Side length of the circular corner close button, in pixels (mockup viewBox).
const CLOSE_SIZE: i32 = 37;
/// The corner close button's icon (small cross).
const CLOSE_PNG: &[u8] = include_bytes!("../../icons/cancel2.png");

/// Decodes an icon PNG, adds it centred in `button` as a canvas, and sets it to recolour white
/// normally and black in the pressed state. Returns the canvas as an [`LvObj`] for press wiring.
fn add_icon(button: &LvButton, png: &[u8]) -> LvObj {
    // `png_decoder` returns ARGB8888 pixels as RGBA; LVGL expects BGRA in memory.
    let (header, mut data) = png_decoder::decode(png).expect("valid icon png");
    for px in data.iter_mut() {
        px.swap(0, 2);
    }
    let canvas = LvCanvas::new(button, data, header.width, header.height).expect("icon canvas");
    canvas.align(lvgl::LvAlign::LV_ALIGN_CENTER, 0, 0);
    // The PNG is a white glyph; recolour it white normally and black when pressed.
    canvas.set_style_image_recolor_opa(LvOpacityLevel::LV_OPA_COVER as u8, 0);
    canvas.set_style_image_recolor(lvgl::color::white(), 0);
    canvas.set_style_image_recolor(lvgl::color::black(), PRESSED_SELECTOR);
    canvas.to_obj()
}

/// Wires the pressed-state look: the interior fills white (cancelling the theme's grow + dim, with
/// an instant transition) and the icon inverts to black. A child does not inherit the button's
/// pressed state, so it is propagated to the icon via press/release events.
fn enable_press_invert(button: &LvButton, parts: Vec<LvObj>) {
    button.set_style_bg_color(lvgl::color::white(), PRESSED_SELECTOR);
    button.set_style_bg_opa(LvOpacityLevel::LV_OPA_COVER as u8, PRESSED_SELECTOR);
    // The default theme dims pressed objects (black recolor); disable so the fill is pure white.
    button.set_style_recolor_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, PRESSED_SELECTOR);
    // The default theme also grows pressed objects; cancel it.
    button.set_style_transform_width(0, PRESSED_SELECTOR);
    button.set_style_transform_height(0, PRESSED_SELECTOR);
    // Override the theme's ~80ms fade so the fill appears/clears instantly.
    button.set_style_transition(Some(PRESS_TRANSITION.as_dsc()), PRESSED_SELECTOR);
    button.set_style_transition(Some(PRESS_TRANSITION.as_dsc()), 0);

    let parts = Rc::new(parts);
    let on_press = Rc::clone(&parts);
    button
        .add_event_cb(LvEventCode::LV_EVENT_PRESSED, move || {
            for part in on_press.iter() {
                part.add_state(LvState::LV_STATE_PRESSED);
            }
        })
        .expect("failed to register press callback");
    let on_release = Rc::clone(&parts);
    button
        .add_event_cb(LvEventCode::LV_EVENT_RELEASED, move || {
            for part in on_release.iter() {
                part.remove_state(LvState::LV_STATE_PRESSED);
            }
        })
        .expect("failed to register release callback");
    button
        .add_event_cb(LvEventCode::LV_EVENT_PRESS_LOST, move || {
            for part in parts.iter() {
                part.remove_state(LvState::LV_STATE_PRESSED);
            }
        })
        .expect("failed to register press-lost callback");
}

/// Common frame styling for an outline icon button (transparent fill, white border, no shadow,
/// no padding).
fn style_outline_button(button: &LvButton, border_width: i32) {
    button.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0); // fill: none
    button.set_style_border_width(border_width, 0);
    button.set_style_border_color(lvgl::color::white(), 0);
    button.set_style_shadow_width(0, 0); // drop the default-theme shadow
    button.set_style_pad_top(0, 0);
    button.set_style_pad_bottom(0, 0);
    button.set_style_pad_left(0, 0);
    button.set_style_pad_right(0, 0);
}

/// Builds a navigation icon button and appends it to `parent`. Returns the button so the caller can
/// attach a click handler and position it.
pub fn build_nav_button(parent: &LvObj, icon: NavIcon) -> LvButton {
    let button = LvButton::new(parent).unwrap();
    button.set_size(SIZE, SIZE);
    button.set_style_radius(19, 0); // mockup rx = 18.5
    style_outline_button(&button, 3);

    let icon_obj = add_icon(&button, icon.png());
    enable_press_invert(&button, vec![icon_obj]);

    button
}

/// Builds the small circular "close" (cancel) button for the top-right corner of workflow screens
/// that already carry bottom navigation, so cancel doesn't crowd the Back/Next/Confirm row. It is
/// marked floating (taken out of the parent's layout) and aligned to the parent's top-right corner.
/// The caller wires the click handler.
pub fn build_close_button(parent: &LvObj) -> LvButton {
    let button = LvButton::new(parent).unwrap();
    button.set_size(CLOSE_SIZE, CLOSE_SIZE);
    button.set_style_radius(lvgl::ffi::LV_RADIUS_CIRCLE as i32, 0); // full circle
    style_outline_button(&button, 2);

    // Take it out of the parent's (flex) layout and pin it to the screen's top-right corner,
    // ~12px from the edges. The offsets push it past the standard 50px side / 40px top screen
    // padding (50-12 right, 12-40 up) so it hugs the real corner, clear of centred title text.
    // Callers on screens with different padding can re-`align` the returned button.
    button.add_flag(lvgl::LvObjFlag::LV_OBJ_FLAG_FLOATING);
    button.align(lvgl::LvAlign::LV_ALIGN_TOP_RIGHT, 38, -28);

    let icon_obj = add_icon(&button, CLOSE_PNG);
    enable_press_invert(&button, vec![icon_obj]);

    button
}
