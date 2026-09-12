//! 第 11 课练习：模块与可见性
//!
//!     cargo test --example ex11
//!
//! 真实项目里模块是分文件的，这里为了能一个文件跑起来，用 `mod 名字 { }`
//! 写在同一个文件里 —— **语义完全一样**，`pub` 的规则也一样。
//!
//! 这一课的重点是**哪些东西该 pub、哪些不该**。
//!
//! 答案：`solutions/exercises/ex11.rs`
#![allow(dead_code, unused_variables)]

mod model {
    /// 数据定义要 `pub`，不然别的模块连类型都写不出来。
    ///
    /// 字段也得 `pub` —— 模块外面默认连字段都读不到。
    #[derive(Debug, Clone, PartialEq)]
    pub struct Task {
        pub id: u32,
        pub title: String,
        pub done: bool,
    }

    impl Task {
        /// 【1】造一条新任务（未完成）。
        pub fn new(id: u32, title: &str) -> Task {
            todo!()
        }
    }
}

mod store {
    // `use super::` = 用上一层的东西。分文件之后这里会写 `use crate::model::Task;`
    use super::model::Task;

    /// 【2】加一条，返回新任务的编号。
    #[allow(clippy::ptr_arg)] // 实现里要 push，写完就不告警了
    pub fn add(tasks: &mut Vec<Task>, title: &str) -> u32 {
        todo!()
    }

    /// 【3】按编号删掉一条，返回被删的标题；找不到返回 `None`。
    ///
    /// 提示：`iter().position(|t| ...)` 找下标，`Vec::remove(i)` 删。
    #[allow(clippy::ptr_arg)] // 实现里要 Vec::remove，写完就不告警了
    pub fn remove(tasks: &mut Vec<Task>, id: u32) -> Option<String> {
        todo!()
    }

    /// 【4】下一个编号 = 现有最大编号 + 1；空清单从 1 开始。
    ///
    /// **注意这里没有 `pub`** —— 它是 store 的内部细节。
    /// 外面只需要知道 `add` 会给一个不重复的编号，不需要知道怎么给的。
    ///
    /// 【动手】给它加上 `pub`，再到测试里加一行 `store::next_id(&[]);`，
    /// 确认能编译。然后把 `pub` 去掉，看报错长什么样 —— 那条报错
    /// （`function \`next_id\` is private`）就是模块边界在起作用。
    fn next_id(tasks: &[Task]) -> u32 {
        todo!()
    }

    #[cfg(test)]
    mod tests {
        // 同一个模块内部，私有的也测得到 —— 这正是单元测试写在模块里的理由。
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
            // 条数是 2，但新编号必须是 4：用「条数 + 1」就会撞号
            assert_eq!(next_id(&tasks), 4);
        }
    }
}

mod view {
    use super::model::Task;

    /// 【5】渲染整份清单。空清单要给一句提示。
    ///
    /// ```text
    /// 我的待办清单
    ///
    /// [✓] 1  买菜
    /// [ ] 2  写周报
    ///
    /// 共 2 项，已完成 1 项。
    /// ```
    ///
    /// 空清单时中间那块换成一行 `（清单是空的，用 add 加一条）`。
    pub fn render(tasks: &[Task]) -> String {
        todo!()
    }
}

fn main() {
    println!("这一课的练习请运行：cargo test --example ex11");
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
