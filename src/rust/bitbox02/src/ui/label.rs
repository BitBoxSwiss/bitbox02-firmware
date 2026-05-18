// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use core::ffi::{CStr, c_char, c_void};
use zeroize::Zeroize;

use super::types::MAX_LABEL_SIZE;

use bitbox02_sys as ffi;

const ELLIPSIS: &[u8; 3] = b"...";
const TEXT_LEN: usize = MAX_LABEL_SIZE + ELLIPSIS.len() + 1;

#[repr(C)]
struct LabelData {
    text: [u8; TEXT_LEN],
    font: *const ffi::UG_FONT,
    upside_down: bool,
    position: ffi::screen_position_t,
    scrollable: bool,
    slider_is_touched: bool,
    slider_was_touched: bool,
    slider_position: u16,
    slider_position_diff: f32,
    text_position: i16,
    text_position_last: i16,
    xoffset: u8,
    yoffset: u8,
}

impl LabelData {
    fn new(
        font: *const ffi::UG_FONT,
        upside_down: bool,
        position: ffi::screen_position_t,
        xoffset: u8,
        yoffset: u8,
        scrollable: bool,
    ) -> Self {
        Self {
            text: [0; TEXT_LEN],
            font,
            upside_down,
            position,
            scrollable,
            slider_is_touched: false,
            slider_was_touched: false,
            slider_position: 0,
            slider_position_diff: 0.,
            text_position: 0,
            text_position_last: 0,
            xoffset,
            yoffset,
        }
    }

    fn text_ptr(&self) -> *const c_char {
        self.text.as_ptr().cast()
    }
}

static COMPONENT_FUNCTIONS: ffi::component_functions_t = ffi::component_functions_t {
    cleanup: Some(cleanup),
    render: Some(render),
    on_event: Some(on_event),
};

fn copy_truncated_text(text: &[u8], out: &mut [u8; TEXT_LEN]) {
    out.fill(0);
    let copied_len = utf8_truncate_len(text, MAX_LABEL_SIZE);
    out[..copied_len].copy_from_slice(&text[..copied_len]);
    if text.len() > MAX_LABEL_SIZE {
        out[copied_len..copied_len + ELLIPSIS.len()].copy_from_slice(ELLIPSIS);
    }
}

fn utf8_truncate_len(text: &[u8], max_len: usize) -> usize {
    if text.len() <= max_len {
        return text.len();
    }
    match core::str::from_utf8(&text[..max_len]) {
        Ok(_) => max_len,
        Err(err) => err.valid_up_to(),
    }
}

unsafe fn label_data_mut<'a>(component: *mut ffi::component_t) -> &'a mut LabelData {
    assert!(!component.is_null());
    let data = unsafe { (*component).data };
    assert!(!data.is_null());
    unsafe { &mut *data.cast::<LabelData>() }
}

fn is_centered_position(position: ffi::screen_position_t) -> bool {
    matches!(
        position,
        ffi::screen_position_t_CENTER
            | ffi::screen_position_t_CENTER_TOP
            | ffi::screen_position_t_CENTER_BOTTOM
    )
}

unsafe fn position_left(parent: *const ffi::component_t, child: *mut ffi::component_t) {
    unsafe {
        (*child).position.left = (*parent).position.left;
        if (*child).position.left == 0 {
            (*child).position.left = 1;
        }
    }
}

unsafe fn position_right(parent: *const ffi::component_t, child: *mut ffi::component_t) {
    unsafe {
        (*child).position.left =
            (*parent).position.left + (*parent).dimension.width - (*child).dimension.width;
        if (*parent).position.left + (*parent).dimension.width == ffi::SCREEN_WIDTH as i16 {
            (*child).position.left -= 1;
        }
    }
}

