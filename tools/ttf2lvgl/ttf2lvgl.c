/*
 * Copyright (c) 2015, Ari Suutari <ari@stonepile.fi>.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *  1. Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *  2. Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *  3. The name of the author may not be used to endorse or promote
 *     products derived from this software without specific prior written
 *     permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS
 * OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT,
 * INDIRECT,  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/*
 *  19.10.2017 jkpublic@kartech.biz - Added support for 8BPP fonts (anti aliased)
 */

#include <ctype.h>
#include <getopt.h>
#include <limits.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <ft2build.h>
#include FT_FREETYPE_H
#include <unicode/utf8.h>

#define SCREEN_WIDTH 132
#define SCREEN_HEIGHT 40

typedef struct {
    int codepoint;
    int adv_w;
    int box_w;
    int box_h;
    int ofs_x;
    int ofs_y;
    size_t bitmap_index;
    size_t bitmap_size;
    uint8_t* bitmap;
} Glyph;

typedef struct {
    int bpp;
    int line_height;
    int baseline;
    Glyph* glyphs;
    size_t glyph_count;
    size_t bitmap_size;
    UChar32* codepoints;
    size_t codepoint_count;
    struct {
        uint32_t range_start;
        uint16_t range_length;
        uint16_t glyph_id_start;
    }* cmaps;
    size_t cmap_count;
} LvglFont;

typedef struct {
    UChar32* codepoints;
    size_t count;
    size_t capacity;
} Codepoints;

static float font_size = 0;
static int dpi = 0;
static int bpp = 1;
static int min_char = 32;
static int max_char = 126;
static bool minmax_used = false;
static bool range_used = false;
static int dump = 0;
static char* font_file = NULL;
static char* show_text = NULL;
static char* font_name_arg = NULL;
static char* output_file_arg = NULL;
static Codepoints range_codepoints = {0};

static void usage(void);

static int max_int(int a, int b)
{
    return a > b ? a : b;
}

static void* xmalloc(size_t size)
{
    void* ptr = malloc(size == 0 ? 1 : size);
    if (ptr == NULL) {
        perror("malloc");
        exit(2);
    }
    return ptr;
}

static void* xcalloc(size_t count, size_t size)
{
    void* ptr = calloc(count == 0 ? 1 : count, size == 0 ? 1 : size);
    if (ptr == NULL) {
        perror("calloc");
        exit(2);
    }
    return ptr;
}

static char* xstrdup(const char* str)
{
    char* out = strdup(str);
    if (out == NULL) {
        perror("strdup");
        exit(2);
    }
    return out;
}

static char* path_basename_no_ext(const char* path)
{
    const char* base_name = strrchr(path, '/');
    base_name = base_name == NULL ? path : base_name + 1;

    char* out = xstrdup(base_name);
    char* dot = strrchr(out, '.');
    if (dot != NULL) {
        *dot = '\0';
    }
    return out;
}

static char* sanitize_identifier(const char* name, bool uppercase)
{
    size_t len = strlen(name);
    char* out = xmalloc(len + 2);
    size_t pos = 0;

    if (len == 0 || isdigit((unsigned char)name[0])) {
        out[pos++] = '_';
    }

    for (size_t i = 0; i < len; i++) {
        unsigned char ch = (unsigned char)name[i];
        if (isalnum(ch)) {
            out[pos++] = (char)(uppercase ? toupper(ch) : ch);
        } else {
            out[pos++] = '_';
        }
    }

    out[pos] = '\0';
    return out;
}

static void lowercase_identifier(char* text)
{
    for (size_t i = 0; text[i] != '\0'; i++) {
        text[i] = (char)tolower((unsigned char)text[i]);
    }
}

static bool is_valid_c_identifier(const char* name)
{
    if (name[0] == '\0' || !(isalpha((unsigned char)name[0]) || name[0] == '_')) {
        return false;
    }
    for (size_t i = 1; name[i] != '\0'; i++) {
        if (!(isalnum((unsigned char)name[i]) || name[i] == '_')) {
            return false;
        }
    }
    return true;
}

static char* font_size_suffix(float requested_size)
{
    char text[64];
    snprintf(text, sizeof(text), "%g", requested_size);

    char* out = xstrdup(text);
    for (size_t i = 0; out[i] != '\0'; i++) {
        if (!isalnum((unsigned char)out[i])) {
            out[i] = '_';
        }
    }
    return out;
}

static char* default_font_name(const char* font_file, float requested_size)
{
    char* base_name = path_basename_no_ext(font_file);
    char* sanitized_base = sanitize_identifier(base_name, false);
    lowercase_identifier(sanitized_base);
    free(base_name);

    char* size_suffix = font_size_suffix(requested_size);
    size_t len = strlen(sanitized_base) + strlen(size_suffix) + 2;
    char* out = xmalloc(len);
    snprintf(out, len, "%s_%s", sanitized_base, size_suffix);
    free(size_suffix);
    free(sanitized_base);
    return out;
}

