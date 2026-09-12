//! 第 04 课：函数
//!
//!     cargo run --example lesson04_functions
//!
//! 函数把"一件事"打包起来，给它一个名字。
//! 参数必须写类型，有返回值就要写 `-> 类型`。

fn main() {
    show_task(1, "学习 Rust 变量", true);
    show_task(2, "练习函数", false);

    let total = 2;
    let done = 1;
    println!();
    println!("{}", summary(total, done));
}

/// 打印一条任务。没有 `->`，说明它不返回值，只做事。
fn show_task(id: u32, title: &str, done: bool) {
    let mark = if done { "✓" } else { " " };
    println!("[{mark}] {id}  {title}");
}

/// 拼一句统计。`-> String` 说明它返回一个字符串。
fn summary(total: u32, done: u32) -> String {
    // format! 和 println! 用法一样，区别是它不打印，而是把结果交出来。
    format!("共 {total} 项，已完成 {done} 项。")
}

/// 最后一行不写分号，就是返回值。这一句等价于 `return a + b;`
fn _add(a: u32, b: u32) -> u32 {
    a + b
}

// 【动手】写一个函数 pending(total, done) -> u32，返回还没完成的条数，
// 并在 main 里打印出来。写完对照 solutions/lesson04.md。
