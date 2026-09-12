//! 对数据做什么：增删改查 + 存盘。

use crate::model::Task;
use std::error::Error;
use std::fs;
use std::path::Path;

/// 文件不存在返回空清单——第一次运行本来就没有，这不是错误。
pub fn load(path: &Path) -> Result<Vec<Task>, Box<dyn Error>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(path)?;
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    let tasks = serde_json::from_str(&text)?;
    Ok(tasks)
}

pub fn save(path: &Path, tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let text = serde_json::to_string_pretty(tasks)?;
    fs::write(path, text)?;
    Ok(())
}

pub fn add(tasks: &mut Vec<Task>, title: &str) -> u32 {
    let id = next_id(tasks);
    tasks.push(Task::new(id, title));
    id
}

/// 完成一条。找不到编号返回 None——调用方决定怎么告诉用户。
pub fn finish(tasks: &mut [Task], id: u32) -> Option<&Task> {
    let task = tasks.iter_mut().find(|t| t.id == id)?;
    task.done = true;
    Some(task)
}

/// 删掉一条，返回被删的标题。
pub fn remove(tasks: &mut Vec<Task>, id: u32) -> Option<String> {
    let pos = tasks.iter().position(|t| t.id == id)?;
    Some(tasks.remove(pos).title)
}

/// 内部细节：没有 pub，外面用不了，将来想换算法也不会影响谁。
///
/// 用"最大 id + 1"而不是"条数 + 1"：删掉中间一条之后，
/// 后者会算出一个已经存在的编号。
fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

#[cfg(test)]
// 测试名用中文，失败信息一眼能看懂是哪条规则破了。
// 中文名里夹着 None / JSON 这类大写 ASCII 会触发 non_snake_case，
// 按模块限定关掉——allow 要贴在最小范围上并写明理由。
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn 空列表从1开始编号() {
        assert_eq!(next_id(&[]), 1);
    }

    #[test]
    fn 编号取最大值加一() {
        let tasks = vec![Task::new(1, "a"), Task::new(2, "b"), Task::new(5, "c")];
        assert_eq!(next_id(&tasks), 6);
    }

    #[test]
    fn 删掉中间一条后编号不重复() {
        let mut tasks = vec![Task::new(1, "a"), Task::new(2, "b"), Task::new(3, "c")];
        remove(&mut tasks, 2);
        // 条数是 2，但新编号必须是 4，不能是 3
        assert_eq!(next_id(&tasks), 4);
    }

    #[test]
    fn 新加的任务未完成() {
        let mut tasks = Vec::new();
        let id = add(&mut tasks, "学习 Rust");
        assert_eq!(id, 1);
        assert_eq!(tasks.len(), 1);
        assert!(!tasks[0].done);
    }

    #[test]
    fn 完成不存在的编号返回None() {
        let mut tasks = vec![Task::new(1, "a")];
        assert!(finish(&mut tasks, 99).is_none());
        // 原数据不受影响
        assert!(!tasks[0].done);
    }

    #[test]
    fn 存盘再读回来数据一致() {
        let path = std::env::temp_dir().join("rust-starter-test-roundtrip.json");
        let mut tasks = Vec::new();
        add(&mut tasks, "第一件事");
        add(&mut tasks, "第二件事");
        finish(&mut tasks, 1);

        save(&path, &tasks).expect("存盘应该成功");
        let loaded = load(&path).expect("读取应该成功");

        assert_eq!(loaded.len(), 2);
        assert!(loaded[0].done);
        assert!(!loaded[1].done);
        assert_eq!(loaded[1].title, "第二件事");

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn 文件不存在读出空清单而不报错() {
        let path = std::env::temp_dir().join("rust-starter-test-absent.json");
        let _ = fs::remove_file(&path);
        let loaded = load(&path).expect("文件不存在不该算错误");
        assert!(loaded.is_empty());
    }
}