static char* output_header_name(const char* output_file, const char* font_name)
{
    if (output_file == NULL) {
        size_t len = strlen(font_name) + 3;
        char* out = xmalloc(len);
        snprintf(out, len, "%s.h", font_name);
        return out;
    }

    char* out = xstrdup(output_file);
    char* slash = strrchr(out, '/');
    char* base = slash == NULL ? out : slash + 1;
    char* dot = strrchr(base, '.');
    if (dot != NULL) {
        strcpy(dot, ".h");
    } else {
        size_t len = strlen(out) + 3;
        out = realloc(out, len);
        if (out == NULL) {
            perror("realloc");
            exit(2);
        }
        strcat(out, ".h");
    }
    return out;
}

static char* public_font_symbol(const char* font_name)
{
    if (strncmp(font_name, "font_", 5) == 0) {
        return xstrdup(font_name);
    }

    size_t len = strlen("font_") + strlen(font_name) + 1;
    char* out = xmalloc(len);
    snprintf(out, len, "font_%s", font_name);
    return out;
}

static void codepoints_append(Codepoints* codepoints, UChar32 codepoint)
{
    if (codepoint < 0 || codepoint > 0x10FFFF) {
        fprintf(stderr, "invalid Unicode code point: U+%X\n", (unsigned int)codepoint);
        exit(1);
    }

    if (codepoints->count == codepoints->capacity) {
        size_t new_capacity = codepoints->capacity == 0 ? 32 : codepoints->capacity * 2;
        UChar32* new_codepoints =
            realloc(codepoints->codepoints, new_capacity * sizeof(*new_codepoints));
        if (new_codepoints == NULL) {
            perror("realloc");
            exit(2);
        }
        codepoints->codepoints = new_codepoints;
        codepoints->capacity = new_capacity;
    }
    codepoints->codepoints[codepoints->count++] = codepoint;
}

static int compare_codepoints(const void* a, const void* b)
{
    UChar32 left = *(const UChar32*)a;
    UChar32 right = *(const UChar32*)b;
    return (left > right) - (left < right);
}

static void codepoints_sort_dedup(Codepoints* codepoints)
{
    if (codepoints->count == 0) {
        return;
    }

    qsort(
        codepoints->codepoints,
        codepoints->count,
        sizeof(*codepoints->codepoints),
        compare_codepoints);

    size_t out = 1;
    for (size_t i = 1; i < codepoints->count; i++) {
        if (codepoints->codepoints[i] != codepoints->codepoints[out - 1]) {
            codepoints->codepoints[out++] = codepoints->codepoints[i];
        }
    }
    codepoints->count = out;
}

static void parse_codepoint(const char* text, UChar32* out)
{
    while (isspace((unsigned char)*text)) {
        text++;
    }

    int base = 10;
    if (text[0] == 'U' || text[0] == 'u') {
        if (text[1] != '+') {
            fprintf(stderr, "invalid code point: %s\n", text);
            exit(1);
        }
        text += 2;
        base = 16;
    } else if (text[0] == '0' && (text[1] == 'x' || text[1] == 'X')) {
        text += 2;
        base = 16;
    }

    char* end = NULL;
    unsigned long value = strtoul(text, &end, base);
    while (end != NULL && isspace((unsigned char)*end)) {
        end++;
    }
    if (end == text || (end != NULL && *end != '\0') || value > 0x10FFFF) {
        fprintf(stderr, "invalid code point: %s\n", text);
        exit(1);
    }
    *out = (UChar32)value;
}

static void parse_codepoint_range_item(char* item, Codepoints* codepoints)
{
    while (isspace((unsigned char)*item)) {
        item++;
    }
    if (*item == '\0') {
        return;
    }

    char* dash = strchr(item, '-');
    if (dash == NULL) {
        UChar32 codepoint;
        parse_codepoint(item, &codepoint);
        codepoints_append(codepoints, codepoint);
        return;
    }

    *dash = '\0';
    UChar32 first;
    UChar32 last;
    parse_codepoint(item, &first);
    parse_codepoint(dash + 1, &last);
    if (last < first) {
        fprintf(
            stderr,
            "invalid descending code point range: U+%04X-U+%04X\n",
            (unsigned int)first,
            (unsigned int)last);
        exit(1);
    }
    for (UChar32 codepoint = first; codepoint <= last; codepoint++) {
        codepoints_append(codepoints, codepoint);
    }
}

