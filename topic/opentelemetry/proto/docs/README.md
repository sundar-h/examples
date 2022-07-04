
## 本实例目标
* 探索 rust的protobuf使用方式
* 对于 protobuf 的package a.b.c.d.e 这种常明明方式，如何支持引用以及IDE跳转到定义

## 使用tonic+prost+tonic_build 来构建protobuf的rust方式
```yaml
[dependencies]
  prost = "*"
  prost-types = "0.10.1"
  tonic = "*"

  [build-dependencies]
  tonic-build = "*"
```

```rust

// build.rs
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
```

## 关于长命名

```rust
pub mod helloworld {
    tonic::include_proto!("a.b.c.d.f");
}

use helloworld::HelloRequest;

```

##  关于IDE补全跳转、补全
 * 直接build以后是不支持的
 需要两步支持
1. 再include_proto宏上 
2. Show Context Action(CMD+.) -->  Show the result of macro expression (cargo expand) 
3. Show Context Action(CMD+.) -->  Show recursive macro expression (cargo expand)
4. 以上两步如果没有出真正展开后的代码的话，就多尝试几次
5. 如果以上两步都出展开宏后的代码了，就可以支持补全protobuf类型和跳转定义了


# 注意踩到的坑
* 不要开启compile_well_known_types 选项，这个选项编译长命名的结果是一堆的super::super::super， 会编译报错
不开启，并且添加prost-types 依赖
* 不用生成out_dir 里面，可以直接使用