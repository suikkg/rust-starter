//! 第 11 课：模块 —— 把程序拆成几块
//!
//!     cargo run --example lesson11_modules
//!
//! 文件一长就该拆。mod 划分区块，pub 决定"外面能不能用"。
//! 本例把三个模块写在同一个文件里方便看；真实项目里一个 mod 通常就是一个 .rs 文件。

/// 数据长什么样
mod model {
    #[derive(Debug, Clone)]
    pub struct Task {
        // 不写 pub，外面就用不了
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

/// 对数据做什么
mod store {
    use super::model::Task; // super = 上一层，这里指文件最外层

    pub fn add(tasks: &mut Vec<Task>, title: &str) -> u32 {
        let id = next_id(tasks); // 同一个 mod 内部，不用 pub 也能互相调用
        tasks.push(Task::new(id, title));
        id
    }

    // 只改元素不增删，所以收切片；上面的 add 要 push，才必须收 &mut Vec
    pub fn finish(tasks: &mut [Task], id: u32) -> bool {
        match tasks.iter_mut().find(|t| t.id == id) {
            Some(t) => {
                t.done = true;
                true
            }
            None => false,
        }
    }

    /// 没有 pub：这是内部细节，外面看不见也不该关心
    fn next_id(tasks: &[Task]) -> u32 {
        tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }
}

/// 怎么显示给人看
mod view {
    use super::model::Task;

    pub fn render(tasks: &[Task]) {
        println!("我的待办清单");
        println!();
        for t in tasks {
            let mark = if t.done { "✓" } else { " " };
            println!("[{}] {}  {}", mark, t.id, t.title);
        }
        let done = tasks.iter().filter(|t| t.done).count();
        println!();
        println!("共 {} 项，已完成 {} 项。", tasks.len(), done);
    }
}

use model::Task;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    store::add(&mut tasks, "学习 Rust 变量");
    store::add(&mut tasks, "练习函数");
    store::add(&mut tasks, "拆分模块");

    store::finish(&mut tasks, 1);

    view::render(&tasks);

    // 【动手】把 store 里的 next_id 前面加上 pub，然后在 main 里调用
    // store::next_id(&tasks)。再把 pub 去掉，读一遍报错。
    // 这就是"可见性"：pub 是你主动开的口子。
}
