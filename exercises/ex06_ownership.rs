//! 第 06 课练习：所有权与借用（最难的一关）
//!
//!     cargo test --example ex06
//!
//! 这一课的测试里有几行是**专门用来卡你的**：它们在调用你的函数之后，
//! 继续使用传进去的那个值。如果你的签名收了所有权，那几行就编译不过。
//!
//! 编译不过的时候，请**完整读一遍报错**再改 —— 报错里通常直接写着改法。
//!
//! 答案：`solutions/exercises/ex06.rs`
#![allow(dead_code, unused_variables)]

/// 【1】所有标题加起来有多少字节。
///
/// 只读，不改 —— 所以收**不可变借用**。
fn total_len(list: &[String]) -> usize {
    todo!()
}

/// 【2】在原字符串末尾追加后缀，**改的是原件**，不返回新串。
///
/// 只有 `&mut` 能做到这件事。
#[allow(clippy::ptr_arg)] // 同上：实现里要 push_str，写完就不告警了
fn append_suffix(s: &mut String, suffix: &str) {
    todo!()
}

/// 【3】取第一个空格之前的那一段；没有空格就返回整串。
///
/// 返回的是**借来的一段**（`&str`），不是新 String —— 不许用 `to_string()`。
/// 提示：`s.find(' ')` 返回 `Option<usize>`，`&s[..i]` 能切出一段。
fn first_word(s: &str) -> &str {
    todo!()
}

/// 【4】把标题改成大写，**不动原件**。
///
/// 这一题和【2】正好相反：这里必须造一个新 String 返回。
fn shouted(s: &str) -> String {
    todo!()
}

fn main() {
    println!("这一课的练习请运行：cargo test --example ex06");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn 借用不夺走所有权() {
        let list = vec!["ab".to_string(), "cde".to_string()];
        assert_eq!(total_len(&list), 5);
        // 关键的一行：调用完之后 list 还归我。签名收了 Vec<String> 这里就编译不过。
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
