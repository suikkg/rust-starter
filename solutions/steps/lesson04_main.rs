// 第 04 课答案：把重复逻辑收进函数。
fn main() {
    show_task(1, "学习 Rust 变量", true);
    show_task(2, "练习函数", false);
    show_task(3, "认识 Vec", false);

    let total = 3;
    let done = 1;
    println!();
    println!("{}", summary(total, done));
    println!("还剩 {} 项没做。", pending(total, done));
}

/// 打印一行。不返回值，所以没有 ->
fn show_task(id: u32, title: &str, done: bool) {
    let mark = if done { "✓" } else { " " };
    println!("[{mark}] {id}  {title}");
}

/// 返回一句话。最后一行不带分号 = 返回它
fn summary(total: u32, done: u32) -> String {
    format!("共 {total} 项，已完成 {done} 项。")
}

/// 额外挑战
fn pending(total: u32, done: u32) -> u32 {
    total - done
}
