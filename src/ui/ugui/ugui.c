/* -------------------------------------------------------------------------------- */
/* -- ÂµGUI - Generic GUI module (C)Achim DÃ¶bler, 2015                            -- */
/* -------------------------------------------------------------------------------- */
// ÂµGUI is a generic GUI module for embedded systems.
// This is a free software that is open for education, research and commercial
// developments under license policy of following terms.
//
//  Copyright (C) 2015, Achim DÃ¶bler, all rights reserved.
//  URL: http://www.embeddedlightning.com/
//
// * The ÂµGUI module is a free software and there is NO WARRANTY.
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
// for reporting bugs and giving examples how to improve ÂµGUI.
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
// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <stdbool.h>
#include <ui/oled/oled.h>
#include <util.h>
#include "ugui.h"

/* Pointer to the gui */
static UG_GUI *gui = NULL;

static void _UG_PutChar( char chr, UG_S16 x, UG_S16 y, UG_COLOR fc, UG_COLOR bc,
                         const UG_FONT *font, bool inverted, bool transparent)
{
    UG_U16 i, j, k, xo, yo, c, bn, actual_char_width;
    UG_U8 b, bt;
    UG_U32 index;
    UG_COLOR color;

    bt = (UG_U8)chr;

    switch (bt ) {
        case 0xF6:
            bt = 0x94;
            break; // ö
        case 0xD6:
            bt = 0x99;
            break; // Ö
        case 0xFC:
            bt = 0x81;
            break; // ü
        case 0xDC:
            bt = 0x9A;
            break; // Ü
        case 0xE4:
            bt = 0x84;
            break; // ä
        case 0xC4:
            bt = 0x8E;
            break; // Ä
        case 0xB5:
            bt = 0xE6;
            break; // µ
        case 0xB0:
            bt = 0xF8;
            break; // °
        default:
            break;
    }

    if (bt < font->start_char || bt > font->end_char) {
        return;
    }

    yo = inverted ? (y + font->char_height) : y;
    bn = font->char_width;
    if ( !bn ) {
        return;
    }
    bn >>= 3;
    if ( font->char_width % 8 ) {
        bn++;
    }
    actual_char_width = (font->widths ? font->widths[bt - font->start_char] :
                         font->char_width);

    if (font->font_type == FONT_TYPE_1BPP) {
        index = (bt - font->start_char) * font->char_height * bn;
        for ( j = 0; j < font->char_height; j++ ) {
            xo = inverted ? (x + actual_char_width) : x;
            c = actual_char_width;
            for ( i = 0; i < bn; i++ ) {
                b = font->p[index++];
                for ( k = 0; (k < 8) && c; k++ ) {
                    if ( b & 0x01 ) {
                        gui->pset(xo, yo, fc);
                    } else if ( !transparent ) {
                        gui->pset(xo, yo, bc);
                    }
                    b >>= 1;
                    if (inverted) {
                        xo--;
                    } else {
                        xo++;
                    }
                    c--;
                }
            }
            if (inverted) {
                yo--;
            } else {
                yo++;
            }
        }
    } else if (font->font_type == FONT_TYPE_8BPP) {
        // inversion not supported
        index = (bt - font->start_char) * font->char_height * font->char_width;
        for ( j = 0; j < font->char_height; j++ ) {
            xo = x;
            for ( i = 0; i < actual_char_width; i++ ) {
                b = font->p[index++];
                color = ((((fc & 0xFF) * b + (bc & 0xFF) * (256 - b)) >> 8) & 0xFF) |//Blue component
                        ((((fc & 0xFF00) * b + (bc & 0xFF00) * (256 - b)) >> 8)  & 0xFF00) |//Green component
                        ((((fc & 0xFF0000) * b + (bc & 0xFF0000) * (256 - b)) >> 8) & 0xFF0000); //Red component
                gui->pset(xo, yo, color);
                xo++;
            }
            index += font->char_width - actual_char_width;
            yo++;
        }
    }
}

