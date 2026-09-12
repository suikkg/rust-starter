//! 第 03 课练习：判断与循环
//!
//!     cargo test --example ex03
//!
//! 这一课的函数签名都写好了，你只填函数体（函数怎么写是第 04 课）。
//!
//! 答案：`solutions/exercises/ex03.rs`
#![allow(dead_code, unused_variables)]

/// 【1】完成了返回 `"✓"`，没完成返回 `" "`（一个空格）。
///
/// 要求用 **`if` 当值**：`let mark = if done { .. } else { .. };`
/// 注意分支里**不要**带分号。
fn mark(done: bool) -> &'static str {
    todo!()
}

/// 【2】数一数有几条已完成。
///
/// 要求用 `for` + `if` + 一个 `let mut` 计数器。
/// （`.filter().count()` 是第 05 课以后的写法，这里先用循环练手。）
fn count_done(flags: &[bool]) -> usize {
    todo!()
}

/// 【3】1 加到 n 的和。`n` 为 0 时返回 0。
///
/// 坑：`1..n` 不含 n，`1..=n` 含 n。写错就差一个数。
fn sum_1_to(n: u32) -> u32 {
    todo!()
}

/// 【4】第一条未完成任务的下标；全都完成了返回 `-1`。
///
/// 提示：`for (i, flag) in flags.iter().enumerate()`，找到就 `return`。
/// （返回 `-1` 是 C 语言的老写法，Rust 有更好的办法 —— 第 08 课的 `Option`。
/// 这里先按老写法做一遍，到第 08 课你会明白它差在哪。）
fn first_pending(flags: &[bool]) -> i32 {
    todo!()
}

fn main() {
    println!("这一课的练习请运行：cargo test --example ex03");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn 勾的显示() {
        assert_eq!(mark(true), "✓");
        assert_eq!(mark(false), " ", "未完成是一个空格，不是空串");
    }

    #[test]
    fn 数已完成的条数() {
        assert_eq!(count_done(&[]), 0);
        assert_eq!(count_done(&[false, false]), 0);
        assert_eq!(count_done(&[true, false, true]), 2);
        assert_eq!(count_done(&[true, true, true]), 3);
    }

    #[test]
    fn 求和的差一错误() {
        assert_eq!(sum_1_to(0), 0);
        assert_eq!(sum_1_to(1), 1);
        assert_eq!(sum_1_to(3), 6, "1+2+3；算出 3 说明用了 1..n 漏掉了末尾");
        assert_eq!(sum_1_to(10), 55);
    }

    #[test]
    fn 找第一条未完成() {
        assert_eq!(first_pending(&[false, true]), 0);
        assert_eq!(first_pending(&[true, true, false]), 2);
        assert_eq!(first_pending(&[true, true]), -1, "全完成了要返回 -1");
        assert_eq!(first_pending(&[]), -1);
    }
}
