//! 第 02 课：变量与可变性
//!
//!     cargo run --example lesson02_variables
//!
//! Rust 的变量默认**不能改**。想改就要写 `mut`。
//! 这不是找麻烦：读代码的人看到没有 `mut`，就知道这个值从头到尾都一样。

// 下面故意先写 `completed = completed + 1` 的长形式再写 `+=`，
// 是为了对照。clippy 会建议直接用 `+=`，这里按函数限定关掉它的这条检查。
#[allow(clippy::assign_op_pattern)]
fn main() {
    // 不可变：绑定之后就固定了
    let title = "学习 Rust 变量";
    println!("任务：{title}");

    // 可变：加了 mut 才能重新赋值
    let mut completed = 0;
    println!("已完成：{completed}");

    completed = completed + 1;
    println!("已完成：{completed}");

    // completed += 1; 是上面那行的简写，效果一样
    completed += 1;
    println!("已完成：{completed}");

    // 布尔值：是 / 否
    let done = false;
    println!("这条任务完成了吗？{done}");

    // 类型可以写出来，也可以让 Rust 自己推断
    let count: i32 = 3; // 整数
    let ratio: f64 = 0.5; // 小数
    let name: String = String::from("kk"); // 可增长的字符串
    println!("{name} 有 {count} 条任务，完成率 {ratio}");

    println!("---");

    // 数字上的坑一：整数相除，小数部分被**扔掉**，不是四舍五入
    let done = 3;
    let total = 4;
    println!("3 / 4 在整数里 = {}", done / total); // 0，不是 0.75
    println!("7 / 2 在整数里 = {}", 7 / 2); // 3

    // 坑二：类型不会自动转，要自己写 as
    // 用 {:.2} 保留两位小数，不然 0.0 会被打印成 "0"，看不出差别
    println!("先转再除 = {:.2}", done as f64 / total as f64); // 0.75 ✓
    println!("先除再转 = {:.2}", (done / total) as f64); // 0.00 ✗

    // 位置很要紧：整数除完已经是 0 了，再转成 f64 也只是 0.0。
    // 算完成率、百分比的时候撞上这个，结果永远是 0 或 1，
    // 而且**编译器一个字都不会说** —— 它是合法代码，只是不是你想要的。

    // 【动手】1. 把 mut 从 completed 前面删掉，再运行一次。
    //           编译器会报错 cannot assign twice to immutable variable。
    //           读懂这句话，再把 mut 加回来。
    //
    //        2. 在上面加一行 `let bad = count + ratio;`（i32 加 f64），
    //           看报错。然后改成 `count as f64 + ratio`。
    //           Rust 宁可让你多写四个字，也不肯帮你猜 —— 这是好事。
}
