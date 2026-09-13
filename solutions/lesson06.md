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

顺序是这样的：**能借就借，借不动就 clone，clone 不了再想别的。**

不要因为「听说 clone 不好」就卡在借用错误上改半小时。
你现在处理的是几十条待办，复制一个字符串的代价约等于 0；
而且 `clone` 写在那里是**可见的**，将来一搜就能找到、回头再优化。

真正该停下来想的只有三种情况：在循环里 clone 很大的东西、
clone 完只是读一读就扔、以及**你 clone 纯粹是因为没读懂那条报错**。
最后这种要先把报错读完——有时正确的修法是换个顺序，不是复制。

**报错对照**

| 报错 | 怎么修 |
|---|---|
| `borrow of moved value` | 传参加 `&` |
| `cannot borrow as mutable` | 变量声明加 `mut` |
| `expected &str, found String` | 传 `&s` |
| `cannot borrow as mutable more than once` | 别同时借两次，拆开做 |
