use bitbox_lvgl::{
    LV_PART_MAIN, LvAlign, lv_color_hex, lv_label_create, lv_label_set_text, lv_obj_align,
    lv_obj_set_style_bg_color, lv_obj_set_style_text_color, lv_screen_active,lv_canvas_create_from_slice
};

use tracing::info;

const SPLASH: &[u8; 952] = include_bytes!("../../splash.png");

pub fn splash() {
    /* Get the currently active screen */
    let scr = lv_screen_active().expect("get active screen");

    let (header, splash) = png_decoder::decode(SPLASH).expect("valid png");
    info!("loader {header:?}");
    let img = lv_canvas_create_from_slice(&scr, splash, header.width, header.height).expect("load png");

    lv_obj_set_style_bg_color(&scr, lv_color_hex(0x000000), LV_PART_MAIN as u32);
    //lv_obj_set_style_text_color(&scr, lv_color_hex(0xffffff), LV_PART_MAIN as u32);

    /* Create a label */
    //let label = lv_label_create(&scr).expect("create label");

    /* Set the label text */
    //lv_label_set_text(&label, "BitBox03\nHello, World!\nFrom LVGL").expect("label set text");

    /* Center it on the screen */
    //lv_obj_align(&label, LvAlign::LV_ALIGN_CENTER, 0, 0);
    lv_obj_align(&img, LvAlign::LV_ALIGN_CENTER, 0, 0);
}
