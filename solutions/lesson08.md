# 第 08 课答案要点

代码：`steps/lesson08_main.rs`

**关键点**

1. `enum` 的变体可以带数据：`Add(String)` 带标题，`List` 什么都不带。
2. `match` 必须覆盖所有分支，漏一个就编译不过。

这条规则的真正价值在**以后**：给 `Command` 加一个新分支时，编译器会把所有需要更新的 `match` 都指给你看。这在大项目里能省掉整类 bug。

3. `Option<T>` 只有 `Some(T)` 和 `None`。Rust 没有 null，所以"可能找不到"必须用它表达。

```rust
fn find_mut(tasks: &mut [Task], id: u32) -> Option<&mut Task> {
    tasks.iter_mut().find(|t| t.id == id)
}
```

**不要用 `.unwrap()`**

`.unwrap()` 在 `None` 时直接让程序崩溃。练习之外别用。要取值就用 `match`、`if let`、`unwrap_or(默认值)`。

**为什么 `next_id` 是"最大 id + 1"**

如果写成 `tasks.len() + 1`：加了 3 条，删掉第 2 条，再加一条就会算出 `3` —— 和已有的第 3 条撞号。答案里专门有一条测试钉住这件事（见 `final/store.rs`）。
