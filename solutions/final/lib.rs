//! 把各模块串起来，对外只提供一个 `run()`。
//!
//! 业务逻辑放在 lib 这一侧，测试才够得着——`main.rs` 里的东西测试调不到。

pub mod model;
pub mod store;
pub mod view;

use std::path::PathBuf;

/// 数据存在哪。
pub fn data_path() -> PathBuf {
    PathBuf::from("todos.json")
}

/// 干活的入口：成功返回要打印的话，失败返回要显示的错误。
///
/// 返回 `Result` 而不是自己 println：这样"出错怎么显示"只在 main 里写一遍。
pub fn run(args: &[String]) -> Result<String, String> {
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("");
    let path = data_path();

    // 存盘错误要变成给人看的话，所以统一 map_err
    let mut tasks = store::load(&path).map_err(|e| format!("读取 {} 失败：{e}", path.display()))?;

    match cmd {
        "add" => {
            let title = args
                .get(1)
                .ok_or("add 后面要跟任务名，例如：add \"学习 Rust\"")?;
            if title.trim().is_empty() {
                return Err("任务名不能是空的".to_string());
            }
            let id = store::add(&mut tasks, title);
            save(&path, &tasks)?;
            Ok(format!("已添加第 {id} 条：{title}"))
        }

        "list" => Ok(view::render(&tasks)),

        "done" => {
            let id = parse_id(args.get(1), "done")?;
            match store::finish(&mut tasks, id) {
                Some(task) => {
                    let title = task.title.clone();
                    save(&path, &tasks)?;
                    Ok(format!("已完成：{title}"))
                }
                None => Err(format!("找不到编号 {id}，先用 list 看看有哪些")),
            }
        }

        "rm" => {
            let id = parse_id(args.get(1), "rm")?;
            match store::remove(&mut tasks, id) {
                Some(title) => {
                    save(&path, &tasks)?;
                    Ok(format!("已删除：{title}"))
                }
                None => Err(format!("找不到编号 {id}")),
            }
        }

        "" => Err(help()),
        other => Err(format!("不认识的命令：{other}\n\n{}", help())),
    }
}

fn save(path: &std::path::Path, tasks: &[model::Task]) -> Result<(), String> {
    store::save(path, tasks).map_err(|e| format!("保存 {} 失败：{e}", path.display()))
}

/// 编号解析只写一遍，add/done/rm 共用。
fn parse_id(raw: Option<&String>, cmd: &str) -> Result<u32, String> {
    let raw = raw.ok_or_else(|| format!("{cmd} 后面要跟编号，例如：{cmd} 1"))?;
    raw.parse()
        .map_err(|_| format!("编号必须是数字，你给的是 {raw}"))
}

fn help() -> String {
    [
        "用法：",
        "  cargo run -- add \"任务名\"      添加",
        "  cargo run -- list              查看全部",
        "  cargo run -- done 1            标记完成",
        "  cargo run -- rm 1              删除",
    ]
    .join("\n")
}

#[cfg(test)]
// 测试名用中文，失败信息一眼能看懂是哪条规则破了。
// 中文名里夹着 None / JSON 这类大写 ASCII 会触发 non_snake_case，
// 按模块限定关掉——allow 要贴在最小范围上并写明理由。
#[allow(non_snake_case)]
mod tests {
    use super::*;

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn 没有命令时给出用法() {
        let err = run(&args(&[])).unwrap_err();
        assert!(err.contains("用法"));
    }

    #[test]
    fn 不认识的命令会报错() {
        let err = run(&args(&["飞天"])).unwrap_err();
        assert!(err.contains("不认识的命令"));
    }

    #[test]
    fn 编号不是数字会报错而不是崩溃() {
        let err = run(&args(&["done", "abc"])).unwrap_err();
        assert!(err.contains("必须是数字"));
    }
}
