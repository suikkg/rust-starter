//! 第 06 课：所有权与借用 —— Rust 最特别的一课
//!
//!     cargo run --example lesson06_ownership
//!
//! 一句话：每份数据同时只有一个"主人"。
//! 把数据交给别人 = 移动（move），交完自己就不能再用了。
//! 只是"给人看一眼" = 借用（&），看完还回来，自己还能继续用。

fn main() {
    // ---------- 1. 移动 ----------
    let a = String::from("学习 Rust");
    let b = a; // a 的所有权移动给了 b
    println!("b = {b}");
    // println!("{a}");        // 打开这一行会报错：borrow of moved value: `a`
    //                         // 因为 a 已经不是主人了。

    // ---------- 2. 借来读 ----------
    let title = String::from("练习函数");
    print_title(&title); // & = 借给它看，不交出所有权
    println!("借完我还能用：{title}"); // 所以这里还能用

    // ---------- 3. 借来改 ----------
    let mut titles = vec![String::from("任务一")];
    add_one(&mut titles); // &mut = 借给它改
    println!("现在有 {} 条", titles.len());

    // ---------- 4. 数字不一样 ----------
    // 整数、布尔这类小类型是"复制"，不是移动，所以下面两行都能用。
    let x = 5;
    let y = x;
    println!("x={x} y={y}");
}

/// &str 是"借来的字符串"，String 是"自己拥有的字符串"。
/// 只读不改，就收 &str —— 这是 Rust 里最常见的参数写法。
fn print_title(title: &str) {
    println!("任务：{title}");
}

/// 要改动对方的数据，参数写 &mut。
fn add_one(titles: &mut Vec<String>) {
    titles.push(String::from("新任务"));
}

// 【动手】把 `// println!("{a}");` 那一行的注释去掉，运行 cargo check，
// 完整读一遍编译器的报错（它会告诉你 a 在哪一行被移动走的）。
// 然后把 `let b = a;` 改成 `let b = a.clone();` 再试一次 —— 报错消失了，为什么？