static void parse_codepoint_ranges(const char* text, Codepoints* codepoints)
{
    char* copy = xstrdup(text);
    char* saveptr = NULL;
    for (char* item = strtok_r(copy, ",", &saveptr); item != NULL;
         item = strtok_r(NULL, ",", &saveptr)) {
        parse_codepoint_range_item(item, codepoints);
    }
    free(copy);
}

static void append_legacy_codepoint_range(Codepoints* codepoints)
{
    if (min_char < 0 || max_char < min_char || max_char > 0x10FFFF) {
        usage();
        exit(1);
    }
    for (int ch = min_char; ch <= max_char; ch++) {
        codepoints_append(codepoints, (UChar32)ch);
    }
}

static Codepoints utf8_to_codepoints(const char* text)
{
    size_t text_len = strlen(text);
    if (text_len > INT32_MAX) {
        fprintf(stderr, "preview text is too long\n");
        exit(1);
    }

    const uint8_t* utf8 = (const uint8_t*)text;
    int32_t utf8_len = (int32_t)text_len;
    int32_t offset = 0;
    Codepoints out = {
        .codepoints = xmalloc((text_len == 0 ? 1 : text_len) * sizeof(*out.codepoints)),
        .count = 0,
        .capacity = text_len == 0 ? 1 : text_len,
    };

    while (offset < utf8_len) {
        int32_t previous_offset = offset;
        UChar32 codepoint;
        U8_NEXT(utf8, offset, utf8_len, codepoint);
        if (codepoint < 0) {
            fprintf(stderr, "invalid UTF-8 sequence at byte offset %d\n", previous_offset);
            exit(1);
        }
        codepoints_append(&out, codepoint);
    }

    return out;
}

static size_t lvgl_bitmap_size(int width, int height, int bits_per_pixel)
{
    if (width <= 0 || height <= 0) {
        return 0;
    }
    if (bits_per_pixel == 8) {
        return (size_t)width * (size_t)height;
    }
    return ((size_t)width * (size_t)height + 7) / 8;
}

static bool ft_mono_pixel(const FT_Bitmap* bitmap, int x, int y)
{
    if (x < 0 || y < 0 || x >= (int)bitmap->width || y >= (int)bitmap->rows) {
        return false;
    }

    int pitch = bitmap->pitch;
    const uint8_t* row = pitch >= 0 ? bitmap->buffer + y * pitch
                                    : bitmap->buffer + ((int)bitmap->rows - 1 - y) * (-pitch);
    uint8_t byte = row[x / 8];
    return (byte & (1 << (7 - (x % 8)))) != 0;
}

static void set_lvgl_pixel(Glyph* glyph, int x, int y, int bits_per_pixel, uint8_t value)
{
    if (x < 0 || y < 0 || x >= glyph->box_w || y >= glyph->box_h) {
        return;
    }

    if (bits_per_pixel == 8) {
        glyph->bitmap[(size_t)y * (size_t)glyph->box_w + (size_t)x] = value;
        return;
    }

    if (value == 0) {
        return;
    }
    size_t bit_index = (size_t)y * (size_t)glyph->box_w + (size_t)x;
    glyph->bitmap[bit_index / 8] |= (uint8_t)(1 << (7 - (bit_index % 8)));
}

static void check_lvgl_small_glyph_limits(const Glyph* glyph)
{
    if (glyph->bitmap_index > 0xFFFFF) {
        fprintf(stderr, "bitmap too large for default LVGL glyph descriptor\n");
        exit(1);
    }
    if (glyph->adv_w < 0 || glyph->adv_w > 255) {
        fprintf(
            stderr,
            "advance width too large for default LVGL glyph descriptor: U+%04X\n",
            glyph->codepoint);
        exit(1);
    }
    if (glyph->box_w > 255 || glyph->box_h > 255 || glyph->ofs_x < -128 || glyph->ofs_x > 127 ||
        glyph->ofs_y < -128 || glyph->ofs_y > 127) {
        fprintf(
            stderr,
            "glyph metrics too large for default LVGL glyph descriptor: U+%04X\n",
            glyph->codepoint);
        exit(1);
    }
}

static void build_cmaps(LvglFont* font)
{
    if (font->codepoint_count == 0) {
        return;
    }

    font->cmaps = xcalloc(font->codepoint_count, sizeof(*font->cmaps));
    size_t cmap_count = 0;
    size_t range_start_index = 0;

    while (range_start_index < font->codepoint_count) {
        size_t range_end_index = range_start_index + 1;
        while (range_end_index < font->codepoint_count &&
               font->codepoints[range_end_index] == font->codepoints[range_end_index - 1] + 1 &&
               range_end_index - range_start_index < UINT16_MAX) {
            range_end_index++;
        }

        font->cmaps[cmap_count].range_start = (uint32_t)font->codepoints[range_start_index];
        font->cmaps[cmap_count].range_length = (uint16_t)(range_end_index - range_start_index);
        font->cmaps[cmap_count].glyph_id_start = (uint16_t)(range_start_index + 1);
        cmap_count++;
        if (cmap_count > 511) {
            fprintf(stderr, "too many LVGL cmaps; combine adjacent code point ranges\n");
            exit(1);
        }
        range_start_index = range_end_index;
    }

    font->cmap_count = cmap_count;
}

