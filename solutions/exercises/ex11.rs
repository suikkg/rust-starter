//! 第 11 课练习的参考答案。
//!
//!     cargo test --example ans11
//!
//! 三个模块的分工，和 `solutions/final/` 里的分文件版本完全一致：
//!
//! - `model` —— 数据长什么样
//! - `store` —— 对数据做什么
//! - `view`  —— 怎么显示给人看
//!
//! 分层的检验标准：`view` 里不该出现"改数据"，`store` 里不该出现"排版"。
#![allow(dead_code)]

mod model {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Task {
        pub id: u32,
        pub title: String,
        pub done: bool,
    }

    impl Task {
        pub fn new(id: u32, title: &str) -> Task {
            Task {
                id,
                title: title.to_string(),
                done: false,
            }
        }
    }
}

mod store {
    use super::model::Task;

    pub fn add(tasks: &mut Vec<Task>, title: &str) -> u32 {
        let id = next_id(tasks);
        tasks.push(Task::new(id, title));
        id
    }

    pub fn remove(tasks: &mut Vec<Task>, id: u32) -> Option<String> {
        // position 找下标，找不到就 ? 提前返回 None
        let pos = tasks.iter().position(|t| t.id == id)?;
        Some(tasks.remove(pos).title)
    }

    /// 没有 pub：它是 store 的内部细节。
    ///
    /// 好处是将来想换算法（比如改成 UUID）只要改这一个函数，
    /// 不用担心别处已经依赖了"编号是连续整数"这件事——因为别处根本调不到它。
    fn next_id(tasks: &[Task]) -> u32 {
        // 用"最大编号 + 1"而不是"条数 + 1"：删掉中间一条之后，
        // 后者会算出一个已经存在的编号，然后两条任务撞号。
        tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn 空清单从1开始编号() {
            assert_eq!(next_id(&[]), 1);
        }

        #[test]
        fn 编号取最大值加一() {
            let tasks = vec![Task::new(1, "a"), Task::new(5, "b")];
            assert_eq!(next_id(&tasks), 6);
        }

        #[test]
        fn 删掉中间一条后编号不重复() {
            let mut tasks = vec![Task::new(1, "a"), Task::new(2, "b"), Task::new(3, "c")];
            remove(&mut tasks, 2);
            assert_eq!(next_id(&tasks), 4);
        }
    }
}

mod view {
    use super::model::Task;

    pub fn render(tasks: &[Task]) -> String {
        let mut out = String::from("我的待办清单\n\n");

        if tasks.is_empty() {
            out.push_str("（清单是空的，用 add 加一条）\n");
        } else {
            for t in tasks {
                let mark = if t.done { "✓" } else { " " };
                out.push_str(&format!("[{}] {}  {}\n", mark, t.id, t.title));
            }
        }

        let done = tasks.iter().filter(|t| t.done).count();
        out.push_str(&format!("\n共 {} 项，已完成 {} 项。", tasks.len(), done));
        out
    }
}

fn main() {
    println!("这是第 11 课练习的答案：cargo test --example ans11");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::model::Task;
    use super::{store, view};

    #[test]
    fn 新任务是未完成的() {
        let t = Task::new(1, "买菜");
        assert_eq!(t.id, 1);
        assert!(!t.done);
    }

    #[test]
    fn 加和删() {
        let mut tasks = Vec::new();
        assert_eq!(store::add(&mut tasks, "买菜"), 1);
        assert_eq!(store::add(&mut tasks, "写周报"), 2);
        assert_eq!(tasks.len(), 2);

        assert_eq!(store::remove(&mut tasks, 1), Some("买菜".to_string()));
        assert_eq!(tasks.len(), 1);
        assert_eq!(
            store::remove(&mut tasks, 99),
            None,
            "删不存在的编号要返回 None"
        );
    }

    #[test]
    fn 渲染整份清单() {
        let mut tasks = vec![Task::new(1, "买菜"), Task::new(2, "写周报")];
        tasks[0].done = true;

        let text = view::render(&tasks);
        assert!(text.starts_with("我的待办清单"));
        assert!(text.contains("[✓] 1  买菜"));
        assert!(text.contains("[ ] 2  写周报"));
        assert!(text.contains("共 2 项，已完成 1 项。"));
    }

    #[test]
    fn 空清单有提示() {
        let text = view::render(&[]);
        assert!(
            text.contains("清单是空的"),
            "空清单不能只显示一句「共 0 项」"
        );
    }
}
