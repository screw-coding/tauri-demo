# tauri-demo

一个 tauri 的 demo，来探索 tauri 的各种功能

## 各种 feature

### 开发模式下忽略文件

开发模式下，**src-tauri** 目录下的任何文件变更都会引起应用的重新构建，**@tauri-apps/cli** v1.1.0 版本以上, 支持了两种忽略方式，这里使用其中一种, **--no-watch**, 即运行时加上 --no-watch 的参数，例如 **cargo tauri dev --no-watch** 或者 **pnpm tauri dev --no-watch**.

### 应用的构建

使用 github action 来进行跨平台的编译与构建, 详细见 **.github/workflows/.release.yml** 文件

todo

- [ ] github action 感觉速度偏慢, 希望找到更好的支持跨平台编译打包的 ci 方案

### 应用的更新

> Tauri offers a built-in updater for the MSI (Windows), AppImage (Linux) and App bundle (macOS) distribution formats.

Tauri为MSI（Windows）、AppImage（Linux）和App bundle（macOS）发布格式提供了一个内置的更新器。

内置的更新器有一个前面机制来保证安全更新应用。
生成密码

```shell
pnpm tauri signer generate -w ~/.tauri/tauri-demo.key
```

