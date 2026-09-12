//! 第 01 课：第一次运行
//!
//!     cargo run --example lesson01_hello
//!
//! 每个可以运行的 Rust 程序都从 `fn main()` 开始。
//! `println!` 末尾的 `!` 说明它是"宏"，不是普通函数 —— 现在只要记住写法。

fn main() {
    println!("你好，Rust");

    // {} 是占位符，逗号后面的值会按顺序填进去。
    let subject = "命令行待办清单";
    let days = 12;
    println!("我要学的是：{}，一共 {} 课", subject, days);

    // 也可以把变量名直接写进大括号里，不用再写逗号后面那一串。
    // 两种写法效果一样，后一种更常用。
    println!("我要学的是：{subject}，一共 {days} 课");
}
