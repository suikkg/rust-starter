//! 第 07 课练习的参考答案。
//!
//!     cargo test --example ans07
#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

impl Task {
    fn new(id: u32, title: &str) -> Task {
        Task {
            id,
            // 字段名和变量名一样时可以只写一次（上面的 id 就是）。
            // title 要从 &str 转成 String，所以这里写全。
            title: title.to_string(),
            // 新任务一律未完成——这条规则写在构造函数里，
            // 就没人能造出一个"生下来就已完成"的任务。
            done: false,
        }
    }

    fn line(&self) -> String {
        let mark = if self.done { "✓" } else { " " };
        format!("[{}] {}  {}", mark, self.id, self.title)
    }

    fn toggle(&mut self) {
        self.done = !self.done;
    }

    fn is_important(&self) -> bool {
        // starts_with 对空串返回 false，不会崩溃
        self.title.starts_with('!')
    }
}

fn important_count(tasks: &[Task]) -> usize {
    let mut n = 0;
    for t in tasks {
        if t.is_important() {
            n += 1;
        }
    }
    n
}

fn main() {
    println!("这是第 07 课练习的答案：cargo test --example ans07");
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
