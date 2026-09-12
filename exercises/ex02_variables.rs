//! 第 02 课练习：变量与可变性
//!
//!     cargo test --example ex02
//!
//! 把每个 `todo!()` 换成真正的实现，直到测试全绿。
//! 一次只做一道，做完就跑一次测试 —— 红变绿的那一下就是你学会了。
//!
//! 答案：`solutions/exercises/ex02.rs`（先自己写 15 分钟）
#![allow(dead_code, unused_variables)]

/// 【1】把 x 加一再返回。
///
/// 要求：**必须**用 `let mut` 先存一个变量、改它、再返回，
/// 不要直接 `x + 1` —— 这一题练的就是 `mut`。
fn add_one(x: i32) -> i32 {
    todo!()
}

/// 【2】把「几分几秒」换算成总秒数。
fn total_secs(mins: u32, secs: u32) -> u32 {
    todo!()
}

/// 【3】标题超过 10 个字符算「长标题」。
fn is_long_title(title_len: usize) -> bool {
    todo!()
}

/// 【4】完成率，返回 0.0 ~ 1.0。
///
/// 坑：`3 / 4` 在整数里等于 0。想清楚该在哪一步转成 `f64`。
/// 一条都没有时返回 `0.0`（除以 0 会得到 `NaN`，那不是我们要的）。
fn done_ratio(done: u32, total: u32) -> f64 {
    todo!()
}

fn main() {
    println!("这一课的练习请运行：cargo test --example ex02");
}

#[cfg(test)]
// 中文测试名里夹大写 ASCII 会触发 non_snake_case，按模块限定关掉。
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn 加一() {
        assert_eq!(add_one(0), 1);
        assert_eq!(add_one(41), 42);
        assert_eq!(add_one(-1), 0);
    }

    #[test]
    fn 换算总秒数() {
        assert_eq!(total_secs(0, 30), 30);
        assert_eq!(total_secs(1, 0), 60);
        assert_eq!(total_secs(2, 5), 125);
    }

    #[test]
    fn 长标题的分界线() {
        assert!(!is_long_title(0));
        assert!(!is_long_title(10), "正好 10 个字不算长");
        assert!(is_long_title(11));
    }

    #[test]
    fn 完成率是小数不是整数() {
        assert_eq!(done_ratio(0, 4), 0.0);
        assert_eq!(done_ratio(2, 4), 0.5);
        assert_eq!(done_ratio(4, 4), 1.0);
        // 整数除法会把这个算成 0.0
        assert_eq!(done_ratio(3, 4), 0.75, "3 / 4 不该等于 0");
    }

    #[test]
    fn 空清单的完成率是0而不是NaN() {
        let r = done_ratio(0, 0);
        assert!(!r.is_nan(), "除以 0 得到了 NaN，要单独处理 total == 0");
        assert_eq!(r, 0.0);
    }
}
