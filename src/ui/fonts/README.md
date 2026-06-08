# ttf2lvgl

We convert LVGL fonts in this directory with the ttf2lvgl tool provided in the `tools` directory.

Example execution:

```
./ttf2lvgl --dump --font <PATH-TO-FONT> --size <SIZE> --range <RANGES> \
  --name <FONT_SYMBOL> --output <FONT_SYMBOL>.c
```

`<RANGES>` is a comma-separated list of individual code points and ranges, for example
`32-127,160,U+0100-U+017F`.
Code points in the requested ranges that are not present in the font are omitted from the generated
LVGL cmaps.

To check the conversion of the font to bitmap you can use the `--show <STRING>` parameter. The tool
decodes `<STRING>` as UTF-8 and prints it with asterixes in the terminal:

```
./ttf2lvgl --show ABCDEF --font <PATH-TO-FONT> --size <SIZE> --range <RANGES>
```

Once you have `dump`ed the font there will be a `.c` and `.h` file in the current directory. Move
these files to this directory. BitBox02 OLED fonts are exported directly as `font_*` LVGL font
objects and the generated headers include `<ugui.h>` and declare `extern const UG_FONT font_*`.
`UG_FONT` is an alias for `lv_font_t`, so no separate uGUI wrapper is needed.

For BitBox02 uGUI use, keep the generated font in the compact subset consumed by `ugui.c`: 1bpp,
`stride = 0`, no kerning, and `LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY` cmaps only. The local
`lv_font_get_glyph_dsc_fmt_txt` and `lv_font_get_bitmap_fmt_txt` callbacks only support this subset.
If the historical uGUI layout height differs from the generated FreeType line height, set
`.line_height` to the uGUI layout height.

## libfreetype note

Since `ttf2lvgl` uses `libfreetype` it actually supports the following font types:

* TrueType fonts (TTF) and TrueType collections (TTC)
* CFF fonts
* WOFF fonts
* OpenType fonts (OTF, both TrueType and CFF variants) and OpenType collections (OTC)
* Type 1 fonts (PFA and PFB)
* CID-keyed Type 1 fonts
* SFNT-based bitmap fonts, including color Emoji
* X11 PCF fonts
* Windows FNT fonts
* BDF fonts (including anti-aliased ones)
* PFR fonts
* Type 42 fonts (limited support)

# LVGL font format

The tool emits LVGL's uncompressed `lv_font_fmt_txt` format with tight glyph bounding boxes. For
1bpp fonts, pixels are packed in LVGL bit order over the tight glyph bitmap. For 8bpp fonts, each
pixel stores the same 16x16 downsampled coverage value that the old `ttf2ugui` tool produced.

Each item in the array is suffixed with a comment that shows which character the bytes correspond
to.
