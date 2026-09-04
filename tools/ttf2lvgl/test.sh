#!/bin/sh

set -eu

export LC_ALL=C

tool=./ttf2lvgl
font=../../src/ui/fonts/monogram.ttf
test_dir=$(mktemp -d "${TMPDIR:-/tmp}/ttf2lvgl.XXXXXX")
trap 'rm -r -- "$test_dir"' EXIT

expect_failure() {
    if "$@" >/dev/null 2>&1; then
        echo "unexpected success: $*" >&2
        exit 1
    fi
}

"$tool" \
    --dump \
    --font "$font" \
    --size 16 \
    --range '32,48-57,97-102' \
    --name test_font \
    --output "$test_dir/test_font.c"
test -s "$test_dir/test_font.c"
test -s "$test_dir/test_font.h"
grep -Fxq '#ifndef TTF2LVGL_TEST_FONT_H' "$test_dir/test_font.h"
grep -Fxq '#define TTF2LVGL_TEST_FONT_H' "$test_dir/test_font.h"
grep -Fxq '#endif /* TTF2LVGL_TEST_FONT_H */' "$test_dir/test_font.h"
if grep -n '[[:blank:]]$' "$test_dir/test_font.c"; then
    echo "generated source contains trailing whitespace" >&2
    exit 1
fi

"$tool" --show 'café' --font "$font" --size 16 --range '32-126,U+00E9' >/dev/null
"$tool" \
    --dump \
    --font "$font" \
    --size 16 \
    --bpp 8 \
    --range 65 \
    --name eight_bpp \
    --output "$test_dir/eight_bpp.c"
"$tool" \
    --dump \
    --font "$font" \
    --size 16 \
    --range 32 \
    --name space_only \
    --output "$test_dir/space_only.c"
grep -q '\.line_height = [1-9]' "$test_dir/space_only.c"

expect_failure "$tool" --show A --font "$font" --size=-1 --range 65
expect_failure "$tool" --show A --font "$font" --size=nan --range 65
expect_failure "$tool" --show A --font "$font" --size=1junk --range 65
expect_failure "$tool" --show A --font "$font" --size=5000 --range 65
expect_failure "$tool" --show A --font "$font" --size=16 --dpi=1junk --range 65
expect_failure "$tool" --show A --font "$font" --size=16 --range '65,,66'
expect_failure "$tool" --show A --font "$font" --size=16 --range U+D800
expect_failure "$tool" --show A --font "$font" --size=16 --range 65 unexpected
expect_failure \
    "$tool" --dump --font "$font" --size=16 --range 65 --output "$test_dir/collision.h"
expect_failure \
    "$tool" --dump --font "$font" --size=16 --range 65 --output "$test_dir/bad\"name.c"
