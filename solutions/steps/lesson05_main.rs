// 第 05 课答案：用 Vec 装任意多条。
fn main() {
    // 两个平行列表是过渡写法，第 07 课会合并成一个 struct
    let mut titles: Vec<String> = Vec::new();
    let mut flags: Vec<bool> = Vec::new();

    titles.push(String::from("学习 Rust 变量"));
    flags.push(true);
    titles.push(String::from("练习函数"));
    flags.push(false);
    titles.push(String::from("认识 Vec"));
    flags.push(false);

    println!("我的待办清单");
    println!();

    for (i, title) in titles.iter().enumerate() {
        // 下标从 0 开始，编号从 1 开始
        let done = flags[i];
        let mark = if done { "✓" } else { " " };
        println!("[{}] {}  {}", mark, i + 1, title);
    }

    // 总数不再写死
    let total = titles.len();
    let done_count = flags.iter().filter(|d| **d).count();
    println!();
    println!("共 {total} 项，已完成 {done_count} 项。");

    // 安全取值：越界不会崩溃
    match titles.get(99) {
        Some(t) => println!("第 100 条是 {t}"),
        None => println!("（没有第 100 条）"),
    }
}
