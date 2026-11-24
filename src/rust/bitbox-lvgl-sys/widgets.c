// SPDX-License-Identifier: Apache-2.0

#include <lvgl.h>
#include <src/core/lv_obj_class_private.h>

extern void bitbox_lv_widget_destructor(const lv_obj_class_t* class_p, lv_obj_t* obj);

const lv_obj_class_t bitbox_lv_label_class = {
    .base_class = &lv_obj_class,
    .destructor_cb = bitbox_lv_widget_destructor,
    .width_def = LV_SIZE_CONTENT,
    .height_def = LV_SIZE_CONTENT,
    .name = "bitbox_label",
};

const lv_obj_class_t bitbox_lv_textarea_class = {
    .base_class = &lv_obj_class,
    .destructor_cb = bitbox_lv_widget_destructor,
    .theme_inheritable = LV_OBJ_CLASS_THEME_INHERITABLE_TRUE,
    .name = "bitbox_textarea",
};

const lv_obj_class_t bitbox_lv_keyboard_class = {
    .base_class = &lv_buttonmatrix_class,
    .destructor_cb = bitbox_lv_widget_destructor,
    .theme_inheritable = LV_OBJ_CLASS_THEME_INHERITABLE_TRUE,
    .name = "bitbox_keyboard",
};
