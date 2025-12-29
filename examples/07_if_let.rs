// examples/07_if_let.rs

fn main() {
    println!("=== if let 简洁控制流 ===\n");

    // 场景 1: 使用 if let 处理 Option
    // 只在 config_max 是 Some 时执行代码，否则什么都不做
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("配置的最大值是: {}", max);
    }
    // 不需要写 else { () }，非常清爽

    println!("\n-------------------\n");

    // 场景 2: if let 也可以搭配 else
    let coin_flip = 0; // 假设 0 是正面，1 是反面
    
    // 这是一个枚举模拟
    enum Coin {
        Heads,
        Tails,
    }

    let coin = Coin::Heads;

    if let Coin::Heads = coin {
        println!("正面朝上！你赢了！");
    } else {
        println!("反面朝上，再试一次吧。");
    }

    println!("\n=== while let 循环匹配 ===\n");

    // 场景 3: while let
    // 只要匹配成功，循环就会一直继续
    // 这在处理像“从栈中弹出数据直到为空”这样的场景时非常有用

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop() 会返回 Option<i32>
    // 当它返回 Some(value) 时，循环继续
    // 当它返回 None 时（栈空了），循环自动停止
    while let Some(top) = stack.pop() {
        println!("从栈顶弹出: {}", top);
    }
    
    println!("栈已空，循环结束。");
}
