package main

import (
	"fmt"

	"github.com/Hiper-Link/go-plugin"
	"github.com/Hiper-Link/plugin-libs/shared"
	// gitee.com/HiperLink/go-plugin
	// gitee.com/HiperLink/plugin-libs/shared
)

var Handshake = plugin.HandshakeConfig{
	ProtocolVersion:  1,
	MagicCookieKey:   "example_plugin_gRPC", // 必须为 plugin_id
	MagicCookieValue: "Hello, HiperLink",
}

type API struct{}

// 加载插件
func (API) OnLoad(p string) ([]byte, error) {
	fmt.Println("Onload")
	return nil, nil
}

// 停用插件
func (API) OnUnload(p string) ([]byte, error) {
	fmt.Println("OnUnload")
	return nil, nil
}

// 安装插件
func (API) OnInstall(p string) ([]byte, error) {
	fmt.Println("OnInstall")
	return nil, nil
}

// 卸载插件
func (API) OnUninstall(p string) ([]byte, error) {
	fmt.Println("OnUninstall")
	return nil, nil
}

// HL 启动
func (API) OnStart(p string) ([]byte, error) {
	fmt.Println("OnStart, Interface: ", p)
	return nil, nil
}

// HL 停止
func (API) OnStop(p string) ([]byte, error) {
	fmt.Println("OnStop")
	return nil, nil
}

// 前后端交互
func (API) Interaction(p string, function string) (string, error) {
	fmt.Println("Interaction")
	return function, nil
}

func main() {
	plugin.Serve(&plugin.ServeConfig{
		HandshakeConfig: Handshake,
		Plugins: map[string]plugin.Plugin{
			"grpc": &shared.GRPCPlugin{Impl: &API{}},
		},
		NetworkType: "tcp",

		// A non-nil value here enables gRPC serving for this plugin...
		GRPCServer: plugin.DefaultGRPCServer,
	})
}
