// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;

pub extern crate bitbox_lvgl_sys as ffi;

pub use ffi::lv_align_t as LvAlign;
pub use ffi::lv_anim_enable_t as LvAnimEnable;
pub use ffi::lv_arc_mode_t as LvArcMode;
pub use ffi::lv_area_t as LvArea;
pub use ffi::lv_bar_mode_t as LvBarMode;
pub use ffi::lv_bar_orientation_t as LvBarOrientation;
pub use ffi::lv_base_dir_t as LvBaseDir;
pub use ffi::lv_blend_mode_t as LvBlendMode;
pub use ffi::lv_border_side_t as LvBorderSide;
pub use ffi::lv_buttonmatrix_ctrl_t as LvButtonmatrixCtrl;
pub use ffi::lv_color_format_t as LvColorFormat;
pub use ffi::lv_color_t as LvColor;
pub use ffi::lv_color32_t as LvColor32;
pub use ffi::lv_display_render_mode_t as LvDisplayRenderMode;
pub use ffi::lv_draw_buf_t as LvDrawBuf;
pub use ffi::lv_flex_align_t as LvFlexAlign;
pub use ffi::lv_flex_flow_t as LvFlexFlow;
pub use ffi::lv_grad_dir_t as LvGradDir;
pub use ffi::lv_grid_align_t as LvGridAlign;
pub use ffi::lv_image_align_t as LvImageAlign;
pub use ffi::lv_image_dsc_t as LvImageDsc;
pub use ffi::lv_indev_state_t as LvIndevState;
pub use ffi::lv_indev_type_t as LvIndevType;
pub use ffi::lv_keyboard_mode_t as LvKeyboardMode;
pub use ffi::lv_label_long_mode_t as LvLabelLongMode;
pub use ffi::lv_log_level_t as LvLogLevel;
pub use ffi::lv_opa_t as LvOpa;
pub use ffi::lv_part_t as LvPart;
pub use ffi::lv_point_t as LvPoint;
pub use ffi::lv_slider_mode_t as LvSliderMode;
pub use ffi::lv_slider_orientation_t as LvSliderOrientation;
pub use ffi::lv_style_selector_t as LvStyleSelector;
pub use ffi::lv_text_align_t as LvTextAlign;
pub use ffi::lv_text_decor_t as LvTextDecor;
pub use ffi::lv_value_precise_t as LvValuePrecise;

pub mod color;
pub mod display;
pub mod indev;
pub mod log;
pub mod system;
pub mod tick;
pub mod timer;
mod util;
pub mod widgets;

pub use display::{LvDisplay, LvDisplayBufferError};
pub use indev::LvIndev;
pub use widgets::arc::{ArcExt, LvArc};
pub use widgets::bar::{BarExt, LvBar};
pub use widgets::button::{ButtonExt, LvButton};
pub use widgets::buttonmatrix::{ButtonmatrixExt, LvButtonmatrix, LvButtonmatrixMapEntry};
pub use widgets::canvas::CanvasExt;
pub use widgets::canvas::{LvCanvas, LvCanvasCreateError};
pub use widgets::class;
pub use widgets::image::{ImageExt, LvImage, LvImageSourceError};
pub use widgets::keyboard::{KeyboardExt, LvKeyboard, LvKeyboardMapEntry, keyboard_def_event_cb};
pub use widgets::label::{LabelExt, LvLabel, LvLabelTextError};
pub use widgets::obj;
pub use widgets::obj::LvObj;
pub use widgets::obj::{LvHandle, LvTypeError, ObjExt};
pub use widgets::slider::{LvSlider, SliderExt};
pub use widgets::spinner::{LvSpinner, SpinnerExt};
pub use widgets::textarea::{LvTextarea, LvTextareaTextError, TextareaExt};
pub use widgets::{LvMapError, LvTextError};
