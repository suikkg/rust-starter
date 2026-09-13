//! 第 13 课练习的**答案**：闭包与迭代器
//!
//!     cargo test --example ans13
//!
//! 先自己写 15 分钟。骨架在 `exercises/ex13_closures.rs`。
#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

/// 【1】数一数还有几条没做完。
///
/// 提示：`.iter().filter(...).count()`
///
/// 注意 `filter` 的闭包拿到的是 `&&Task`（引用的引用）—— 因为 `iter()`
/// 已经给了 `&Task`，`filter` 又借了一次。`t.done` 照样能写，
/// Rust 会自动解引用。
fn count_pending(tasks: &[Task]) -> usize {
    tasks.iter().filter(|t| !t.done).count()
}

/// 【2】把所有标题收成一个 `Vec<String>`。
///
/// 提示：`.iter().map(...).collect()`
///
/// **先试试写 `.map(|t| t.title)`，看编译器说什么。** 那个报错值得读完：
/// `t` 是 `&Task`，`t.title` 是借来的 `String`，不能从借来的东西里搬走。
/// （第 06 课那条规矩，换了个地方出现。）
fn titles(tasks: &[Task]) -> Vec<String> {
    // clone() 是必须的：t 是 &Task，t.title 是借来的，搬不走。
    // 想省掉这次 clone 就得改签名返回 Vec<&str>——由调用方决定要不要拥有。
    tasks.iter().map(|t| t.title.clone()).collect()
}

/// 【3】找第一条没做完的。都做完了返回 `None`。
///
/// 提示：`.iter().find(...)`
///
/// `find` 返回的正是第 08 课那个 `Option` —— 「找不到」在类型里写着，
/// 调用方躲不掉。
fn first_pending(tasks: &[Task]) -> Option<&Task> {
    tasks.iter().find(|t| !t.done)
}

/// 【4】找标题**字数**最多的那一条，返回它的标题。空列表返回 `None`。
///
/// 提示：`.iter().max_by_key(...)`
///
/// **字数不是 `len()`。** `"学习".len()` 是 6（字节数），
/// `"学习".chars().count()` 才是 2。用错了，"abcd" 会被判成比 "学习编程" 长。
/// （第 05 课那个坑，这里再撞一次。）
fn longest_title(tasks: &[Task]) -> Option<&str> {
    tasks
        .iter()
        .max_by_key(|t| t.title.chars().count())
        .map(|t| t.title.as_str())
}

/// 【5】把每一条的标题都过一遍 `f`，原地改掉。
///
/// `f` 是**传进来的闭包**。调用的样子：
///
/// ```ignore
/// rename_all(&mut tasks, |t| format!("[待办] {t}"));
/// ```
///
/// 提示：`.iter_mut().for_each(...)`，或者这一题用 `for` 也行 ——
/// 要改原值的时候 `for` 往往更清楚。
///
/// `impl Fn(&str) -> String` 读作「任何一个吃 `&str` 吐 `String` 的东西」。
/// 函数和闭包都算。
fn rename_all(tasks: &mut [Task], f: impl Fn(&str) -> String) {
    tasks.iter_mut().for_each(|t| t.title = f(&t.title));
}

fn main() {
    println!("这是答案，用 cargo test --example ans13 跑");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 样例() -> Vec<Task> {
        vec![
            Task {
                id: 1,
                title: "学习变量".into(),
                done: true,
            },
            Task {
                id: 2,
                title: "练习函数".into(),
                done: false,
            },
            Task {
                id: 3,
                title: "搞懂所有权和借用".into(),
                done: false,
            },
        ]
    }

    #[test]
    fn 数未完成() {
        assert_eq!(count_pending(&样例()), 2);
        assert_eq!(count_pending(&[]), 0);
    }

    #[test]
    fn 全做完了是零() {
        let done: Vec<Task> = 样例()
            .into_iter()
            .map(|mut t| {
                t.done = true;
                t
            })
            .collect();
        assert_eq!(count_pending(&done), 0);
    }

    #[test]
    fn 取标题() {
        assert_eq!(
            titles(&样例()),
            vec!["学习变量", "练习函数", "搞懂所有权和借用"]
        );
    }

    #[test]
    fn 取标题不能动原来的列表() {
        // 参数是 &[Task]，所以原列表必须原封不动 —— 这就是为什么要 clone。
        let tasks = 样例();
        let _ = titles(&tasks);
        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks[0].title, "学习变量");
    }

    #[test]
    fn 找第一条未完成() {
        assert_eq!(first_pending(&样例()).map(|t| t.id), Some(2));
    }

    #[test]
    fn 没有未完成的时候返回None() {
        let mut tasks = 样例();
        tasks.iter_mut().for_each(|t| t.done = true);
        assert!(first_pending(&tasks).is_none());
    }

    #[test]
    fn 最长标题按字数不按字节() {
        assert_eq!(longest_title(&样例()), Some("搞懂所有权和借用"));
        assert_eq!(longest_title(&[]), None);
    }

    #[test]
    fn 八个英文字母短过八个汉字() {
        // 用 len() 的话这两个都是 8 和 24，结果会反过来
        let tasks = vec![
            Task {
                id: 1,
                title: "abcdefgh".into(),
                done: false,
            },
            Task {
                id: 2,
                title: "搞懂所有权和借用".into(),
                done: false,
            },
        ];
        assert_eq!(longest_title(&tasks), Some("搞懂所有权和借用"));
    }

    #[test]
    fn 批量改名() {
        let mut tasks = 样例();
        rename_all(&mut tasks, |t| format!("[待办] {t}"));
        assert_eq!(tasks[0].title, "[待办] 学习变量");
        assert_eq!(tasks[2].title, "[待办] 搞懂所有权和借用");
    }

    #[test]
    fn 闭包能抓住外面的变量() {
        // 这是闭包比函数强的地方：prefix 是外面的，闭包直接用上了
        let prefix = "今天：";
        let mut tasks = 样例();
        rename_all(&mut tasks, |t| format!("{prefix}{t}"));
        assert_eq!(tasks[1].title, "今天：练习函数");
    }
}
