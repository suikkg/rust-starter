//! 第 08 课练习的参考答案。
//!
//!     cargo test --example ans08
#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
enum Command {
    Add(String),
    Done(u32),
    List,
    Unknown(String),
}

fn parse(word: &str, rest: &str) -> Command {
    match word {
        "add" => Command::Add(rest.to_string()),
        "done" => match rest.parse::<u32>() {
            Ok(id) => Command::Done(id),
            // 编号不是数字：认不出来，把命令原话带上交给上层报错
            Err(_) => Command::Unknown(word.to_string()),
        },
        "list" => Command::List,
        other => Command::Unknown(other.to_string()),
    }
}

fn describe(cmd: &Command) -> String {
    // 四个分支写全，没有 _ => 兜底。
    // 将来给 Command 加一个 Remove(u32)，编译器会在这里报
    // non-exhaustive patterns，逼着你把新命令的文案也补上——
    // 有 _ => 的话，新命令会静默变成"其它"，没人会发现。
    match cmd {
        Command::Add(title) => format!("加一条：{title}"),
        Command::Done(id) => format!("完成第 {id} 条"),
        Command::List => "列出全部".to_string(),
        Command::Unknown(word) => format!("不认识的命令：{word}"),
    }
}

fn find_title(tasks: &[(u32, String)], id: u32) -> Option<&str> {
    for (tid, title) in tasks {
        if *tid == id {
            return Some(title);
        }
    }
    None
    // 迭代器写法：
    // tasks.iter().find(|(tid, _)| *tid == id).map(|(_, t)| t.as_str())
}

fn title_or_default(tasks: &[(u32, String)], id: u32) -> String {
    match find_title(tasks, id) {
        Some(title) => title.to_string(),
        None => "（没有这一条）".to_string(),
    }
    // 等价的短写法：
    // find_title(tasks, id).map_or_else(|| "（没有这一条）".to_string(), |t| t.to_string())
}

fn main() {
    println!("这是第 08 课练习的答案：cargo test --example ans08");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    fn 样例() -> Vec<(u32, String)> {
        vec![(1, "买菜".to_string()), (3, "写周报".to_string())]
    }

    #[test]
    fn 解析命令() {
        assert_eq!(parse("add", "买菜"), Command::Add("买菜".to_string()));
        assert_eq!(parse("done", "3"), Command::Done(3));
        assert_eq!(parse("list", ""), Command::List);
        assert_eq!(parse("飞天", ""), Command::Unknown("飞天".to_string()));
    }

    #[test]
    fn 编号不是数字算不认识() {
        assert_eq!(parse("done", "三"), Command::Unknown("done".to_string()));
        assert_eq!(parse("done", ""), Command::Unknown("done".to_string()));
    }

    #[test]
    fn 描述四种命令() {
        assert_eq!(describe(&Command::Add("买菜".into())), "加一条：买菜");
        assert_eq!(describe(&Command::Done(3)), "完成第 3 条");
        assert_eq!(describe(&Command::List), "列出全部");
        assert_eq!(
            describe(&Command::Unknown("飞天".into())),
            "不认识的命令：飞天"
        );
    }

    #[test]
    fn 找得到和找不到() {
        let tasks = 样例();
        assert_eq!(find_title(&tasks, 1), Some("买菜"));
        assert_eq!(find_title(&tasks, 3), Some("写周报"));
        assert_eq!(find_title(&tasks, 2), None, "编号 2 不存在，要返回 None");
    }

    #[test]
    fn 找不到时给默认值而不是崩溃() {
        let tasks = 样例();
        assert_eq!(title_or_default(&tasks, 1), "买菜");
        assert_eq!(title_or_default(&tasks, 99), "（没有这一条）");
    }
}
