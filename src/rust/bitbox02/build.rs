fn main() {
    #[cfg(feature = "testing")]
    {
        // CMAKE_CURRENT_BINARY_DIR is required for building tests
        if let Ok(cmake_dir) = std::env::var("CMAKE_CURRENT_BINARY_DIR") {
            println!("cargo:rustc-link-search={}/../lib", cmake_dir);
            // c and rust code merged :O
            println!("cargo:rustc-link-lib=bitbox_merged");
            println!(
                "cargo:rerun-if-changed={}/../lib/libbitbox_merged.a",
                cmake_dir
            );

            // external libs
            println!("cargo:rustc-link-lib=wallycore");
            println!("cargo:rustc-link-lib=secp256k1");
            println!("cargo:rustc-link-lib=base32");
            println!("cargo:rustc-link-lib=ctaes");
            println!("cargo:rustc-link-lib=fatfs");
            println!("cargo:rustc-link-lib=sd-mock");

            // system libs
            println!("cargo:rustc-link-lib=cmocka");
        }
    }
}
