// pub mod helloworld {
//     tonic::include_proto!("helloworld");
// }

// 这种不在本目录下生成文件，只有再cargo build/run的时候再对应的target/debug/helloworld_rust_helloworld.rs 有问津
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
