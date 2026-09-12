// 第 10 课答案：数据存进 todos.json，关掉程序还在。
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Serialize = 能变成 JSON，Deserialize = 能读回来
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(msg) => println!("{msg}"),
        Err(msg) => {
            eprintln!("错误：{msg}");
            std::process::exit(1);
        }
    }
}

fn data_path() -> PathBuf {
    PathBuf::from("todos.json")
}

fn run(args: &[String]) -> Result<String, String> {
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("");
    let path = data_path();

    // 每个命令开头先读，改完再写
    let mut tasks = load(&path).map_err(|e| format!("读取 {} 失败：{e}", path.display()))?;

    match cmd {
        "add" => {
            let title = args.get(1).ok_or("add 后面要跟任务名")?;
            if title.trim().is_empty() {
                return Err("任务名不能是空的".to_string());
            }
            let id = next_id(&tasks);
            tasks.push(Task::new(id, title));
            write(&path, &tasks)?;
            Ok(format!("已添加第 {id} 条：{title}"))
        }
        "list" => {
            let mut out = String::from("我的待办清单\n\n");
            if tasks.is_empty() {
                out.push_str("（清单是空的，用 add 加一条）\n");
            }
            for t in &tasks {
                out.push_str(&t.line());
                out.push('\n');
            }
            let done = tasks.iter().filter(|t| t.done).count();
            out.push_str(&format!("\n共 {} 项，已完成 {} 项。", tasks.len(), done));
            Ok(out)
        }
        "done" => {
            let id = parse_id(args.get(1), "done")?;
            match tasks.iter_mut().find(|t| t.id == id) {
                Some(task) => {
                    task.done = true;
                    let title = task.title.clone();
                    write(&path, &tasks)?;
                    Ok(format!("已完成：{title}"))
                }
                None => Err(format!("找不到编号 {id}，先用 list 看看有哪些")),
            }
        }
        "rm" => {
            let id = parse_id(args.get(1), "rm")?;
            match tasks.iter().position(|t| t.id == id) {
                Some(pos) => {
                    let removed = tasks.remove(pos);
                    write(&path, &tasks)?;
                    Ok(format!("已删除：{}", removed.title))
                }
                None => Err(format!("找不到编号 {id}")),
            }
        }
        "" => Err(help()),
        other => Err(format!("不认识的命令：{other}\n\n{}", help())),
    }
}

/// 文件不存在返回空清单——第一次运行本来就没有，这不是错误。
/// 想不到这一点的话，程序第一次跑就会报错。
fn load(path: &Path) -> Result<Vec<Task>, Box<dyn Error>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(path)?;
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    let tasks = serde_json::from_str(&text)?; // ? 会把 serde 的错误往外扔
    Ok(tasks)
}

fn save(path: &Path, tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let text = serde_json::to_string_pretty(tasks)?;
    fs::write(path, text)?;
    Ok(())
}

/// Box<dyn Error> 转成给人看的话
fn write(path: &Path, tasks: &[Task]) -> Result<(), String> {
    save(path, tasks).map_err(|e| format!("保存 {} 失败：{e}", path.display()))
}

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
