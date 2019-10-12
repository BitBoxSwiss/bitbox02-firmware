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

///
/// This file generates rust bindings for c-functions that are needed.
///
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    // OUT_DIR is always set by cargo, so it is safe to unwrap here
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("delay_us")
        .whitelist_function("delay_ms")
        .whitelist_function("UG_PutString")
        .whitelist_function("UG_FontSelect")
        .whitelist_function("UG_ClearBuffer")
        .whitelist_function("UG_SendBuffer")
        .whitelist_function("workflow_confirm")
        .whitelist_function("ui_screen_stack_push")
        .whitelist_function("ui_screen_stack_pop")
        .whitelist_function("label_create")
        .whitelist_function("bitboxbase_watchdog_reset")
        .whitelist_function("leds_turn_small_led")
        .whitelist_function("leds_turn_big_led")
        .whitelist_function("wally_sha256")
        .whitelist_type("component_t")
        .whitelist_type("BitBoxBaseRequest")
        .whitelist_var(".*_tag")
        .whitelist_var("font_font_a_9X9")
        .whitelist_var("WALLY_OK")
        .use_core()
        .ctypes_prefix("c_types")
        .clang_arg("-D__SAMD51J20A__")
        .clang_arg("-DPB_NO_PACKED_STRUCTS=1")
        .clang_arg("-DPB_FIELD_16BIT=1")
        .clang_arg("-fshort-enums");

    // Fetch sysroot from the environment. If there isn't any sysroot assume we are building for
    // testing
    let sysroot = env::var_os("SYSROOT").map(|s| {
        let mut sysroot = String::from("--sysroot=");
        sysroot.push_str(s.to_str().expect("invalid utf-8"));
        sysroot
    });
    if let Some(sysroot) = sysroot {
        bindings = bindings.clang_args(&[
            &sysroot,
            "--target=thumbv7em-none-eabi",
            "-mcpu=cortex-m4",
            "-mthumb",
            "-mfloat-abi=soft",
        ]);
    } else {
        bindings = bindings.clang_arg("-DTESTING")
    }

    if let Some(includes) = env::var_os("INCLUDES") {
        for include in includes.to_str().expect("invalid utf-8").split_whitespace() {
            let mut dir = String::from("-I");
            dir.push_str(include);
            dbg!(&dir);
            bindings = bindings.clang_arg(&dir);
        }
    }

    let bindings = bindings.generate().expect("Unable to generate bindings!");

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
