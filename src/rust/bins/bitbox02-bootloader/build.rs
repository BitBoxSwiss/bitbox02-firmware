use std::env;
use std::fs;
use std::path::{Path, PathBuf};

type BuildResult<T> = Result<T, String>;

const TARGET_FEATURES: &[(&str, &str)] = &[
    ("target-bb02-bl-multi", "bb02-bl-multi"),
    (
        "target-bb02-bl-multi-development",
        "bb02-bl-multi-development",
    ),
    (
        "target-bb02-bl-multi-development-locked",
        "bb02-bl-multi-development-locked",
    ),
    (
        "target-bb02-bl-multi-production",
        "bb02-bl-multi-production",
    ),
    ("target-bb02-bl-btconly", "bb02-bl-btconly"),
    (
        "target-bb02-bl-btconly-development",
        "bb02-bl-btconly-development",
    ),
    (
        "target-bb02-bl-btconly-production",
        "bb02-bl-btconly-production",
    ),
    ("target-bb02p-bl-multi", "bb02p-bl-multi"),
    (
        "target-bb02p-bl-multi-development",
        "bb02p-bl-multi-development",
    ),
    (
        "target-bb02p-bl-multi-development-locked",
        "bb02p-bl-multi-development-locked",
    ),
    (
        "target-bb02p-bl-multi-production",
        "bb02p-bl-multi-production",
    ),
    ("target-bb02p-bl-btconly", "bb02p-bl-btconly"),
    (
        "target-bb02p-bl-btconly-development",
        "bb02p-bl-btconly-development",
    ),
    (
        "target-bb02p-bl-btconly-production",
        "bb02p-bl-btconly-production",
    ),
];

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
    emit_rerun_if_changed_path(&repo_root.join("bootloader.ld"));
    emit_rerun_if_changed_path(&repo_root.join("external/asf4-drivers"));

    copy_linker_script(&repo_root, &out_dir)?;
    emit_linker_args(&out_dir, &repo_root, variant)?;

    Ok(())
}

fn variant() -> BuildResult<&'static str> {
    let enabled: Vec<&'static str> = TARGET_FEATURES
        .iter()
        .filter_map(|(feature, variant)| {
            let env_var = format!(
                "CARGO_FEATURE_{}",
                feature.replace('-', "_").to_ascii_uppercase()
            );
            env::var_os(env_var).map(|_| *variant)
        })
        .collect();

    match enabled.as_slice() {
        [variant] => Ok(*variant),
        [] => Err("no bootloader target feature is enabled".to_string()),
        _ => Err("multiple bootloader target features are enabled".to_string()),
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
    println!("cargo::rustc-link-arg=-Wl,--defsym=STACK_SIZE=0x10000");
    println!("cargo::rustc-link-arg=-Wl,--defsym=HEAP_SIZE=0x18000");

    if uses_qtouch() {
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
    }

    println!("cargo::rustc-link-arg=-lc");
    println!("cargo::rustc-link-arg=-lnosys");
    println!("cargo::rustc-link-arg=-lm");
    println!("cargo::rustc-link-arg=-lgcc");
    Ok(())
}

fn uses_qtouch() -> bool {
    env::var_os("CARGO_FEATURE_BOOTLOADER_DEVDEVICE").is_some()
        || env::var_os("CARGO_FEATURE_PLATFORM_BITBOX02PLUS").is_some()
}

fn copy_linker_script(repo_root: &Path, out_dir: &Path) -> BuildResult<()> {
    let src = repo_root.join("bootloader.ld");
    let dst = out_dir.join("bootloader.ld");
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
