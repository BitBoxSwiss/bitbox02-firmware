ttf2lvgl
========

`ttf2lvgl` converts fonts supported by FreeType into LVGL's uncompressed
`lv_font_fmt_txt` C format.

The rasterizer intentionally matches the old `ttf2ugui` tool:

* `--bpp=1` uses `FT_LOAD_RENDER | FT_LOAD_TARGET_MONO` and stores the same
  monochrome glyph pixels in LVGL bit order.
* `--bpp=8` renders the font at 16x size with the same monochrome target,
  downsamples every 16x16 block, and stores the same 0..255 coverage values.

Glyphs are emitted with tight bounding boxes. LVGL metrics (`ofs_x`, `ofs_y`,
`adv_w`, `line_height`, and `base_line`) preserve the FreeType placement.
Requested code points that are not present in the font's charmap are omitted
from the generated glyph descriptors and cmaps.
The `--show` preview decodes its input as UTF-8 with ICU before rendering.
Generated font objects use the local `lv_font_get_glyph_dsc_fmt_txt` and
`lv_font_get_bitmap_fmt_txt` callbacks. The BitBox02 OLED implementation of
those callbacks only supports 1bpp, unstrided, no-kerning, `FORMAT0_TINY`
fonts. Dumped fonts are exported as `const UG_FONT font_<name>` and the
generated header includes `<ugui.h>` and declares that `UG_FONT` object. If
`--name` already starts with `font_`, the public symbol is not double-prefixed.

Examples:

```
./ttf2lvgl --font Luna.ttf --dpi 140 --size 14 --range 32-127 --dump
./ttf2lvgl --font Luna.ttf --dpi 140 --size 14 --range 32-127 --show "aString"
./ttf2lvgl --font Luna.ttf --dpi 140 --size 14 --range 32-127 --dump --bpp=8
./ttf2lvgl --font Luna.ttf --size 14 --range 32-127,160,U+0100-U+017F \
  --dump --name luna_14 --output luna_14.c
```

If `--range` is omitted, the legacy `--minchar`/`--maxchar` range is used,
defaulting to `32-126`.
If `--name` is omitted, the base name uses the lowercased font file basename and
requested font size. If `--name` is given, it must be a valid C identifier and
is used as the base name. If `--output` is omitted, the file name follows the
base name.

Compiling
---------

FreeType and ICU development headers are required. On systems with
`pkg-config`, run:

```
make
```

The Makefile falls back to the common FreeType include and library paths if
`pkg-config freetype2` is unavailable, and to `-licuuc -licudata` if
`pkg-config icu-uc` is unavailable.
