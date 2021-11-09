package main

import "fmt"

type A struct {
}

type Echo interface {
	echo(string)
}

func (a *A) Echo(body string) {
	fmt.Println(body)
}

var a = A{}
