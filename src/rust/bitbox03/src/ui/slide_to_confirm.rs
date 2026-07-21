// SPDX-License-Identifier: Apache-2.0

//! "Slide to confirm" control: a rounded-outline track with an arrow knob that the user drags all
//! the way to the right to confirm (used instead of a tap for high-stakes confirmations such as
//! transactions, mirroring the BitBox02 hold gesture). The track frame is drawn by LVGL; the knob
//! arrow, the direction chevrons and the slide-target arrow outline are bitmaps.
//!
//! Layering: the chevrons/target outline are siblings *below* the slider, so the knob (drawn as
//! part of the slider) glides over them. Each marker is hidden once the knob's leading edge
//! reaches it, so passed markers do not reappear behind the knob, matching the mockup's end
//! state of a bare track with the arrow at the far right. An incomplete slide glides the knob
//! back with an explicit animation whose per-frame callback applies the same thresholds, so the
//! markers reappear one by one as the knob passes back over them (LVGL's built-in bar animation
//! cannot do this: it emits no per-frame events and get_value reports the animation's target).
//!
//! Gesture hardening: reaching the far end must be the result of a deliberate slide, so on top of
//! LVGL's knob-only hit test the value-change handler rejects advances from non-pointer input or
//! touch samples far off the track, and caps how far the knob may advance per input sample —
//! a single glitched touch coordinate cannot teleport the knob to the end and confirm.

use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::Cell;

use bitbox_lvgl::{
    self as lvgl, CanvasExt, LabelExt, LvCanvas, LvHandle, LvImageDsc, LvLabel, LvObj, LvObjFlag,
    LvOpacityLevel, LvSlider, ObjExt, class,
};
use core::ptr::NonNull;

/// Track (and component) width; the standard content width of a screen.
const TRACK_WIDTH: i32 = 380;
/// Track height, matching the navigation button height.
const TRACK_HEIGHT: i32 = 82;
const TRACK_BORDER: i32 = 3;
const TRACK_RADIUS: i32 = 20;
/// Distance from the track edge to the knob-arrow center at rest. Applied as MAIN-part padding,
/// which is what LVGL insets the knob travel by.
const KNOB_INSET: i32 = 38;
/// Knob center travel in pixels. The slider range is `0..=TRAVEL`, i.e. one value unit per pixel.
const TRAVEL: i32 = TRACK_WIDTH - 2 * KNOB_INSET;
/// Duration of the knob gliding back after an incomplete slide.
const SNAP_BACK_MS: u32 = 250;
/// Number of direction chevrons between the knob and the target outline.
const CHEVRON_COUNT: i32 = 5;
/// Upper bound on how far the knob may advance per input sample. A real slide delivers a stream
/// of samples, so even a very fast flick (~80 px per sample at a 60 Hz touch controller is
/// roughly a full-track swipe in 80 ms) still reaches the end; a single spurious/glitched sample
/// can advance the knob by at most this much, so it can never confirm on its own.
const MAX_ADVANCE_PER_EVENT: i32 = 80;
/// How far above/below the track a touch sample may stray while still advancing the knob.
/// Rejects e.g. a second contact reported elsewhere on the screen while the knob is held.
const TOUCH_Y_GRACE: i32 = 40;

/// The knob: filled arrow (white on transparent).
const ARROW_PNG: &[u8] = include_bytes!("../../icons/slide_arrow.png");
/// The slide target at the right end of the track: outline of the knob arrow.
const ARROW_OUTLINE_PNG: &[u8] = include_bytes!("../../icons/slide_arrow_outline.png");
/// Direction chevron (gray on transparent).
const CHEVRON_PNG: &[u8] = include_bytes!("../../icons/slide_chevron.png");

/// Decodes an icon PNG (white glyph on transparent) into LVGL's in-memory pixel order.
fn decode_icon(png: &[u8]) -> (u32, u32, Vec<[u8; 4]>) {
    // `png_decoder` returns ARGB8888 pixels as RGBA; LVGL expects BGRA in memory.
    let (header, mut data) = png_decoder::decode(png).expect("valid icon png");
    for px in data.iter_mut() {
        px.swap(0, 2);
    }
    (header.width, header.height, data)
}

/// Hides each marker the knob's leading edge has reached at `value`; shows the rest. Used on
/// live value changes while dragging and per animation frame while gliding back, so markers
/// disappear and reappear one by one in both directions.
fn update_markers(markers: &[(i32, LvObj)], value: i32) {
    for (hide_at, marker) in markers {
        if value >= *hide_at {
            marker.add_flag(LvObjFlag::LV_OBJ_FLAG_HIDDEN);
        } else {
            marker.remove_flag(LvObjFlag::LV_OBJ_FLAG_HIDDEN);
        }
    }
}

