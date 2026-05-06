// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

const LVGL_C_FILES: &[&str] = &[
    // Core
    "src/core/lv_group.c",
    "src/core/lv_obj.c",
    "src/core/lv_obj_class.c",
    "src/core/lv_obj_draw.c",
    "src/core/lv_obj_event.c",
    "src/core/lv_obj_id_builtin.c",
    "src/core/lv_obj_pos.c",
    "src/core/lv_obj_property.c",
    "src/core/lv_obj_scroll.c",
    "src/core/lv_obj_style.c",
    "src/core/lv_obj_style_gen.c",
    "src/core/lv_obj_tree.c",
    "src/core/lv_refr.c",
    // Display
    "src/display/lv_display.c",
    // Draw
    "src/draw/convert/lv_draw_buf_convert.c",
    "src/draw/lv_draw.c",
    "src/draw/lv_draw_arc.c",
    "src/draw/lv_draw_buf.c",
    "src/draw/lv_draw_image.c",
    "src/draw/lv_draw_label.c",
    "src/draw/lv_draw_line.c",
    "src/draw/lv_draw_mask.c",
    "src/draw/lv_draw_rect.c",
    "src/draw/lv_draw_triangle.c",
    "src/draw/lv_image_decoder.c",
    "src/draw/sw/blend/lv_draw_sw_blend.c",
    "src/draw/sw/blend/lv_draw_sw_blend_to_al88.c",
    "src/draw/sw/blend/lv_draw_sw_blend_to_argb8888.c",
    "src/draw/sw/blend/lv_draw_sw_blend_to_argb8888_premultiplied.c",
    "src/draw/sw/blend/lv_draw_sw_blend_to_i1.c",
    "src/draw/sw/blend/lv_draw_sw_blend_to_l8.c",
    "src/draw/sw/blend/lv_draw_sw_blend_to_rgb565.c",
    "src/draw/sw/blend/lv_draw_sw_blend_to_rgb565_swapped.c",
    "src/draw/sw/blend/lv_draw_sw_blend_to_rgb888.c",
    "src/draw/sw/lv_draw_sw.c",
    "src/draw/sw/lv_draw_sw_arc.c",
    "src/draw/sw/lv_draw_sw_border.c",
    "src/draw/sw/lv_draw_sw_box_shadow.c",
    "src/draw/sw/lv_draw_sw_fill.c",
    "src/draw/sw/lv_draw_sw_grad.c",
    "src/draw/sw/lv_draw_sw_img.c",
    "src/draw/sw/lv_draw_sw_letter.c",
    "src/draw/sw/lv_draw_sw_line.c",
    "src/draw/sw/lv_draw_sw_mask.c",
    "src/draw/sw/lv_draw_sw_mask_rect.c",
    "src/draw/sw/lv_draw_sw_transform.c",
    "src/draw/sw/lv_draw_sw_triangle.c",
    "src/draw/sw/lv_draw_sw_utils.c",
    // Font
    "src/font/lv_font.c",
    "src/font/lv_font_fmt_txt.c",
    "src/font/lv_font_montserrat_14.c",
    // Input
    "src/indev/lv_indev.c",
    "src/indev/lv_indev_gesture.c",
    "src/indev/lv_indev_scroll.c",
    // Layouts
    "src/layouts/flex/lv_flex.c",
    "src/layouts/grid/lv_grid.c",
    "src/layouts/lv_layout.c",
    // Built-in binary image decoder
    "src/libs/bin_decoder/lv_bin_decoder.c",
    // LodePNG image decoder
    "src/libs/lodepng/lodepng.c",
    "src/libs/lodepng/lv_lodepng.c",
    // LVGL initialization
    "src/lv_init.c",
    // Cache
    "src/misc/cache/class/lv_cache_lru_ll.c",
    "src/misc/cache/class/lv_cache_lru_rb.c",
    "src/misc/cache/class/lv_cache_sc_da.c",
    "src/misc/cache/instance/lv_image_cache.c",
    "src/misc/cache/instance/lv_image_header_cache.c",
    "src/misc/cache/lv_cache.c",
    "src/misc/cache/lv_cache_entry.c",
    // Misc
    "src/misc/lv_anim.c",
    "src/misc/lv_anim_timeline.c",
    "src/misc/lv_area.c",
    "src/misc/lv_array.c",
    "src/misc/lv_async.c",
    "src/misc/lv_bidi.c",
    "src/misc/lv_circle_buf.c",
    "src/misc/lv_color.c",
    "src/misc/lv_color_op.c",
    "src/misc/lv_event.c",
    "src/misc/lv_fs.c",
    "src/misc/lv_grad.c",
    "src/misc/lv_iter.c",
    "src/misc/lv_ll.c",
    "src/misc/lv_log.c",
    "src/misc/lv_lru.c",
    "src/misc/lv_math.c",
    "src/misc/lv_matrix.c",
    "src/misc/lv_palette.c",
    "src/misc/lv_rb.c",
    "src/misc/lv_style.c",
    "src/misc/lv_style_gen.c",
    "src/misc/lv_templ.c",
    "src/misc/lv_text.c",
    "src/misc/lv_text_ap.c",
    "src/misc/lv_timer.c",
    "src/misc/lv_tree.c",
    "src/misc/lv_utils.c",
    // OS abstraction
    "src/osal/lv_os.c",
    "src/osal/lv_os_none.c",
    // Other enabled modules
    "src/others/observer/lv_observer.c",
    // Stdlib
    "src/stdlib/builtin/lv_mem_core_builtin.c",
    "src/stdlib/builtin/lv_sprintf_builtin.c",
    "src/stdlib/builtin/lv_string_builtin.c",
    "src/stdlib/builtin/lv_tlsf.c",
    "src/stdlib/lv_mem.c",
    // Themes enabled in lv_conf.h
    "src/themes/default/lv_theme_default.c",
    "src/themes/lv_theme.c",
    "src/themes/mono/lv_theme_mono.c",
    "src/themes/simple/lv_theme_simple.c",
    // Tick
    "src/tick/lv_tick.c",
    // Widget property-name tables
    "src/widgets/property/lv_animimage_properties.c",
    "src/widgets/property/lv_dropdown_properties.c",
    "src/widgets/property/lv_image_properties.c",
    "src/widgets/property/lv_keyboard_properties.c",
    "src/widgets/property/lv_label_properties.c",
    "src/widgets/property/lv_obj_properties.c",
    "src/widgets/property/lv_roller_properties.c",
    "src/widgets/property/lv_slider_properties.c",
    "src/widgets/property/lv_style_properties.c",
    "src/widgets/property/lv_textarea_properties.c",
    // Widgets enabled in lv_conf.h
    "src/widgets/animimage/lv_animimage.c",
    "src/widgets/arc/lv_arc.c",
    "src/widgets/arclabel/lv_arclabel.c",
    "src/widgets/bar/lv_bar.c",
    "src/widgets/button/lv_button.c",
    "src/widgets/buttonmatrix/lv_buttonmatrix.c",
    "src/widgets/calendar/lv_calendar.c",
    "src/widgets/calendar/lv_calendar_header_arrow.c",
    "src/widgets/calendar/lv_calendar_header_dropdown.c",
    "src/widgets/canvas/lv_canvas.c",
    "src/widgets/chart/lv_chart.c",
    "src/widgets/checkbox/lv_checkbox.c",
    "src/widgets/dropdown/lv_dropdown.c",
    "src/widgets/image/lv_image.c",
    "src/widgets/imagebutton/lv_imagebutton.c",
    "src/widgets/keyboard/lv_keyboard.c",
    "src/widgets/label/lv_label.c",
    "src/widgets/led/lv_led.c",
    "src/widgets/line/lv_line.c",
    "src/widgets/list/lv_list.c",
    "src/widgets/menu/lv_menu.c",
    "src/widgets/msgbox/lv_msgbox.c",
    "src/widgets/roller/lv_roller.c",
    "src/widgets/scale/lv_scale.c",
    "src/widgets/slider/lv_slider.c",
    "src/widgets/span/lv_span.c",
    "src/widgets/spinbox/lv_spinbox.c",
    "src/widgets/spinner/lv_spinner.c",
    "src/widgets/switch/lv_switch.c",
    "src/widgets/table/lv_table.c",
    "src/widgets/tabview/lv_tabview.c",
    "src/widgets/textarea/lv_textarea.c",
    "src/widgets/tileview/lv_tileview.c",
    "src/widgets/win/lv_win.c",
];

