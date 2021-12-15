package main

// 必须go build 以后运行, 不能直接go run

/*
// #cgo CFLAGS: -I${SRCDIR}/include
// #cgo LDFLAGS: -L${SRCDIR}/lib -lecho

// #include "echo.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

void printBytes(const uint8_t *bytes, size_t len) {
  char string[100];
  memcpy(string, bytes, len);
  string[len] = '\0';
  printf("%s\n", string);
}
*/
import "C"
import (
	"encoding/json"
	"reflect"
	"unsafe"
)

type People struct {
	Name string
	Age  int
}

func main() {
	p := People{
		Name: "test",
		Age:  8,
	}

	b, err := json.Marshal(&p)
	if err != nil {
		panic(err)
	}
	// C.echo()
	PrintBytes(b)
}

func PrintBytes(b []byte) {
	p := (*reflect.SliceHeader)(unsafe.Pointer(&b))
	C.printBytes((*C.uint8_t)(unsafe.Pointer(p.Data)), C.size_t(len(b)))
}
