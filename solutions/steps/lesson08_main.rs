// 第 08 课答案：用 enum 表达命令，用 Option 处理"找不到"。
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
            title: title.to_string(),
            done: false,
        }
    }

    fn line(&self) -> String {
        let mark = if self.done { "✓" } else { " " };
        format!("[{}] {}  {}", mark, self.id, self.title)
    }
}

/// 只可能是这几种之一。拼错是编译错误，不是运行时惊喜。
enum Command {
    Add(String),
    Done(u32),
    Remove(u32),
    List,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    // 下一课换成真正的命令行参数
    let commands = vec![
        Command::Add("学习 Rust 变量".to_string()),
        Command::Add("练习函数".to_string()),
        Command::Add("认识 enum".to_string()),
        Command::Done(1),
        Command::Done(99), // 故意给一个不存在的
        Command::Remove(2),
        Command::List,
    ];

    for cmd in commands {
        // match 必须覆盖每一个分支，漏一个就编译不过
        match cmd {
            Command::Add(title) => {
                let id = next_id(&tasks);
                tasks.push(Task::new(id, &title));
                println!("已添加第 {id} 条");
            }
            Command::Done(id) => match find_mut(&mut tasks, id) {
                Some(task) => {
                    task.done = true;
                    println!("已完成：{}", task.title);
                }
                // 找不到要给提示，不能崩溃
                None => println!("找不到编号 {id}"),
            },
            Command::Remove(id) => match tasks.iter().position(|t| t.id == id) {
                Some(pos) => {
                    let removed = tasks.remove(pos);
                    println!("已删除：{}", removed.title);
                }
                None => println!("找不到编号 {id}"),
            },
            Command::List => {
                println!();
                println!("我的待办清单");
                println!();
                for t in &tasks {
                    println!("{}", t.line());
                }
                let done = tasks.iter().filter(|t| t.done).count();
                println!();
                println!("共 {} 项，已完成 {} 项。", tasks.len(), done);
            }
        }
    }
}

/// 可能找不到，所以返回 Option——Rust 没有 null
fn find_mut(tasks: &mut [Task], id: u32) -> Option<&mut Task> {
    tasks.iter_mut().find(|t| t.id == id)
}

/// 用"最大 id + 1"而不是"条数 + 1"：删掉中间一条后者会算出重复编号
fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}
