# Chrom Tool

一个 Chromium 系浏览器本地数据管理工具。

## 功能

- 扫描浏览器用户资料
- 查看已安装插件，并支持删除插件
- 查看书签，并支持删除书签
- 查看已保存登录站点
- 清理历史相关文件
- 支持自定义浏览器路径配置

## 当前支持

- Windows
- macOS

已适配的浏览器：

- Google Chrome
- Microsoft Edge
- Brave
- Vivaldi
- Yandex Browser
- Chromium

## 说明

- 项目主要面向 Chromium 系浏览器本地资料目录
- 部分删除或清理操作在浏览器运行中可能失败，建议先关闭对应浏览器

## CI 与发布

- GitHub Actions 会在推送 `v*` 格式的 tag 后自动执行构建和发布
- Release 产物会附带 GitHub Attestation 证明构建来源

### 发布新版本

1. 确保代码已经推送到 `main`
2. 创建版本 tag：`git tag -a vX.Y.Z -m "Release version X.Y.Z"`
3. 推送 tag：`git push origin vX.Y.Z`
4. 等待 GitHub Actions 自动完成构建、Attestation 和 Release 发布

### 修改 CI 配置

- 发布工作流位于 `.github/workflows/release.yml`
- CodeQL 工作流位于 `.github/workflows/codeql.yml`
- 修改后请提交并推送到 GitHub，再创建新 tag 验证构建结果
