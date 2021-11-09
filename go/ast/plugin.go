package main

import (
	"go/ast"
	"go/parser"
	"go/token"
)

func main() {
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "", src, 0)
	panicIfError(err)

	//深度优先遍历打印每一个节点
	ast.Inspect(f, func(node ast.Node) bool {
		ast.Print(fset, node)
		return true
	})
}

func panicIfError(err error) {
	if err != nil {
		panic(err)
	}
}
