// ============================================================
// Rust 学习项目 - 主程序
// ============================================================
//
// 📚 学习模块导航：
//
// 【所有权与借用】examples/ 目录
//   cargo run --example 01_ownership_basics
//   cargo run --example 02_move_semantics
//   cargo run --example 03_copy_trait
//   cargo run --example 04_borrowing
//   cargo run --example 05_mutable_borrow
//
// 【结构体与方法】本程序
//   cargo run 1  -> 什么是结构体
//   cargo run 2  -> impl 块与方法
//   cargo run 3  -> 关联函数（String::from 的秘密）
//
// 【文档注释】本程序
//   cargo run 4  -> 什么是文档注释
//   cargo run 5  -> 文档测试（Doc Test）
//
// ============================================================

mod learning;

use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // 有参数，运行指定课程
        match args[1].as_str() {
            "1" | "struct" => learning::structs_methods::run_01_struct(),
            "2" | "impl" => learning::structs_methods::run_02_impl(),
            "3" | "from" | "associated" => learning::structs_methods::run_03_associated(),
            "4" | "doc" => learning::doc_comments::run_01_doc_comment(),
            "5" | "doctest" => learning::doc_comments::run_02_doc_test(),
            _ => print_help(),
        }
    } else {
        // 无参数，显示帮助
        print_help();
    }

    wait_for_enter();
}

fn print_help() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              🦀 Rust 学习项目 - 交互式教程                   ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║                                                              ║");
    println!("║  📘 所有权与借用（examples/ 目录）                           ║");
    println!("║  ─────────────────────────────────────────────────────────   ║");
    println!("║  cargo run --example 01_ownership_basics  所有权基础         ║");
    println!("║  cargo run --example 02_move_semantics    Move 语义          ║");
    println!("║  cargo run --example 03_copy_trait        Copy 与 Clone      ║");
    println!("║  cargo run --example 04_borrowing         不可变借用         ║");
    println!("║  cargo run --example 05_mutable_borrow    可变借用           ║");
    println!("║                                                              ║");
    println!("║  📙 结构体与方法（理解 String::from）                        ║");
    println!("║  ─────────────────────────────────────────────────────────   ║");
    println!("║  cargo run 1    什么是结构体                                 ║");
    println!("║  cargo run 2    impl 块与方法                                ║");
    println!("║  cargo run 3    关联函数（String::from 的秘密！）            ║");
    println!("║                                                              ║");
    println!("║  📕 文档注释与文档测试                                       ║");
    println!("║  ─────────────────────────────────────────────────────────   ║");
    println!("║  cargo run 4    什么是文档注释                               ║");
    println!("║  cargo run 5    文档测试（最强大的功能！）                   ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("💡 示例：cargo run 3");
}

fn wait_for_enter() {
    println!();
    print!("按回车键退出...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
