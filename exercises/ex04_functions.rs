//! 第 04 课练习：函数
//!
//!     cargo test --example ex04
//!
//! 这一课开始，函数体和**签名的理解**一起练：注意每个参数为什么是这个类型。
//!
//! 答案：`solutions/exercises/ex04.rs`
#![allow(dead_code, unused_variables)]

/// 【1】拼那句统计。
///
/// 输出正好是：`共 5 项，已完成 2 项。`
/// 提示：`format!("共 {total} 项，已完成 {done} 项。")`
fn summary(total: u32, done: u32) -> String {
    todo!()
}

/// 【2】还剩几条没做。
///
/// 坑：`u32` 不能是负数。如果 `done > total`（数据坏了），
/// `total - done` 会**运行时崩溃**（debug 下 panic: attempt to subtract with overflow）。
/// 这时该返回 0。查一下 `u32::saturating_sub`。
fn pending(total: u32, done: u32) -> u32 {
    todo!()
}

/// 【3】渲染清单里的一行。
///
/// 完成：`[✓] 1  学习 Rust`
/// 未完成：`[ ] 2  练习函数`
///
/// 注意 `]` 和编号之间一个空格，编号和标题之间**两个**空格。
fn task_line(id: u32, title: &str, done: bool) -> String {
    todo!()
}

/// 【4】给标题加个感叹号后缀，原标题不变。
///
/// 参数为什么是 `&str` 而不是 `String`？想清楚再写 ——
/// 测试里会在调用之后**继续使用**原来那个 String。
fn urgent(title: &str) -> String {
    todo!()
}

fn main() {
    println!("这一课的练习请运行：cargo test --example ex04");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn 统计那句话() {
        assert_eq!(summary(5, 2), "共 5 项，已完成 2 项。");
        assert_eq!(summary(0, 0), "共 0 项，已完成 0 项。");
    }

    #[test]
    fn 剩余条数() {
        assert_eq!(pending(5, 2), 3);
        assert_eq!(pending(2, 2), 0);
    }

    #[test]
    fn 数据坏了也不崩溃() {
        // 已完成比总数还多，只能是数据坏了。这时要返回 0，不是崩溃。
        assert_eq!(pending(1, 3), 0, "u32 减成负数会 panic，用 saturating_sub");
    }

    #[test]
    fn 渲染一行() {
        assert_eq!(task_line(1, "学习 Rust", true), "[✓] 1  学习 Rust");
        assert_eq!(task_line(2, "练习函数", false), "[ ] 2  练习函数");
    }

    #[test]
    fn 加急标题不夺走原标题() {
        let title = String::from("交周报");
        assert_eq!(urgent(&title), "交周报！");
        // 这一行是重点：如果 urgent 收的是 String，下面这句编译不过
        assert_eq!(title.len(), "交周报".len(), "原来的 title 还要能用");
    }
}
