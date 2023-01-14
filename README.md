# 🌈 HiperLink | 🔗 嗨皮立刻 - 🧩 示例插件

[![Go version](https://img.shields.io/badge/Golang-v1.19-blue?style=flat)](https://go.dev/)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-brightgreen)


## 🚧 | 简介

> 嗨皮立刻, 英文名: HiperLink

采用 `Golang` + `Vue` 技术栈, 基于 [Hiper](https://www.the.bb/) 的插件化、轻量化互联互通平台 🎉

## 🎉 | 特性

* ✅ 全平台
* ✅ 轻量化
* ✅ 插件化
* ✅ 高性能
* ✅ 易用性

## 🛠 | 使用方法

插件文件夹目录树

```sh
├── plugin_id              # 插件`文件夹`名**必须**为`插件id`，否则无法加载插件
│   ├── main               # 可执行程序，或脚本语言源代码
│   ├── metadata.json      # 插件元数据
│   └── src                # 前端界面文件夹
│       ├── config.html    # 插件设置界面
│       └── index.html     # 插件应用界面
```

前端框架：![Vuetify](https://img.shields.io/badge/Vuetify-^3.1.1-blue?style=flat) [https://next.vuetifyjs.com/](https://next.vuetifyjs.com/)

插件图标可用列表: [https://materialdesignicons.com/](https://materialdesignicons.com/)

### ※PS: 插件连接方式 **`netRPC`仅`Golang`可用**，`gRPC`支持`Golang`、`C++`、`Python`、`Java`、`Node.JS`、`Rust`、`PHP`、`C#/.NET`、`Kotlin`、`Objective-C`、`Dart`等开发语言
