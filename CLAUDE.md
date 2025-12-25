# Rust Learning 项目配置

## 📋 项目概述

- **项目名称**：rust-learning
- **项目类型**：Rust 语言学习项目
- **项目路径**：`D:\Dev\Rust\rust-learning`
- **GitHub 仓库**：https://github.com/johnnyee/rust-learning

## 🛠️ 技术栈

- **语言**：Rust 1.91+
- **构建工具**：Cargo
- **IDE 推荐**：VS Code + rust-analyzer 插件

## 📁 目录结构

```
rust-learning/
├── Cargo.toml          # 项目配置和依赖管理
├── Cargo.lock          # 依赖版本锁定文件
├── src/
│   ├── main.rs         # 主程序入口
│   └── lib.rs          # 库模块（可选）
├── examples/           # 示例代码
├── tests/              # 集成测试
├── benches/            # 性能基准测试
└── docs/               # 学习笔记
```

## 🎯 开发规范

### 代码风格

- 遵循 Rust 官方风格指南
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 进行代码检查

### 命名规范

- 文件名：snake_case（如 `my_module.rs`）
- 函数名：snake_case（如 `calculate_sum`）
- 结构体/枚举：PascalCase（如 `MyStruct`）
- 常量：SCREAMING_SNAKE_CASE（如 `MAX_VALUE`）

### 提交规范

提交信息格式：`<type>: <description>`

类型包括：
- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建/工具相关

## 🔧 常用命令

```bash
# 编译项目
cargo build

# 运行项目
cargo run

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 生成文档
cargo doc --open

# 发布版本构建
cargo build --release
```

## 📚 学习资源索引

- 官方文档：https://doc.rust-lang.org/
- Rust Book：https://doc.rust-lang.org/book/
- 标准库文档：https://doc.rust-lang.org/std/

## ⚠️ 注意事项

1. 每次学习新概念后，在 `examples/` 目录下创建对应的示例文件
2. 重要的学习笔记记录在 `docs/` 目录
3. 代码提交前运行 `cargo fmt` 和 `cargo clippy`
