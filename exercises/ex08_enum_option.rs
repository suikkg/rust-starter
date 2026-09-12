//! 第 08 课练习：enum 与 Option
//!
//!     cargo test --example ex08
//!
//! 这一课要把「用字符串表示命令」换成「用枚举表示命令」，
//! 再把第 03 课那个 `-1` 换成 `Option`。
//!
//! 答案：`solutions/exercises/ex08.rs`
#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone, PartialEq)]
enum Command {
    Add(String),
    Done(u32),
    List,
    /// 不认识的命令，把原话带上 —— 报错时要能告诉用户他敲了什么。
    Unknown(String),
}

/// 【1】把一行输入解析成命令。
///
/// - `("add", "买菜")` → `Command::Add("买菜")`
/// - `("done", "3")`   → `Command::Done(3)`
/// - `("done", "三")`  → `Command::Unknown("done")`（编号不是数字）
/// - `("list", "")`    → `Command::List`
/// - 其它            → `Command::Unknown(原话)`
///
/// 提示：`rest.parse::<u32>()` 返回 `Result`，`.ok()` 能把它变成 `Option`。
fn parse(word: &str, rest: &str) -> Command {
    todo!()
}

/// 【2】把命令翻译成一句人话。
///
/// - `Add(t)`     → `加一条：买菜`
/// - `Done(id)`   → `完成第 3 条`
/// - `List`       → `列出全部`
/// - `Unknown(w)` → `不认识的命令：飞天`
///
/// 要求 `match` 写全四个分支，**不要用 `_ =>` 兜底** ——
/// 将来加了新命令，编译器要能提醒你这里也得改。
fn describe(cmd: &Command) -> String {
    todo!()
}

/// 【3】按编号找标题；找不到返回 `None`。
///
/// 这就是第 03 课那个 `-1` 的正确写法：`Option` 逼着调用方处理「没找到」。
fn find_title(tasks: &[(u32, String)], id: u32) -> Option<&str> {
    todo!()
}

/// 【4】找到就返回标题，找不到返回 `"（没有这一条）"`。
///
/// 提示：`Option` 上的 `unwrap_or` / `map_or`。**不要用 `.unwrap()`** ——
/// 那个在 None 的时候会崩溃，正是这一课要避免的事。
fn title_or_default(tasks: &[(u32, String)], id: u32) -> String {
    todo!()
}

fn main() {
    println!("这一课的练习请运行：cargo test --example ex08");
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
