//! 第 03 课练习的参考答案。
//!
//!     cargo test --example ans03
#![allow(dead_code)]

fn mark(done: bool) -> &'static str {
    // if 当值用：两个分支里都不带分号，类型也必须一样（都是 &str）。
    if done {
        "✓"
    } else {
        " "
    }
}

fn count_done(flags: &[bool]) -> usize {
    let mut n = 0;
    for flag in flags {
        // flag 的类型是 &bool，前面加 * 取出里面的 bool
        if *flag {
            n += 1;
        }
    }
    n
    // 学到第 05 课之后，这段可以写成 flags.iter().filter(|f| **f).count()
}

fn sum_1_to(n: u32) -> u32 {
    let mut sum = 0;
    // ..= 含末尾。写成 1..n 就会少加一个 n。
    // n 为 0 时 1..=0 是空区间，循环一次都不进，sum 保持 0。
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn first_pending(flags: &[bool]) -> i32 {
    for (i, flag) in flags.iter().enumerate() {
        if !*flag {
            // 找到就立刻返回，不用跑完整个循环
            return i as i32;
        }
    }
    -1
    // 到第 08 课你会把返回类型换成 Option<usize>：
    // 那样「没找到」是类型的一部分，调用方不可能忘记处理它。
}

fn main() {
    println!("这是第 03 课练习的答案：cargo test --example ans03");
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
