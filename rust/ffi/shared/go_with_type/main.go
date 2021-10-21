package main

import "C"

// go build -o shared.so -buildmode=c-shared main.go


//export ReturnConstChar
func ReturnConstChar() *C.char {
//     fmt.Println("ReturnConstChar")
//     return C.CString("hello world")
    return C.CString("")
}

//export Add
func Add(a, b C.int) C.int {
    return a+b
}

func main() {}