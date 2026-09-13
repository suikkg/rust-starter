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
fn find_mut(tasks: &mut [Task], id: u32) -> Option<&mut Task> {
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
opt.map(|t| t.title.clone()) // 有值就变换一下，还是 Option
if let Some(t) = opt { }     // 只关心有值的情况
```

**不要用 `.unwrap()`。** 它的意思是「我保证有，没有就让程序崩掉」——
在写给别人用的程序里，这句保证几乎总会被打脸。
`unwrap_or` / `match` / `if let` 才是正路。

### `Option` 上最常用的几个

```rust
let found: Option<&Task> = tasks.iter().find(|t| t.id == 3);

found.is_some()                       // 有没有            → bool
found.map(|t| t.title.clone())        // 有就变换一下       → Option<String>
found.map_or("（没有）".to_string(),   // 有就变换、没有给默认 → String
             |t| t.title.clone())
found.unwrap_or(&fallback)            // 没有就用这个       → &Task
found.and_then(|t| t.parent())        // 变换出来还是 Option → 摊平一层
```

`map` 和 `map_or` 差一个字，结果差一层：

| | 返回 |
|---|---|
| `opt.map(f)` | `Option<新类型>` —— **还是 Option** |
| `opt.map_or(默认, f)` | **新类型** —— 已经把 None 处理掉了 |

想要「有就这样、没有就那样」并且**直接拿到结果**，用 `map_or`。
练习第 4 题考的就是这个。

### 把字符串变成数字：`parse`

用户从命令行敲进来的永远是字符串。转数字可能失败，所以 `parse` 返回
`Result`（第 09 课细讲）：

```rust
let n: Result<u32, _> = "42".parse();     // Ok(42)
let n: Result<u32, _> = "四二".parse();    // Err(...)
```

`Result` 和 `Option` 是一对表兄弟：都表示「可能没有」，
区别是 `Result` 的失败分支**带着原因**，`Option` 的 `None` 什么都不带。

只关心成不成、不关心为什么，用 `.ok()` 把 `Result` 降级成 `Option`：

```rust
let id: Option<u32> = "42".parse().ok();       // Some(42)
let id: Option<u32> = "四二".parse().ok();      // None
```

写 `parse` 时常常要说清楚转成什么类型，两种写法任选：

```rust
let n: u32 = "42".parse().unwrap_or(0);
let n = "42".parse::<u32>().unwrap_or(0);
```

**`.unwrap()` 会在 `None` 时直接崩溃，练习之外不要用。**

## 5. 动手任务

改 `src/main.rs`：

1. 定义 `enum Command { Add(String), Done(u32), List }`
2. 在 `main` 里放一个 `Vec<Command>`，按顺序处理（下一课才接真正的命令行）
3. 写 `fn find_mut(tasks: &mut [Task], id: u32) -> Option<&mut Task>`
   （第 06 课那条：只改内容不增删条数，收 `&mut [T]` 就够，比 `&mut Vec<T>` 通用）
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

```
error[E0282]: type annotations needed
```
→ `parse()` 不知道要转成什么类型。写成 `parse::<u32>()`，或者给变量标类型。

参考答案：`solutions/lesson08.md`

---

## 练习：让测试替你验收

```bash
cargo test --example ex08
```

`exercises/ex08_*.rs` 里有 4 道小题（5 个测试），每道题的函数体都是 `todo!()`，你把它换成实现。

- **一次只做一道**，做完就跑一次。红变绿的那一下就是你学会了
- 失败信息会指到具体哪一行、期望什么、实际什么
- 全绿了这一课才算过 —— 上面的验收清单靠自觉，测试不会放水

答案：`solutions/exercises/ex08.rs`（先自己写 15 分钟）。想看答案跑起来什么样：
`cargo test --example ans08`。
