package sdk

import "C"

var PluginName string
var PluginVersion string
var PluginType string
var PluginLang = "Golang"

// PluginContext 不传递所有权
type PluginContext struct {
	RequestID          string // 唯一请求标识
	InitializerTimeout int32  // 初始化超时时间
	Timeout            int32  // Open 函数运行超时时间
	Payload            string // 参数
}

func FromContext(ctx *C.struct_Context) *PluginContext {
	return &PluginContext{
		RequestID:          C.GoString(ctx.RequestID),
		InitializerTimeout: C.Int(ctx.InitializerTimeout),
		Timeout:            C.Int(ctx.Timeout),
		Payload:            C.GoString(ctx.Payload),
	}
}
