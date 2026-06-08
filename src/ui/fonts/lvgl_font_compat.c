// SPDX-License-Identifier: Apache-2.0

#include "lvgl.h"

#include <stdbool.h>
#include <string.h>

static bool _font_dsc_supported(const lv_font_fmt_txt_dsc_t* font_dsc)
{
    if (font_dsc == NULL || font_dsc->glyph_bitmap == NULL || font_dsc->glyph_dsc == NULL ||
        (font_dsc->cmap_num != 0 && font_dsc->cmaps == NULL) || font_dsc->bpp != 1 ||
        font_dsc->stride != 0 || font_dsc->bitmap_format != LV_FONT_FMT_TXT_PLAIN ||
        font_dsc->kern_dsc != NULL || font_dsc->kern_scale != 0 || font_dsc->kern_classes != 0) {
        return false;
    }

    for (uint16_t i = 0; i < font_dsc->cmap_num; i++) {
        const lv_font_fmt_txt_cmap_t* cmap = &font_dsc->cmaps[i];
        if (cmap->type != LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY || cmap->unicode_list != NULL ||
            cmap->glyph_id_ofs_list != NULL || cmap->list_length != 0) {
            return false;
        }
    }
    return true;
}

static uint32_t _get_glyph_id(const lv_font_fmt_txt_dsc_t* font_dsc, uint32_t codepoint)
{
    if (codepoint == '\0') {
        return 0;
    }

    for (uint16_t i = 0; i < font_dsc->cmap_num; i++) {
        const lv_font_fmt_txt_cmap_t* cmap = &font_dsc->cmaps[i];
        uint32_t relative_codepoint = codepoint - cmap->range_start;
        if (relative_codepoint < cmap->range_length) {
            return cmap->glyph_id_start + relative_codepoint;
        }
    }
    return 0;
}

__attribute__((weak)) bool lv_font_get_glyph_dsc_fmt_txt(
    const lv_font_t* font,
    lv_font_glyph_dsc_t* dsc_out,
    uint32_t unicode_letter,
    uint32_t unicode_letter_next)
{
    (void)unicode_letter_next;
    if (font == NULL || font->dsc == NULL || dsc_out == NULL) {
        return false;
    }

    bool is_tab = unicode_letter == '\t';
    if (is_tab) {
        unicode_letter = ' ';
    }

    const lv_font_fmt_txt_dsc_t* font_dsc = (const lv_font_fmt_txt_dsc_t*)font->dsc;
    if (!_font_dsc_supported(font_dsc)) {
        return false;
    }

    uint32_t glyph_id = _get_glyph_id(font_dsc, unicode_letter);
    if (glyph_id == 0) {
        return false;
    }

    const lv_font_fmt_txt_glyph_dsc_t* glyph_dsc = &font_dsc->glyph_dsc[glyph_id];

    memset(dsc_out, 0, sizeof(*dsc_out));
    dsc_out->resolved_font = font;
    dsc_out->adv_w = (glyph_dsc->adv_w + (1 << 3)) >> 4;
    dsc_out->box_w = glyph_dsc->box_w;
    dsc_out->box_h = glyph_dsc->box_h;
    dsc_out->ofs_x = glyph_dsc->ofs_x;
    dsc_out->ofs_y = glyph_dsc->ofs_y;
    dsc_out->stride = 0;
    dsc_out->format = LV_FONT_GLYPH_FORMAT_A1;
    dsc_out->is_placeholder = false;
    dsc_out->gid.index = glyph_id;

    if (is_tab) {
        dsc_out->adv_w *= 2;
    }

    return true;
}

__attribute__((weak)) const void* lv_font_get_bitmap_fmt_txt(
    lv_font_glyph_dsc_t* glyph_dsc,
    lv_draw_buf_t* draw_buf)
{
    (void)draw_buf;
    if (glyph_dsc == NULL || glyph_dsc->resolved_font == NULL ||
        glyph_dsc->resolved_font->dsc == NULL || glyph_dsc->gid.index == 0 ||
        glyph_dsc->format != LV_FONT_GLYPH_FORMAT_A1 || glyph_dsc->stride != 0) {
        return NULL;
    }

    const lv_font_fmt_txt_dsc_t* font_dsc =
        (const lv_font_fmt_txt_dsc_t*)glyph_dsc->resolved_font->dsc;
    if (!_font_dsc_supported(font_dsc)) {
        return NULL;
    }

    const lv_font_fmt_txt_glyph_dsc_t* font_glyph_dsc =
        &font_dsc->glyph_dsc[glyph_dsc->gid.index];
    return &font_dsc->glyph_bitmap[font_glyph_dsc->bitmap_index];
}
