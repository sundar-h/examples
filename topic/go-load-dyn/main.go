package main

import (
	"log"
	"plugin"
)

type Echo interface {
	Echo()
}

func init() {
	log.Println("Main init here...")
}

func main() {
	//p, err := plugin.Open("./plugins/plugins.so")
	p, err := plugin.Open("./plugins/echo.so")
	if err != nil {
		panic(err)
	}

	e, err := p.Lookup("E")
	if err != nil {
		panic(err)
	}

	i, ok := e.(Echo)
	if !ok {
		panic("no!!!")
	}

	i.Echo()
}
