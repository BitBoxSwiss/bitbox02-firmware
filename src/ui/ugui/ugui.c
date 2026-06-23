/* -------------------------------------------------------------------------------- */
/* -- µGUI - Generic GUI module (C)Achim Döbler, 2015                            -- */
/* -------------------------------------------------------------------------------- */
// µGUI is a generic GUI module for embedded systems.
// This is a free software that is open for education, research and commercial
// developments under license policy of following terms.
//
//  Copyright (C) 2015, Achim Döbler, all rights reserved.
//  URL: http://www.embeddedlightning.com/
//
// * The µGUI module is a free software and there is NO WARRANTY.
// * No restriction on use. You can use, modify and redistribute it for
//   personal, non-profit or commercial products UNDER YOUR RESPONSIBILITY.
// * Redistributions of source code must retain the above copyright notice.
//
/* -------------------------------------------------------------------------------- */
/* -- MY SPECIAL THANKS GO TO                                                    -- */
/* -------------------------------------------------------------------------------- */
// Andrey Filimonov (-->https://github.com/Sermus)
// for giving valuable suggestions, reporting bugs and adding several new features.
// Andrey also put a lot of work in the implementaion of anti-aliased font support.
//
// Mikhail Podkur (-->https://github.com/MikhailPodkur)
// for adding cyrillic 8x12 font, checkbox feature and RGB565 support.
//
// Gustavo Denardin
// for giving valuable suggestions regarding real-time os support.
//
// Samuel Kleiser
// for reporting bugs and giving examples how to improve µGUI.
/* -------------------------------------------------------------------------------- */
/* -- REVISION HISTORY                                                           -- */
/* -------------------------------------------------------------------------------- */
//  Dec 20, 2015  V0.31 Checkbox component with all funtions added.
//                      Cyrillic font 8x12 added.
//                      RGB565 color schema added.
//                      Windows components font could be getted from current GUI by default
//  Mar 18, 2015  V0.3  Driver support added.
//                      Window and object support added.
//                      Touch support added.
//                      Fixed some minor bugs.
//
//  Oct 20, 2014  V0.2  Function UG_DrawRoundFrame() added.
//                      Function UG_FillRoundFrame() added.
//                      Function UG_DrawArc() added.
//                      Fixed some minor bugs.
//
//  Oct 11, 2014  V0.1  First release.
/* -------------------------------------------------------------------------------- */
// SPDX-License-Identifier: Apache-2.0

#include <stdbool.h>
#include <string.h>
#include <ui/oled/oled.h>
#include <util.h>
#include <utils_assert.h>
#include "ugui.h"

/* Pointer to the gui */
static UG_GUI *gui = NULL;

typedef struct {
    bool active;
    UG_S16 x;
    UG_S16 y;
    UG_S16 width;
    UG_S16 height;
} ug_rotation_t;

static ug_rotation_t rotation = {0};

static void _copy_slice(char* out, size_t out_len, const char* start, size_t len)
{
    if (out_len == 0) {
        return;
    }
    const size_t copy_len = MIN(len, out_len - 1);
    memcpy(out, start, copy_len);
    out[copy_len] = '\0';
}

static void _UG_PSet(UG_S16 x, UG_S16 y, UG_COLOR c)
{
    ASSERT(gui != NULL);

    if (rotation.active) {
        x = rotation.x + rotation.width - 1 - (x - rotation.x);
        y = rotation.y + rotation.height - 1 - (y - rotation.y);
    }
    gui->pset(x, y, c);
}

static bool _UG_IsUtf8Continuation(uint8_t byte)
{
    return (byte & 0xC0) == 0x80;
}

/**
 * Decodes the next UTF-8 codepoint and returns the byte after it.
 *
 * The leading byte determines the sequence length. ASCII bytes are returned
 * directly, valid 2-, 3-, and 4-byte sequences are assembled from their payload
 * bits, and malformed sequences fall back to returning the leading byte so
 * iteration always advances.
 */
