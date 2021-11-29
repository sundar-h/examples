package main

// 必须go build 以后运行, 不能直接go run

/*
#cgo CFLAGS: -I${SRCDIR}/include
#cgo LDFLAGS: -L${SRCDIR}/lib -lecho

#include "echo.h"
*/
import "C"

func main() {
	C.echo()
}
