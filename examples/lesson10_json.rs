//! 第 10 课：存文件与 JSON
//!
//!     cargo run --example lesson10_json
//!
//! 程序关掉之后数据还在 —— 靠的是把内存里的结构体写进文件。
//! serde 负责"结构体 <-> JSON 文本"的来回转换。

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Serialize = 能变成 JSON，Deserialize = 能从 JSON 读回来。
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn main() {
    // 写到临时目录，免得弄脏项目
    let path = std::env::temp_dir().join("rust-starter-demo.json");

    let tasks = vec![
        Task {
            id: 1,
            title: "学习 Rust 变量".into(),
            done: true,
        },
        Task {
            id: 2,
            title: "练习函数".into(),
            done: false,
        },
    ];

    match save(&path, &tasks) {
        Ok(()) => println!("已保存到 {}", path.display()),
        Err(e) => {
            eprintln!("保存失败：{e}");
            return;
        }
    }

    // 看看文件里到底长什么样
    let text = fs::read_to_string(&path).unwrap_or_default();
    println!("---- 文件内容 ----");
    println!("{text}");
    println!("------------------");

    match load(&path) {
        Ok(loaded) => {
            println!("读回来 {} 条：", loaded.len());
            for t in &loaded {
                let mark = if t.done { "✓" } else { " " };
                println!("[{}] {}  {}", mark, t.id, t.title);
            }
        }
        Err(e) => eprintln!("读取失败：{e}"),
    }
}

/// Box<dyn std::error::Error> 的意思是"任何一种错误都行"。
/// 入门阶段够用；正式项目里会定义自己的错误类型。
fn save(path: &Path, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    let text = serde_json::to_string_pretty(tasks)?; // ? = 失败就把错误往外扔
    fs::write(path, text)?;
    Ok(())
}

fn load(path: &Path) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    // 文件不存在不算错误 —— 第一次运行本来就没有
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(path)?;
    let tasks = serde_json::from_str(&text)?;
    Ok(tasks)
}

// 【动手】手动把那个 json 文件改坏（比如删掉一个引号），再运行一次，
// 看 serde 报的错能不能告诉你错在第几行第几列。
