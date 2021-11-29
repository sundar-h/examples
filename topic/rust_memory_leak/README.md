## rust内存泄漏 检测实验

[最新官网文档说明 std::alloc](https://doc.rust-lang.org/stable/std/alloc/)
In a given program, the standard library has one “global” memory allocator that is used for example by Box<T> and Vec<T>.
Currently the default global allocator is unspecified. Libraries, however, like cdylibs and staticlibs are guaranteed to use the System by default.


### 我的实现结果

Vec可以检测到
Box::new 检测不到

* Mac: 下编译 valgrind检测不到内存泄漏
* Linxu: 下可以检测到 (Vec)