// 第 09 课答案：接命令行参数，用 Result 处理错误。
// 数据还存在内存里，跑完就没了——下一课解决。
use std::env;

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

fn main() {
    // 第 0 个是程序自己的路径
    let args: Vec<String> = env::args().skip(1).collect();

    // main 只负责显示，不含业务逻辑
    match run(&args) {
        Ok(msg) => println!("{msg}"),
        Err(msg) => {
            eprintln!("错误：{msg}");
            std::process::exit(1);
        }
    }
}

/// 干活的入口：成功返回要打印的话，失败返回要显示的错误。
fn run(args: &[String]) -> Result<String, String> {
    // 这一行里有 Option 的三种常见用法
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("");

    // 暂时每次都从零开始，所以 add 之外的命令看不到数据——下一课接文件
    let mut tasks: Vec<Task> = Vec::new();

    match cmd {
        "add" => {
            let title = args
                .get(1)
                .ok_or("add 后面要跟任务名，例如：add \"学习 Rust\"")?;
            if title.trim().is_empty() {
                return Err("任务名不能是空的".to_string());
            }
            let id = next_id(&tasks);
            tasks.push(Task::new(id, title));
            Ok(format!("已添加第 {id} 条：{title}"))
        }
        "list" => {
            let mut out = String::from("我的待办清单\n\n");
            if tasks.is_empty() {
                out.push_str("（清单是空的）\n");
            }
            for t in &tasks {
                out.push_str(&t.line());
                out.push('\n');
            }
            Ok(out)
        }
        "done" => {
            let id = parse_id(args.get(1), "done")?;
            Ok(format!("已完成第 {id} 条"))
        }
        "rm" => {
            let id = parse_id(args.get(1), "rm")?;
            Ok(format!("已删除第 {id} 条"))
        }
        "" => Err(help()),
        other => Err(format!("不认识的命令：{other}\n\n{}", help())),
    }
}

/// 编号解析只写一遍，done 和 rm 共用。
/// 末尾的 ? 意思是：成功取出值继续走，失败立刻返回这个错误。
fn parse_id(raw: Option<&String>, cmd: &str) -> Result<u32, String> {
    let raw = raw.ok_or_else(|| format!("{cmd} 后面要跟编号，例如：{cmd} 1"))?;
    raw.parse()
        .map_err(|_| format!("编号必须是数字，你给的是 {raw}"))
}

fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn help() -> String {
    [
        "用法：",
        "  cargo run -- add \"任务名\"",
        "  cargo run -- list",
        "  cargo run -- done 1",
        "  cargo run -- rm 1",
    ]
    .join("\n")
}
