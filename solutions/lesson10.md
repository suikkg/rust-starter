# 第 10 课答案要点

代码：`steps/lesson10_main.rs`

**关键点**

1. 两个 derive 就够了：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task { ... }
```

2. 来回转换各一行：

```rust
serde_json::to_string_pretty(&tasks)?    // 结构体 → JSON
serde_json::from_str(&text)?             // JSON → 结构体
```

3. 错误类型用 `Box<dyn std::error::Error>`。文件错误和 JSON 错误是两种，`?` 需要它们能转成同一种，这个写法最省事。

**最容易漏的一点**

```rust
if !path.exists() {
    return Ok(Vec::new());
}
```

**文件不存在不是错误** —— 第一次运行本来就没有。想不到这一点，程序第一次跑就会报错。

答案里还多了一层：

```rust
if text.trim().is_empty() {
    return Ok(Vec::new());
}
```

空文件也要当成空清单。这类"正常的空情况"要主动想到，测试也该覆盖它。

**读写顺序**

每个命令都是：开头 `load` → 改内存里的数据 → 结尾 `save`。只有真正改了数据的命令（add/done/rm）才需要 `save`，`list` 不用。
