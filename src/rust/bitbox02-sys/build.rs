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

    let sysroot_env = &PathBuf::from(env::var_os("SYSROOT").expect("No sysroot"));
    let sysroot = {
        let mut sysroot = String::from("--sysroot=");
        sysroot.push_str(sysroot_env.to_str().expect("invalid utf-8"));
        sysroot
    };

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("delay_ms")
        .whitelist_function("delay_us")
        .whitelist_function("UG_PutString")
        .whitelist_function("UG_FontSelect")
        .whitelist_function("UG_ClearBuffer")
        .whitelist_function("UG_SendBuffer")
        .whitelist_function("workflow_confirm")
        .whitelist_var("font_font_a_9X9")
        .use_core()
        .ctypes_prefix("c_types")
        .clang_arg(&sysroot)
        .clang_arg("-D__SAMD51J20A__");

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
