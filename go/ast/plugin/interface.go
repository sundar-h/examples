package main

/*
#include "plugin.c"
*/
import "C"
import (
	"unsafe"
)

var pluginInfo *C.struct_PluginInfo
var customPlugin *CustomSink

func CPluginInfo(pi *PluginInfo) {
	//C.CString 堆分配的内存,需要手动释放
	//var cmsg *C.char = C.CString("hi")
	//defer C.free(unsafe.Pointer(cmsg))
	pluginInfo = &C.struct_PluginInfo{}
	pluginInfo.name = C.CString(pi.Name)
	pluginInfo.language = C.CString(pi.Language)
	pluginInfo.version = C.CString(pi.Version)
	pluginInfo.plugin_type = C.CString(pi.PluginType)
}

//export Info
func Info() *C.struct_PluginInfo {

	//这个控制权不能交出去, 外部host语言需要&*使用
	return pluginInfo
}

// config 没有传递控制权过来, 当前不支持多次初始化, 不支持并发安全保护
//export Initilize
func Initilize(config *C.char) *C.char {
	//已经初始化过
	if pluginInfo != nil {
		return nil
	}

	cfg := C.GoString(config)

	customPlugin = &CustomSink{}
	if err := customPlugin.Initilize(cfg); err != nil {
		// 这里内存控制权交出去了，需要外部host语言负责清理
		return C.CString(err.Error())
	}

	CPluginInfo(customPlugin.PluginInfo)

	return C.nil
}

//保证仅执行一次, 不支持并发安全保护
//export Finalize
func Finalize() *C.char {
	if pluginInfo == nil {
		return nil
	}

	if err := customPlugin.Finalize(); err != nil {
		// 这里内存控制权交出去了，需要外部host语言负责清理
		return C.CString(err.Error())
	}

	clean()

	return nil
}

func clean() {
	if pluginInfo == nil {
		return
	}

	C.free(unsafe.Pointer(pluginInfo.name))
	C.free(unsafe.Pointer(pluginInfo.language))
	C.free(unsafe.Pointer(pluginInfo.version))
	C.free(unsafe.Pointer(pluginInfo.plugin_type))

	pluginInfo = nil
}

func main() {
}
