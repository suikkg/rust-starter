# 第 07 课答案要点

代码：`steps/lesson07_main.rs`

**关键点**

1. struct 把散的字段捆成整体，消灭了"两个列表长度对不上"这类错位。
2. `impl` 块里两种函数：

```rust
fn new(id: u32, title: &str) -> Task   // 没有 self → Task::new(...)
fn line(&self) -> String               // &self    → task.line()，只读
fn finish(&mut self)                   // &mut self → 要改自己
```

`&self` / `&mut self` 就是第 06 课的借用规则用在自己身上。

3. `#[derive(Debug, Clone)]` 是让编译器自动实现。`Debug` 给 `{:?}` 用，调试时极其常用。

**为什么 id 要单独存**

不用下标当编号，是因为删除之后下标会变：删掉第 1 条，原来的第 2 条就变成下标 0 了。`id` 存在结构体里就不受影响。

**报错**

```
error[E0596]: cannot borrow `tasks[_]` as mutable
```
→ 要调用 `&mut self` 的方法，那个变量本身必须是 `let mut`。
