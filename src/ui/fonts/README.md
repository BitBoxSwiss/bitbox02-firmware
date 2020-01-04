# ttf2ugui

We have converted the fonts in this directory with the ttf2ugui tool provided in `tools` directory.

Example execution:

```
./ttf2ugui --dump --font <PATH-TO-FONT>  --size <SIZE>
```

To check the conversion of the font to bitmap you can use the `--show <STRING>` parameter. The tool
will then print out the `<STRING>` with asterixes in the terminal:

```
./ttf2ugui --show ABCDEF --font <PATH-TO-FONT>  --size <SIZE>
```

Once you have `dump`ed the font there will be a `.c` and `.h` file in the current directory named
somthing like `FontName_<WIDTH>X<HEIGHT>`. Move this file to this directory and make some small
adjustments to it so that it compiles.

## libfreetype note

Since `ttf2ugui` uses `libfreetype` it actually supports the following font types:

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

# ugui font format

The bitmaps are layed out line by line and every line uses `ceil(width/8)` bytes. A 9 pixel wide
font will use 16 bits per line (7 bits per line are not used). See for example underscore `_` and
exclamation mark `!` to get a good understanding of the layout.

Each item in the array is suffixed with a comment that shows which character the bytes correspond
to.