static void _UG_PutString( UG_S16 x, UG_S16 y, UG_S16 *xout, UG_S16 *yout, const char *str,
                    int autobreak, int calconly, bool inverted )
{
    if (gui == NULL) {
        return;
    }

    UG_S16 xp, yp;
    UG_U8 cw;
    char chr;
    UG_S16 max_x = x;

    xp = x;
    yp = y;

    const int str_length = strlens(str);
    uint8_t line = 0;

    for (int i = 0; i < str_length; i++) {
        chr = (char)(inverted ? str[str_length - 1 - i] : str[i]);
        if (chr != '\n' && (chr < gui->font.start_char || chr > gui->font.end_char)) {
            continue;
        }
        if ( chr == '\n' ) {
            if (autobreak == 1) {
                xp = gui->x_dim;
            } else {
                line++;
                xp = x;
                yp += gui->font.char_height + gui->char_v_space;
            }
            continue;
        }
        cw = gui->font.widths ? gui->font.widths[chr - gui->font.start_char] :
             gui->font.char_width;

        if ( autobreak == 1 && xp + cw > gui->x_dim - 1 ) {
            line++;
            xp = x;
            yp += gui->font.char_height + gui->char_v_space;
        }

        if (!calconly) {
            UG_PutChar(chr, xp, yp, gui->fore_color, gui->back_color, inverted);
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
        *yout = yp + gui->font.char_height;
    }
}

UG_S16 UG_Init( UG_GUI *g, void (*p)(UG_S16, UG_S16, UG_COLOR),
                const UG_FONT *font, UG_S16 x, UG_S16 y )
{
    if (g == NULL || p == NULL || font == NULL) {
        return 0;
    }

    g->pset = (void(*)(UG_S16, UG_S16, UG_COLOR))p;
    g->x_dim = x;
    g->y_dim = y;
    g->font = *font;
    g->char_h_space = 1;
    g->char_v_space = 1;
    g->fore_color = C_WHITE;
    g->back_color = C_BLACK;

    gui = g;
    return 1;
}

void UG_FontSelect( const UG_FONT *font )
{
    if (gui && font) {
        gui->font = *font;
    }
}

void UG_FillScreen( UG_COLOR c )
{
    if (gui) {
        UG_FillFrame(0, 0, gui->x_dim - 1, gui->y_dim - 1, c);
    }
}

void UG_FillFrame( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_COLOR c )
{
    if (gui == NULL) {
        return;
    }

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
            gui->pset(n, m, c);
        }
    }
}

void UG_FillRoundFrame( UG_S16 x1, UG_S16 y1, UG_S16 x2, UG_S16 y2, UG_S16 r, UG_COLOR c )
{
    if (gui == NULL) {
        return;
    }

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
    if (gui) {
        gui->pset(x0, y0, c);
    }
}

void UG_DrawCircle( UG_S16 x0, UG_S16 y0, UG_S16 r, UG_COLOR c )
{
    if (gui == NULL) {
        return;
    }

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
        gui->pset(x0 - x, y0 + y, c);
        gui->pset(x0 - x, y0 - y, c);
        gui->pset(x0 + x, y0 + y, c);
        gui->pset(x0 + x, y0 - y, c);
        gui->pset(x0 - y, y0 + x, c);
        gui->pset(x0 - y, y0 - x, c);
        gui->pset(x0 + y, y0 + x, c);
        gui->pset(x0 + y, y0 - x, c);

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
    if (gui == NULL) {
        return;
    }

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
            gui->pset(x0 + x, y0 - y, c);
        }
        if ( s & 0x02 ) {
            gui->pset(x0 + y, y0 - x, c);
        }

        // Q2
        if ( s & 0x04 ) {
            gui->pset(x0 - y, y0 - x, c);
        }
        if ( s & 0x08 ) {
            gui->pset(x0 - x, y0 - y, c);
        }

        // Q3
        if ( s & 0x10 ) {
            gui->pset(x0 - x, y0 + y, c);
        }
        if ( s & 0x20 ) {
            gui->pset(x0 - y, y0 + x, c);
        }

        // Q4
        if ( s & 0x40 ) {
            gui->pset(x0 + y, y0 + x, c);
        }
        if ( s & 0x80 ) {
            gui->pset(x0 + x, y0 + y, c);
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
    if (gui == NULL) {
        return;
    }

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

    gui->pset(drawx, drawy, c);

    if ( dxabs >= dyabs ) {
        for ( n = 0; n < dxabs; n++ ) {
            y += dyabs;
            if ( y >= dxabs ) {
                y -= dxabs;
                drawy += sgndy;
            }
            drawx += sgndx;
            gui->pset(drawx, drawy, c);
        }
    } else {
        for ( n = 0; n < dyabs; n++ ) {
            x += dxabs;
            if ( x >= dyabs ) {
                x -= dyabs;
                drawx += sgndx;
            }
            drawy += sgndy;
            gui->pset(drawx, drawy, c);
        }
    }
}