static const char* _UG_NextCodepoint(const char* str, uint32_t* codepoint)
{
    const uint8_t* bytes = (const uint8_t*)str;
    if (bytes[0] == '\0') {
        *codepoint = '\0';
        return str;
    }
    if (bytes[0] < 0x80) {
        *codepoint = bytes[0];
        return str + 1;
    }
    if (bytes[0] >= 0xC2 && bytes[0] <= 0xDF && _UG_IsUtf8Continuation(bytes[1])) {
        *codepoint = ((uint32_t)(bytes[0] & 0x1F) << 6) | (uint32_t)(bytes[1] & 0x3F);
        return str + 2;
    }
    if (bytes[0] >= 0xE0 && bytes[0] <= 0xEF && _UG_IsUtf8Continuation(bytes[1]) &&
        _UG_IsUtf8Continuation(bytes[2])) {
        *codepoint = ((uint32_t)(bytes[0] & 0x0F) << 12) |
                     ((uint32_t)(bytes[1] & 0x3F) << 6) | (uint32_t)(bytes[2] & 0x3F);
        return str + 3;
    }
    if (bytes[0] >= 0xF0 && bytes[0] <= 0xF4 && _UG_IsUtf8Continuation(bytes[1]) &&
        _UG_IsUtf8Continuation(bytes[2]) && _UG_IsUtf8Continuation(bytes[3])) {
        *codepoint = ((uint32_t)(bytes[0] & 0x07) << 18) |
                     ((uint32_t)(bytes[1] & 0x3F) << 12) |
                     ((uint32_t)(bytes[2] & 0x3F) << 6) | (uint32_t)(bytes[3] & 0x3F);
        return str + 4;
    }
    *codepoint = bytes[0];
    return str + 1;
}

static bool _UG_GetGlyph(const UG_FONT* font, uint32_t codepoint, lv_font_glyph_dsc_t* glyph_dsc)
{
    for (const lv_font_t* f = font; f != NULL; f = f->fallback) {
        if (f->get_glyph_dsc == NULL) {
            continue;
        }
        if (f->get_glyph_dsc(f, glyph_dsc, codepoint, 0)) {
            glyph_dsc->resolved_font = f;
            return true;
        }
    }
    return false;
}

bool UG_GetCharWidth(const UG_FONT* font, uint32_t codepoint, UG_U16* width)
{
    lv_font_glyph_dsc_t glyph_dsc;
    if (!_UG_GetGlyph(font, codepoint, &glyph_dsc)) {
        return false;
    }
    *width = glyph_dsc.adv_w;
    return true;
}

static bool _UG_GlyphPixelSet(
    const uint8_t* bitmap,
    const lv_font_glyph_dsc_t* glyph_dsc,
    UG_U16 x,
    UG_U16 y)
{
    uint32_t bit_index = (uint32_t)y * glyph_dsc->box_w + x;
    return ((bitmap[bit_index / 8] >> (7 - (bit_index % 8))) & 0x01) != 0;
}

static void _UG_PutCodepoint( uint32_t codepoint, UG_S16 x, UG_S16 y, UG_COLOR fc, UG_COLOR bc,
                              const UG_FONT *font, bool inverted, bool transparent)
{
    lv_font_glyph_dsc_t glyph_dsc;
    if (!_UG_GetGlyph(font, codepoint, &glyph_dsc)) {
        return;
    }
    const lv_font_t* resolved_font = glyph_dsc.resolved_font;
    if (resolved_font == NULL || glyph_dsc.format != LV_FONT_GLYPH_FORMAT_A1 ||
        glyph_dsc.stride != 0) {
        return;
    }

    if (x + glyph_dsc.adv_w < 0 || x > gui->x_dim) {
        return;
    }

    if (!transparent) {
        for (UG_S16 row = 0; row < font->line_height; row++) {
            for (UG_U16 col = 0; col < glyph_dsc.adv_w; col++) {
                _UG_PSet(x + col, y + row, bc);
            }
        }
    }
    if (codepoint == '\t' || glyph_dsc.box_w == 0 || glyph_dsc.box_h == 0 ||
        resolved_font->get_glyph_bitmap == NULL) {
        return;
    }

    const uint8_t* bitmap = (const uint8_t*)resolved_font->get_glyph_bitmap(&glyph_dsc, NULL);
    if (bitmap == NULL) {
        return;
    }

    UG_S16 glyph_y =
        font->line_height - resolved_font->base_line - glyph_dsc.box_h - glyph_dsc.ofs_y;
    for (UG_U16 row = 0; row < glyph_dsc.box_h; row++) {
        for (UG_U16 col = 0; col < glyph_dsc.box_w; col++) {
            UG_S16 xo = x + glyph_dsc.ofs_x + col;
            UG_S16 yo = y + glyph_y + row;
            if (inverted) {
                xo = x + glyph_dsc.adv_w - 1 - (glyph_dsc.ofs_x + col);
                yo = y + font->line_height - 1 - (glyph_y + row);
            }
            if (_UG_GlyphPixelSet(bitmap, &glyph_dsc, col, row)) {
                _UG_PSet(xo, yo, fc);
            }
        }
    }
}

