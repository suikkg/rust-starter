# 第 08 课：enum、match 与 Option

**目标**：用类型表达"几种情况之一"，并且安全地处理"可能找不到"。

## 1. 先跑实验

```bash
cargo run --example lesson08_enum_option
```

## 2. enum

```rust
enum Command {
    Add(String),   // 带数据
    Done(u32),
    List,          // 不带数据
}
```

比用字符串到处比较强得多：`"lst"` 拼错了要到运行时才发现，`Command::Lst` 当场编译失败。

## 3. match 必须穷尽

```rust
match cmd {
    Command::Add(title) => { }
    Command::Done(id)   => { }
    Command::List       => { }
}
```

少写一个分支就编译不过。这是 Rust 的一个大优势：**以后给 enum 加新情况时，编译器会把所有需要更新的地方都指给你看**，不会有漏改。

## 4. Option —— Rust 没有 null

```rust
enum Option<T> {
    Some(T),   // 有值
    None,      // 没有
}
```

"按编号找任务"可能找不到，所以返回 `Option`：

```rust
fn find_mut(tasks: &mut Vec<Task>, id: u32) -> Option<&mut Task> {
    tasks.iter_mut().find(|t| t.id == id)
}

match find_mut(&mut tasks, id) {
    Some(task) => task.done = true,
    None => println!("找不到编号 {id}"),
}
```

常用的简写：

```rust
opt.unwrap_or("默认值")       // 没有就用默认值
opt.is_some()                // 有没有
if let Some(t) = opt { }     // 只关心有值的情况
```

**`.unwrap()` 会在 `None` 时直接崩溃，练习之外不要用。**

## 5. 动手任务

改 `src/main.rs`：

1. 定义 `enum Command { Add(String), Done(u32), List }`
2. 在 `main` 里放一个 `Vec<Command>`，按顺序处理（下一课才接真正的命令行）
3. 写 `fn find_mut(tasks: &mut Vec<Task>, id: u32) -> Option<&mut Task>`
4. `Done(99)` 这种找不到的情况要打印"找不到编号 99"，**不能崩溃**

## 6. 验收

```bash
cargo run
```

- [ ] 处理了一个不存在的编号，程序正常打印提示并继续跑
- [ ] 给 `Command` 加第四个分支后 `cargo check`，见过编译器提示 `non-exhaustive patterns`
- [ ] 能说清 `Option` 和 "null" 的区别：Option 逼着你处理没有值的情况

## 7. 常见错误

```
error[E0004]: non-exhaustive patterns: `Command::Remove(_)` not covered
```
→ `match` 漏了一个分支。这是好事，按提示补上。

参考答案：`solutions/lesson08.md`
