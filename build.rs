fn main() {
    // Compile the proto files
    println!("cargo:rerun-if-changed=proto/aura.proto");
    tonic_build::compile_protos("proto/aura.proto").unwrap();
}
