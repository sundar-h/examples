##  
* 事件处理引擎
* 函数计算




* Pipeline
* Plugin
* WASM
* 事件流



* CNCF Sandbox 项目 [tremor-runtime事件处理引擎](https://nullderef.com/blog/plugin-dynload/)
* [rust-plugins 目录系列四篇网站](https://nullderef.com/series/rust-plugins/)




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

c --> cgo: [c-for-go](https://github.com/xlab/c-for-go)




## 项目
[Tremor事件驱动系统](https://www.tremor.rs/)
[hyper](https://github.com/hyperium/hyper)
[pipewire](https://gitlab.freedesktop.org/pipewire/pipewire-rs)



## 函数计算
[AWS Lambda Function](https://docs.aws.amazon.com/zh_cn/lambda/latest/dg/golang-handler.html)
[AWS Lambda Funciton Golang SDK](https://github.com/aws/aws-lambda-go)

[阿里云 函数计算 代码开发](https://help.aliyun.com/document_detail/157704.htm?spm=a2c4g.11186623.0.0.5240199ebmnTRl#section-fn9-a36-vik)
[阿里云 函数计算](https://help.aliyun.com/document_detail/181580.html)
[阿里云 函数计算 Golang SDK](https://github.com/aliyun/fc-go-sdk)





<!-- # [dependencies.plugin]
# path = "core/plugin" -->