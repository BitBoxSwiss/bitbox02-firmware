use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let repo_root = manifest_dir.join("../../..");
    let lv_conf = repo_root.join("src/lvgl/lv_conf.h");
    println!("cargo::rustc-env=LV_CONF_PATH={}", lv_conf.display());
}
