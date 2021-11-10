## 命令行

* Stringer

[golang 类型系统语法说明](https://golang.org/ref/spec)
[cli package](https://github.com/spf13/cobra)
[goast-viewer](https://yuroyoro.github.io/goast-viewer/)
[元编程](https://draveness.me/golang/docs/part4-advanced/ch08-metaprogramming/golang-code-gen/)



[Go ast](https://www.jianshu.com/p/443bd82863f8)
[](https://zhuanlan.zhihu.com/p/28516587)


[stringer 源码](go get golang.org/x/tools/cmd/stringer)

1. 接受两个参数
2. 生成一个golang项目(go mod 管理)
3. 初始化一个plugin.go 代码模板
4. 自动生成main.go(包装对应的cgo细节等)
5. 编译执行为对应的动态链接库

第四步 可以使//go:generate stringer -type ErrCode -linecomment 的形式
也可以使直接一个工具

1. 初始化代码模板
f plug init -name custom_plugin -type go 
2. 生成cgo相关代码并生成对应的动态链接库
f plug generate -target_dir
3. 显示源码
f plug show


1. 生成代码模板
2. 生成底层cgo代码
3. 生成动态链接库