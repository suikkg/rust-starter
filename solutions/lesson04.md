# 第 04 课答案要点

代码：`steps/lesson04_main.rs`

**关键点**

1. 参数必须写类型，Rust 不猜。
2. `-> String` 表示返回字符串；最后一行不带分号就是返回值。
3. `format!` 和 `println!` 用法一样，区别是它把结果交出来而不是打印。

**为什么参数写 `&str` 不写 `String`**

```rust
fn show_task(id: u32, title: &str, done: bool)
```

`&str` 是"借来看看"，`String` 是"交给我"。只读的字符串参数写 `&str`，调用方之后还能继续用自己的字符串。第 06 课会看到写成 `String` 会出什么事。

**最常见的错**

```
expected `String`, found `()`
```
→ 函数最后一行多了个分号，等于什么都没返回。
