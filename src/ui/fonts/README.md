# Supported BitBox02 characters

The BitBox02 regular text fonts support English, German, Spanish, Italian, and French,
among the most widely spoken languages in Europe.

The character set includes printable ASCII and the additional letters needed for these languages.
The language letters are based on the
[Unicode CLDR main exemplars](https://www.unicode.org/cldr/charts/48/by_type/core_data.alphabetic_information.main.html),
with uppercase forms included except capital sharp S (`ẞ`, U+1E9E), whose support varies across
fonts.
Non-breaking space (U+00A0) and soft hyphen (U+00AD) are excluded because they can look like an
ordinary space or hyphen.
This is a set of precomposed characters: input must use NFC for
accented letters; decomposed base letters followed by combining accents are not included.

The exact supported set is **162 characters**:

* All 95 printable ASCII characters, U+0020–U+007E, including ordinary space:

  ```text
  !"#$%&'()*+,-./0123456789:;<=>?@
  ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`
  abcdefghijklmnopqrstuvwxyz{|}~
  ```

* The following additional letters (overlap between languages is intentional):

  | Language | Uppercase | Lowercase |
  | --- | --- | --- |
  | English | None beyond ASCII `A–Z` | None beyond ASCII `a–z` |
  | German | `Ä Ö Ü` | `ä ö ü ß` |
  | Spanish | `Á É Í Ñ Ó Ú Ü` | `á é í ñ ó ú ü` |
  | Italian | `À È É Ì Ò Ó Ù` | `à è é ì ò ó ù` |
  | French | `À Â Æ Ç È É Ê Ë Î Ï Ô Œ Ù Û Ü Ÿ` | `à â æ ç è é ê ë î ï ô œ ù û ü ÿ` |

* Punctuation and symbols: `¡ § ª « ° º » ¿ – — ‚ „ … ‹ › €`.

Coverage by font:

| Fonts | Supported characters |
| --- | --- |
| `regular_9`, `regular_11`, `regular_12` | All 162 characters above |
| `password_9`, `password_12` | Same coverage as the corresponding regular font, with a visible space glyph |
| `monogram_16`, `regular_9_bootloader` | Printable ASCII only (U+0020–U+007E) |
| `monogram_16_bootloader` | Space, `0123456789`, and `abcdef` |

Monogram is for technical text; the bundled `monogram.ttf` has no accented letters. The bootloader
fonts intentionally retain their smaller subsets. Password fonts inherit the regular fonts through
their fallback pointers and do not need separate regeneration.

## Regenerating the regular text fonts

Use this exact decimal range list with `ttf2lvgl`; do not include whole Latin Unicode blocks:

```sh
FONT_RANGE='32-126,161,167,170-171,176,186-187,191-194,196,198-207,209-212,214,217-220,223-226,228,230-239,241-244,246,249-252,255,338-339,376,8211-8212,8218,8222,8230,8249-8250,8364'
```

For BitBox02, build `tools/ttf2lvgl`. Starting at the repository root, with `FONT_RANGE` set as above:

```sh
./scripts/dev_exec.sh make -C tools/ttf2lvgl
REGULAR_FONT_PATH=/path/to/regular.ttf
cd src/ui/fonts
for size in 9 11 12; do
    ../../../tools/ttf2lvgl/ttf2lvgl --dump --bpp 1 --size "$size" --dpi 72 \
        --font "$REGULAR_FONT_PATH" --range "$FONT_RANGE" \
        --name "regular_$size" --output "regular_$size.c"
done
```

After generation, retain these uGUI layout metrics in the public font descriptors:

| Font | `.line_height` | `.base_line` |
| --- | --- | --- |
| `regular_9` | 9 | 2 |
| `regular_11` | 10 | 2 |
| `regular_12` | 12 | 3 |

Preserve the checked-in ASCII glyph pixels and spacing when regenerating. These glyphs contain
manual bitmap or metric adjustments that raw FreeType output does not reproduce:

| Font | Adjusted glyphs |
| --- | --- |
| `regular_9` | `0 A V j t` |
| `regular_11` | `B W j t` |
| `regular_12` | `f` |

The ASCII rendering snapshots in `test/unit-test/test_ugui.c` verify the pixels, placement,
advance widths, and line heights, including the password-font fallbacks.

# ttf2lvgl

We convert fonts for the BitBox02 OLED with the ttf2lvgl tool provided in the `tools` directory.

Example execution:

```
./ttf2lvgl --dump --font <PATH-TO-FONT> --size <SIZE> --range <RANGES> \
  --name <FONT_SYMBOL> --output <FONT_SYMBOL>.c
```

`<RANGES>` is a comma-separated list of individual code points and ranges, for example
`32-126,161,U+0104-U+0107`.
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

Bitmap groups and glyph descriptors are annotated with their Unicode code points.
