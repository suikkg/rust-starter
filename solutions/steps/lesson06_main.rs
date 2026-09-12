// 第 06 课答案：显示和修改都改成借用参数。
fn main() {
    let mut titles: Vec<String> = Vec::new();
    let mut flags: Vec<bool> = Vec::new();

    // &mut：借给 add 去改
    add(&mut titles, &mut flags, "学习 Rust 变量");
    add(&mut titles, &mut flags, "练习函数");
    add(&mut titles, &mut flags, "理解借用");

    flags[0] = true;

    // &：只借来看
    render(&titles, &flags);

    // 借完还能继续用——这就是 & 和 move 的区别
    println!();
    println!("main 里还能继续用：一共 {} 条", titles.len());
}

/// 只读，所以参数是 &。写 &[String] 比 &Vec<String> 更通用，是 Rust 的习惯。
fn render(titles: &[String], flags: &[bool]) {
    println!("我的待办清单");
    println!();
    for (i, title) in titles.iter().enumerate() {
        let mark = if flags[i] { "✓" } else { " " };
        println!("[{}] {}  {}", mark, i + 1, title);
    }
    let done = flags.iter().filter(|d| **d).count();
    println!();
    println!("共 {} 项，已完成 {} 项。", titles.len(), done);
}

/// 要往里加东西，所以是 &mut。
/// title 只读不改，所以是 &str——不夺走调用方的字符串。
fn add(titles: &mut Vec<String>, flags: &mut Vec<bool>, title: &str) {
    titles.push(title.to_string());
    flags.push(false);
}
