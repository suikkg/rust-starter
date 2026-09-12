//! 第 09 课练习：命令行参数与 Result
//!
//!     cargo test --example ex09
//!
//! 这一课练的是**给用户看的错误**。每道题的测试都在检查错误信息里
//! 有没有把出错的原文带上 —— 「编号不对」和「编号 `三` 不是数字」
//! 差别是用户要不要靠猜。
//!
//! 答案：`solutions/exercises/ex09.rs`
#![allow(dead_code, unused_variables)]

/// 【1】把参数解析成任务编号。
///
/// 失败时的错误信息**必须带上原文**，例如：
/// `编号要是数字，收到的是「三」`
fn parse_id(arg: &str) -> Result<u32, String> {
    todo!()
}

/// 【2】取第一个参数当命令；一个参数都没有时报错。
///
/// 没有参数时返回 `Err("请给一个命令，例如：add 买菜".to_string())`。
fn pick_command(args: &[String]) -> Result<&str, String> {
    todo!()
}

/// 【3】解析 `done 3` 这种两段式命令，返回 `(命令, 编号)`。
///
/// 要求用 **`?`** 把上面两个函数串起来，不要重写一遍解析逻辑。
/// 参数少于两个时报错：`done 后面要跟编号，例如：done 1`
fn parse_two(args: &[String]) -> Result<(String, u32), String> {
    todo!()
}

/// 【4】给脚本用的退出码：成功 0，失败 1。
///
/// 这是真实工具的惯例 —— 脚本靠退出码判断该不该继续往下跑。
fn exit_code(result: &Result<(), String>) -> i32 {
    todo!()
}

fn main() {
    println!("这一课的练习请运行：cargo test --example ex09");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    fn 参数(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn 解析编号() {
        assert_eq!(parse_id("3"), Ok(3));
        assert_eq!(parse_id("0"), Ok(0));
        assert!(parse_id("三").is_err());
        assert!(parse_id("").is_err());
        assert!(parse_id("-1").is_err(), "u32 装不下负数");
    }

    #[test]
    fn 报错要带上原文() {
        let err = parse_id("三").unwrap_err();
        assert!(
            err.contains('三'),
            "错误信息里要有用户敲的原文，当前是：{err}"
        );
    }

    #[test]
    fn 没有参数时报错() {
        assert_eq!(pick_command(&参数(&["add"])), Ok("add"));
        assert!(pick_command(&[]).is_err());
        let err = pick_command(&[]).unwrap_err();
        assert!(err.contains("add"), "错误信息里要给个例子，当前是：{err}");
    }

    #[test]
    fn 两段式命令() {
        assert_eq!(
            parse_two(&参数(&["done", "3"])),
            Ok(("done".to_string(), 3))
        );
        assert!(parse_two(&参数(&["done"])).is_err(), "少了编号");
        assert!(parse_two(&参数(&["done", "三"])).is_err(), "编号不是数字");
        assert!(parse_two(&[]).is_err());
    }

    #[test]
    fn 退出码() {
        assert_eq!(exit_code(&Ok(())), 0);
        assert_eq!(exit_code(&Err("出错了".to_string())), 1);
    }
}