fn run_bindgen(wrapper: &Path, output: &Path, clang_args: &[String]) -> Result<(), &'static str> {
    let res = Command::new("bindgen")
        .arg("--output")
        .arg(output)
        .arg("--use-core")
        .arg("--with-derive-default")
        .arg("--rustified-enum")
        .arg(".*")
        .arg(wrapper)
        .arg("--")
        .args(clang_args)
        .output()
        .expect("failed to run bindgen");

    if !res.status.success() {
        println!(
            "bindgen-out:\n{}\n\nbindgen-err:\n{}",
            std::str::from_utf8(&res.stdout).unwrap_or("invalid utf8"),
            std::str::from_utf8(&res.stderr).unwrap_or("invalid utf8"),
        );
        return Err("bindgen failed");
    }
    Ok(())
}

fn main() -> Result<(), &'static str> {
    println!("cargo::rerun-if-changed=wrapper.h");

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let repo_root = manifest_dir.join("../../..");

    let wrapper = manifest_dir.join("wrapper.h");
    if !wrapper.is_file() {
        return Err("wrapper.h not found");
    }

    let lvgl_dir = repo_root.join("external/lvgl");
    let lvgl_header = lvgl_dir.join("lvgl.h");
    if !lvgl_header.is_file() {
        return Err(
            "external/lvgl/lvgl.h not found. Is the external/lvgl submodule initialized and checked out?",
        );
    }
    println!("cargo::rerun-if-changed={}", lvgl_header.display());
    println!("cargo::rerun-if-changed={}", lvgl_dir.display());

    let lv_conf = manifest_dir.join("lv_conf.h");
    println!("cargo::rerun-if-changed={}", lv_conf.display());

    let debug = env::var("PROFILE").unwrap() == "debug";

    if let Err(err) = Command::new("bindgen").arg("--version").output() {
        if err.kind() == ErrorKind::NotFound {
            return Err("`bindgen` executable was not found. Check your PATH.");
        }
        return Err("failed to execute `bindgen --version`");
    }

    let cflags = [
        format!("-I{}", lvgl_dir.display()),
        "-DLV_KCONFIG_IGNORE".to_owned(),
        "-DLV_LVGL_H_INCLUDE_SIMPLE".to_owned(),
        format!("-DLV_CONF_PATH=\"{}\"", lv_conf.display()),
        "-DLV_CONF_INCLUDE_SIMPLE".to_owned(),
    ];

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set")).join("bindings.rs");

    let mut lvgl_build = cc::Build::new();
    for source in LVGL_C_FILES {
        lvgl_build.file(lvgl_dir.join(source));
    }
    for flag in &cflags {
        lvgl_build.flag(flag);
    }
    lvgl_build.warnings(false);
    lvgl_build.extra_warnings(false);
    lvgl_build.debug(debug);
    lvgl_build.compile("lvgl");

    let mut fonts = cc::Build::new();
    fonts.file(manifest_dir.join("../../ui/fonts/inter_regular_32.c"));
    fonts.file(manifest_dir.join("../../ui/fonts/inter_regular_48.c"));
    fonts.file(manifest_dir.join("../../ui/fonts/inter_bold_32.c"));
    fonts.file(manifest_dir.join("../../ui/fonts/inter_bold_48.c"));
    for flag in &cflags {
        fonts.flag(flag);
    }
    fonts.compile("lvgl_fonts");
    run_bindgen(&wrapper, &out_path, &cflags)
}
