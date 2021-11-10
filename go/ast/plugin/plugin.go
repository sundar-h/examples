package main

import "C"
import (
	"bytes"
	"encoding/json"
	"fmt"
)

// 不可变更
type PluginInfo struct {
	Name       string
	Language   string
	PluginType string
	Version    string
}

type Plugin interface {
	Info() *C.PluginInfo
	Initilize(string) error
	Finalize() error
}

type Config struct {
}

//名字必须与设置相同
//go:generate command arg1 arg2
type CustomPlugin struct {
	*PluginInfo
}

func (p *CustomPlugin) Info() *PluginInfo {
	return p.PluginInfo
}

func (p *CustomPlugin) Initilize(config string) error {
	p.PluginInfo = &PluginInfo{
		Name:       "CustomPlugin",
		Language:   "Golang",
		PluginType: "Sink",
		Version:    "v0.0.1",
	}

	var cfg Config
	if err := json.Unmarshal(bytes.NewBufferString(config).Bytes(), &cfg); err != nil {
		fmt.Printf("%s Initilize(%s) failed: %v", "example", config, err)
		return err
	}

	return nil
}

func (p *CustomPlugin) Finalize() error {
	return nil
}
