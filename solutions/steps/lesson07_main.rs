// 第 07 课答案：两个平行列表合并成一个 Vec<Task>。
#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

impl Task {
    /// 关联函数：用 Task::new(...) 调用
    fn new(id: u32, title: &str) -> Task {
        Task {
            id,
            title: title.to_string(),
            done: false,
        }
    }

    /// 方法：用 task.line() 调用，&self = 只读
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
    let mut tasks = vec![
        Task::new(1, "学习 Rust 变量"),
        Task::new(2, "练习函数"),
        Task::new(3, "认识 struct"),
    ];

    // 要调用 &mut self 的方法，变量本身得是 mut
    tasks[0].finish();

    render(&tasks);

    println!();
    println!("原始数据：{:?}", tasks[0]); // {:?} 靠的是 #[derive(Debug)]
}

fn render(tasks: &[Task]) {
    println!("我的待办清单");
    println!();
    for task in tasks {
        println!("{}", task.line());
    }
    let done = tasks.iter().filter(|t| t.done).count();
    println!();
    println!("共 {} 项，已完成 {} 项。", tasks.len(), done);
}
