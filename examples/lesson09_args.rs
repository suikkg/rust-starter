//! 第 09 课：命令行参数、Result 与 ?
//!
//!     cargo run --example lesson09_args -- add "学习 Rust"
//!     cargo run --example lesson09_args -- done 1
//!     cargo run --example lesson09_args -- done abc
//!     cargo run --example lesson09_args
//!
//! -- 后面的东西才是给程序的参数。
//! Result 表示"可能失败"：Ok(成功值) 或 Err(错误)。

use std::env;

fn main() {
    // args() 的第 0 个是程序自己的路径，所以 skip(1)
    let args: Vec<String> = env::args().skip(1).collect();

    // 交给一个返回 Result 的函数去做，main 只负责显示结果。
    // 这样"出错怎么显示"只写一遍。
    match run(&args) {
        Ok(msg) => println!("{msg}"),
        Err(msg) => {
            eprintln!("错误：{msg}");
            std::process::exit(1);
        }
    }
}

fn run(args: &[String]) -> Result<String, String> {
    // as_deref() 把 Option<&String> 变成 Option<&str>，好比较
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("");

    match cmd {
        "add" => {
            let title = args
                .get(1)
                .ok_or_else(|| "add 后面要跟任务名，例如 add \"学习 Rust\"".to_string())?;
            Ok(format!("已添加：{title}"))
        }
        "done" => {
            let raw = args.get(1).ok_or("done 后面要跟编号，例如 done 1")?;
            // parse 返回 Result。失败时 map_err 换成我们自己的话，
            // 末尾的 ? 意思是"出错就立刻返回这个错误"。
            let id: u32 = raw
                .parse()
                .map_err(|_| format!("编号必须是数字，你给的是 {raw}"))?;
            Ok(format!("已完成第 {id} 条"))
        }
        "" => Err("请给一个命令：add / done".to_string()),
        other => Err(format!("不认识的命令：{other}")),
    }
}

// 【动手】跑一次 `cargo run --example lesson09_args -- done abc`，
// 确认程序是"报错退出"而不是崩溃（不应该出现 panicked at 字样）。
