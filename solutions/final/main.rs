//! 程序入口。只做三件事：读参数、调 run、显示结果。
//!
//! 注意包名：Cargo.toml 里是 rust-starter，代码里要写 rust_starter（中划线变下划线）。

use rust_starter::run;
use std::env;

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
