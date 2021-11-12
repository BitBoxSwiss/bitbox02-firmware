fn main() {
    prost_build::compile_protos(&["hww.proto"], &["../../../messages"]).unwrap();
}
