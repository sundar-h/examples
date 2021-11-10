package main

//该声明为GenDecl TOK=token.IMPORT
import (
	"go/ast"
	"go/parser"
	"go/token"
)

//该声明为GenDecl TOK=token.TYPE
type Product struct {
	Name string
}

//该声明为GenDecl TOK=token.VAR
var product Product

//该声明为FunDecl
func main() {
	fset := token.NewFileSet()
	//解析为*ast.File节点
	f, err := parser.ParseFile(fset, "", src, 0)
	panicIfError(err)

	//深度优先遍历打印每一个节点
	ast.Inspect(f, func(node ast.Node) bool {
		//打印ast节点
		ast.Print(fset, node)
		return true
	})
}

func panicIfError(err error) {
	if err != nil {
		panic(err)
	}
}