void UG_MeasureString(UG_S16 *xout, UG_S16 *yout, const char *str)
{
    _UG_PutString(0, 0, xout, yout, str, 1, 1, false);
}

/**
 * Measures the size of a text string ignoring screen dimensions (no auto break
 * at screen width).
 */
void UG_MeasureStringNoBreak(UG_S16 *xout, UG_S16 *yout, const char *str)
{
    _UG_PutString(0, 0, xout, yout, str, 0, 1, false);
}

/**
 * Measures the size of a centered text. This function allows multi-line input.
 * Each line will be centered individually.
 * Catch-22: a line that is bigger than 128 characters will be cut.
 * Auto-break is disabled with this feature.
 */
void UG_MeasureStringCentered(UG_S16 *xout, UG_S16 *yout, const char *str)
{
    if (gui == NULL) {
        return;
    }

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
            snprintf(line, sizeof(line), "%.*s", c - start, start);
            _UG_PutString(0, 0, &calc_width_line, &calc_height_line, line, 0, 1, false);
            *yout += calc_height_line;
            *yout += gui->char_v_space;
            *xout = MAX(*xout, calc_width_line);
            start = c + 1;
        }
    }
    snprintf(line, sizeof(line), "%.*s", c - start, start);
    _UG_PutString(0, 0, &calc_width_line, &calc_height_line, line, 0, 1, false);
    *yout += calc_height_line;
    *yout += gui->char_v_space;
    *xout = MAX(*xout, calc_width_line);
}

static bool _is_whitespace(char c) {
    return c == '\n' || c == '\0' || c == ' ' || c == '\t' || c == '\r';
}

static UG_S16 _word_width(const char* p) {
    const UG_FONT* font = &gui->font;
    UG_S16 x = 0;
    while(!_is_whitespace(*p)) {
        x += font->widths[(UG_U8)*p - font->start_char];
        p += 1;
    }
    return x;
}

// Try to wrap string at spaces if it is longer than `width` pixels.
// Since the purpose of this function is to wrap title text, only wrap a single time. The rest of
// the title may span/overflow the whole screen.
//
// str_out capacity must be at least strlen(str) + 1.
void UG_WrapTitleString(const char* str, char* str_out, UG_S16 width) {
    if (gui == NULL || str == NULL || str_out == NULL) {
        return;
    }
    const char* start = str;
    const UG_FONT* font = &gui->font;
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
            if (x + font->widths[(UG_U8)' ' - font->start_char] + wwidth > width) {
                *str_out = '\n';
                str_out += 1;
                str += 1;
                break;
            } else {
                *str_out = *str;
                str_out += 1;
                str += 1;
            }
            while (!_is_whitespace(*str)) {
                *str_out = *str;
                if (*str >= font->start_char){
                    x += font->widths[(UG_U8)*str - font->start_char];
                }
                str_out += 1;
                str += 1;
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
        *str_out = *str;
        if (*str >= font->start_char && *str < font->end_char){
            x += font->widths[(UG_U8)*str - font->start_char];
        }
        str_out += 1;
        str += 1;
    }

    // Copy any bytes that are left
    while(*str != '\0') {
        *str_out++ = *str++;
    }
}