unsafe fn position_label(component: *mut ffi::component_t, position: ffi::screen_position_t) {
    assert!(!component.is_null());
    let parent = unsafe { (*component).parent };
    assert!(!parent.is_null());

    unsafe {
        match position {
            ffi::screen_position_t_CENTER => {
                (*component).position.top = (*parent).position.top + (*parent).dimension.height / 2
                    - (*component).dimension.height / 2;
                (*component).position.left = (*parent).position.left
                    + (*parent).dimension.width / 2
                    - (*component).dimension.width / 2;
            }
            ffi::screen_position_t_CENTER_TOP => {
                (*component).position.top = (*parent).position.top;
                (*component).position.left = (*parent).position.left
                    + (*parent).dimension.width / 2
                    - (*component).dimension.width / 2;
            }
            ffi::screen_position_t_CENTER_BOTTOM => {
                (*component).position.top = (*parent).position.top + (*parent).dimension.height
                    - (*component).dimension.height;
                (*component).position.left = (*parent).position.left
                    + (*parent).dimension.width / 2
                    - (*component).dimension.width / 2;
            }
            ffi::screen_position_t_LEFT_TOP => {
                (*component).position.top = (*parent).position.top;
                position_left(parent, component);
            }
            ffi::screen_position_t_LEFT_BOTTOM => {
                (*component).position.top = (*parent).position.top + (*parent).dimension.height
                    - (*component).dimension.height;
                position_left(parent, component);
            }
            ffi::screen_position_t_LEFT_CENTER => {
                (*component).position.top = (*parent).position.top + (*parent).dimension.height / 2
                    - (*component).dimension.height / 2;
                position_left(parent, component);
            }
            ffi::screen_position_t_RIGHT_CENTER => {
                (*component).position.top = (*parent).position.top + (*parent).dimension.height / 2
                    - (*component).dimension.height / 2;
                position_right(parent, component);
            }
            ffi::screen_position_t_RIGHT_TOP => {
                (*component).position.top = (*parent).position.top;
                position_right(parent, component);
            }
            ffi::screen_position_t_RIGHT_BOTTOM => {
                (*component).position.top = (*parent).position.top + (*parent).dimension.height
                    - (*component).dimension.height;
                position_right(parent, component);
            }
            _ => panic!("position undefined or currently not implemented"),
        }
    }
}

unsafe fn render_subcomponents(component: *mut ffi::component_t) {
    assert!(!component.is_null());
    unsafe {
        for i in 0..(*component).sub_components.amount as usize {
            let sub_component = (*component).sub_components.sub_components[i];
            if !(*sub_component).disabled {
                match (*(*sub_component).f).render {
                    Some(render) => render(sub_component),
                    None => panic!("missing component render function"),
                }
            }
        }
    }
}

unsafe fn cleanup_subcomponents(component: *mut ffi::component_t) {
    assert!(!component.is_null());
    unsafe {
        for i in 0..(*component).sub_components.amount as usize {
            let sub_component = (*component).sub_components.sub_components[i];
            match (*(*sub_component).f).cleanup {
                Some(cleanup) => cleanup(sub_component),
                None => panic!("missing component cleanup function"),
            }
        }
    }
}

unsafe fn add_sub_component(parent: *mut ffi::component_t, child: *mut ffi::component_t) {
    assert!(!parent.is_null());
    assert!(!child.is_null());
    unsafe {
        let index = (*parent).sub_components.amount as usize;
        if index + 1 >= (*parent).sub_components.sub_components.len() {
            panic!("Not enough memory to add sub component");
        }
        (*parent).sub_components.sub_components[index] = child;
        (*parent).sub_components.amount += 1;
        (*child).parent = parent;
    }
}

unsafe fn measure_label_dimensions(component: *mut ffi::component_t) {
    assert!(!component.is_null());
    unsafe {
        ffi::UG_FontSetVSpace(2);
        let data = label_data_mut(component);
        ffi::UG_FontSelect(data.font);
        if data.scrollable {
            ffi::UG_MeasureStringNoBreak(
                &mut (*component).dimension.width,
                &mut (*component).dimension.height,
                data.text_ptr(),
            );
            if (*component).dimension.width < ffi::SCREEN_WIDTH as i16 {
                data.scrollable = false;
            }
        } else if is_centered_position(data.position) {
            ffi::UG_MeasureStringCentered(
                &mut (*component).dimension.width,
                &mut (*component).dimension.height,
                data.text_ptr(),
            );
        } else {
            ffi::UG_MeasureString(
                &mut (*component).dimension.width,
                &mut (*component).dimension.height,
                data.text_ptr(),
            );
        }
        ffi::UG_FontSetVSpace(0);
    }
}

/// cbindgen:ignore
#[unsafe(no_mangle)]
pub unsafe extern "C" fn label_update(component: *mut ffi::component_t, text: *const c_char) {
    assert!(!text.is_null());
    unsafe {
        let text = CStr::from_ptr(text).to_bytes();
        copy_truncated_text(text, &mut label_data_mut(component).text);
        measure_label_dimensions(component);

        if (*component).parent.is_null() {
            return;
        }
        let data = label_data_mut(component);
        let position = data.position;
        let xoffset = data.xoffset;
        let yoffset = data.yoffset;
        position_label(component, position);
        (*component).position.top += i16::from(yoffset);
        (*component).position.left += i16::from(xoffset);
    }
}

