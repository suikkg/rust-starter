//! 第 08 课：enum、match、Option —— 表达"几种情况之一"
//!
//!     cargo run --example lesson08_enum_option
//!
//! enum 表示"只可能是这几种之一"。
//! match 要求你把每一种都处理掉，漏一种就编译不过 —— 这是 Rust 帮你堵漏洞。

/// 用户想干什么。比用字符串到处比较安全得多：拼错就是编译错误。
#[derive(Debug)]
enum Command {
    Add(String), // 带一个字符串：要加的标题
    Done(u32),   // 带一个数字：要完成的编号
    List,        // 不带数据
}

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn main() {
    let mut tasks = vec![
        Task {
            id: 1,
            title: "学习 Rust 变量".into(),
            done: false,
        },
        Task {
            id: 2,
            title: "练习函数".into(),
            done: false,
        },
    ];

    let commands = vec![
        Command::Add("认识 enum".to_string()),
        Command::Done(1),
        Command::Done(99), // 故意给一个不存在的编号
        Command::List,
    ];

    for cmd in commands {
        // match 必须覆盖每一个分支
        match cmd {
            Command::Add(title) => {
                let id = tasks.len() as u32 + 1;
                tasks.push(Task {
                    id,
                    title,
                    done: false,
                });
                println!("已添加第 {id} 条");
            }
            Command::Done(id) => match find_mut(&mut tasks, id) {
                // Option 只有两种：Some(有) 和 None(没有)
                Some(task) => {
                    task.done = true;
                    println!("已完成：{}", task.title);
                }
                None => println!("找不到编号 {id}"),
            },
            Command::List => {
                println!("---");
                for t in &tasks {
                    let mark = if t.done { "✓" } else { " " };
                    println!("[{}] {}  {}", mark, t.id, t.title);
                }
                println!("---");
            }
        }
    }
}

/// 返回 Option 而不是直接返回 Task：找不到时没有东西可返回，
/// 这正是 Option 存在的理由 —— Rust 没有 null。
///
/// 参数是 `&mut [Task]` 不是 `&mut Vec<Task>`：这里只改元素，不增删，
/// 所以切片就够了。要 push 才需要 Vec（见 Command::Add 分支）。
fn find_mut(tasks: &mut [Task], id: u32) -> Option<&mut Task> {
    tasks.iter_mut().find(|t| t.id == id)
}

// 【动手】给 Command 加一个 Remove(u32) 分支。
// 加完先别写 match 分支，直接 cargo check —— 看编译器怎么提醒你漏了一种情况。
