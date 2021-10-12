package main

import (
	"C"
	"fmt"
)

var name string

//export SetName
func SetName(str *C.char) {
	name = C.GoString(str)
}

// typedef void (*rust_callback)(int32_t);
// rust_callback cb;

//export Callback
type Callback = func()

var cb Callback

//export TriggerCallback
func TriggerCallback() {
	fmt.Println("call TriggerCallback...")
	// 	cb(C.CString(name))
	cb()
}

//export RegisterCallback
func RegisterCallback(callback Callback) int {
	fmt.Println("call RegisterCallback...")
	cb = callback
	return 1
}

func main() {
}