void UG_PutString( UG_S16 x, UG_S16 y, const char *str, bool inverted)
{
    _UG_PutString(x, y, NULL, NULL, str, 1, 0, inverted);
}

void UG_PutStringNoBreak( UG_S16 x, UG_S16 y, const char *str, bool inverted)
{
    _UG_PutString(x, y, NULL, NULL, str, 0, 0, inverted);
}

/**
 * Creates a centered text. This function allows multi-line input.
 * Each line will be centered individually.
 * - a line that is bigger than UG_MAX_LINE_COLS characters will be cut.
 * - a text that has more than UG_MAX_LINE_ROWS rows will have the UG_MAX_LINE_ROWS'th line contain
 *   the overflowing lines on top of each other.
 * Auto-break is disabled with this feature.
 */
void UG_PutStringCentered( UG_S16 x, UG_S16 y, UG_S16 width, UG_S16 height, const char *str, bool inverted) {
    if (gui == NULL) {
        return;
    }

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
            snprintf(lines[current_line], sizeof(lines[current_line]), "%.*s", c - start, start);
            current_line++;
            start = c + 1;
        }
    }
    snprintf(lines[current_line], sizeof(lines[current_line]), "%.*s", c - start, start);

    // calculate the height of each line
    _UG_PutString(0, 0, NULL, &calc_height, "W", 0, 1, inverted);
    y = y + (height - ((calc_height + gui->char_v_space) * num_lines)) / 2;
    for (uint16_t i = 0; i < num_lines; i++) {
        UG_S16 current_y = y + (i * (calc_height + gui->char_v_space));
        _UG_PutString(0, 0, &calc_width, NULL, lines[i], 0, 1, inverted);
        UG_S16 pos_x = x + (width - calc_width) / 2;
        _UG_PutString(pos_x, current_y, NULL, NULL, lines[i], 0, 0, inverted);
    }
}

void UG_PutStringNoBreakCenter( UG_S16 x, UG_S16 y, UG_S16 width, const char *str, bool inverted)
{
    if (gui == NULL) {
        return;
    }

    UG_S16 calc_width;
    if (x == 0 && width == 0) {
        width = gui->x_dim - 1;
    }
    _UG_PutString(x, y, &calc_width, NULL, str, 0, 1, inverted);
    _UG_PutString(x + (width - calc_width) / 2, y, NULL, NULL, str, 0, 0, inverted);
}

void UG_PutChar( char chr, UG_S16 x, UG_S16 y, UG_COLOR fc, UG_COLOR bc, bool inverted )
{
    if (gui == NULL) {
        return;
    }

    _UG_PutChar(chr, x, y, fc, bc, &gui->font, inverted, false);
}

void UG_PutCharTransparent( char chr, UG_S16 x, UG_S16 y, UG_COLOR fc, bool inverted )
{
    if (gui == NULL) {
        return;
    }

    _UG_PutChar(chr, x, y, fc, 0x00, &gui->font, inverted, true);
}

void UG_SetForecolor( UG_COLOR c )
{
    if (gui) {
        gui->fore_color = c;
    }
}

void UG_SetBackcolor( UG_COLOR c )
{
    if (gui) {
        gui->back_color = c;
    }
}

UG_S16 UG_GetXDim( void )
{
    if (gui == NULL) {
        return 0;
    }

    return gui->x_dim;
}

UG_S16 UG_GetYDim( void )
{
    if (gui == NULL) {
        return 0;
    }

    return gui->y_dim;
}

void UG_FontSetHSpace( UG_U16 s )
{
    if (gui) {
        gui->char_h_space = s;
    }
}

void UG_FontSetVSpace( UG_U16 s )
{
    if (gui) {
        gui->char_v_space = s;
    }
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
