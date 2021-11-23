## 插件实现方案调研总结
1. 可行方案
2. 辅助工具集
3. Demo

### 可行方案
* IPC(进程间通信)
* 网络协议通信(RPC, RESTfull 等)
* FFI-ABI
* Wasm

* Rust
  * cbindgen-demo: export rust to c
  * cgo-call-rust: golang as host, call rust
* Golang
  * go --> c
  * c --> go

### Rust

工具

* Rust as library (Export to C): cbindgen 导出CABI接口
`cbindgen --lang c --config cbindgen.toml`
* Rust as host (import C ABI): bindgen
`bindgen echo.h -o a.rs`

### Golang

工具

* golang as library (export to c): go tool cgo 
`go tool cgo -godefs sdk/type.go`
* golang as host (import C ABI): 根据c header 自动生成cgo
`go get github.com/xlab/c-for-go`

