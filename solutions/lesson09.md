# 第 09 课答案要点

代码：`steps/lesson09_main.rs`

**记住这个骨架** —— 几乎所有 Rust 命令行程序都长这样：

```rust
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(msg) => println!("{msg}"),
        Err(msg) => { eprintln!("错误：{msg}"); std::process::exit(1); }
    }
}

fn run(args: &[String]) -> Result<String, String> { ... }
```

好处：业务逻辑集中在 `run` 里，"出错怎么显示"只写一遍，而且 `run` 以后能被测试直接调用。

**`?` 到底做了什么**

```rust
let id: u32 = raw.parse().map_err(|_| format!("编号必须是数字"))?;
```

等价于：

```rust
let id: u32 = match raw.parse() {
    Ok(v) => v,
    Err(_) => return Err(format!("编号必须是数字")),
};
```

`?` 只能用在返回 `Result`（或 `Option`）的函数里。在 `main` 里用会报错。

**`Option` 和 `Result` 的分工**

- `Option`：有 / 没有。找不到任务 → `None`
- `Result`：成了 / 失败了并且有原因。编号不是数字 → `Err("编号必须是数字...")`

`ok_or(...)` 就是把 `Option` 转成 `Result`，顺便补上失败理由。

**这一课的验收重点是"不崩溃"**

任何输入都不该出现 `panicked at`。用户打错字是常态，不是异常。
