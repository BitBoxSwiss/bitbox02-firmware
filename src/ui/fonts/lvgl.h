// SPDX-License-Identifier: Apache-2.0

#ifndef _UI_FONTS_LVGL_H_
#define _UI_FONTS_LVGL_H_

#include <include/lvgl/font/lv_font.h>
#include <include/lvgl/font/lv_font_fmt_txt.h>
#include <include/lvgl/lv_version.h>

#ifndef LV_VERSION_CHECK
    #define LV_VERSION_CHECK(x, y, z) \
        ((x) == LVGL_VERSION_MAJOR && \
         ((y) < LVGL_VERSION_MINOR || ((y) == LVGL_VERSION_MINOR && (z) <= LVGL_VERSION_PATCH)))
#endif

#endif
