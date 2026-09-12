//! 第 10 课练习的参考答案。
//!
//!     cargo test --example ans10
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn to_json(tasks: &[Task]) -> Result<String, serde_json::Error> {
    // pretty 版带缩进和换行。存盘文件是给人看的——出问题时直接打开就能读。
    serde_json::to_string_pretty(tasks)
}

fn from_json(text: &str) -> Result<Vec<Task>, serde_json::Error> {
    // 返回类型已经写明是 Vec<Task>，serde 靠它推断该解析成什么，
    // 所以这里不用写 from_str::<Vec<Task>>(text)。
    serde_json::from_str(text)
}

fn load_or_empty(path: &Path) -> Result<Vec<Task>, String> {
    // 第一次运行本来就没有这个文件。这不是错误，是正常起点。
    // 把它当错误报给用户，用户第一次打开程序就会看到一句吓人的话。
    if !path.exists() {
        return Ok(Vec::new());
    }

    let text =
        std::fs::read_to_string(path).map_err(|e| format!("读取 {} 失败：{e}", path.display()))?;

    // 空文件同理：上次存盘存了个空清单，或者文件刚被创建。
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }

    // 到这一步说明文件有内容却读不出来——这才是真的坏了，必须报错。
    // 悄悄返回空清单是最坏的处理方式：用户的数据还在文件里，
    // 程序却显示"清单是空的"，然后下一次存盘把它覆盖掉。
    from_json(&text).map_err(|e| format!("{} 不是合法的清单文件：{e}", path.display()))
}

fn save(path: &Path, tasks: &[Task]) -> Result<(), String> {
    // 目录不存在就先建。少了这一步，把清单放在 ~/.todo/tasks.json
    // 这种地方时，第一次存盘必然失败。
    if let Some(dir) = path.parent() {
        if !dir.as_os_str().is_empty() {
            std::fs::create_dir_all(dir)
                .map_err(|e| format!("创建目录 {} 失败：{e}", dir.display()))?;
        }
    }

    let text = to_json(tasks).map_err(|e| format!("序列化失败：{e}"))?;
    std::fs::write(path, text).map_err(|e| format!("写入 {} 失败：{e}", path.display()))
}

fn main() {
    println!("这是第 10 课练习的答案：cargo test --example ans10");
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

    fn 临时路径(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("rust-starter-ans10-{name}.json"))
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
        let root = std::env::temp_dir().join("rust-starter-ans10-sub");
        let path = root.join("深一层/tasks.json");
        let _ = std::fs::remove_dir_all(&root);

        save(&path, &样例()).expect("父目录不存在时应该自己建出来");
        let back = load_or_empty(&path).expect("存完应该读得回来");
        assert_eq!(back, 样例());

        let _ = std::fs::remove_dir_all(&root);
    }
}
