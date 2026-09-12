//! 第 06 课练习的参考答案。
//!
//!     cargo test --example ans06
//!
//! 这一课的四个签名，正好是四种典型选择：
//!
//! | 签名 | 什么时候用 |
//! |---|---|
//! | `&[T]` / `&str` | 只读 —— **默认先想它** |
//! | `&mut T` | 要改调用方那个值 |
//! | `-> &str` | 返回原数据里的一段，不复制 |
//! | `-> String` | 造了个新的还回去 |
#![allow(dead_code)]

fn total_len(list: &[String]) -> usize {
    let mut n = 0;
    for s in list {
        n += s.len();
    }
    n
    // 收 &[String] 而不是 Vec<String>：只是借来数一数，数完还给调用方。
    // 收 Vec<String> 的话，调用一次之后调用方那个 list 就没了。
}

fn append_suffix(s: &mut String, suffix: &str) {
    // &mut = 借来并且能改。改的是调用方那一个 String，不是副本。
    s.push_str(suffix);
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        // &s[..i] 切出来的还是借用，没有复制任何字符。
        // 返回值的生命周期自动跟着参数 s——只有一个引用参数时不用手写 <'a>。
        Some(i) => &s[..i],
        None => s,
    }
}

fn shouted(s: &str) -> String {
    // to_uppercase 本来就会造一个新 String，原件一个字节都不动。
    s.to_uppercase()
}

fn main() {
    println!("这是第 06 课练习的答案：cargo test --example ans06");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn 借用不夺走所有权() {
        let list = vec!["ab".to_string(), "cde".to_string()];
        assert_eq!(total_len(&list), 5);
        assert_eq!(total_len(&list), 5, "第二次调用说明第一次没把 list 吃掉");
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn 可变借用改的是原件() {
        let mut title = String::from("周报");
        append_suffix(&mut title, "（急）");
        assert_eq!(title, "周报（急）", "要改原件，不是返回新串");
    }

    #[test]
    fn 切出第一个词() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn 造新串时原件不变() {
        let title = String::from("report");
        assert_eq!(shouted(&title), "REPORT");
        assert_eq!(title, "report", "原件不该被改动");
    }
}
