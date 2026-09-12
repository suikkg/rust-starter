//! 第 10 课练习：文件与 JSON
//!
//!     cargo test --example ex10
//!
//! 这一课的重点不是「怎么调 serde」，是**哪些情况不算错误**：
//! 文件不存在、文件是空的 —— 第一次运行本来就这样，不该报错给用户看。
//!
//! 答案：`solutions/exercises/ex10.rs`
#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

/// 【1】序列化成**带缩进**的 JSON 文本。
///
/// 提示：`serde_json::to_string_pretty`。存盘用带缩进的，人打开文件能看懂。
fn to_json(tasks: &[Task]) -> Result<String, serde_json::Error> {
    todo!()
}

/// 【2】从 JSON 文本读回来。
fn from_json(text: &str) -> Result<Vec<Task>, serde_json::Error> {
    todo!()
}

/// 【3】从文件读；**文件不存在或内容为空，返回空清单，不算错误**。
///
/// 三种情况分清楚：
/// - 文件不存在 → `Ok(vec![])`
/// - 文件是空的或只有空白 → `Ok(vec![])`
/// - 文件有内容但不是合法 JSON → `Err(...)`，**这个才是真错误**
///
/// 提示：`path.exists()`、`std::fs::read_to_string`、`text.trim().is_empty()`。
/// 错误统一转成 `String`：`.map_err(|e| e.to_string())`。
fn load_or_empty(path: &Path) -> Result<Vec<Task>, String> {
    todo!()
}

/// 【4】存盘。父目录不存在要先建出来。
///
/// 提示：`path.parent()`、`std::fs::create_dir_all`。
fn save(path: &Path, tasks: &[Task]) -> Result<(), String> {
    todo!()
}

fn main() {
    println!("这一课的练习请运行：cargo test --example ex10");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    fn 样例() -> Vec<Task> {
        vec![
            Task {
                id: 1,
                title: "买菜".into(),
                done: true,
            },
            Task {
                id: 2,
                title: "写周报".into(),
                done: false,
            },
        ]
    }

    /// 每个测试用自己的临时文件，互不打架。
    fn 临时路径(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("rust-starter-ex10-{name}.json"))
    }

    #[test]
    fn 存成JSON再读回来数据一致() {
        let text = to_json(&样例()).expect("序列化应该成功");
        assert!(text.contains("买菜"));
        assert!(text.contains('\n'), "用 to_string_pretty，存出来要能给人看");

        let back = from_json(&text).expect("反序列化应该成功");
        assert_eq!(back, 样例());
    }

    #[test]
    fn 文件不存在读出空清单() {
        let path = 临时路径("absent");
        let _ = std::fs::remove_file(&path);
        let tasks = load_or_empty(&path).expect("文件不存在不该算错误");
        assert!(tasks.is_empty());
    }

    #[test]
    fn 空文件也读出空清单() {
        let path = 临时路径("empty");
        std::fs::write(&path, "   \n").unwrap();
        let tasks = load_or_empty(&path).expect("空文件不该算错误");
        assert!(tasks.is_empty());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn 内容坏了才算错误() {
        let path = 临时路径("broken");
        std::fs::write(&path, "这不是 json").unwrap();
        assert!(
            load_or_empty(&path).is_err(),
            "有内容但读不出来，这才是真错误"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn 存盘会自己建目录() {
        let dir = std::env::temp_dir().join("rust-starter-ex10-sub/深一层");
        let path = dir.join("tasks.json");
        let _ = std::fs::remove_dir_all(std::env::temp_dir().join("rust-starter-ex10-sub"));

        save(&path, &样例()).expect("父目录不存在时应该自己建出来");
        let back = load_or_empty(&path).expect("存完应该读得回来");
        assert_eq!(back, 样例());

        let _ = std::fs::remove_dir_all(std::env::temp_dir().join("rust-starter-ex10-sub"));
    }
}
