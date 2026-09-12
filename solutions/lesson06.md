# 第 06 课答案要点

代码：`steps/lesson06_main.rs`

**这一课卡住是正常的。** 每个学 Rust 的人都在这里慢下来过。

**三句话**

| 写法 | 意思 | 之后调用方还能用吗 |
|---|---|---|
| `f(x)` | 把 x 交出去（移动） | ❌ |
| `f(&x)` | 借给它看 | ✅ |
| `f(&mut x)` | 借给它改 | ✅ |

**参数该写哪种**

```rust
fn render(titles: &[String], flags: &[bool])          // 只读 → &
fn add(titles: &mut Vec<String>, title: &str)         // 要改 → &mut
```

注意 `&[String]` 比 `&Vec<String>` 更通用，是 Rust 的习惯写法（数组和 Vec 都能传进来）。只读参数优先写 `&[T]` 和 `&str`。

**为什么整数不用管这些**

`i32`、`bool`、`f64` 这类固定大小的类型是**复制**不是移动，所以你前几课从没撞上过。一碰 `String` 和 `Vec` 就开始报错了。

**`.clone()` 的正确态度**

它能解决问题，但是真的复制了一份数据。入门期偶尔用一下没关系，但每写一个 `.clone()` 都该先问一句"能不能改成借用"。到处 clone 是新手最容易养成的坏习惯。

**报错对照**

| 报错 | 怎么修 |
|---|---|
| `borrow of moved value` | 传参加 `&` |
| `cannot borrow as mutable` | 变量声明加 `mut` |
| `expected &str, found String` | 传 `&s` |
| `cannot borrow as mutable more than once` | 别同时借两次，拆开做 |
