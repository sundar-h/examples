package main

import "github.com/example/myast/plugin/sdk"

type CustomSink struct {
}

func (p *CustomSink) Name() string {
	return "mqtt"
}

func (p *CustomSink) Open(ctx *sdk.PluginContext) error {
	return nil
}

func main() {
	sdk.PluginName = ""
	sdk.PluginVersion = ""
	sdk.PluginType = ""

	plugin := &CustomSink{}
	sdk.RegisterSink(plugin)
}