static Codepoints filter_present_codepoints(FT_Face face, const Codepoints* codepoints)
{
    Codepoints present_codepoints = {0};

    for (size_t i = 0; i < codepoints->count; i++) {
        UChar32 codepoint = codepoints->codepoints[i];
        if (FT_Get_Char_Index(face, (FT_ULong)codepoint) != 0) {
            codepoints_append(&present_codepoints, codepoint);
        }
    }

    return present_codepoints;
}

static LvglFont* convert_font(
    const char* font,
    int display_dpi,
    float requested_size,
    int bits_per_pixel,
    const Codepoints* codepoints)
{
    int bpp_mul;
    switch (bits_per_pixel) {
    case 1:
        bpp_mul = 1;
        break;
    case 8:
        bpp_mul = 16;
        break;
    default:
        fprintf(stderr, "Bits per pixel must be 1 or 8, not %d\n", bits_per_pixel);
        exit(1);
    }

    FT_Library library;
    int error = FT_Init_FreeType(&library);
    if (error) {
        fprintf(stderr, "ft init err %d\n", error);
        exit(1);
    }

    FT_Face face;
    error = FT_New_Face(library, font, 0, &face);
    if (error) {
        fprintf(stderr, "new face err %d\n", error);
        exit(1);
    }

    if (display_dpi > 0) {
        error = FT_Set_Char_Size(
            face, 0, (FT_F26Dot6)(requested_size * 64 * bpp_mul), display_dpi, display_dpi);
    } else {
        error = FT_Set_Pixel_Sizes(face, 0, (FT_UInt)(requested_size * bpp_mul));
    }
    if (error) {
        fprintf(stderr, "set pixel sizes err %d\n", error);
        exit(1);
    }

    Codepoints present_codepoints = filter_present_codepoints(face, codepoints);
    if (present_codepoints.count == 0) {
        fprintf(stderr, "none of the requested code points are present in the font\n");
        exit(1);
    }
    if (present_codepoints.count > UINT16_MAX) {
        fprintf(stderr, "too many glyphs present for LVGL cmap glyph ids\n");
        exit(1);
    }

    int max_ascent = 0;
    int max_descent = 0;

    for (size_t i = 0; i < present_codepoints.count; i++) {
        UChar32 codepoint = present_codepoints.codepoints[i];
        error = FT_Load_Char(face, (FT_ULong)codepoint, FT_LOAD_RENDER | FT_LOAD_TARGET_MONO);
        if (error) {
            fprintf(stderr, "load char U+%04X err %d\n", (unsigned int)codepoint, error);
            exit(1);
        }

        int descent = max_int(0, (int)face->glyph->bitmap.rows - face->glyph->bitmap_top);
        int ascent =
            max_int(0, max_int(face->glyph->bitmap_top, (int)face->glyph->bitmap.rows) - descent);

        max_descent = max_int(max_descent, descent);
        max_ascent = max_int(max_ascent, ascent);
    }

    LvglFont* out = xcalloc(1, sizeof(*out));
    out->bpp = bits_per_pixel;
    out->line_height = (max_ascent + max_descent) / bpp_mul;
    out->baseline = max_ascent / bpp_mul;
    out->glyph_count = present_codepoints.count;
    out->glyphs = xcalloc(out->glyph_count, sizeof(*out->glyphs));
    out->codepoint_count = present_codepoints.count;
    out->codepoints = xmalloc(out->codepoint_count * sizeof(*out->codepoints));
    memcpy(
        out->codepoints,
        present_codepoints.codepoints,
        out->codepoint_count * sizeof(*out->codepoints));
    build_cmaps(out);

    for (size_t glyph_index = 0; glyph_index < present_codepoints.count; glyph_index++) {
        UChar32 codepoint = present_codepoints.codepoints[glyph_index];
        error = FT_Load_Char(face, (FT_ULong)codepoint, FT_LOAD_RENDER | FT_LOAD_TARGET_MONO);
        if (error) {
            fprintf(stderr, "load char U+%04X err %d\n", (unsigned int)codepoint, error);
            exit(1);
        }

        Glyph* glyph = &out->glyphs[glyph_index];
        glyph->codepoint = codepoint;
        glyph->adv_w = (int)((face->glyph->advance.x >> 6) / bpp_mul);
        glyph->box_w = (int)face->glyph->bitmap.width / bpp_mul;
        glyph->box_h = (int)face->glyph->bitmap.rows / bpp_mul;
        glyph->ofs_x = face->glyph->bitmap_left / bpp_mul;
        glyph->ofs_y = (face->glyph->bitmap_top / bpp_mul) - glyph->box_h;
        glyph->bitmap_index = out->bitmap_size;
        glyph->bitmap_size = lvgl_bitmap_size(glyph->box_w, glyph->box_h, bits_per_pixel);
        glyph->bitmap = xcalloc(glyph->bitmap_size, 1);

        for (int y = 0; y < glyph->box_h; y++) {
            for (int x = 0; x < glyph->box_w; x++) {
                int coverage = 0;
                for (int y_idx = 0; y_idx < bpp_mul; y_idx++) {
                    for (int x_idx = 0; x_idx < bpp_mul; x_idx++) {
                        if (ft_mono_pixel(
                                &face->glyph->bitmap, x * bpp_mul + x_idx, y * bpp_mul + y_idx)) {
                            coverage++;
                        }
                    }
                }

                if (bits_per_pixel == 1) {
                    set_lvgl_pixel(glyph, x, y, bits_per_pixel, coverage != 0 ? 1 : 0);
                } else {
                    set_lvgl_pixel(glyph, x, y, bits_per_pixel, (uint8_t)((255 * coverage) / 256));
                }
            }
        }

        check_lvgl_small_glyph_limits(glyph);
        out->bitmap_size += glyph->bitmap_size;
    }

    FT_Done_Face(face);
    FT_Done_FreeType(library);
    free(present_codepoints.codepoints);
    return out;
}

