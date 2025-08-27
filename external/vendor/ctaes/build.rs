fn main() {
    cc::Build::new()
        .file("depend/ctaes/ctaes.c")
        .compile("ctaes_clib");
    println!("cargo:rerun-if-changed=depend/ctaes/ctaes.c");
    println!("cargo:rerun-if-changed=depend/ctaes/ctaes.h");
    println!("cargo:rustc-link-lib=static=ctaes_clib");
}
