# 第 10 课：存文件与 JSON

**目标**：关掉程序再打开，任务还在。

## 1. 先跑实验

```bash
cargo run --example lesson10_json
```

## 2. 两个新依赖

`Cargo.toml` 里已经写好了：

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

- `serde` 定义"怎么把结构体变成数据"的规则
- `serde_json` 是其中的 JSON 实现

## 3. 让 struct 能存能读

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}
```

加两个 derive 就够了。剩下的：

```rust
let text = serde_json::to_string_pretty(&tasks)?;   // 结构体 → JSON 文本
fs::write(path, text)?;

let text = fs::read_to_string(path)?;
let tasks: Vec<Task> = serde_json::from_str(&text)?;  // JSON 文本 → 结构体
```

## 4. 错误类型

文件操作和 JSON 解析是两种不同的错误，`?` 要求它们能转成同一种。入门阶段最省事的写法：

```rust
fn load(path: &Path) -> Result<Vec<Task>, Box<dyn std::error::Error>> { }
```

`Box<dyn Error>` 的意思是"任何一种错误都行"。正式项目会定义自己的错误类型，现在够用。

## 5. 一个重要判断

**文件不存在不算错误** —— 第一次运行本来就没有：

```rust
if !path.exists() {
    return Ok(Vec::new());
}
```

想不到这一点的话，你的程序第一次跑就会报错。这类"正常的空情况"要主动想到。

## 6. 动手任务

改 `src/main.rs`：

1. 给 `Task` 加上 `Serialize, Deserialize`
2. 写 `fn load(path) -> Result<Vec<Task>, Box<dyn Error>>` 和 `fn save(path, &[Task]) -> Result<(), Box<dyn Error>>`
3. `run` 开头先 `load`，改完数据后 `save`
4. 文件固定用项目目录下的 `todos.json`

## 7. 验收

```bash
cargo run -- add "第一件事"
cargo run -- add "第二件事"
cargo run -- done 1
cargo run -- list          # ← 关键：新进程，数据还在
cat todos.json
```

- [ ] 三次独立运行之后，`list` 能看到之前加的两条，第一条是完成状态
- [ ] `todos.json` 内容可读
- [ ] 删掉 `todos.json` 再 `cargo run -- list`，程序输出空清单而**不报错**
- [ ] 手动把 `todos.json` 改坏（删掉一个引号），运行后给出可读的错误提示

## 8. 常见错误

```
error[E0277]: `?` couldn't convert the error to `String`
```
→ 返回类型写成了 `Result<_, String>`，改成 `Box<dyn std::error::Error>`。

```
Error: missing field `done` at line 5 column 3
```
→ JSON 和结构体字段对不上。这正是 serde 在帮你把问题指出来。

参考答案：`solutions/lesson10.md`

---

## 练习：让测试替你验收

```bash
cargo test --example ex10
```

`exercises/ex10_*.rs` 里有 5 道小题，每道题的函数体都是 `todo!()`，你把它换成实现。

- **一次只做一道**，做完就跑一次。红变绿的那一下就是你学会了
- 失败信息会指到具体哪一行、期望什么、实际什么
- 全绿了这一课才算过 —— 上面的验收清单靠自觉，测试不会放水

答案：`solutions/exercises/ex10.rs`（先自己写 15 分钟）。想看答案跑起来什么样：
`cargo test --example ans10`。