static void _UG_PutString( UG_S16 x, UG_S16 y, UG_S16 *xout, UG_S16 *yout, const char *str,
                    int autobreak, int calconly )
{
    ASSERT(gui != NULL);

    UG_S16 xp, yp;
    UG_U16 cw;
    uint32_t codepoint;
    UG_S16 max_x = x;

    xp = x;
    yp = y;

    const char* cursor = str;

    while (*cursor != '\0') {
        cursor = _UG_NextCodepoint(cursor, &codepoint);
        if (codepoint != '\n' && !UG_GetCharWidth(gui->font, codepoint, &cw)) {
            continue;
        }
        if ( codepoint == '\n' ) {
            if (autobreak == 1) {
                xp = gui->x_dim;
            } else {
                xp = x;
                yp += gui->font->line_height + gui->char_v_space;
            }
            continue;
        }

        if ( autobreak == 1 && xp + cw > gui->x_dim - 1 ) {
            xp = x;
            yp += gui->font->line_height + gui->char_v_space;
        }

        if (!calconly) {
            _UG_PutCodepoint(
                codepoint, xp, yp, gui->fore_color, gui->back_color, gui->font, false, false);
        }

        xp += cw + gui->char_h_space;
        if (xp > max_x) {
            max_x = xp;
        }
    }
    if (xout) {
        *xout = max_x;
    }
    if (yout) {
        *yout = yp + gui->font->line_height;
    }
}

UG_S16 UG_Init( UG_GUI *g, void (*p)(UG_S16, UG_S16, UG_COLOR),
                const UG_FONT *font, UG_S16 x, UG_S16 y )
{
    ASSERT(g != NULL);
    ASSERT(p != NULL);
    ASSERT(font != NULL);

    g->pset = p;
    g->x_dim = x;
    g->y_dim = y;
    g->font = font;
    g->char_h_space = 1;
    g->char_v_space = 1;
    g->fore_color = C_WHITE;
    g->back_color = C_BLACK;
    rotation.active = false;

    gui = g;
    return 1;
}

void UG_FontSelect( const UG_FONT *font )
{
    ASSERT(gui != NULL);
    ASSERT(font != NULL);

    gui->font = font;
}

void UG_FillScreen( UG_COLOR c )
{
    ASSERT(gui != NULL);

    UG_FillFrame(0, 0, gui->x_dim - 1, gui->y_dim - 1, c);
}

void UG_FillFrame( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_COLOR c )
{
    ASSERT(gui != NULL);

    UG_S16 n, m;

    if ( x2 < x1 ) {
        n = x2;
        x2 = x1;
        x1 = n;
    }
    if ( y2 < y1 ) {
        n = y2;
        y2 = y1;
        y1 = n;
    }

    for ( m = y1; m <= y2; m++ ) {
        for ( n = x1; n <= x2; n++ ) {
            _UG_PSet(n, m, c);
        }
    }
}

void UG_FillRoundFrame( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_S16 r, UG_COLOR c )
{
    ASSERT(gui != NULL);

    UG_S16  x, y, xd;

    if ( x2 < x1 ) {
        x = x2;
        x2 = x1;
        x1 = x;
    }
    if ( y2 < y1 ) {
        y = y2;
        y2 = y1;
        y1 = y;
    }

    if ( r <= 0 ) {
        return;
    }

    xd = 3 - (r << 1);
    x = 0;
    y = r;

    UG_FillFrame(x1 + r, y1, x2 - r, y2, c);

    while ( x <= y ) {
        if ( y > 0 ) {
            UG_DrawLine(x2 + x - r, y1 - y + r, x2 + x - r, y + y2 - r, c);
            UG_DrawLine(x1 - x + r, y1 - y + r, x1 - x + r, y + y2 - r, c);
        }
        if ( x > 0 ) {
            UG_DrawLine(x1 - y + r, y1 - x + r, x1 - y + r, x + y2 - r, c);
            UG_DrawLine(x2 + y - r, y1 - x + r, x2 + y - r, x + y2 - r, c);
        }
        if ( xd < 0 ) {
            xd += (x << 2) + 6;
        } else {
            xd += ((x - y) << 2) + 10;
            y--;
        }
        x++;
    }
}

