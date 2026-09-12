//! 第 07 课练习：struct 结构体
//!
//!     cargo test --example ex07
//!
//! `Task` 的定义已经给好了，你写它的 `impl`。
//! 注意每个方法的第一个参数：`&self`（只读）还是 `&mut self`（要改）。
//!
//! 答案：`solutions/exercises/ex07.rs`
#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

impl Task {
    /// 【1】造一条新任务，新任务一律是未完成。
    ///
    /// 注意 `title` 收的是 `&str`，而字段是 `String` —— 中间要转一下。
    fn new(id: u32, title: &str) -> Task {
        todo!()
    }

    /// 【2】渲染成一行：`[✓] 1  学习 Rust` / `[ ] 2  练习`
    ///
    /// 只读，所以是 `&self`。
    fn line(&self) -> String {
        todo!()
    }

    /// 【3】翻转完成状态：做完的变没做，没做的变做完。
    ///
    /// 要改自己，所以是 `&mut self`。
    fn toggle(&mut self) {
        todo!()
    }

    /// 【4】标题以 `!` 开头就算重要。
    fn is_important(&self) -> bool {
        todo!()
    }
}

/// 【5】清单里有几条重要任务。
fn important_count(tasks: &[Task]) -> usize {
    todo!()
}

fn main() {
    println!("这一课的练习请运行：cargo test --example ex07");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn 新任务是未完成的() {
        let t = Task::new(1, "学习 Rust");
        assert_eq!(t.id, 1);
        assert_eq!(t.title, "学习 Rust");
        assert!(!t.done, "新任务不该是已完成");
    }

    #[test]
    fn 渲染一行() {
        let mut t = Task::new(1, "学习 Rust");
        assert_eq!(t.line(), "[ ] 1  学习 Rust");
        t.done = true;
        assert_eq!(t.line(), "[✓] 1  学习 Rust");
    }

    #[test]
    fn 翻转两次回到原样() {
        let mut t = Task::new(1, "a");
        t.toggle();
        assert!(t.done);
        t.toggle();
        assert!(!t.done, "翻转两次应该回到未完成");
    }

    #[test]
    fn 重要任务看标题开头() {
        assert!(Task::new(1, "!交周报").is_important());
        assert!(!Task::new(2, "买菜").is_important());
        assert!(!Task::new(3, "").is_important(), "空标题不该崩溃");
    }

    #[test]
    fn 数重要任务() {
        let tasks = vec![
            Task::new(1, "!急事"),
            Task::new(2, "买菜"),
            Task::new(3, "!很急"),
        ];
        assert_eq!(important_count(&tasks), 2);
        assert_eq!(important_count(&[]), 0);
    }
}