static void print_bytes(FILE* out, const uint8_t* bytes, size_t len)
{
    for (size_t i = 0; i < len; i++) {
        if (i % 12 == 0) {
            fprintf(out, "    ");
        }
        fprintf(out, "0x%02X,", bytes[i]);
        if (i + 1 < len) {
            fprintf(out, " ");
        }
        if (i % 12 == 11 || i + 1 == len) {
            fprintf(out, "\n");
        }
    }
}

static void print_utf8_codepoint(FILE* out, UChar32 codepoint)
{
    if (codepoint <= 0x7F) {
        fputc((char)codepoint, out);
    } else if (codepoint <= 0x7FF) {
        fputc((char)(0xC0 | (codepoint >> 6)), out);
        fputc((char)(0x80 | (codepoint & 0x3F)), out);
    } else if (codepoint <= 0xFFFF) {
        fputc((char)(0xE0 | (codepoint >> 12)), out);
        fputc((char)(0x80 | ((codepoint >> 6) & 0x3F)), out);
        fputc((char)(0x80 | (codepoint & 0x3F)), out);
    } else {
        fputc((char)(0xF0 | (codepoint >> 18)), out);
        fputc((char)(0x80 | ((codepoint >> 12) & 0x3F)), out);
        fputc((char)(0x80 | ((codepoint >> 6) & 0x3F)), out);
        fputc((char)(0x80 | (codepoint & 0x3F)), out);
    }
}

static void print_glyph_comment(FILE* out, UChar32 codepoint)
{
    fprintf(out, "    /* U+%04X \"", (unsigned int)codepoint);
    switch (codepoint) {
    case '\0':
        fprintf(out, "\\0");
        break;
    case '\b':
        fprintf(out, "\\b");
        break;
    case '\t':
        fprintf(out, "\\t");
        break;
    case '\n':
        fprintf(out, "\\n");
        break;
    case '\f':
        fprintf(out, "\\f");
        break;
    case '\r':
        fprintf(out, "\\r");
        break;
    case '"':
        fprintf(out, "\\\"");
        break;
    case '\\':
        fprintf(out, "\\\\");
        break;
    default:
        if ((codepoint < 0x20) || (codepoint >= 0x7F && codepoint <= 0x9F) ||
            (codepoint >= 0xD800 && codepoint <= 0xDFFF)) {
            fprintf(out, "\\u%04X", (unsigned int)codepoint);
        } else {
            print_utf8_codepoint(out, codepoint);
        }
        break;
    }
    fprintf(out, "\" */\n");
}

static void print_codepoint_ranges(FILE* out, const LvglFont* font)
{
    for (size_t i = 0; i < font->cmap_count; i++) {
        const uint32_t range_start = font->cmaps[i].range_start;
        const uint32_t range_end = range_start + font->cmaps[i].range_length - 1;
        if (i > 0) {
            fprintf(out, ",");
        }
        if (range_start == range_end) {
            fprintf(out, "%u", range_start);
        } else {
            fprintf(out, "%u-%u", range_start, range_end);
        }
    }
}

