// ============================================================
// Rust 学习项目 - 导航主程序
// ============================================================
//
// 📚 学习模块导航
//
// 本项目采用 Cargo Workspace 结构，每个概念一个独立包：
//
// ┌────────────────────────────────────────────────────────────┐
// │  c01-ownership   所有权学习模块                            │
// │  c02-borrowing   借用学习模块                              │
// │  c03-enums       枚举与模式匹配模块                        │
// │  c04-collections 集合学习模块                              │
// │  c05-structs     结构体与方法模块                          │
// │  c06-docs        文档注释学习模块                          │
// └────────────────────────────────────────────────────────────┘
//
// ============================================================

use std::io::{self, Write};

fn main() {
    print_banner();
    print_modules();
    wait_for_enter();
}

fn print_banner() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                                                                  ║");
    println!("║              🦀 Rust 学习项目 - 交互式教程                       ║");
    println!("║                                                                  ║");
    println!("║              JohnYe 的 Rust 学习之旅                             ║");
    println!("║                                                                  ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
}

fn print_modules() {
    println!("📦 学习模块列表");
    println!("────────────────────────────────────────────────────────────────────");
    println!();

    // c01-ownership
    println!("📘 c01-ownership - 所有权学习");
    println!("   cargo run -p c01-ownership --bin c01-basics          # 所有权基础");
    println!("   cargo run -p c01-ownership --bin c01-move-semantics  # Move 语义");
    println!("   cargo run -p c01-ownership --bin c01-copy-trait      # Copy 与 Clone");
    println!();

    // c02-borrowing
    println!("📗 c02-borrowing - 借用学习");
    println!("   cargo run -p c02-borrowing --bin c02-basics          # 不可变借用");
    println!("   cargo run -p c02-borrowing --bin c02-mutable         # 可变借用");
    println!();

    // c03-enums
    println!("📙 c03-enums - 枚举与模式匹配");
    println!("   cargo run -p c03-enums --bin c03-pattern-matching    # 模式匹配");
    println!("   cargo run -p c03-enums --bin c03-if-let              # if let 语法");
    println!();

    // c04-collections
    println!("📕 c04-collections - 集合");
    println!("   cargo run -p c04-collections --bin c04-basics        # Vec/HashMap");
    println!();

    // c05-structs
    println!("📓 c05-structs - 结构体与方法");
    println!("   cargo run -p c05-structs --bin c05-what-is-struct    # 什么是结构体");
    println!("   cargo run -p c05-structs --bin c05-impl-block        # impl 块与方法");
    println!("   cargo run -p c05-structs --bin c05-associated-fn     # 关联函数");
    println!();

    // c06-docs
    println!("📔 c06-docs - 文档注释");
    println!("   cargo run -p c06-docs --bin c06-doc-comment          # 文档注释语法");
    println!("   cargo run -p c06-docs --bin c06-doc-test             # 文档测试");
    println!();

    println!("────────────────────────────────────────────────────────────────────");
    println!();
    println!("💡 提示：复制上面的命令到终端运行对应的学习模块");
    println!();
}

fn wait_for_enter() {
    print!("按回车键退出...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
