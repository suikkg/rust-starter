// 这是你的待办清单程序。
//
// 现在它只会打印几行字 —— 这是第 01 课的起点。
// 从第 02 课开始，你会亲手把它改成一个真正能用的工具：
//
//     cargo run -- add "学习 Rust 变量"
//     cargo run -- list
//     cargo run -- done 1
//
// 每一课都只改这一个文件（第 11 课才会拆成多个文件）。
// 改坏了不要紧：solutions/ 里有每一课的完整答案。

fn main() {
    println!("我的待办清单");
    println!();
    println!("[ ] 1  学习 Rust");
    println!();
    println!("共 1 项，已完成 0 项。");
}