/// State the snap-back animation callback operates on. It is kept alive by the slider's event
/// closures (via `Rc`); the animation's `var` pointer aliases it, so the animation is deleted
/// whenever the slider is pressed again and, crucially, when the slider is deleted.
struct SnapBack {
    slider: *mut lvgl::ffi::lv_obj_t,
    markers: Rc<Vec<(i32, LvObj)>>,
    last_value: Rc<Cell<i32>>,
}

/// Animation frame of the glide back to the start: moves the knob and updates the markers, so
/// each chevron reappears exactly when the knob passes back over it.
extern "C" fn snap_back_step(var: *mut core::ffi::c_void, value: i32) {
    let state = unsafe { &*(var as *const SnapBack) };
    unsafe { lvgl::ffi::lv_slider_set_value(state.slider, value, false) };
    state.last_value.set(value);
    update_markers(&state.markers, value);
}

/// Stores decoded icon pixels in a hidden canvas child of `parent` and returns the image
/// descriptor LVGL filled in for them, for use as a style background image on objects that live
/// no longer than `parent`.
fn attach_icon_pixels<C: class::LvClass>(
    parent: &LvHandle<C>,
    (width, height, data): (u32, u32, Vec<[u8; 4]>),
) -> NonNull<LvImageDsc> {
    let canvas = LvCanvas::new(parent, data, width, height).expect("icon canvas");
    canvas.add_flag(LvObjFlag::LV_OBJ_FLAG_HIDDEN);
    canvas.get_image().expect("canvas image descriptor")
}

