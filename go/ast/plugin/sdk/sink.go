package sdk

import "C"

var sinkInstance Sink

// 外部使用一个函数统一接受错误, 所有错误都转移控制权
type Sink interface {
	Name() string
	Open(ctx *PluginContext) error
}

func RegisterSink(s Sink) {
	sinkInstance = s
}

//export Name
func Name() *C.Char {
	name := sinkInstance.Name()

	return C.CString(name)
}

//export Open
func Open(ctx *C.struct_Context) *C.char {
	if ctx == nil {
	}

	if err := sinkInstance.Open(ctx); err != nil {
		return C.CString(err.Error())
	}

	return C.Null
}
