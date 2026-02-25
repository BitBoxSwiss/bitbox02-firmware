use bitbox_lvgl::{
    LV_PART_MAIN, LvAlign, lv_canvas_create_from_slice, lv_color_hex, lv_obj_align,
    lv_obj_set_size, lv_obj_set_style_bg_color, lv_screen_active, lv_spinner_create, lv_spinner_set_anim_params,
};

use tracing::info;

const SPLASH: &[u8; 952] = include_bytes!("../../splash.png");

pub fn splash() {
    // Get the currently active screen.
    let scr = lv_screen_active().expect("get active screen");

    let (header, splash) = png_decoder::decode(SPLASH).expect("valid png");
    info!("loader {header:?}");
    let img =
        lv_canvas_create_from_slice(&scr, splash, header.width, header.height).expect("load png");
    let spinner = lv_spinner_create(&scr).expect("create spinner");
    lv_spinner_set_anim_params(&spinner, 3000 /* duration */, 240 /*angle*/);

    lv_obj_set_style_bg_color(&scr, lv_color_hex(0x000000), LV_PART_MAIN as u32);
    lv_obj_align(&img, LvAlign::LV_ALIGN_CENTER, 0, 0);
    lv_obj_set_size(&spinner, 128, 128);
    lv_obj_align(&spinner, LvAlign::LV_ALIGN_BOTTOM_MID, 0, -100);
}