/// Builds the "Slide to confirm" control (label + track) and appends it to `parent`.
/// `on_confirm` runs once, when the knob has been dragged all the way to the right.
/// Returns the component's root object so the caller can position it.
pub fn build_slide_to_confirm(parent: &LvObj, on_confirm: impl FnMut() + 'static) -> LvObj {
    let root = LvObj::with_parent(parent).unwrap();
    root.set_width(TRACK_WIDTH);
    root.set_height(lvgl::ffi::LV_SIZE_CONTENT as i32);
    root.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    root.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_COLUMN);
    root.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    root.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    // The default theme's container style carries its own (dark) text color; restore white.
    root.set_style_text_color(lvgl::color::white(), 0);
    root.set_style_border_width(0, 0);
    root.set_style_pad_top(0, 0);
    root.set_style_pad_bottom(0, 0);
    root.set_style_pad_left(0, 0);
    root.set_style_pad_right(0, 0);
    root.set_style_pad_row(10, 0);

    let label = LvLabel::new(&root).unwrap();
    label.set_text("Slide to confirm").unwrap();
    label.set_style_text_font(
        lvgl::fonts::INTER_REGULAR_24,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );

    let track = LvObj::with_parent(&root).unwrap();
    track.set_size(TRACK_WIDTH, TRACK_HEIGHT);
    track.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    track.set_style_border_width(0, 0);
    track.set_style_pad_top(0, 0);
    track.set_style_pad_bottom(0, 0);
    track.set_style_pad_left(0, 0);
    track.set_style_pad_right(0, 0);

    // The knob arrow is decoded once: its width feeds the marker hide thresholds here, and the
    // pixels become the slider's knob image below.
    let (arrow_width, arrow_height, arrow_data) = decode_icon(ARROW_PNG);
    let arrow_w = arrow_width as i32;

    // All chevrons share one decoded pixel buffer, stored in a hidden canvas child of the track
    // (the markers referencing it are also track children, so it outlives them).
    let chevron = decode_icon(CHEVRON_PNG);
    let (chevron_w, chevron_h) = (chevron.0 as i32, chevron.1 as i32);
    let chevron_image = attach_icon_pixels(&track, chevron);

    // Chevrons and target outline: added before (= drawn below) the slider. Each entry is
    // (slider value at which the knob's leading edge reaches the marker, marker object).
    let mut markers: Vec<(i32, LvObj)> = Vec::new();
    for i in 1..=CHEVRON_COUNT {
        let marker = LvObj::with_parent(&track).unwrap();
        marker.set_size(chevron_w, chevron_h);
        marker.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
        marker.set_style_border_width(0, 0);
        marker.set_style_pad_top(0, 0);
        marker.set_style_pad_bottom(0, 0);
        marker.set_style_pad_left(0, 0);
        marker.set_style_pad_right(0, 0);
        // Markers sit under the slider and must not swallow presses the slider's knob-only hit
        // test rejects.
        marker.remove_flag(LvObjFlag::LV_OBJ_FLAG_CLICKABLE);
        // SAFETY: the descriptor and its pixel buffer belong to a hidden canvas child of the
        // track; the marker is also a track child, so the style cannot be drawn after the
        // descriptor is freed. LVGL's image cache is disabled (LV_CACHE_DEF_SIZE = 0 in
        // lv_conf.h); with a cache enabled, a stale cache entry keyed by this descriptor's
        // address could outlive the canvas.
        unsafe {
            marker.set_style_bg_image_src(Some(&*chevron_image.as_ptr()), 0);
        }
        let center = KNOB_INSET + TRAVEL * i / (CHEVRON_COUNT + 1);
        marker.align(lvgl::LvAlign::LV_ALIGN_LEFT_MID, center - chevron_w / 2, 0);
        markers.push((center - chevron_w / 2 - KNOB_INSET - arrow_w / 2, marker));
    }
    {
        let (width, height, data) = decode_icon(ARROW_OUTLINE_PNG);
        let w = width as i32;
        let canvas = LvCanvas::new(&track, data, width, height).expect("icon canvas");
        let center = KNOB_INSET + TRAVEL;
        canvas.align(lvgl::LvAlign::LV_ALIGN_LEFT_MID, center - w / 2, 0);
        markers.push((center - w / 2 - KNOB_INSET - arrow_w / 2, canvas.to_obj()));
    }

    let slider = LvSlider::new(&track).unwrap();
    slider.set_size(TRACK_WIDTH, TRACK_HEIGHT);
    slider.set_pos(0, 0);
    slider.set_range(0, TRAVEL);
    slider.set_orientation(lvgl::LvSliderOrientation::LV_SLIDER_ORIENTATION_HORIZONTAL);
    // Track frame on the MAIN part; its horizontal padding insets the knob travel.
    slider.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    slider.set_style_border_width(TRACK_BORDER, 0);
    slider.set_style_border_color(lvgl::color::white(), 0);
    slider.set_style_radius(TRACK_RADIUS, 0);
    slider.set_style_pad_top(0, 0);
    slider.set_style_pad_bottom(0, 0);
    slider.set_style_pad_left(KNOB_INSET, 0);
    slider.set_style_pad_right(KNOB_INSET, 0);
    // No progress fill.
    slider.set_style_bg_opa(
        LvOpacityLevel::LV_OPA_TRANSP as u8,
        lvgl::LvPart::LV_PART_INDICATOR as u32,
    );

    let knob_selector = lvgl::LvPart::LV_PART_KNOB as u32;
    let knob_pressed_selector = knob_selector | lvgl::LvState::LV_STATE_PRESSED as u32;
    // The knob rectangle itself is invisible; the arrow bitmap is drawn as its background image.
    slider.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, knob_selector);
    slider.set_style_border_width(0, knob_selector);
    slider.set_style_shadow_width(0, knob_selector);
    slider.set_style_radius(0, knob_selector);
    slider.set_style_pad_top(0, knob_selector);
    slider.set_style_pad_bottom(0, knob_selector);
    slider.set_style_pad_left(0, knob_selector);
    slider.set_style_pad_right(0, knob_selector);
    // Cancel the default theme's pressed-state grow and dim so the arrow is unaffected.
    slider.set_style_transform_width(0, knob_pressed_selector);
    slider.set_style_transform_height(0, knob_pressed_selector);
    slider.set_style_recolor_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, knob_pressed_selector);

    // The knob arrow pixels live in a hidden canvas child of the slider: LVGL fills in the image
    // descriptor, and the pixel buffer is freed together with the slider.
    let knob_image = attach_icon_pixels(&slider, (arrow_width, arrow_height, arrow_data));
    // SAFETY: the descriptor and its pixel buffer belong to a hidden canvas child of the slider,
    // so they stay valid for as long as the slider (and thus this style) can be drawn. LVGL's
    // image cache is disabled (LV_CACHE_DEF_SIZE = 0 in lv_conf.h); with a cache enabled, a
    // stale cache entry keyed by this descriptor's address could outlive the canvas.
    unsafe {
        slider.set_style_bg_image_src(Some(&*knob_image.as_ptr()), knob_selector);
    }

    // Only a drag that starts on the knob moves the slider; pressing elsewhere on the track does
    // nothing (the slider's LV_EVENT_HIT_TEST handler checks the knob area).
    slider.add_flag(LvObjFlag::LV_OBJ_FLAG_ADV_HITTEST);

    let slider = Rc::new(slider);
    let markers = Rc::new(markers);
    let confirmed = Rc::new(Cell::new(false));
    // The knob position accepted by the last value-change, the baseline for the per-sample
    // advance cap.
    let last_value = Rc::new(Cell::new(0));
    let snap_back = Rc::new(SnapBack {
        slider: slider.as_ptr(),
        markers: Rc::clone(&markers),
        last_value: Rc::clone(&last_value),
    });

    // The animation callback dereferences `snap_back` and the slider, so no animation may
    // outlive the slider. Its event closures (below) keep the state alive until then.
    {
        let snap_back = Rc::clone(&snap_back);
        slider
            .add_event_cb(lvgl::LvEventCode::LV_EVENT_DELETE, move || unsafe {
                lvgl::ffi::lv_anim_delete(
                    Rc::as_ptr(&snap_back) as *mut core::ffi::c_void,
                    Some(snap_back_step),
                );
            })
            .expect("failed to register delete callback");
    }

    // On press, cancel a still-running snap-back animation and place the knob under the finger,
    // so a re-grabbed knob tracks the new drag from the start.
    {
        let slider_cb = Rc::clone(&slider);
        let last_value = Rc::clone(&last_value);
        let snap_back = Rc::clone(&snap_back);
        slider
            .add_event_cb(lvgl::LvEventCode::LV_EVENT_PRESSED, move || {
                unsafe {
                    lvgl::ffi::lv_anim_delete(
                        Rc::as_ptr(&snap_back) as *mut core::ffi::c_void,
                        Some(snap_back_step),
                    );
                }
                let indev = unsafe { lvgl::ffi::lv_indev_active() };
                if indev.is_null() {
                    return;
                }
                let mut point = lvgl::LvPoint { x: 0, y: 0 };
                let mut area = lvgl::LvArea {
                    x1: 0,
                    y1: 0,
                    x2: 0,
                    y2: 0,
                };
                unsafe {
                    lvgl::ffi::lv_indev_get_point(indev, &mut point);
                    lvgl::ffi::lv_obj_get_coords(slider_cb.as_ptr(), &mut area);
                }
                let value = (point.x - area.x1 - KNOB_INSET).clamp(0, TRAVEL);
                slider_cb.set_value(value, false);
                last_value.set(value);
            })
            .expect("failed to register press callback");
    }

    {
        let slider_cb = Rc::clone(&slider);
        let markers = Rc::clone(&markers);
        let confirmed = Rc::clone(&confirmed);
        let last_value = Rc::clone(&last_value);
        let mut on_confirm = on_confirm;
        slider
            .add_event_cb(lvgl::LvEventCode::LV_EVENT_VALUE_CHANGED, move || {
                let value = slider_cb.get_value();
                let previous = last_value.get();

                // Gesture hardening (see module docs). A null input device means the value was
                // set programmatically by firmware code (e.g. render tooling), which is trusted.
                let indev = unsafe { lvgl::ffi::lv_indev_active() };
                let advance_allowed = if indev.is_null() {
                    true
                } else if unsafe { lvgl::ffi::lv_indev_get_type(indev) }
                    != lvgl::LvIndevType::LV_INDEV_TYPE_POINTER
                {
                    false
                } else {
                    let mut point = lvgl::LvPoint { x: 0, y: 0 };
                    let mut area = lvgl::LvArea {
                        x1: 0,
                        y1: 0,
                        x2: 0,
                        y2: 0,
                    };
                    unsafe {
                        lvgl::ffi::lv_indev_get_point(indev, &mut point);
                        lvgl::ffi::lv_obj_get_coords(slider_cb.as_ptr(), &mut area);
                    }
                    point.y >= area.y1 - TOUCH_Y_GRACE && point.y <= area.y2 + TOUCH_Y_GRACE
                };

                let value = if !advance_allowed && value > previous {
                    // Suspicious sample: keep the knob where it was (retreating is always fine).
                    slider_cb.set_value(previous, false);
                    previous
                } else if indev.is_null() {
                    value
                } else if value > previous + MAX_ADVANCE_PER_EVENT {
                    let capped = previous + MAX_ADVANCE_PER_EVENT;
                    slider_cb.set_value(capped, false);
                    capped
                } else {
                    value
                };
                last_value.set(value);

                update_markers(&markers, value);
                if value >= TRAVEL && !confirmed.replace(true) {
                    on_confirm();
                }
            })
            .expect("failed to register value-changed callback");
    }

    // An incomplete slide glides the knob back with the marker-restoring animation.
    for code in [
        lvgl::LvEventCode::LV_EVENT_RELEASED,
        lvgl::LvEventCode::LV_EVENT_PRESS_LOST,
    ] {
        let slider_cb = Rc::clone(&slider);
        let confirmed = Rc::clone(&confirmed);
        let snap_back = Rc::clone(&snap_back);
        slider
            .add_event_cb(code, move || {
                if confirmed.get() {
                    return;
                }
                let current = slider_cb.get_value();
                if current == 0 {
                    return;
                }
                unsafe {
                    let mut anim = core::mem::zeroed::<lvgl::ffi::lv_anim_t>();
                    lvgl::ffi::lv_anim_init(&mut anim);
                    lvgl::ffi::lv_anim_set_var(
                        &mut anim,
                        Rc::as_ptr(&snap_back) as *mut core::ffi::c_void,
                    );
                    lvgl::ffi::lv_anim_set_exec_cb(&mut anim, Some(snap_back_step));
                    lvgl::ffi::lv_anim_set_values(&mut anim, current, 0);
                    lvgl::ffi::lv_anim_set_duration(&mut anim, SNAP_BACK_MS);
                    lvgl::ffi::lv_anim_start(&anim);
                }
            })
            .expect("failed to register release callback");
    }

    root
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::boxed::Box;
    use std::collections::VecDeque;
    use std::sync::{LazyLock, Mutex, MutexGuard, Once};
    use std::time::{Duration, Instant};
    use std::vec;

    use bitbox_lvgl::{
        LvArea, LvDisplay, LvDisplayRenderMode, LvIndev, LvIndevState, LvIndevType, LvPoint, ffi,
    };

    use super::*;

    const WIDTH: i32 = 480;
    const HEIGHT: i32 = 800;

    extern "C" fn now_ms() -> u32 {
        static START: LazyLock<Instant> = LazyLock::new(Instant::now);
        START.elapsed().as_millis() as u32
    }

    static LVGL_TEST_LOCK: Mutex<()> = Mutex::new(());
    static INIT: Once = Once::new();

    /// Serializes tests and lazily brings up LVGL with a headless 480×800 display and a tick
    /// source, so input processing, layouting and animations run for real.
    fn lock_and_init() -> MutexGuard<'static, ()> {
        // A failed test leaves the shared LVGL state usable, so ignore lock poisoning instead of
        // cascading one failure into every later test.
        let guard = LVGL_TEST_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        INIT.call_once(|| {
            lvgl::system::init();
            lvgl::tick::set_cb(Some(now_ms));
            let draw_buf: &'static mut [u32] =
                Box::leak(vec![0u32; (WIDTH * HEIGHT) as usize].into_boxed_slice());
            let display = LvDisplay::new(WIDTH, HEIGHT).expect("create display");
            display
                .set_buffers(
                    draw_buf,
                    None,
                    LvDisplayRenderMode::LV_DISPLAY_RENDER_MODE_PARTIAL,
                )
                .expect("set display buffers");
            // Dropping the handle does not delete the LVGL display; it lives for the whole
            // test process.
            display.set_flush_cb(|_display, _area, _px_map| {});
        });
        guard
    }

    /// Runs the LVGL timer loop (input reading, layout, animation, rendering) for `ms`.
    fn pump_for(ms: u64) {
        let deadline = Instant::now() + Duration::from_millis(ms);
        while Instant::now() < deadline {
            lvgl::timer::handler();
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    struct TouchSample {
        x: i32,
        y: i32,
        pressed: bool,
    }

    /// A scripted LVGL pointer device (same read model as `io::touchscreen::TouchScreen`: the
    /// queue front is the current state; entries past the first are drained one per read). Unlike
    /// the production type it deletes its input device on drop, so a finished test cannot keep
    /// replaying its last sample into later tests.
    struct ScriptedTouch {
        indev: LvIndev,
        queue: NonNull<VecDeque<TouchSample>>,
    }

    extern "C" fn scripted_read_cb(indev: *mut ffi::lv_indev_t, data: *mut ffi::lv_indev_data_t) {
        let queue = unsafe { ffi::lv_indev_get_user_data(indev) };
        debug_assert!(!queue.is_null());
        let queue = unsafe { &mut *(queue as *mut VecDeque<TouchSample>) };
        let data = unsafe { &mut *data };
        if let Some(next) = queue.front() {
            data.point = LvPoint {
                x: next.x,
                y: next.y,
            };
            data.state = if next.pressed {
                LvIndevState::LV_INDEV_STATE_PRESSED
            } else {
                LvIndevState::LV_INDEV_STATE_RELEASED
            };
        }
        if queue.len() > 1 {
            queue.pop_front();
            data.continue_reading = !queue.is_empty();
        }
    }

    impl ScriptedTouch {
        fn new() -> Self {
            let queue: &'static mut VecDeque<TouchSample> = Box::leak(Box::new(VecDeque::new()));
            let queue_ptr = NonNull::from(&mut *queue);
            let indev = LvIndev::new().expect("create input device");
            indev.set_type(LvIndevType::LV_INDEV_TYPE_POINTER);
            indev.set_read_cb(Some(scripted_read_cb));
            indev.set_user_data(Some(queue));
            Self {
                indev,
                queue: queue_ptr,
            }
        }

        fn push(&mut self, x: i32, y: i32, pressed: bool) {
            unsafe { self.queue.as_mut() }.push_back(TouchSample { x, y, pressed });
        }
    }

    impl Drop for ScriptedTouch {
        fn drop(&mut self) {
            unsafe {
                ffi::lv_indev_delete(self.indev.as_ptr());
                drop(Box::from_raw(self.queue.as_ptr()));
            }
        }
    }

    struct Harness {
        touch: ScriptedTouch,
        screen: LvObj,
        slider: LvSlider,
        confirmed: Rc<Cell<bool>>,
    }

    fn coords(obj: &impl ObjExt) -> LvArea {
        let mut area = LvArea {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        };
        unsafe { ffi::lv_obj_get_coords(obj.as_ptr(), &mut area) };
        area
    }

    impl Harness {
        fn new() -> Self {
            let touch = ScriptedTouch::new();
            let screen = LvObj::new().unwrap();
            let confirmed = Rc::new(Cell::new(false));
            let confirmed_cb = Rc::clone(&confirmed);
            build_slide_to_confirm(&screen, move || confirmed_cb.set(true));
            unsafe { ffi::lv_screen_load(screen.as_ptr()) };
            pump_for(60); // layout + first render

            let slider = Self::find_slider(&Self::track_of(&screen));
            Self {
                touch,
                screen,
                slider,
                confirmed,
            }
        }

        fn track_of(screen: &LvObj) -> LvObj {
            let root = screen.child(0).expect("component root");
            root.child(1).expect("track")
        }

        /// Locates the slider among the track's children by widget class, so the tests do not
        /// depend on the exact child order.
        fn find_slider(track: &LvObj) -> LvSlider {
            let mut index = 0;
            loop {
                let child = track.child(index).expect("track has a slider child");
                match child.try_downcast::<class::SliderTag>() {
                    Ok(slider) => return slider,
                    Err(_) => index += 1,
                }
            }
        }

        /// The chevrons (index 0..CHEVRON_COUNT) and the target outline (index CHEVRON_COUNT).
        /// Track child 0 is the hidden canvas holding the shared chevron pixels.
        fn marker_hidden(&self, index: i32) -> bool {
            let marker = Self::track_of(&self.screen)
                .child(index + 1)
                .expect("marker");
            unsafe { ffi::lv_obj_has_flag(marker.as_ptr(), LvObjFlag::LV_OBJ_FLAG_HIDDEN) }
        }

        /// Screen coordinates of the knob-arrow center at the current slider value.
        fn knob_center(&self) -> (i32, i32) {
            let area = coords(&self.slider);
            (
                area.x1 + KNOB_INSET + self.slider.get_value(),
                (area.y1 + area.y2) / 2,
            )
        }

        /// Queues a press at `from`, a stepped move to `to`, and optionally a release, WITHOUT
        /// running LVGL; samples are consumed one per refresh period once the caller pumps.
        fn queue_drag(&mut self, from: (i32, i32), to: (i32, i32), release: bool) {
            const STEPS: i32 = 8;
            self.touch.push(from.0, from.1, true);
            for i in 1..=STEPS {
                self.touch.push(
                    from.0 + (to.0 - from.0) * i / STEPS,
                    from.1 + (to.1 - from.1) * i / STEPS,
                    true,
                );
            }
            if release {
                self.touch.push(to.0, to.1, false);
            }
        }

        /// Queues a drag and runs LVGL long enough to consume every queued sample.
        fn drag(&mut self, from: (i32, i32), to: (i32, i32), release: bool) {
            self.queue_drag(from, to, release);
            pump_for(12 * 40);
        }

        /// Queues stepped move samples (no press/release edge) and consumes them.
        fn move_to(&mut self, from: (i32, i32), to: (i32, i32)) {
            const STEPS: i32 = 8;
            for i in 1..=STEPS {
                self.touch.push(
                    from.0 + (to.0 - from.0) * i / STEPS,
                    from.1 + (to.1 - from.1) * i / STEPS,
                    true,
                );
            }
            pump_for(12 * 40);
        }

        fn release_at(&mut self, at: (i32, i32)) {
            self.touch.push(at.0, at.1, false);
            pump_for(120);
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

    #[test]
    fn test_full_slide_confirms() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        let knob = harness.knob_center();
        assert!(!harness.confirmed.get());
        harness.drag(knob, (knob.0 + TRAVEL + 30, knob.1), true);

        assert!(harness.confirmed.get());
        // The knob stays at the far end (no snap-back after a confirm), the mockup's end state.
        assert_eq!(harness.slider.get_value(), TRAVEL);
        for index in 0..=CHEVRON_COUNT {
            assert!(harness.marker_hidden(index), "marker {index} visible");
        }
    }

    #[test]
    fn test_partial_slide_snaps_back() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        let knob = harness.knob_center();
        harness.drag(knob, (knob.0 + 150, knob.1), false);

        assert!(!harness.confirmed.get());
        let value = harness.slider.get_value();
        assert!((140..=160).contains(&value), "unexpected value {value}");
        // The knob's leading edge has passed the first three chevrons.
        assert!(harness.marker_hidden(0));
        assert!(harness.marker_hidden(1));
        assert!(harness.marker_hidden(2));
        assert!(!harness.marker_hidden(3));
        assert!(!harness.marker_hidden(4));
        assert!(!harness.marker_hidden(CHEVRON_COUNT)); // target outline

        harness.release_at((knob.0 + 150, knob.1));
        pump_for(SNAP_BACK_MS as u64 + 100); // let the glide-back animation finish

        assert!(!harness.confirmed.get());
        assert_eq!(harness.slider.get_value(), 0);
        for index in 0..=CHEVRON_COUNT {
            assert!(!harness.marker_hidden(index), "marker {index} hidden");
        }
    }

    #[test]
    fn test_snap_back_reshows_markers_progressively() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        let knob = harness.knob_center();
        harness.drag(knob, (knob.0 + 150, knob.1), false);
        harness.touch.push(knob.0 + 150, knob.1, false);
        // Half-way through the snap-back the knob sits at roughly value 75: the first chevron
        // (hidden at ~35) must still be hidden, while the third (hidden at ~137) has already
        // been passed back over and must be visible again.
        pump_for(SNAP_BACK_MS as u64 / 2);
        assert!(
            harness.marker_hidden(0),
            "first chevron reappeared too soon"
        );
        assert!(!harness.marker_hidden(2), "third chevron still hidden");

        pump_for(SNAP_BACK_MS as u64 + 100);
        assert_eq!(harness.slider.get_value(), 0);
        for index in 0..=CHEVRON_COUNT {
            assert!(!harness.marker_hidden(index), "marker {index} hidden");
        }
    }

    #[test]
    fn test_backward_drag_reshows_markers() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        let knob = harness.knob_center();
        harness.drag(knob, (knob.0 + 200, knob.1), false);
        assert!(harness.marker_hidden(0));
        assert!(harness.marker_hidden(3));

        // Retreat without releasing: the passed markers must reappear.
        harness.move_to((knob.0 + 200, knob.1), (knob.0 + 20, knob.1));
        assert!(!harness.confirmed.get());
        let value = harness.slider.get_value();
        assert!((10..=30).contains(&value), "unexpected value {value}");
        for index in 0..=CHEVRON_COUNT {
            assert!(!harness.marker_hidden(index), "marker {index} hidden");
        }

        harness.release_at((knob.0 + 20, knob.1));
        pump_for(SNAP_BACK_MS as u64 + 100); // let the glide-back animation finish
        assert_eq!(harness.slider.get_value(), 0);
    }

    #[test]
    fn test_tap_on_knob_does_not_confirm() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        let knob = harness.knob_center();
        harness.touch.push(knob.0, knob.1, true);
        pump_for(100);
        harness.release_at(knob);

        assert!(!harness.confirmed.get());
        assert_eq!(harness.slider.get_value(), 0);
    }

    #[test]
    fn test_drag_from_track_does_not_confirm() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        // Start the drag on the track, well right of the knob: with LV_OBJ_FLAG_ADV_HITTEST the
        // press must not grab the slider, so sliding to the end must not confirm.
        let knob = harness.knob_center();
        let start = (knob.0 + 150, knob.1);
        harness.drag(start, (knob.0 + TRAVEL + 30, knob.1), true);

        assert!(!harness.confirmed.get());
        assert_eq!(harness.slider.get_value(), 0);
        for index in 0..=CHEVRON_COUNT {
            assert!(!harness.marker_hidden(index), "marker {index} hidden");
        }
    }

    #[test]
    fn test_fast_flick_confirms() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        // A very fast flick delivers only a few samples with large deltas (just under the
        // per-sample advance cap at a 60 Hz touch controller). It must still confirm.
        let knob = harness.knob_center();
        harness.touch.push(knob.0, knob.1, true);
        for step in [76, 152, 228, 304] {
            harness.touch.push(knob.0 + step, knob.1, true);
        }
        harness.touch.push(knob.0 + 304, knob.1, false);
        pump_for(400);

        assert!(harness.confirmed.get());
        assert_eq!(harness.slider.get_value(), TRAVEL);
    }

    #[test]
    fn test_press_release_jump_does_not_confirm() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        // A press on the knob followed by a release at the far end with no motion samples in
        // between is indistinguishable from a glitched sample and must not confirm. (This is
        // also why the graphical simulator streams cursor motion instead of only reporting the
        // press/release end points.)
        let knob = harness.knob_center();
        harness.touch.push(knob.0, knob.1, true);
        harness.touch.push(knob.0 + TRAVEL + 30, knob.1, false);
        pump_for(300);

        assert!(!harness.confirmed.get());
        assert_eq!(harness.slider.get_value(), 0);
    }

    #[test]
    fn test_single_jump_sample_does_not_confirm() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        // Finger rests on the knob; one glitched sample reports the far end of the track, the
        // next sample is back at the finger. The advance cap must keep this from confirming.
        let knob = harness.knob_center();
        harness.touch.push(knob.0, knob.1, true);
        harness.touch.push(knob.0 + TRAVEL + 30, knob.1, true);
        harness.touch.push(knob.0, knob.1, true);
        harness.touch.push(knob.0, knob.1, false);
        pump_for(300);

        assert!(!harness.confirmed.get());
        assert_eq!(harness.slider.get_value(), 0);
    }

    #[test]
    fn test_off_track_samples_do_not_advance() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        // Finger presses the knob, then the reported coordinates jump to the top-right of the
        // screen (e.g. a second contact) and stay there. Samples that far off the track must not
        // advance the knob, no matter how long they persist.
        let knob = harness.knob_center();
        harness.touch.push(knob.0, knob.1, true);
        harness.touch.push(knob.0 + TRAVEL + 30, knob.1 - 200, true);
        pump_for(600); // the last sample repeats for many refresh periods

        assert!(!harness.confirmed.get());
        assert_eq!(harness.slider.get_value(), 0);

        harness.release_at(knob);
        assert!(!harness.confirmed.get());
    }

    #[test]
    fn test_regrab_during_snap_back_confirms() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        // A short slide is released (snap-back animation starts), and one refresh period later
        // the knob is re-grabbed at its rest position and slid to the end in one motion. The
        // re-grab lands mid-animation; it must cancel the snap-back and the slide must confirm.
        let knob = harness.knob_center();
        harness.queue_drag(knob, (knob.0 + 60, knob.1), true);
        harness.queue_drag(knob, (knob.0 + TRAVEL + 30, knob.1), true);
        pump_for(1000);

        assert!(harness.confirmed.get());
        assert_eq!(harness.slider.get_value(), TRAVEL);
    }

    #[test]
    fn test_press_lost_snaps_back() {
        let _lock = lock_and_init();
        let mut harness = Harness::new();

        let knob = harness.knob_center();
        harness.drag(knob, (knob.0 + 150, knob.1), false);
        let value = harness.slider.get_value();
        assert!((140..=160).contains(&value), "unexpected value {value}");

        // Abandon the press: with wait_until_release set, LVGL delivers LV_EVENT_PRESS_LOST
        // (instead of LV_EVENT_RELEASED) once the touch reports released.
        unsafe { ffi::lv_indev_wait_release(harness.touch.indev.as_ptr()) };
        harness.release_at((knob.0 + 150, knob.1));
        pump_for(SNAP_BACK_MS as u64 + 200);

        assert!(!harness.confirmed.get());
        assert_eq!(harness.slider.get_value(), 0);
        for index in 0..=CHEVRON_COUNT {
            assert!(!harness.marker_hidden(index), "marker {index} hidden");
        }
    }
}
