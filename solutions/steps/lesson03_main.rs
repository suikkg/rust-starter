// 第 03 课答案：用 if 决定勾，用 for 打印多条。
fn main() {
    let title = "学习 Rust 变量";
    let done = true;

    // if 当值用：两个分支类型要一致，都是 &str
    let mark = if done { "✓" } else { " " };

    println!("我的待办清单");
    println!();
    println!("[{mark}] 1  {title}");

    // 剩下两条先用循环凑出来，第 05 课换成真正的列表
    for i in 2..=3 {
        println!("[ ] {i}  第 {i} 条任务");
    }

    let total = 3;
    let done_count = if done { 1 } else { 0 };
    println!();
    println!("共 {total} 项，已完成 {done_count} 项。");
}
