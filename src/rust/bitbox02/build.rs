fn main() {
    #[cfg(feature = "testing")]
    {
        if let Ok(cmake_dir) = std::env::var("CMAKE_CURRENT_BINARY_DIR") {
            println!("cargo::rustc-link-search={}/../lib", cmake_dir);
            // c and rust code merged :O
            println!("cargo::rustc-link-lib=bitbox_merged");
            println!(
                "cargo::rerun-if-changed={}/../lib/libbitbox_merged.a",
                cmake_dir
            );
            println!("cargo::rerun-if-changed=build.rs");

            // external libs
            println!("cargo::rustc-link-lib=wallycore");
            println!("cargo::rustc-link-lib=secp256k1");
            println!("cargo::rustc-link-lib=ctaes");
            println!("cargo::rustc-link-lib=fatfs");
            println!("cargo::rustc-link-lib=sd-mock");

            // system libs
            println!("cargo::rustc-link-lib=cmocka");
        } else {
            // This is useful in case project is built by tool that doesn't need to link the final
            // target, like rust-analyzer and clippy.
            println!(
                "cargo::warning=Missing env variable CMAKE_CURRENT_BINARY_DIR, linking will fail"
            );
        }
    }
}
