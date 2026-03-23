// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const MAX_BACKUP_GENERATOR_LEN: usize = 19;

fn main() {
    let version = firmware_version_short();
    println!("cargo::rustc-env=FIRMWARE_VERSION_SHORT={version}");
}

fn firmware_version_short() -> String {
    let manifest_path = versions_manifest_path();
    println!("cargo::rerun-if-changed={}", manifest_path.display());

    let manifest = fs::read_to_string(&manifest_path).unwrap_or_else(|err| {
        panic!(
            "failed to read versions manifest at {}: {err}",
            manifest_path.display()
        )
    });
    let manifest: serde_json::Value = serde_json::from_str(&manifest).unwrap_or_else(|err| {
        panic!(
            "failed to parse versions manifest at {}: {err}",
            manifest_path.display()
        )
    });

    let version = manifest
        .get("firmware")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| {
            panic!(
                "versions manifest at {} does not contain a string 'firmware' entry",
                manifest_path.display()
            )
        });

    if !is_release_version(version) {
        panic!(
            "versions manifest entry 'firmware' must be a release semver like v1.2.3, got: {}",
            version
        );
    }
    if version.len() > MAX_BACKUP_GENERATOR_LEN {
        panic!(
            "firmware version '{}' exceeds backup generator limit of {} bytes",
            version, MAX_BACKUP_GENERATOR_LEN
        );
    }

    version.to_owned()
}

fn versions_manifest_path() -> PathBuf {
    Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../../versions.json")
}

fn is_release_version(version: &str) -> bool {
    let Some(version) = version.strip_prefix('v') else {
        return false;
    };
    let mut parts = version.split('.');
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(major), Some(minor), Some(patch), None) => {
            is_numeric_identifier(major)
                && is_numeric_identifier(minor)
                && is_numeric_identifier(patch)
        }
        _ => false,
    }
}

fn is_numeric_identifier(part: &str) -> bool {
    !part.is_empty()
        && part.bytes().all(|byte| byte.is_ascii_digit())
        && (part == "0" || !part.starts_with('0'))
}
