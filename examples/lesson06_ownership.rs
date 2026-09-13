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
    println!("---");

    // 「改原件」和「造新件」是两件事，看签名就知道是哪种
    let mut t = String::from("学习 Rust");
    add_suffix(&mut t, "！"); // &mut：它会动 t
    println!("改原件之后：{t}");

    let loud = shout(&t); // &：它只是读 t
    println!("造新件：{loud}");
    println!("原件还是：{t}"); // 一个字没变

    println!("---");

    // 这几个要分清：动原件的 vs 造新件的
    let mut s = String::from("abc");
    s.push_str("de"); // 动原件
    println!("push_str 之后：{s}");

    let up = s.to_uppercase(); // 造新件
    println!("to_uppercase 返回：{up}，原件还是：{s}");

    // 规律：名字像动词祈使句的（push / clear）改原件，
    //       名字像 to_xxx / replace 的造新件。标准库通用的命名习惯。

    println!("---");

    // 切一段出来：切不到怎么办？返回 Option，逼你处理
    for title in ["学习 Rust 语法", "没有空格"] {
        match title.split_once(' ') {
            Some((head, tail)) => println!("{title:14} → 头「{head}」尾「{tail}」"),
            None => println!("{title:14} → 没有空格，切不出来"),
        }
    }
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

/// 改原件：借来改，不返回东西。
fn add_suffix(title: &mut String, suffix: &str) {
    title.push_str(suffix);
}

/// 造新件：借来读，返回一个新的。原件一个字不动。
fn shout(title: &str) -> String {
    title.to_uppercase()
}

// 【动手】1. 把 `// println!("{a}");` 那一行的注释去掉，运行 cargo check，
//           完整读一遍编译器的报错（它会告诉你 a 在哪一行被移动走的）。
//           然后把 `let b = a;` 改成 `let b = a.clone();` 再试一次 —— 报错消失了，为什么？
//
//        2. 把 `let mut t = ...; add_suffix(&mut t, "！");` 里的 `&mut` 去掉，
//           看报错。再把 add_suffix 的参数从 `&mut String` 改成 `&String`，
//           看又变成什么报错。两条都读完。
