//! 第 05 课练习的参考答案。
//!
//!     cargo test --example ans05
#![allow(dead_code)]

fn add_title(list: &mut Vec<String>, title: &str) {
    // &str 转 String 才能放进 Vec<String>
    list.push(title.to_string());
}

fn nth_or(list: &[String], index: usize, fallback: &str) -> String {
    // .get() 返回 Option：越界时是 None，而不是像 list[index] 那样直接崩溃。
    match list.get(index) {
        Some(s) => s.clone(),
        None => fallback.to_string(),
    }
    // 第 08 课之后可以写成：
    // list.get(index).cloned().unwrap_or_else(|| fallback.to_string())
}

fn longest(list: &[String]) -> String {
    let mut best = String::new();
    let mut best_len = 0;
    for s in list {
        // 用 chars().count() 而不是 len()：len() 数的是字节，
        // 一个汉字占 3 个字节，按字节比会得到反直觉的结果。
        let n = s.chars().count();
        // 严格大于，所以一样长时先出现的那条留着不被换掉
        if n > best_len {
            best_len = n;
            best = s.clone();
        }
    }
    best
}

fn count_with_prefix(list: &[String], prefix: &str) -> usize {
    let mut n = 0;
    for s in list {
        if s.starts_with(prefix) {
            n += 1;
        }
    }
    n
    // 迭代器写法：list.iter().filter(|s| s.starts_with(prefix)).count()
}

fn main() {
    println!("这是第 05 课练习的答案：cargo test --example ans05");
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
