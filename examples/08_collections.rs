// examples/08_collections.rs
use std::collections::HashMap;

fn main() {
    println!("=== 1. Vector (动态数组) ===\n");
    
    // 创建一个可变的 Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // 使用宏快速创建
    let v2 = vec![10, 20, 30, 40, 50];

    // 访问元素：两种方式
    // 方式 A: 索引访问 (如果不安全会直接 Panic 崩溃)
    let third = &v2[2];
    println!("第三个元素是: {}", third);

    // 方式 B: .get() 方法 (返回 Option，更安全)
    match v2.get(100) {
        Some(val) => println!("第100个元素是: {}", val),
        None => println!("抱歉，没有第100个元素 (安全处理了越界)"),
    }

    // 遍历修改
    for i in &mut v {
        *i += 10; // 解引用并修改值
    }
    println!("修改后的 v: {:?}", v);

    println!("\n=== 2. String (UTF-8 字符串) ===\n");

    // 创建 String
    let mut s = String::from("Hello");
    s.push_str(", Rust!"); // 追加字符串
    s.push('✨');          // 追加单个字符

    println!("拼接结果: {}", s);

    // 字符串连接
    let s1 = String::from("Tic");
    let s2 = String::from("Tac");
    let s3 = String::from("Toe");

    // format! 宏是最直观的拼接方式，不会夺取所有权
    let game = format!("{}-{}-{}", s1, s2, s3);
    println!("格式化拼接: {}", game);

    // ⚠️ 注意：Rust 不支持 s[0] 这样的索引访问，因为 UTF-8 字符长度不固定

    println!("\n=== 3. HashMap (哈希表/字典) ===\n");

    let mut scores = HashMap::new();

    // 插入数据
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 覆盖旧值
    scores.insert(String::from("Blue"), 25); 

    // 🌟 Entry API：只有键不存在时才插入
    // "Yellow" 已经存在，不会变；"Red" 不存在，会插入 30
    scores.entry(String::from("Yellow")).or_insert(500); 
    scores.entry(String::from("Red")).or_insert(30);

    println!("当前分数表: {:?}", scores);

    // 典型应用：统计单词出现次数
    let text = "hello world hello rust";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert 返回对应值的这种可变引用
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("单词统计: {:?}", map);
}
