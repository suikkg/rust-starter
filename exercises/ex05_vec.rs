//! 第 05 课练习：Vec 列表
//!
//!     cargo test --example ex05
//!
//! 答案：`solutions/exercises/ex05.rs`
#![allow(dead_code, unused_variables)]

/// 【1】往清单末尾加一条。
///
/// 注意参数是 `&mut Vec<String>`：要改别人的东西，就得借可变的（第 06 课细讲）。
// 函数体还是 todo!() 的时候，clippy 看不出这里真的需要 Vec（实现里要 push），
// 会建议换成 &[String]。写完实现之后这条告警自己就没了。
#[allow(clippy::ptr_arg)]
fn add_title(list: &mut Vec<String>, title: &str) {
    todo!()
}

/// 【2】取第 index 条；越界就返回 fallback。
///
/// **不要用 `list[index]`** —— 越界会直接崩溃。用 `.get(index)`，
/// 它返回 `Option`（第 08 课细讲），这里先用 `match` 或 `if let` 应付。
fn nth_or(list: &[String], index: usize, fallback: &str) -> String {
    todo!()
}

/// 【3】最长的那条标题（按字符数）。空清单返回空串。
///
/// 一样长就返回**先出现**的那条。
fn longest(list: &[String]) -> String {
    todo!()
}

/// 【4】数一数有几条以 prefix 开头。
///
/// 提示：`s.starts_with(prefix)`。
fn count_with_prefix(list: &[String], prefix: &str) -> usize {
    todo!()
}

fn main() {
    println!("这一课的练习请运行：cargo test --example ex05");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    fn 样例() -> Vec<String> {
        vec![
            "买菜".to_string(),
            "写周报".to_string(),
            "写单元测试".to_string(),
        ]
    }

    #[test]
    fn 加一条() {
        let mut list = Vec::new();
        add_title(&mut list, "买菜");
        add_title(&mut list, "写周报");
        assert_eq!(list.len(), 2);
        assert_eq!(list[1], "写周报");
    }

    #[test]
    fn 越界不崩溃() {
        let list = 样例();
        assert_eq!(nth_or(&list, 0, "（无）"), "买菜");
        assert_eq!(nth_or(&list, 99, "（无）"), "（无）", "越界要返回 fallback");
        assert_eq!(nth_or(&[], 0, "（空）"), "（空）");
    }

    #[test]
    fn 最长的标题() {
        assert_eq!(longest(&样例()), "写单元测试");
        assert_eq!(longest(&[]), "");
        // 一样长取先出现的
        let tie = vec!["abc".to_string(), "xyz".to_string()];
        assert_eq!(longest(&tie), "abc");
    }

    #[test]
    fn 按前缀数条数() {
        let list = 样例();
        assert_eq!(count_with_prefix(&list, "写"), 2);
        assert_eq!(count_with_prefix(&list, "买"), 1);
        assert_eq!(count_with_prefix(&list, "跑"), 0);
    }
}
