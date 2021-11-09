##  
* 事件处理引擎
* 函数计算




* Pipeline
* Plugin
* WASM
* 事件流



* CNCF Sandbox 项目 [tremor-runtime事件处理引擎](https://nullderef.com/blog/plugin-dynload/)
* [rust-plugins 系列四篇网站](https://nullderef.com/series/rust-plugins/)




* Web~Vue
* 画图
* 函数计算
* Rust
* Golang 源码阅读
* 知识体系积累




* 性能
* 是否可以避免序列化(repr(C))
* 是否可以受用泛型
* dyn使用
* Wasmer, Wasmtime


精简的插件
* 插件是功能性代码
连接器: Connector
插件应该尽可能简单，通信细节留给运行时
插件和运行时
* 线程安全
* FFI-safe: 不适用&str, CStr

包装精简插件 --> 管理器: 
* 管理器将通信和底层功能(插件)解耦
插件上层包装一个包装器(管理器)
* 处理通信或其它类似的任务(async、通道等)


公共接口直接使用C来编写也好
或者使用Extern "C"
C -> RUST: rust-bindgen
RUST -> C: cbindgen




## 项目
[hyper](https://github.com/hyperium/hyper)
[pipewire](https://gitlab.freedesktop.org/pipewire/pipewire-rs)
