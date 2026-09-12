# 第 07 课：struct —— 一条任务是一个整体

**目标**：不再用两个平行的列表凑数据，改成一个 `Task` 结构体。

## 1. 先跑实验

```bash
cargo run --example lesson07_struct
```

## 2. 为什么要 struct

上一课的写法有个隐患：

```rust
let titles = vec!["A", "B", "C"];
let flags  = vec![true, false];        // 少了一个！
```

两个列表长度不一致，程序就错位了，编译器也帮不了你。把它们捆成一个整体就不会：

```rust
#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}
```

## 3. impl —— 属于 Task 的函数

```rust
impl Task {
    fn new(id: u32, title: &str) -> Task {      // 没有 self：用 Task::new(...) 调
        Task { id, title: title.to_string(), done: false }
    }

    fn line(&self) -> String {                  // &self：只读，用 task.line() 调
        let mark = if self.done { "✓" } else { " " };
        format!("[{}] {}  {}", mark, self.id, self.title)
    }

    fn finish(&mut self) {                      // &mut self：要改自己
        self.done = true;
    }
}
```

`&self` / `&mut self` 就是上一课的借用规则用在自己身上。

## 4. derive 是什么

```rust
#[derive(Debug, Clone)]
```

意思是"编译器，请帮我自动实现这几样"：

- `Debug` → 可以用 `{:?}` 打印，调试时极其常用
- `Clone` → 可以 `.clone()` 复制一份

## 5. 动手任务

改 `src/main.rs`：

1. 定义 `struct Task { id, title, done }`
2. 写 `Task::new(id, title)` 和 `fn line(&self) -> String`
3. 把两个平行 `Vec` 换成一个 `Vec<Task>`
4. 遍历打印用 `task.line()`
5. 统计已完成条数：`tasks.iter().filter(|t| t.done).count()`

额外挑战：加一个 `fn finish(&mut self)`，在 `main` 里把第一条标成完成。

## 6. 验收

```bash
cargo run
```

- [ ] 程序里只剩一个 `Vec<Task>`，没有平行列表了
- [ ] 用 `{:?}` 打印过一个 `Task`，看到了 `Task { id: 1, title: "...", done: false }`
- [ ] 能说清 `Task::new(...)` 和 `task.line()` 这两种调用形式的区别

## 7. 常见错误

```
error[E0599]: no method named `line` found ... `&mut` is required
```
→ 要改数据的方法必须是 `&mut self`，而且调用它的变量得是 `mut` 的。

```
error[E0277]: `Task` doesn't implement `Debug`
```
→ 结构体上面漏了 `#[derive(Debug)]`。

参考答案：`solutions/lesson07.md`
