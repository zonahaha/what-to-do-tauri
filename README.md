# What To Do - 现代化待办事项应用

一个基于 Tauri + Vue 3 构建的现代化桌面待办事项应用，具有美观的界面和流畅的用户体验。

## ✨ 功能特性

- 🎨 **现代化 UI 设计** - 采用渐变背景和毛玻璃效果
- 🌈 **多主题支持** - 5种预设主题颜色，支持实时切换
- 📱 **响应式布局** - 适配不同屏幕尺寸
- ✅ **智能排序** - 已完成项目自动移至列表底部
- 🎯 **流畅动画** - 项目移动和主题切换都有平滑动画
- 🔄 **实时同步** - 数据实时保存和更新
- 🎛️ **自定义窗口** - 支持透明标题栏和自定义尺寸

## 🚀 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- 操作系统：macOS, Windows, Linux

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建应用

```bash
# 构建所有平台
npm run tauri build

# 构建 macOS 版本
npm run tauri:build:mac

# 构建 Windows 版本
npm run tauri:build:win
```

## 📦 自动构建

本项目配置了 GitHub Actions 自动构建流程：

- 每次推送到 `main` 分支时自动触发构建
- 支持 macOS、Windows、Linux 三个平台
- 构建产物会自动上传到 GitHub Releases

## 🛠️ 技术栈

- **前端框架**: Vue 3
- **构建工具**: Vite
- **桌面框架**: Tauri 2.0
- **后端语言**: Rust
- **样式**: CSS3 + 现代特性

## 🎨 主题系统

应用内置 5 种预设主题：
- 蓝色主题
- 紫色主题  
- 绿色主题
- 橙色主题
- 红色主题

点击右上角的调色盘图标可以快速切换主题。

## 📱 窗口配置

- **默认尺寸**: 390x790 (iPhone Pro 尺寸)
- **最小尺寸**: 320x568
- **最大尺寸**: 500x1000
- **支持调整**: 是
- **透明标题栏**: macOS 支持

## 🔧 开发说明

### 项目结构

```
what-to-do-tauri/
├── src/                    # Vue 前端代码
│   ├── components/        # Vue 组件
│   ├── assets/           # 静态资源
│   └── App.vue           # 主应用组件
├── src-tauri/            # Tauri 后端代码
│   ├── src/              # Rust 源代码
│   ├── icons/            # 应用图标
│   └── tauri.conf.json   # Tauri 配置
└── public/               # 公共资源
```

### 主要组件

- `App.vue` - 主应用组件，包含状态管理和主题系统
- `AddTodo.vue` - 添加待办事项组件
- `TodoList.vue` - 待办事项列表组件

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

**享受您的待办事项管理体验！** ✨
