fn main() {
    #[cfg(feature = "testing")]
    {
        let cmake_dir = std::env::var("CMAKE_CURRENT_BINARY_DIR").unwrap();
        println!("cargo:rustc-link-search={}/../lib", cmake_dir);
        // c and rust code merged :O
        println!("cargo:rustc-link-lib=bitbox_merged");

        // external libs
        println!("cargo:rustc-link-lib=wallycore");
        println!("cargo:rustc-link-lib=secp256k1");
        println!("cargo:rustc-link-lib=bignum");
        println!("cargo:rustc-link-lib=sha3");
        println!("cargo:rustc-link-lib=base32");
        println!("cargo:rustc-link-lib=ctaes");

        // system libs
        println!("cargo:rustc-link-lib=cmocka");
    }
}