static void dump_font(const LvglFont* font, const char* font_file_path, const char* font_name)
{
    char* symbol = NULL;
    if (font_name == NULL) {
        symbol = default_font_name(font_file_path, font_size);
    } else {
        if (!is_valid_c_identifier(font_name)) {
            fprintf(stderr, "invalid --name: must be a valid C identifier\n");
            exit(1);
        }
        symbol = xstrdup(font_name);
    }
    char* symbol_upper = sanitize_identifier(symbol, true);
    char* public_symbol = public_font_symbol(symbol);

    char* out_file = output_file_arg == NULL ? NULL : xstrdup(output_file_arg);
    if (out_file == NULL) {
        size_t len = strlen(symbol) + 3;
        out_file = xmalloc(len);
        snprintf(out_file, len, "%s.c", symbol);
    }

    char* header_file = output_header_name(out_file, symbol);
    char* header_basename = strrchr(header_file, '/');
    header_basename = header_basename == NULL ? header_file : header_basename + 1;

    FILE* out = fopen(out_file, "w");
    if (out == NULL) {
        perror(out_file);
        exit(2);
    }

    fprintf(
        out, "/*******************************************************************************\n");
    fprintf(out, " * Size: %g px\n", font_size);
    fprintf(out, " * Bpp: %d\n", font->bpp);
    fprintf(out, " * Opts: --bpp %d --size %g", font->bpp, font_size);
    if (dpi > 0) {
        fprintf(out, " --dpi %d", dpi);
    }
    fprintf(out, " --font %s --range ", font_file_path);
    print_codepoint_ranges(out, font);
    fprintf(out, " --format lvgl -o %s\n", out_file);
    fprintf(
        out,
        " ******************************************************************************/\n\n");

    fprintf(out, "#ifdef __has_include\n");
    fprintf(out, "    #if __has_include(\"lvgl.h\")\n");
    fprintf(out, "        #ifndef LV_LVGL_H_INCLUDE_SIMPLE\n");
    fprintf(out, "            #define LV_LVGL_H_INCLUDE_SIMPLE\n");
    fprintf(out, "        #endif\n");
    fprintf(out, "    #endif\n");
    fprintf(out, "#endif\n\n");
    fprintf(out, "#ifdef LV_LVGL_H_INCLUDE_SIMPLE\n");
    fprintf(out, "    #include \"lvgl.h\"\n");
    fprintf(out, "#else\n");
    fprintf(out, "    #include \"lvgl/lvgl.h\"\n");
    fprintf(out, "#endif\n\n");
    fprintf(out, "#include \"%s\"\n\n", header_basename);

    fprintf(out, "#ifndef %s\n", symbol_upper);
    fprintf(out, "#define %s 1\n", symbol_upper);
    fprintf(out, "#endif\n\n");
    fprintf(out, "#if %s\n\n", symbol_upper);

    fprintf(out, "static LV_ATTRIBUTE_LARGE_CONST const uint8_t glyph_bitmap[] = {\n");
    if (font->bitmap_size == 0) {
        fprintf(out, "    0x00\n");
    } else {
        for (size_t i = 0; i < font->glyph_count; i++) {
            const Glyph* glyph = &font->glyphs[i];
            print_glyph_comment(out, glyph->codepoint);
            if (glyph->bitmap_size > 0) {
                print_bytes(out, glyph->bitmap, glyph->bitmap_size);
            }
            if (i + 1 < font->glyph_count && glyph->bitmap_size > 0) {
                fprintf(out, "\n");
            }
        }
    }
    fprintf(out, "};\n\n");

    fprintf(out, "static const lv_font_fmt_txt_glyph_dsc_t glyph_dsc[] = {\n");
    fprintf(
        out,
        "    {.bitmap_index = 0, .adv_w = 0, .box_w = 0, .box_h = 0, .ofs_x = 0, "
        ".ofs_y = 0} /* id = 0 reserved */,\n");
    for (size_t i = 0; i < font->glyph_count; i++) {
        const Glyph* glyph = &font->glyphs[i];
        fprintf(
            out,
            "    {.bitmap_index = %zu, .adv_w = %d, .box_w = %d, .box_h = %d, "
            ".ofs_x = %d, .ofs_y = %d} /* U+%04X */%s\n",
            glyph->bitmap_index,
            glyph->adv_w << 4,
            glyph->box_w,
            glyph->box_h,
            glyph->ofs_x,
            glyph->ofs_y,
            glyph->codepoint,
            i + 1 < font->glyph_count ? "," : "");
    }
    fprintf(out, "};\n\n");

    fprintf(out, "static const lv_font_fmt_txt_cmap_t cmaps[] = {\n");
    for (size_t i = 0; i < font->cmap_count; i++) {
        fprintf(
            out,
            "    {.range_start = %u, .range_length = %u, .glyph_id_start = %u,\n"
            "     .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0,\n"
            "     .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY}%s\n",
            font->cmaps[i].range_start,
            font->cmaps[i].range_length,
            font->cmaps[i].glyph_id_start,
            i + 1 < font->cmap_count ? "," : "");
    }
    fprintf(out, "};\n\n");

    fprintf(out, "static const lv_font_fmt_txt_dsc_t font_dsc = {\n");
    fprintf(out, "    .glyph_bitmap = glyph_bitmap,\n");
    fprintf(out, "    .glyph_dsc = glyph_dsc,\n");
    fprintf(out, "    .cmaps = cmaps,\n");
    fprintf(out, "    .kern_dsc = NULL,\n");
    fprintf(out, "    .kern_scale = 0,\n");
    fprintf(out, "    .cmap_num = %zu,\n", font->cmap_count);
    fprintf(out, "    .bpp = %d,\n", font->bpp);
    fprintf(out, "    .kern_classes = 0,\n");
    fprintf(out, "    .bitmap_format = 0,\n");
    fprintf(out, "    .stride = 0,\n");
    fprintf(out, "};\n\n");

    fprintf(out, "const UG_FONT %s = {\n", public_symbol);
    fprintf(out, "    .get_glyph_dsc = lv_font_get_glyph_dsc_fmt_txt,\n");
    fprintf(out, "    .get_glyph_bitmap = lv_font_get_bitmap_fmt_txt,\n");
    fprintf(out, "    .line_height = %d,\n", font->line_height);
    fprintf(out, "    .base_line = %d,\n", font->line_height - font->baseline);
    fprintf(out, "    .subpx = LV_FONT_SUBPX_NONE,\n");
    fprintf(out, "    .underline_position = -1,\n");
    fprintf(out, "    .underline_thickness = 0,\n");
    fprintf(out, "    .static_bitmap = 0,\n");
    fprintf(out, "    .dsc = &font_dsc,\n");
    fprintf(out, "    .fallback = NULL,\n");
    fprintf(out, "    .user_data = NULL,\n");
    fprintf(out, "};\n\n");
    fprintf(out, "#endif /* %s */\n", symbol_upper);
    fclose(out);

    out = fopen(header_file, "w");
    if (out == NULL) {
        perror(header_file);
        exit(2);
    }
    fprintf(out, "#ifndef _%s_H_\n", symbol_upper);
    fprintf(out, "#define _%s_H_\n\n", symbol_upper);
    fprintf(out, "#include <ugui.h>\n\n");
    fprintf(out, "extern const UG_FONT %s;\n\n", public_symbol);
    fprintf(out, "#endif\n");
    fclose(out);

    free(public_symbol);
    free(symbol);
    free(symbol_upper);
    free(out_file);
    free(header_file);
}

