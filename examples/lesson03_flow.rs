//! 第 03 课：判断与循环
//!
//!     cargo run --example lesson03_flow
//!
//! if 做判断，for 做遍历，while 做"条件满足就一直做"。

fn main() {
    let done = true;

    // if 判断。注意条件不用写括号，但大括号不能省。
    if done {
        println!("[✓] 学习 Rust");
    } else {
        println!("[ ] 学习 Rust");
    }

    // if 也可以当成"值"用，整个 if 表达式的结果赋给变量。
    // 注意 { "✓" } 里面没有分号 —— 没有分号才是"返回这个值"。
    let mark = if done { "✓" } else { " " };
    println!("[{mark}] 学习 Rust");

    println!("---");

    // for 循环：把 1、2、3 依次拿出来。1..=3 含 3，1..3 不含 3。
    for i in 1..=3 {
        println!("第 {i} 条任务");
    }

    println!("---");

    // while 循环：条件为真就一直转
    let mut left = 3;
    while left > 0 {
        println!("还剩 {left} 条没做");
        left -= 1;
    }
    println!("全部做完了");

    // 【动手】把 1..=3 改成 1..3，运行看少了哪一行，想清楚为什么。
}
