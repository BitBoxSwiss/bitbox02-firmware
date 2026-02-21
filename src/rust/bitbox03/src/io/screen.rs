use bitbox_lvgl::{LvAlign, lv_label_create, lv_label_set_text, lv_obj_align, lv_screen_active};
pub fn splash() {
    /* Get the currently active screen */
    let scr = lv_screen_active().expect("get active screen");

    /* Create a label */
    let label = lv_label_create(&scr).expect("create label");

    /* Set the label text */
    lv_label_set_text(&label, "BitBox03\nHello, World!\nFrom LVGL").expect("label set text");

    /* Center it on the screen */
    lv_obj_align(&label, LvAlign::LV_ALIGN_CENTER, 0, 0);
}
