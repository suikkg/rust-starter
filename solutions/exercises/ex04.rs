//! 第 04 课练习的参考答案。
//!
//!     cargo test --example ans04
#![allow(dead_code)]

fn summary(total: u32, done: u32) -> String {
    // format! 里可以直接写变量名，不用再写一遍 {} 和参数
    format!("共 {total} 项，已完成 {done} 项。")
}

fn pending(total: u32, done: u32) -> u32 {
    // saturating_sub：减到 0 就停，不会绕回一个天文数字，也不会 panic。
    // 直接写 total - done，在 done > total 时 debug 构建会崩溃、
    // release 构建会得到 4294967294 这种值——后者更可怕，因为它不报错。
    total.saturating_sub(done)
}

fn task_line(id: u32, title: &str, done: bool) -> String {
    let mark = if done { "✓" } else { " " };
    format!("[{mark}] {id}  {title}")
}

fn urgent(title: &str) -> String {
    // 收 &str = 只借来看看。调用方的 String 还归它自己，调用之后照样能用。
    // 如果这里写 title: String，调用方那个变量就被"吃掉"了（第 06 课）。
    format!("{title}！")
}

fn main() {
    println!("这是第 04 课练习的答案：cargo test --example ans04");
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
        assert_eq!(title.len(), "交周报".len(), "原来的 title 还要能用");
    }
}
