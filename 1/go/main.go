//go build -buildmode=c-shared .
package main

import (
	"fmt"

	"github.com/example/echo/sdk"
)

type P string

var p P

func init() {
	fmt.Println("执行init函数....................")
	sdk.Register(&p)
}

func (p *P) Echo(i *sdk.GPeople) {
	fmt.Printf("Golang Echo: %s\n", i.String())
}

func main() {
}
