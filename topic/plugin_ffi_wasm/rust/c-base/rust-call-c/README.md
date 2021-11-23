## Rust 单向调用C

* Pass Object: Pointer, Function
* 回调


## 问题列表


1. std::os::raw or libc ?
两者都提供了c的原生类型, c idiomatic type
 * 优先使用标准库的类型, 
 * 在一些no std 场景中 使用libc(因为libc并不依赖std)



## build.rs
[脚本技术](https://doc.rust-lang.org/cargo/reference/build-script-examples.html)

##
[The Rust Reference - C](https://doc.rust-lang.org/stable/reference/type-layout.html?highlight=ffi#the-c-representation)
[教程ffi](https://wiki.jikexueyuan.com/project/rust-primer/ffi/calling-ffi-function.html)
[RUST死灵书](https://www.bookstack.cn/read/rustonomicon_zh-CN/src-11.FFI.md#C%E4%BB%A3%E7%A0%81%E5%88%B0Rust%E5%87%BD%E6%95%B0%E7%9A%84%E5%9B%9E%E8%B0%83)
[nomicon-ffi](https://doc.rust-lang.org/nomicon/ffi.html)