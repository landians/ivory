fn main() {
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    config
        .out_dir("src/proto")
        .compile_protos(&["src/proto/msg.proto"], &["."])
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=msg.proto");
}
