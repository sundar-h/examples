package main

// include "../protocols/c_protocols.c"
//import "C"

// Golang Plugin 目标
// 屏蔽C细节
// 内存安全

//tools: 一个自动生成golang --> ffi.rs 的工具库
// 要求遵循Plugin规范
// 目标，1. 屏蔽 cgo的交互
//      2. 内存安全保证

type Plugin interface {
	Init(config string) int
	Err() error
	Finalize() int
	Info() string
}

type Object struct {
	name string
}

func main() {
}
