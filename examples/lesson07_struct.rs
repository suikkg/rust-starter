//! 第 07 课：struct —— 把一条任务的几个字段绑在一起
//!
//!     cargo run --example lesson07_struct
//!
//! 之前编号、标题、完成状态是三个散的变量，很容易对错位。
//! struct 把它们捆成一个整体：一条任务就是一个 Task。

/// derive 是"让编译器自动帮我实现"。
/// Debug 让 {:?} 能打印它，Clone 让它能复制。
#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

/// impl 块里放"属于 Task 的函数"。
impl Task {
    /// 没有 self 参数 = 关联函数，用 Task::new(...) 调用，通常用来造新的。
    fn new(id: u32, title: &str) -> Task {
        Task {
            id,
            title: title.to_string(),
            done: false,
        }
    }

    /// 有 &self = 方法，用 task.line() 调用，只读不改。
    fn line(&self) -> String {
        let mark = if self.done { "✓" } else { " " };
        format!("[{}] {}  {}", mark, self.id, self.title)
    }

    /// &mut self = 要改自己
    fn finish(&mut self) {
        self.done = true;
    }
}

fn main() {
    let mut tasks = vec![Task::new(1, "学习 Rust 变量"), Task::new(2, "练习函数")];

    // 改第一条。tasks[0] 前面加 &mut 才能调用 finish。
    tasks[0].finish();

    println!("我的待办清单");
    println!();
    for task in &tasks {
        println!("{}", task.line());
    }

    // {:?} 靠的就是上面那个 #[derive(Debug)]
    println!();
    println!("原始数据：{:?}", tasks[0]);

    // 【动手】给 Task 加一个字段 note: String，
    // 改 new()，再让 line() 在有备注时把它显示出来。
}
