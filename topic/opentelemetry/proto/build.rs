// pub mod helloworld {
//     tonic::include_proto!("helloworld");
// }

use std::path::PathBuf;

// 这种不在本目录下生成文件，只有再cargo build/run的时候再对应的target/debug/helloworld_rust_helloworld.rs 有问津
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/helloworld.proto")?;
    // 不能使用: compile_well_known_types 这个选项，不然引用google/protobuf/timestamp.proto的话
    //编译结果是一堆super::super,, --> pub updated_at: ::core::option::Option<super::super::super::super::super::google::protobuf::Timestamp>,
    // 报错就是super太多了
    // tonic_build::configure().build_server(false).compile_well_known_types(true).compile(&["proto/helloworld.proto"], &["proto"]).unwrap();
    // 不适用compile_well_known_types 选项编译结果是: pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    // 需要引用 cargo add prost-types
    // let out_dir = PathBuf::from("./src"); // Add this
    tonic_build::configure().build_server(false)
        // .out_dir(&out_dir)
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();
    Ok(())
}
