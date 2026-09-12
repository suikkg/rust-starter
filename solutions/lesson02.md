# 第 02 课答案要点

代码：`steps/lesson02_main.rs`

**关键点**

1. `let` 默认不可变。这不是限制，是**信息**：读代码的人看到没有 `mut`，就知道这个值不会变。
2. `let mut done = false;` 里的 `mut` 是给变量的，不是给类型的。
3. 报错 `cannot assign twice to immutable variable` 就是漏了 `mut`。

**容易踩的坑**

```rust
let title: String = "学习 Rust";   // ❌ 类型不对
let title: String = "学习 Rust".to_string();   // ✅
let title = "学习 Rust";                        // ✅ 这是 &str，也够用
```

双引号字面量的类型是 `&str`，不是 `String`。第 06 课讲清楚为什么要分两种。