void UG_DrawFrame( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_COLOR c )
{
    UG_DrawLine(x1, y1, x2, y1, c);
    UG_DrawLine(x1, y2, x2, y2, c);
    UG_DrawLine(x1, y1, x1, y2, c);
    UG_DrawLine(x2, y1, x2, y2, c);
}

void UG_DrawRoundFrame( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_S16 r, UG_COLOR c )
{
    UG_S16 n;
    if ( x2 < x1 ) {
        n = x2;
        x2 = x1;
        x1 = n;
    }
    if ( y2 < y1 ) {
        n = y2;
        y2 = y1;
        y1 = n;
    }

    if ( r > x2 ) {
        return;
    }
    if ( r > y2 ) {
        return;
    }

    UG_DrawLine(x1 + r, y1, x2 - r, y1, c);
    UG_DrawLine(x1 + r, y2, x2 - r, y2, c);
    UG_DrawLine(x1, y1 + r, x1, y2 - r, c);
    UG_DrawLine(x2, y1 + r, x2, y2 - r, c);
    UG_DrawArc(x1 + r, y1 + r, r, 0x0C, c);
    UG_DrawArc(x2 - r, y1 + r, r, 0x03, c);
    UG_DrawArc(x1 + r, y2 - r, r, 0x30, c);
    UG_DrawArc(x2 - r, y2 - r, r, 0xC0, c);
}

void UG_DrawPixel( UG_S16 x0, UG_S16 y0, UG_COLOR c )
{
    ASSERT(gui != NULL);

    _UG_PSet(x0, y0, c);
}

void UG_DrawCircle( UG_S16 x0, UG_S16 y0, UG_S16 r, UG_COLOR c )
{
    ASSERT(gui != NULL);

    UG_S16 x, y, xd, yd, e;

    if ( x0 < 0 ) {
        return;
    }
    if ( y0 < 0 ) {
        return;
    }
    if ( r <= 0 ) {
        return;
    }

    xd = 1 - (r << 1);
    yd = 0;
    e = 0;
    x = r;
    y = 0;

    while ( x >= y ) {
        _UG_PSet(x0 - x, y0 + y, c);
        _UG_PSet(x0 - x, y0 - y, c);
        _UG_PSet(x0 + x, y0 + y, c);
        _UG_PSet(x0 + x, y0 - y, c);
        _UG_PSet(x0 - y, y0 + x, c);
        _UG_PSet(x0 - y, y0 - x, c);
        _UG_PSet(x0 + y, y0 + x, c);
        _UG_PSet(x0 + y, y0 - x, c);

        y++;
        e += yd;
        yd += 2;
        if ( ((e << 1) + xd) > 0 ) {
            x--;
            e += xd;
            xd += 2;
        }
    }
}

void UG_FillCircle( UG_S16 x0, UG_S16 y0, UG_S16 r, UG_COLOR c )
{
    UG_S16  x, y, xd;

    if ( x0 < 0 ) {
        return;
    }
    if ( y0 < 0 ) {
        return;
    }
    if ( r <= 0 ) {
        return;
    }

    xd = 3 - (r << 1);
    x = 0;
    y = r;

    while ( x <= y ) {
        if ( y > 0 ) {
            UG_DrawLine(x0 - x, y0 - y, x0 - x, y0 + y, c);
            UG_DrawLine(x0 + x, y0 - y, x0 + x, y0 + y, c);
        }
        if ( x > 0 ) {
            UG_DrawLine(x0 - y, y0 - x, x0 - y, y0 + x, c);
            UG_DrawLine(x0 + y, y0 - x, x0 + y, y0 + x, c);
        }
        if ( xd < 0 ) {
            xd += (x << 2) + 6;
        } else {
            xd += ((x - y) << 2) + 10;
            y--;
        }
        x++;
    }
    UG_DrawCircle(x0, y0, r, c);
}

