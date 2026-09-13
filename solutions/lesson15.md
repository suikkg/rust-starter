# 第 15 课答案要点

**`Display`**

```rust
use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mark = if self.done { "✓" } else { " " };
        write!(f, "[{mark}] {}  {}", self.id, self.title)
    }
}
```

然后 `view.rs` 里那一堆手拼就没了：

```rust
// 前
println!("[{}] {}  {}", if t.done { "✓" } else { " " }, t.id, t.title);

// 后
println!("{t}");
```

**好处不只是短**：显示格式从三处（列表、单条、错误提示）收敛到一处。
哪天要改成 `[x]`，改一个地方。

**`Priority`**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    #[default]          // 配合 #[derive(Default)]
    Low,
}
```

排序：

```rust
self.tasks.sort_by_key(|t| t.priority);
```

`sort_by_key` 是**稳定排序** —— 优先级相同的两条，原来谁在前面还是谁在前面。
这正是你要的（不然每次 `list` 顺序都可能变）。

**`AppError`（第 3 步）**

```rust
#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    NotFound(u32),
    BadArgs(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "读写文件失败：{e}"),
            AppError::Json(e) => write!(f, "存档格式不对：{e}"),
            AppError::NotFound(id) => write!(f, "没有第 {id} 条"),
            AppError::BadArgs(s) => write!(f, "参数不对：{s}"),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}
impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self { AppError::Json(e) }
}
```

写完这两个 `From`，原来那些 `?` 一个字都不用改。

**为什么比 `Result<_, String>` 好**：调用方能 `match` 它。
`NotFound` 该退出码 1，`Io` 可能要重试 —— 字符串分不出这个区别。

## 练习里的坑

**第 1 题**：`write!` 后面加了分号 → `expected Result, found ()`。
Rust 的「最后一个表达式就是返回值」在这里又出现了（第 04 课）。

**第 2 题**：只加 `Ord` 会连环报错，因为 `Ord` 要 `Eq`、`Eq` 要 `PartialEq`、
`Ord` 还要 `PartialOrd`。四个一起加。

**照编译器的提示补就行** —— 它会一个一个告诉你少了哪个。

**第 3 题**：`?` 报 `couldn't convert the error`，说明 `From` 没写对
（比如泛型参数写反了：是 `From<下层错误> for 本层错误`）。

**第 4 题**：`items.iter().map(|x| x.to_string())` 里的 `to_string()`
是 `Display` 白送的。要是你的 `T` 约束写成了 `T: Debug`，
那就只能 `format!("{x:?}")`，出来的是调试格式不是给人看的格式。
