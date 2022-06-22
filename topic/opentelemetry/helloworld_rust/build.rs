// pub mod helloworld {
//     tonic::include_proto!("helloworld");
// }

// 这种不在本目录下生成文件，只有再cargo build/run的时候再对应的target/debug/helloworld_rust_helloworld.rs 有问津
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}

// 下面这种是直接生成文件，并放到src目录下
// use std::{env, path::PathBuf};
//
// fn main() {
//     let proto_file = "proto/helloworld.proto";
//     let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()); // Add this
//
//     tonic_build::configure()
//         .build_server(true)
//         .file_descriptor_set_path(out_dir.join("greeter_descriptor.bin")) // Add this
//         .out_dir("./src")
//         .compile(&[proto_file], &["."])
//         .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
//
//     println!("cargo:rerun-if-changed={}", proto_file);
// }
