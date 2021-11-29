package main

import (
	"log"
)

//包名称必须是main
//没有main函数
//必须有可以导出(访问)的变量或者方法
//go build -buildmode=plugin -o plugin1.so plugin1.go

func init() {
	log.Println("plugin init here")
}

type Echo string

func (e Echo) Echo() {
	log.Println(e)
}

//go build -buildmode=plugin -o=plugin_doctor.so plugin_bad_docter.go

// exported as symbol named "E"
var E = Echo("Hello World!")
