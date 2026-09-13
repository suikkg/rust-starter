//! 第 05 课：Vec —— 装多条数据
//!
//!     cargo run --example lesson05_vec
//!
//! Vec<T> 是"可以变长的列表"。T 是里面装什么类型。

fn main() {
    // 建一个空列表，往里加东西。要加东西就得 mut。
    let mut titles: Vec<String> = Vec::new();
    titles.push(String::from("学习 Rust 变量"));
    titles.push(String::from("练习函数"));
    titles.push(String::from("认识 Vec"));

    println!("一共 {} 条", titles.len());
    println!("---");

    // 遍历：&titles 表示"借来看看"，不把列表交出去（第 06 课细讲）
    for title in &titles {
        println!("- {title}");
    }

    println!("---");

    // 想要序号就用 enumerate，i 从 0 开始，所以显示时 +1
    for (i, title) in titles.iter().enumerate() {
        println!("{} {}", i + 1, title);
    }

    println!("---");

    // 按下标取。titles[10] 会直接让程序崩溃（panic）。
    // get 更安全：取到就是 Some(值)，取不到就是 None（第 08 课细讲）。
    match titles.get(1) {
        Some(t) => println!("第 2 条是：{t}"),
        None => println!("没有第 2 条"),
    }
    match titles.get(99) {
        Some(t) => println!("第 100 条是：{t}"),
        None => println!("没有第 100 条"),
    }

    // 删掉一条（下标从 0 算）
    titles.remove(0);
    println!("删掉第 1 条后还剩 {} 条", titles.len());

    println!("---");

    // 列表里装的是字符串，这几个方法马上就用得上
    let title = "  学习 Rust 语法  ";
    println!("原串：{title:?}");
    println!("去空白：{:?}", title.trim());
    println!("是不是空的：{}", title.trim().is_empty());
    println!("开头是「学习」吗：{}", title.trim().starts_with("学习"));
    println!("含有 Rust 吗：{}", title.contains("Rust"));

    println!("---");

    // 最要紧的一条：len() 是**字节数**，不是字数
    for t in ["abcd", "学习编程"] {
        println!(
            "「{t}」len()={}  chars().count()={}",
            t.len(),
            t.chars().count()
        );
    }
    println!();
    println!("四个汉字 len() 是 12，比八个字母还「长」——");
    println!("比长短、截断显示、判断「超过 10 个字」，一律用 chars().count()。");

    // 【动手】1. 把 titles.get(1) 改成 titles[99] 运行一次，看崩溃信息长什么样，
    //           再改回来。以后要习惯用 get。
    //
    //        2. 加一行 `println!("{}", "学习编程"[0..3]);` 看会怎样。
    //           （提示：它连编译都过不去。中文不能这么切。）
}
