//go build -buildmode=c-shared .
package main

import (
	"bytes"
	"fmt"

	"github.com/example/echo/sdk"
)

type P string

var p P

func init() {
	fmt.Println("执行init函数....................")
	sdk.Register(&p)
}

func (p *P) Echo(i *sdk.People) {
	fmt.Printf("Golang Echo %+v\n", i)
	fmt.Println("Golang Echo", bytes.NewBuffer(i.Content).String())
}

func main() {
}