static const Glyph* glyph_for_codepoint(const LvglFont* font, UChar32 codepoint)
{
    size_t left = 0;
    size_t right = font->codepoint_count;
    while (left < right) {
        size_t mid = left + (right - left) / 2;
        if (font->codepoints[mid] == codepoint) {
            return &font->glyphs[mid];
        }
        if (font->codepoints[mid] < codepoint) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return NULL;
}

static uint8_t lvgl_opacity(const Glyph* glyph, int bits_per_pixel, int x, int y)
{
    if (x < 0 || y < 0 || x >= glyph->box_w || y >= glyph->box_h || glyph->bitmap_size == 0) {
        return 0;
    }
    if (bits_per_pixel == 8) {
        return glyph->bitmap[(size_t)y * (size_t)glyph->box_w + (size_t)x];
    }

    size_t bit_index = (size_t)y * (size_t)glyph->box_w + (size_t)x;
    return (glyph->bitmap[bit_index / 8] & (1 << (7 - (bit_index % 8)))) != 0 ? 255 : 0;
}

static void show_font(const LvglFont* font, const Codepoints* text)
{
    uint8_t screen[SCREEN_HEIGHT][SCREEN_WIDTH];
    memset(screen, 0, sizeof(screen));

    for (int x = 0; x < SCREEN_WIDTH; x++) {
        screen[0][x] = 255;
        screen[SCREEN_HEIGHT - 1][x] = 255;
    }
    for (int y = 0; y < SCREEN_HEIGHT; y++) {
        screen[y][0] = 255;
        screen[y][SCREEN_WIDTH - 1] = 255;
    }

    int pen_x = 2;
    int pen_y = 2;
    int baseline_y = font->baseline;

    for (size_t i = 0; i < text->count; i++) {
        const Glyph* glyph = glyph_for_codepoint(font, text->codepoints[i]);
        if (glyph == NULL) {
            continue;
        }

        int glyph_x = pen_x + glyph->ofs_x;
        int glyph_y = pen_y + baseline_y - glyph->box_h - glyph->ofs_y;
        for (int y = 0; y < glyph->box_h; y++) {
            for (int x = 0; x < glyph->box_w; x++) {
                uint8_t opacity = lvgl_opacity(glyph, font->bpp, x, y);
                int out_x = glyph_x + x;
                int out_y = glyph_y + y;
                if (opacity == 0 || out_x < 0 || out_y < 0 || out_x >= SCREEN_WIDTH ||
                    out_y >= SCREEN_HEIGHT) {
                    continue;
                }
                screen[out_y][out_x] = opacity;
            }
        }
        pen_x += glyph->adv_w;
    }

    for (int y = 0; y < SCREEN_HEIGHT; y++) {
        for (int x = 0; x < SCREEN_WIDTH; x++) {
            if (screen[y][x] == 0) {
                putchar(' ');
            } else if (screen[y][x] == 255) {
                putchar('*');
            } else {
                putchar('+');
            }
        }
        putchar('\n');
    }
}

static void usage(void)
{
    fprintf(
        stderr,
        "ttf2lvgl {--show text|--dump} --font=fontfile [--dpi=displaydpi] "
        "--size=fontsize [--bpp=bitsperpixel] [--range=RANGES] "
        "[--name=symbol] [--output=file.c]\n");
    fprintf(
        stderr,
        "Ranges are comma-separated code points or ranges, e.g. 32-127,160,U+0100-U+017F.\n");
    fprintf(stderr, "If --dpi is not given, font size is assumed to be pixels.\n");
    fprintf(stderr, "Bits per pixel must be 1 or 8. Default is 1.\n");
}

static struct option longopts[] = {
    {"show", required_argument, NULL, 'a'},
    {"dump", no_argument, &dump, 1},
    {"dpi", required_argument, NULL, 'd'},
    {"minchar", required_argument, NULL, 'z'},
    {"maxchar", required_argument, NULL, 'e'},
    {"size", required_argument, NULL, 's'},
    {"font", required_argument, NULL, 'f'},
    {"bpp", required_argument, NULL, 'b'},
    {"range", required_argument, NULL, 'r'},
    {"name", required_argument, NULL, 'n'},
    {"output", required_argument, NULL, 'o'},
    {NULL, 0, NULL, 0},
};

int main(int argc, char** argv)
{
    int ch;
    while ((ch = getopt_long(argc, argv, "", longopts, NULL)) != -1) {
        switch (ch) {
        case 'f':
            font_file = optarg;
            break;
        case 'a':
            show_text = optarg;
            break;
        case 's':
            sscanf(optarg, "%f", &font_size);
            break;
        case 'd':
            dpi = atoi(optarg);
            break;
        case 'b':
            bpp = atoi(optarg);
            if (bpp != 1 && bpp != 8) {
                fprintf(stderr, "Bits per pixel must be 1 or 8. Default is 1.\n");
                exit(1);
            }
            break;
        case 'z':
            min_char = atoi(optarg);
            minmax_used = true;
            break;
        case 'e':
            max_char = atoi(optarg);
            minmax_used = true;
            break;
        case 'r':
            range_used = true;
            parse_codepoint_ranges(optarg, &range_codepoints);
            break;
        case 'n':
            font_name_arg = optarg;
            break;
        case 'o':
            output_file_arg = optarg;
            break;
        case 0:
            break;
        default:
            usage();
            exit(1);
        }
    }

    if (range_used && minmax_used) {
        fprintf(stderr, "--range cannot be combined with --minchar or --maxchar\n");
        exit(1);
    }
    if (!range_used) {
        append_legacy_codepoint_range(&range_codepoints);
    }
    codepoints_sort_dedup(&range_codepoints);

    if ((!dump && show_text == NULL) || font_file == NULL || font_size == 0 ||
        range_codepoints.count == 0) {
        usage();
        exit(1);
    }

    LvglFont* font = convert_font(font_file, dpi, font_size, bpp, &range_codepoints);

    if (show_text != NULL) {
        Codepoints show_codepoints = utf8_to_codepoints(show_text);
        show_font(font, &show_codepoints);
        free(show_codepoints.codepoints);
    }

    if (dump) {
        dump_font(font, font_file, font_name_arg);
    }

    for (size_t i = 0; i < font->glyph_count; i++) {
        free(font->glyphs[i].bitmap);
    }
    free(font->glyphs);
    free(font->codepoints);
    free(font->cmaps);
    free(font);
    free(range_codepoints.codepoints);
}
