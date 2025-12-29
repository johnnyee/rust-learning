// examples/06_pattern_matching.rs

// 定义一个枚举，代表不同类型的消息
// 注意：枚举的成员可以携带不同类型的数据，这就是 Rust 枚举强大的地方
#[derive(Debug)]
enum Message {
    Quit,                       // 不带数据
    Move { x: i32, y: i32 },    // 包含匿名结构体
    Write(String),              // 包含 String
    ChangeColor(i32, i32, i32), // 包含三个 i32
}

fn process_message(msg: Message) {
    // 使用 match 进行模式匹配
    match msg {
        // 1. 匹配简单的变体
        Message::Quit => {
            println!("收到退出指令：正在清理资源...");
        }

        // 2. 匹配并解构结构体风格的变体
        Message::Move { x, y } => {
            println!("移动指令：坐标向 x: {}, y: {} 移动", x, y);
        }

        // 3. 匹配并解构元组风格的变体
        Message::Write(text) => {
            println!("文本消息：'{}'", text);
        }

        // 4. 带有“守卫 (Guard)”的匹配
        // 只有当 r, g, b 都大于 200 时才匹配这个分支
        Message::ChangeColor(r, g, b) if r > 200 && g > 200 && b > 200 => {
            println!("收到高亮颜色：R:{}, G:{}, B:{} (这颜色太亮了！)", r, g, b);
        }

        // 5. 使用通配符 `_` 处理剩余情况
        // 这里的 _r, _g, _b 只是普通变量名，但习惯上用 _ 开头表示忽略未使用的变量警告
        Message::ChangeColor(r, g, b) => {
            println!("收到普通颜色：R:{}, G:{}, B:{}", r, g, b);
        }
    }
}

fn main() {
    println!("=== Rust 模式匹配示例 ===\n");

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 30 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 255, 255), // 会匹配高亮分支
        Message::ChangeColor(0, 0, 0),       // 会匹配普通分支
    ];

    for msg in messages {
        process_message(msg);
    }

    println!("\n=== 特殊匹配：Option 枚举 ===\n");
    // Option 是 Rust 处理空值的方式，也是模式匹配最常用的场景
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    print_option(some_number);
    print_option(no_number);
}

fn print_option(x: Option<i32>) {
    match x {
        Some(i) => println!("有一个值：{}", i),
        None => println!("这里什么都没有"),
    }
}
