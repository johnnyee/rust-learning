# Rust Learning 项目配置

## 📋 项目概述

- **项目名称**：rust-learning
- **项目类型**：Rust 语言学习项目（Cargo Workspace）
- **项目路径**：`D:\Dev\Rust\rust-learning`
- **GitHub 仓库**：https://github.com/johnnyee/rust-learning

## 🛠️ 技术栈

- **语言**：Rust 1.91+
- **构建工具**：Cargo Workspace
- **IDE 推荐**：VS Code + rust-analyzer 插件

## 📁 目录结构

```
rust-learning/
├── Cargo.toml                    # Workspace 根配置
├── Cargo.lock                    # 依赖版本锁定
├── src/
│   └── main.rs                   # 导航主程序
│
├── crates/                       # 学习模块（每个概念一个独立包）
│   ├── c01-ownership/            # 所有权学习
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── bin/
│   │           ├── basics.rs         # 所有权基础
│   │           ├── move_semantics.rs # Move 语义
│   │           └── copy_trait.rs     # Copy 与 Clone
│   │
│   ├── c02-borrowing/            # 借用学习
│   │   └── src/bin/
│   │       ├── basics.rs         # 不可变借用
│   │       └── mutable.rs        # 可变借用
│   │
│   ├── c03-enums/                # 枚举与模式匹配
│   │   └── src/bin/
│   │       ├── pattern_matching.rs
│   │       └── if_let.rs
│   │
│   ├── c04-collections/          # 集合
│   │   └── src/bin/
│   │       └── basics.rs         # Vec/HashMap
│   │
│   ├── c05-structs/              # 结构体与方法
│   │   └── src/bin/
│   │       ├── what_is_struct.rs
│   │       ├── impl_block.rs
│   │       └── associated_fn.rs
│   │
│   └── c06-docs/                 # 文档注释
│       └── src/bin/
│           ├── doc_comment.rs
│           └── doc_test.rs
│
└── docs/                         # 学习笔记
```

## 🚀 运行方式

```bash
# 显示导航菜单
cargo run

# 运行指定模块（格式：cargo run -p <包名> --bin <二进制名>）
cargo run -p c01-ownership --bin c01-basics
cargo run -p c02-borrowing --bin c02-mutable
cargo run -p c03-enums --bin c03-pattern-matching
cargo run -p c04-collections --bin c04-basics
cargo run -p c05-structs --bin c05-what-is-struct
cargo run -p c06-docs --bin c06-doc-comment
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
- 二进制名：kebab-case（如 `c01-basics`）

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
# 编译整个 workspace
cargo build --workspace

# 编译指定包
cargo build -p c01-ownership

# 运行测试
cargo test --workspace

# 代码格式化
cargo fmt --all

# 代码检查
cargo clippy --workspace

# 生成文档
cargo doc --workspace --open

# 清理构建产物
cargo clean
```

## 📚 学习资源索引

- 官方文档：https://doc.rust-lang.org/
- Rust Book：https://doc.rust-lang.org/book/
- 标准库文档：https://doc.rust-lang.org/std/

## ⚠️ 注意事项

1. 新增学习模块时，在 `crates/` 目录下创建新的包
2. 记得在根 `Cargo.toml` 的 `members` 中添加新包
3. 重要的学习笔记记录在 `docs/` 目录
4. 代码提交前运行 `cargo fmt --all` 和 `cargo clippy --workspace`
