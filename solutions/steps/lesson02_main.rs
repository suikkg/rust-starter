// 第 02 课答案：把内容从 println! 里挪进变量。
fn main() {
    let title = "学习 Rust 变量";
    let mut done = false;

    println!("我的待办清单");
    println!();
    println!("[ ] 1  {title}");
    println!("完成状态：{done}");

    // 改状态需要 mut
    done = true;
    println!();
    println!("做完之后：");
    println!("[✓] 1  {title}");
    println!("完成状态：{done}");
}
