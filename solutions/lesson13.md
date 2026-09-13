# 第 13 课答案要点

**数已完成条数**

```rust
// 前
let mut done = 0;
for t in &self.tasks {
    if t.done { done += 1; }
}

// 后
let done = self.tasks.iter().filter(|t| t.done).count();
```

**按编号找任务**

```rust
// 前
for t in &self.tasks {
    if t.id == id { return Some(t); }
}
None

// 后
self.tasks.iter().find(|t| t.id == id)
```

`find` 直接返回 `Option<&Task>` —— 和你手写的那五行是同一个类型，
但少了一次「忘了写最后那个 `None`」的机会。

**改所有元素**

```rust
// 这种就别硬套迭代器了
for t in &mut self.tasks {
    t.done = true;
}

// 硬套出来是这样，不见得更清楚
self.tasks.iter_mut().for_each(|t| t.done = true);
```

## 三个容易忘的

**1. `filter` 的闭包拿到的是 `&&T`**

```rust
tasks.iter().filter(|t| !t.done)
//            ^ 这个 t 是 &&Task
```

`iter()` 给了 `&Task`，`filter` 又借了一次。`t.done` 照样能写 ——
Rust 会自动解引用。但要是写 `*t` 或者 `t.clone()` 就会发现类型不太对，
这时候 `**t` 才是那个 `Task`。

**2. `sum()` 和 `collect()` 都要你说清楚类型**

```rust
let total: usize = tasks.iter().map(|t| t.title.chars().count()).sum();
//         ^^^^^ 少了这个就是 E0282
```

**3. 想省掉 clone 就改签名**

```rust
fn titles(tasks: &[Task]) -> Vec<String>   // 要 clone
fn titles(tasks: &[Task]) -> Vec<&str>     // 不用 clone，但调用方要负责生命周期
```

真实项目里两种都常见。**返回 `&str` 的那种更省，但调用方不能在
原列表没了之后还拿着它** —— 编译器会盯着这件事。

## 练习里的坑

**第 2 题**：先写 `.map(|t| t.title)` 试试，报错是

```
error[E0507]: cannot move out of `t.title` which is behind a shared reference
```

读完它，再加 `.clone()`。**这个报错在真实项目里会反复见到**，
值得撞一次。

**第 4 题**：`max_by_key(|t| t.title.len())` 是错的 ——
`len()` 是字节数。`"abcdefgh"` 是 8 字节，`"搞懂所有权和借用"` 是 24 字节，
用 `len()` 的话后者"更长"纯属巧合（换成 `"abcdefghijklmnopqrstuvwxyz"` 就反了）。
要 `chars().count()`。

**第 5 题**：`impl Fn(&str) -> String` 是参数的类型。
调用它就是 `f(&t.title)`，和调普通函数一样。
