// ============================================================
// 结构体与方法 - 学习模块
// ============================================================
//
// 📚 学习路径：
//   01. 什么是结构体
//   02. impl 块与方法
//   03. 关联函数（String::from 的秘密）
//
// ============================================================

pub mod _01_what_is_struct;
pub mod _02_impl_block;
pub mod _03_associated_functions;

// 重新导出 run 函数，方便调用
pub use _01_what_is_struct::run as run_01_struct;
pub use _02_impl_block::run as run_02_impl;
pub use _03_associated_functions::run as run_03_associated;