void UG_DrawArc( UG_S16 x0, UG_S16 y0, UG_S16 r, UG_U8 s, UG_COLOR c )
{
    ASSERT(gui != NULL);

    UG_S16 x, y, xd, yd, e;

    if ( x0 < 0 ) {
        return;
    }
    if ( y0 < 0 ) {
        return;
    }
    if ( r <= 0 ) {
        return;
    }

    xd = 1 - (r << 1);
    yd = 0;
    e = 0;
    x = r;
    y = 0;

    while ( x >= y ) {
        // Q1
        if ( s & 0x01 ) {
            _UG_PSet(x0 + x, y0 - y, c);
        }
        if ( s & 0x02 ) {
            _UG_PSet(x0 + y, y0 - x, c);
        }

        // Q2
        if ( s & 0x04 ) {
            _UG_PSet(x0 - y, y0 - x, c);
        }
        if ( s & 0x08 ) {
            _UG_PSet(x0 - x, y0 - y, c);
        }

        // Q3
        if ( s & 0x10 ) {
            _UG_PSet(x0 - x, y0 + y, c);
        }
        if ( s & 0x20 ) {
            _UG_PSet(x0 - y, y0 + x, c);
        }

        // Q4
        if ( s & 0x40 ) {
            _UG_PSet(x0 + y, y0 + x, c);
        }
        if ( s & 0x80 ) {
            _UG_PSet(x0 + x, y0 + y, c);
        }

        y++;
        e += yd;
        yd += 2;
        if ( ((e << 1) + xd) > 0 ) {
            x--;
            e += xd;
            xd += 2;
        }
    }
}

void UG_DrawLine( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_COLOR c )
{
    ASSERT(gui != NULL);

    UG_S16 n, dx, dy, sgndx, sgndy, dxabs, dyabs, x, y, drawx, drawy;

    dx = x2 - x1;
    dy = y2 - y1;
    dxabs = (dx > 0) ? dx : -dx;
    dyabs = (dy > 0) ? dy : -dy;
    sgndx = (dx > 0) ? 1 : -1;
    sgndy = (dy > 0) ? 1 : -1;
    x = dyabs >> 1;
    y = dxabs >> 1;
    drawx = x1;
    drawy = y1;

    _UG_PSet(drawx, drawy, c);

    if ( dxabs >= dyabs ) {
        for ( n = 0; n < dxabs; n++ ) {
            y += dyabs;
            if ( y >= dxabs ) {
                y -= dxabs;
                drawy += sgndy;
            }
            drawx += sgndx;
            _UG_PSet(drawx, drawy, c);
        }
    } else {
        for ( n = 0; n < dyabs; n++ ) {
            x += dxabs;
            if ( x >= dyabs ) {
                x -= dyabs;
                drawx += sgndx;
            }
            drawy += sgndy;
            _UG_PSet(drawx, drawy, c);
        }
    }
}

void UG_MeasureString(UG_S16 *xout, UG_S16 *yout, const char *str)
{
    _UG_PutString(0, 0, xout, yout, str, 1, 1);
}

/**
 * Measures the size of a text string ignoring screen dimensions (no auto break
 * at screen width).
 */
void UG_MeasureStringNoBreak(UG_S16 *xout, UG_S16 *yout, const char *str)
{
    _UG_PutString(0, 0, xout, yout, str, 0, 1);
}

/**
 * Measures the size of a centered text. This function allows multi-line input.
 * Each line will be centered individually.
 * Catch-22: a line that is bigger than 128 characters will be cut.
 * Auto-break is disabled with this feature.
 */
