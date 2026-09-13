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

## 5. 存盘前要先建目录

`fs::write` **不会**替你把目录建出来。路径里有一层不存在，它就直接失败：

```rust
use std::fs;

if let Some(dir) = path.parent() {
    fs::create_dir_all(dir)?;      // 已经存在也不报错，放心调
}
fs::write(path, text)?;
```

`create_dir_all` 会把缺的每一层都建出来，而且**目录已经在了也算成功** ——
所以不用先判断再建。

## 6. 三种「读不出来」，处理方式完全不同

这一节是这一课真正要学的东西。

| 情况 | 该怎么办 | 为什么 |
|---|---|---|
| 文件不存在 | 返回空清单，**不算错误** | 第一次运行本来就没有 |
| 文件是空的 | 返回空清单，**不算错误** | 上次存了个空清单，或文件刚建好 |
| 文件有内容但**读不出来** | **必须报错** | 见下 |

```rust
fn load(path: &Path) -> Result<Vec<Task>, String> {
    if !path.exists() {
        return Ok(Vec::new());          // 正常起点，不是错误
    }
    let text = fs::read_to_string(path).map_err(|e| format!("读取失败：{e}"))?;
    if text.trim().is_empty() {
        return Ok(Vec::new());          // 空文件同理
    }
    serde_json::from_str(&text)         // 到这一步读不出来，那就是真的坏了
        .map_err(|e| format!("{} 不是合法的清单文件：{e}", path.display()))
}
```

### 第三种为什么必须报错

「读不出来就当空清单」看起来很宽容，实际上是这一课最贵的一个 bug：

```text
1. 文件里有你三个月的待办，但某次存盘被打断，JSON 缺了个括号
2. 程序读不出来，悄悄当成空清单
3. 屏幕上显示「还没有任何任务」
4. 你加了一条新的 —— 存盘
5. 三个月的数据被这一条覆盖掉了
```

**「读不出来」和「本来就没有」在程序里必须是两件事。**
悄悄把前者当成后者，代价是用户的数据。

> 这条规矩在隔壁 `cpe-mini` 里会反复出现：
> `None`（没测到）不等于 `Some(0.0)`（测到是 0）、
> 「取消了没跑」不等于「报告里没有这一行」。**都是同一件事。**

## 7. 给已经存在的数据加字段

你的 `todos.json` 已经存了十条任务。现在要给 `Task` 加一个 `tag` 字段 ——
旧文件里没有这个字段，直接读会失败：

```text
missing field `tag` at line 3 column 5
```

办法是告诉 serde「没有就用默认值」：

```rust
#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    done: bool,
    #[serde(default)]              // 旧文件没有 tag → 用 String::default()（空串）
    tag: String,
}
```

想要别的默认值就自己指定一个函数：

```rust
    #[serde(default = "default_tag")]
    tag: String,
}

fn default_tag() -> String {
    "未分类".to_string()
}
```

> **改数据结构时，先想清楚已经存在的数据怎么办。**
> 这在真实项目里叫「兼容性」，是 `cpe-mini` 第 08 课和第 17 课的主题。
> 现在你的"历史数据"只有一个 `todos.json`，但习惯要从这里养起。

## 8. 动手任务

改 `src/main.rs`：

1. 给 `Task` 加上 `Serialize, Deserialize`
2. 写 `fn load(path) -> Result<Vec<Task>, Box<dyn Error>>` 和 `fn save(path, &[Task]) -> Result<(), Box<dyn Error>>`
   —— `save` 里记得先 `create_dir_all`，`load` 里区分第 6 节那三种情况
3. `run` 开头先 `load`，改完数据后 `save`
4. 文件固定用项目目录下的 `todos.json`

## 9. 验收

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
- [ ] 手动把 `todos.json` 改坏（删掉一个引号），程序给出**可读的错误提示**，
      而不是显示一个空清单（第 6 节那个会吃掉数据的 bug）
- [ ] 说得出「文件不存在」和「文件读不出来」为什么必须分开处理
- [ ] 知道给已有数据加字段要用 `#[serde(default)]`

## 10. 常见错误

```
error[E0277]: `?` couldn't convert the error to `String`
```
→ 返回类型写成了 `Result<_, String>`，改成 `Box<dyn std::error::Error>`。

```
Error: missing field `done` at line 5 column 3
```
→ JSON 和结构体字段对不上。这正是 serde 在帮你把问题指出来。
如果是**给已有数据加了新字段**，用 `#[serde(default)]`（第 7 节）。

```
Error: No such file or directory (os error 2)
```
→ 要写入的目录不存在。存盘前先 `create_dir_all`（第 5 节）。

**还有一个不报错的：** 程序说「还没有任何任务」，但 `todos.json` 里明明有数据。
那是坏文件被当成了空清单 —— 第 6 节那个会吃掉数据的 bug。

参考答案：`solutions/lesson10.md`

---

## 练习：让测试替你验收

```bash
cargo test --example ex10
```

`exercises/ex10_*.rs` 里有 4 道小题（5 个测试），每道题的函数体都是 `todo!()`，你把它换成实现。

- **一次只做一道**，做完就跑一次。红变绿的那一下就是你学会了
- 失败信息会指到具体哪一行、期望什么、实际什么
- 全绿了这一课才算过 —— 上面的验收清单靠自觉，测试不会放水

答案：`solutions/exercises/ex10.rs`（先自己写 15 分钟）。想看答案跑起来什么样：
`cargo test --example ans10`。