unsafe extern "C" fn render(component: *mut ffi::component_t) {
    assert!(!component.is_null());
    unsafe {
        let data = label_data_mut(component);

        if data.scrollable {
            let x = i32::from(data.slider_position) * (ffi::SCREEN_WIDTH as i32 - 1)
                / ffi::MAX_SLIDER_POS as i32;
            let y = ffi::SCREEN_HEIGHT as i16 - 1;
            if !data.slider_was_touched {
                data.text_position =
                    (*component).dimension.width / 2 + ffi::SCREEN_WIDTH as i16 / 6;
                data.text_position_last = data.text_position;
                render_subcomponents(component);
            } else if data.slider_is_touched {
                ffi::UG_DrawLine(
                    (ffi::SCREEN_WIDTH as i32).min(x + 3) as i16,
                    y,
                    0.max(x - 3) as i16,
                    y,
                    ffi::screen_front_color,
                );
            }
        }

        ffi::UG_FontSetVSpace(2);
        ffi::UG_FontSelect(data.font);
        if data.scrollable {
            ffi::UG_PutStringNoBreak(
                data.text_position - (*component).dimension.width / 2,
                (*component).position.top,
                data.text_ptr(),
                data.upside_down,
            );
        } else if is_centered_position(data.position) {
            ffi::UG_PutStringCentered(
                (*component).position.left + data.text_position,
                (*component).position.top,
                (*component).dimension.width,
                (*component).dimension.height,
                data.text_ptr(),
                data.upside_down,
            );
        } else {
            ffi::UG_PutString(
                (*component).position.left + data.text_position,
                (*component).position.top,
                data.text_ptr(),
                data.upside_down,
            );
        }
        ffi::UG_FontSetVSpace(0);
    }
}

fn sigmoid(velocity: i32) -> f32 {
    let velocity_f = velocity as f32;
    0.0018_f32 * velocity_f * velocity.unsigned_abs() as f32
        / (1. + 0.002_f32 * velocity_f * velocity_f)
}

unsafe extern "C" fn on_event(event: *const ffi::event_t, component: *mut ffi::component_t) {
    assert!(!event.is_null());
    unsafe {
        if (*event).data.source != ffi::bottom_slider {
            return;
        }
        let data = label_data_mut(component);
        if !data.scrollable {
            return;
        }
        match (*event).id {
            id if id == ffi::event_types::EVENT_SLIDE as u8 => {
                let margin = ffi::SCREEN_WIDTH as i16 / 5;
                data.slider_position_diff += sigmoid((*event).data.velocity);
                data.text_position = data.text_position_last + data.slider_position_diff as i16;
                data.text_position = (-(margin) - (*component).dimension.width / 2
                    + ffi::SCREEN_WIDTH as i16)
                    .max(data.text_position)
                    .min((*component).dimension.width / 2 + margin);
                data.slider_position = (*event).data.position;
                data.slider_is_touched = true;
                data.slider_was_touched = true;
            }
            id if id == ffi::event_types::EVENT_SLIDE_RELEASED as u8 => {
                data.text_position_last = data.text_position;
                data.slider_position_diff = 0.;
                data.slider_is_touched = false;
            }
            id if id == ffi::event_types::EVENT_CONTINUOUS_TAP as u8 => {
                data.slider_position = (*event).data.position;
                data.slider_is_touched = true;
                data.slider_was_touched = true;
            }
            _ => {}
        }
    }
}

