use std::env;
use std::fs;
use std::path::{Path, PathBuf};

type BuildResult<T> = Result<T, String>;

fn main() -> BuildResult<()> {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let repo_root = manifest_dir
        .join("../../../..")
        .canonicalize()
        .map_err(|err| format!("failed to resolve repo root: {err}"))?;
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let variant = variant()?;

    emit_rerun_if_changed_path(&manifest_dir.join("build.rs"));
    emit_rerun_if_changed_path(&manifest_dir.join("Cargo.toml"));
    emit_rerun_if_changed_path(&repo_root.join("external/asf4-drivers"));

    copy_linker_script(&manifest_dir, &out_dir)?;
    emit_linker_args(&out_dir, &repo_root, variant)?;

    Ok(())
}

fn variant() -> BuildResult<&'static str> {
    let multi = env::var_os("CARGO_FEATURE_MULTI").is_some();
    let btc_only = env::var_os("CARGO_FEATURE_BTC_ONLY").is_some();
    match (multi, btc_only) {
        (true, false) => Ok("firmware"),
        (false, true) => Ok("firmware-btc"),
        (true, true) => Err("both `multi` and `btc-only` features are enabled".to_string()),
        (false, false) => Err("no firmware variant feature is enabled".to_string()),
    }
}

fn emit_linker_args(out_dir: &Path, repo_root: &Path, variant: &str) -> BuildResult<()> {
    println!("cargo::rustc-link-search={}", out_dir.display());
    let profile_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("OUT_DIR did not contain a Cargo profile directory");

    println!(
        "cargo::rustc-link-arg=-Wl,-Map={}",
        profile_dir.join(format!("{variant}.map")).display()
    );

    let qtouch_dir = repo_root.join("external/asf4-drivers/qtouch/lib/gcc");
    for lib in [
        "qtm_acq_samd51_0x000f",
        "qtm_binding_layer_cm4_0x0005",
        "qtm_touch_key_cm4_0x0002",
    ] {
        let qtouch_lib = qtouch_dir.join(format!("lib{lib}.a"));
        emit_rerun_if_changed_path(&qtouch_lib);
        println!("cargo::rustc-link-arg={}", qtouch_lib.display());
    }
    println!("cargo::rustc-link-arg=-lc");
    println!("cargo::rustc-link-arg=-lnosys");
    println!("cargo::rustc-link-arg=-lm");
    println!("cargo::rustc-link-arg=-lgcc");
    Ok(())
}

fn copy_linker_script(manifest_dir: &Path, out_dir: &Path) -> BuildResult<()> {
    let src = manifest_dir.join("bitbox02-firmware.ld");
    let dst = out_dir.join("bitbox02-firmware.ld");
    emit_rerun_if_changed_path(&src);
    fs::copy(&src, &dst).map_err(|err| {
        format!(
            "failed to copy linker script {} -> {}: {err}",
            src.display(),
            dst.display()
        )
    })?;
    Ok(())
}

fn emit_rerun_if_changed_path(path: &Path) {
    println!("cargo::rerun-if-changed={}", path.display());
}