void UG_MeasureStringCentered(UG_S16 *xout, UG_S16 *yout, const char *str)
{
    ASSERT(gui != NULL);

    *xout = 0;
    *yout = 0;
    UG_S16 calc_width_line = 0;
    UG_S16 calc_height_line = 0;
    const char* c;
    const uint8_t max_line = 128;
    char line[max_line];
    const char* start = str;
    for (c = str; *c != '\0'; c++) {
        if (*c == '\n') {
            _copy_slice(line, sizeof(line), start, (size_t)(c - start));
            _UG_PutString(0, 0, &calc_width_line, &calc_height_line, line, 0, 1);
            *yout += calc_height_line;
            *yout += gui->char_v_space;
            *xout = MAX(*xout, calc_width_line);
            start = c + 1;
        }
    }
    _copy_slice(line, sizeof(line), start, (size_t)(c - start));
    _UG_PutString(0, 0, &calc_width_line, &calc_height_line, line, 0, 1);
    *yout += calc_height_line;
    *yout += gui->char_v_space;
    *xout = MAX(*xout, calc_width_line);
}

static bool _is_whitespace(char c) {
    return c == '\n' || c == '\0' || c == ' ' || c == '\t' || c == '\r';
}

static UG_S16 _word_width(const char* p) {
    const UG_FONT* font = gui->font;
    UG_S16 x = 0;
    while(!_is_whitespace(*p)) {
        uint32_t codepoint = 0;
        p = _UG_NextCodepoint(p, &codepoint);
        UG_U16 char_width = 0;
        if (UG_GetCharWidth(font, codepoint, &char_width)) {
            x += char_width;
        }
    }
    return x;
}

static void _copy_codepoint(const char** str, char** str_out, UG_S16* x)
{
    const UG_FONT* font = gui->font;
    const char* next;
    uint32_t codepoint = 0;
    UG_U16 char_width = 0;
    next = _UG_NextCodepoint(*str, &codepoint);
    while (*str < next) {
        **str_out = **str;
        *str_out += 1;
        *str += 1;
    }
    if (UG_GetCharWidth(font, codepoint, &char_width)) {
        *x += char_width;
    }
}

// Try to wrap string at spaces if it is longer than `width` pixels.
// Since the purpose of this function is to wrap title text, only wrap a single time. The rest of
// the title may span/overflow the whole screen.
//
// str_out capacity must be at least strlen(str) + 1.
void UG_WrapTitleString(const char* str, char* str_out, UG_S16 width) {
    ASSERT(gui != NULL);
    ASSERT(str != NULL);
    ASSERT(str_out != NULL);

    const char* start = str;
    const UG_FONT* font = gui->font;
    UG_S16 x = 0;

    // This loop will copy bytes until the first newline.
    // * Either we find a newline in the input string, or,
    // * a newline is inserted in case the content would overflow `width`.
    //   A newline could be inserted:
    //   - before the whole content in case the first word doesn't fit, or,
    //   - where there is a space if the following word doesn't fit.
    while(*str != '\0') {
        if (*str == '\n') {
            break;
        }
        if (*str == ' ') {
            UG_S16 wwidth = _word_width(str+1);
            UG_U16 space_width = 0;
            UG_GetCharWidth(font, ' ', &space_width);
            if (x + space_width + wwidth > width) {
                *str_out = '\n';
                str_out += 1;
                str += 1;
                break;
            }
            *str_out = *str;
            str_out += 1;
            str += 1;
            while (!_is_whitespace(*str)) {
                _copy_codepoint(&str, &str_out, &x);
            }
            continue;
        }
        // If the first word doesn't fit. Insert a newline
        if(start == str) {
            UG_S16 wwidth = _word_width(str);
            if (wwidth > width) {
                *str_out = '\n';
                str_out++;
                break;
            }
        }
        _copy_codepoint(&str, &str_out, &x);
    }

    // Copy any bytes that are left
    while(*str != '\0') {
        *str_out++ = *str++;
    }
}

void UG_RenderRotated180(
    UG_S16 x,
    UG_S16 y,
    UG_S16 width,
    UG_S16 height,
    UG_RenderCallback render,
    void* ctx)
{
    ASSERT(gui != NULL);
    ASSERT(render != NULL);

    if (width <= 0 || height <= 0) {
        return;
    }

    const ug_rotation_t previous_rotation = rotation;
    rotation = (ug_rotation_t){
        .active = true,
        .x = x,
        .y = y,
        .width = width,
        .height = height,
    };
    render(ctx);
    rotation = previous_rotation;
}

