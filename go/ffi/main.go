package main

import "C"

// 需要注意内存管理
[cgo#wiki](https://github.com/golang/go/wiki/cgo)

//export send
func send(config *C.char) *C.char {
	// return C.CString()
}

func main() {
}
