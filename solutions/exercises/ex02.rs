//! 第 02 课练习的参考答案。
//!
//!     cargo test --example ans02
//!
//! 和自己的比一比：`diff exercises/ex02_variables.rs solutions/exercises/ex02.rs`
#![allow(dead_code)]

fn add_one(x: i32) -> i32 {
    // 这一题的重点是 mut：不加 mut 的变量赋一次值之后就不能再改了。
    let mut n = x;
    n += 1;
    n
}

fn total_secs(mins: u32, secs: u32) -> u32 {
    mins * 60 + secs
}

fn is_long_title(title_len: usize) -> bool {
    // 「超过 10」是 >，不是 >=。差一个字符就是差一个 bug。
    title_len > 10
}

fn done_ratio(done: u32, total: u32) -> f64 {
    if total == 0 {
        // 0 / 0 在浮点里是 NaN，NaN 和任何数比较都是 false，
        // 会让后面所有判断都静默失效。所以这一支要单独处理。
        return 0.0;
    }
    // 先转 f64 再除。写成 (done / total) as f64 就是整数除法，3/4 会变成 0。
    done as f64 / total as f64
}

fn main() {
    println!("这是第 02 课练习的答案：cargo test --example ans02");
}

#[cfg(test)]
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
        assert_eq!(done_ratio(3, 4), 0.75, "3 / 4 不该等于 0");
    }

    #[test]
    fn 空清单的完成率是0而不是NaN() {
        let r = done_ratio(0, 0);
        assert!(!r.is_nan(), "除以 0 得到了 NaN，要单独处理 total == 0");
        assert_eq!(r, 0.0);
    }
}