void UG_PutString( UG_S16 x, UG_S16 y, const char *str)
{
    _UG_PutString(x, y, NULL, NULL, str, 1, 0);
}

void UG_PutStringNoBreak( UG_S16 x, UG_S16 y, const char *str)
{
    _UG_PutString(x, y, NULL, NULL, str, 0, 0);
}

/**
 * Creates a centered text. This function allows multi-line input.
 * Each line will be centered individually.
 * - a line that is bigger than UG_MAX_LINE_COLS characters will be cut.
 * - a text that has more than UG_MAX_LINE_ROWS rows will have the UG_MAX_LINE_ROWS'th line contain
 *   the overflowing lines on top of each other.
 * Auto-break is disabled with this feature.
 */
void UG_PutStringCentered( UG_S16 x, UG_S16 y, UG_S16 width, UG_S16 height, const char *str) {
    ASSERT(gui != NULL);

    UG_S16 calc_width;
    if (x == 0 && width == 0) {
        width = gui->x_dim - 1;
    }
    UG_S16 calc_height;
    const char* c;
    uint8_t num_lines = 1;
    for (c = str; *c != '\0'; c++) {
        if (*c == '\n') {
            num_lines++;
        }
        // Limit num_lines to avoid stack overflows
        if (num_lines > UG_MAX_LINE_ROWS) {
            break;
        }
    }
    uint8_t current_line = 0;
    const uint8_t max_line = UG_MAX_LINE_COLS;
    char lines[num_lines][max_line];
    const char* start = str;
    for (c = str; *c != '\0'; c++) {
        if (*c == '\n' && current_line < UG_MAX_LINE_ROWS) {
            _copy_slice(
                lines[current_line], sizeof(lines[current_line]), start, (size_t)(c - start));
            current_line++;
            start = c + 1;
        }
    }
    _copy_slice(lines[current_line], sizeof(lines[current_line]), start, (size_t)(c - start));

    // calculate the height of each line
    _UG_PutString(0, 0, NULL, &calc_height, "W", 0, 1);
    y = y + (height - ((calc_height + gui->char_v_space) * num_lines)) / 2;
    for (uint16_t i = 0; i < num_lines; i++) {
        UG_S16 current_y = y + (i * (calc_height + gui->char_v_space));
        _UG_PutString(0, 0, &calc_width, NULL, lines[i], 0, 1);
        UG_S16 pos_x = x + (width - calc_width) / 2;
        _UG_PutString(pos_x, current_y, NULL, NULL, lines[i], 0, 0);
    }
}

void UG_PutStringNoBreakCenter( UG_S16 x, UG_S16 y, UG_S16 width, const char *str)
{
    ASSERT(gui != NULL);

    UG_S16 calc_width;
    if (x == 0 && width == 0) {
        width = gui->x_dim - 1;
    }
    _UG_PutString(x, y, &calc_width, NULL, str, 0, 1);
    _UG_PutString(x + (width - calc_width) / 2, y, NULL, NULL, str, 0, 0);
}

void UG_PutChar( char chr, UG_S16 x, UG_S16 y, UG_COLOR fc, UG_COLOR bc )
{
    ASSERT(gui != NULL);

    _UG_PutCodepoint((UG_U8)chr, x, y, fc, bc, gui->font, false, false);
}

void UG_SetForecolor( UG_COLOR c )
{
    ASSERT(gui != NULL);

    gui->fore_color = c;
}

void UG_SetBackcolor( UG_COLOR c )
{
    ASSERT(gui != NULL);

    gui->back_color = c;
}

UG_S16 UG_GetXDim( void )
{
    ASSERT(gui != NULL);

    return gui->x_dim;
}

UG_S16 UG_GetYDim( void )
{
    ASSERT(gui != NULL);

    return gui->y_dim;
}

void UG_FontSetHSpace( UG_U16 s )
{
    ASSERT(gui != NULL);

    gui->char_h_space = s;
}

void UG_FontSetVSpace( UG_U16 s )
{
    ASSERT(gui != NULL);

    gui->char_v_space = s;
}

void UG_SendBuffer(void) {
#ifndef TESTING
    oled_send_buffer();
#endif
}

void UG_ClearBuffer(void) {
#ifndef TESTING
    oled_clear_buffer();
#endif
}