unsafe extern "C" fn cleanup(component: *mut ffi::component_t) {
    assert!(!component.is_null());
    unsafe {
        let data = label_data_mut(component);
        data.text.zeroize();
        cleanup_subcomponents(component);
        drop(Box::from_raw(data));
        drop(Box::from_raw(component));
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn label_create_internal(
    text: *const c_char,
    upside_down: bool,
    font: *const ffi::UG_FONT,
    position: ffi::screen_position_t,
    xoffset: u8,
    yoffset: u8,
    scrollable: bool,
    parent: *mut ffi::component_t,
) -> *mut ffi::component_t {
    assert!(!text.is_null());
    let font = if font.is_null() {
        unsafe { &ffi::font_font_a_11X10 }
    } else {
        font
    };
    let data = Box::into_raw(Box::new(LabelData::new(
        font,
        upside_down,
        position,
        xoffset,
        yoffset,
        scrollable,
    )));
    let label = Box::into_raw(Box::new(ffi::component_t {
        f: &COMPONENT_FUNCTIONS,
        data: data.cast::<c_void>(),
        parent,
        ..ffi::component_t::default()
    }));

    unsafe {
        if scrollable {
            add_sub_component(
                label,
                ffi::knight_rider_create(label, (ffi::SCREEN_HEIGHT - 1) as u8),
            );
        }
        label_update(label, text);
    }
    label
}

/// cbindgen:ignore
#[unsafe(no_mangle)]
pub unsafe extern "C" fn label_create(
    text: *const c_char,
    font: *const ffi::UG_FONT,
    position: ffi::screen_position_t,
    parent: *mut ffi::component_t,
) -> *mut ffi::component_t {
    unsafe { label_create_internal(text, false, font, position, 0, 0, false, parent) }
}

/// cbindgen:ignore
#[unsafe(no_mangle)]
pub unsafe extern "C" fn label_create_offset(
    text: *const c_char,
    font: *const ffi::UG_FONT,
    position: ffi::screen_position_t,
    xoffset: u8,
    yoffset: u8,
    parent: *mut ffi::component_t,
) -> *mut ffi::component_t {
    unsafe { label_create_internal(text, false, font, position, xoffset, yoffset, false, parent) }
}

/// cbindgen:ignore
#[unsafe(no_mangle)]
pub unsafe extern "C" fn label_create_scrollable(
    text: *const c_char,
    font: *const ffi::UG_FONT,
    position: ffi::screen_position_t,
    parent: *mut ffi::component_t,
) -> *mut ffi::component_t {
    unsafe { label_create_internal(text, false, font, position, 0, 0, true, parent) }
}

/// cbindgen:ignore
#[unsafe(no_mangle)]
pub unsafe extern "C" fn label_create_scrollable_offset(
    text: *const c_char,
    font: *const ffi::UG_FONT,
    position: ffi::screen_position_t,
    xoffset: u8,
    yoffset: u8,
    parent: *mut ffi::component_t,
) -> *mut ffi::component_t {
    unsafe { label_create_internal(text, false, font, position, xoffset, yoffset, true, parent) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_truncated_text() {
        let mut out = [0; TEXT_LEN];
        let input = [b'a'; MAX_LABEL_SIZE + 1];

        copy_truncated_text(&input, &mut out);

        assert_eq!(&out[..MAX_LABEL_SIZE], &[b'a'; MAX_LABEL_SIZE]);
        assert_eq!(&out[MAX_LABEL_SIZE..MAX_LABEL_SIZE + 3], b"...");
        assert_eq!(out[MAX_LABEL_SIZE + 3], 0);
    }

    #[test]
    fn test_copy_truncated_text_boundary() {
        let mut out = [0xff; TEXT_LEN];
        let input = [b'a'; MAX_LABEL_SIZE];

        copy_truncated_text(&input, &mut out);

        assert_eq!(&out[..MAX_LABEL_SIZE], &[b'a'; MAX_LABEL_SIZE]);
        assert_eq!(out[MAX_LABEL_SIZE], 0);
        assert_eq!(out[MAX_LABEL_SIZE + 1], 0);
    }

    #[test]
    fn test_copy_truncated_text_utf8_boundary() {
        let mut out = [0; TEXT_LEN];
        let mut input = [b'a'; MAX_LABEL_SIZE + 2];
        input[MAX_LABEL_SIZE - 1] = 0xc3;
        input[MAX_LABEL_SIZE] = 0xa9;
        input[MAX_LABEL_SIZE + 1] = b'x';

        copy_truncated_text(&input, &mut out);

        assert_eq!(&out[..MAX_LABEL_SIZE - 1], &[b'a'; MAX_LABEL_SIZE - 1]);
        assert_eq!(
            &out[MAX_LABEL_SIZE - 1..MAX_LABEL_SIZE - 1 + ELLIPSIS.len()],
            ELLIPSIS
        );
        assert_eq!(out[MAX_LABEL_SIZE - 1 + ELLIPSIS.len()], 0);
        assert!(CStr::from_bytes_until_nul(&out).unwrap().to_str().is_ok());
    }

    #[test]
    fn test_copy_truncated_text_utf8_boundary_exact() {
        let mut out = [0; TEXT_LEN];
        let mut input = [b'a'; MAX_LABEL_SIZE + 1];
        input[MAX_LABEL_SIZE - 2] = 0xc3;
        input[MAX_LABEL_SIZE - 1] = 0xa9;
        input[MAX_LABEL_SIZE] = b'x';

        copy_truncated_text(&input, &mut out);

        assert_eq!(&out[..MAX_LABEL_SIZE - 2], &[b'a'; MAX_LABEL_SIZE - 2]);
        assert_eq!(&out[MAX_LABEL_SIZE - 2..MAX_LABEL_SIZE], "é".as_bytes());
        assert_eq!(
            &out[MAX_LABEL_SIZE..MAX_LABEL_SIZE + ELLIPSIS.len()],
            ELLIPSIS
        );
        assert_eq!(out[MAX_LABEL_SIZE + ELLIPSIS.len()], 0);
        assert!(CStr::from_bytes_until_nul(&out).unwrap().to_str().is_ok());
    }

    #[test]
    fn test_sigmoid() {
        assert_eq!(sigmoid(0), 0.);
        assert!(sigmoid(10) > 0.);
        assert!(sigmoid(-10) < 0.);
        assert_eq!(sigmoid(10), -sigmoid(-10));
    }
}
