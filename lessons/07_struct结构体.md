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

### 第一个参数决定这个函数怎么调

`&self` / `&mut self` 就是上一课的借用规则**用在自己身上**：

| 第一个参数 | 叫什么 | 怎么调 | 调完原件还在吗 |
|---|---|---|---|
| 没有 self | **关联函数** | `Task::new(1, "x")` | —— |
| `&self` | 方法，只读 | `task.line()` | ✅ |
| `&mut self` | 方法，要改 | `task.finish()` | ✅（而且被改了） |
| `self` | 方法，**吃掉自己** | `task.into_parts()` | ❌ |

前三种你天天用。第四种（直接写 `self`）表示「这个方法会把对象消耗掉」，
常见于 `into_xxx` 这类转换函数 —— 现在只要认得出来就行。

> **看第一个参数，就知道这个方法会不会动你的数据、调完还能不能用。**
> 和上一课看函数签名是同一件事。

### `Self` 是「当前这个类型」的简写

```rust
impl Task {
    fn new(id: u32, title: &str) -> Self {       // Self 就是 Task
        Self { id, title: title.to_string(), done: false }
    }
}
```

写 `Task` 还是 `Self` 都行，但**真实代码里几乎都写 `Self`** ——
哪天类型改名了，`impl` 块里一个字都不用动。
（隔壁 `cpe-mini` 里满屏都是 `Self`，先在这儿认个脸。）

### 那个 `id,` 为什么没有值

```rust
Self { id, title: title.to_string(), done: false }
//     ^^ 这里只写了字段名，没写值
```

**字段名和变量名一样时可以省略**，`id` 就是 `id: id` 的简写。
不是笔误，是 Rust 的常用写法，读真实代码时到处都是。

## 4. derive 是什么

```rust
#[derive(Debug, Clone)]
```

意思是"编译器，请帮我自动实现这几样"：

- `Debug` → 可以用 `{:?}` 打印，调试时极其常用
- `Clone` → 可以 `.clone()` 复制一份

还有几个后面会用到的，现在认个脸：

| derive | 给你什么 | 什么时候加 |
|---|---|---|
| `Debug` | `{:?}` | **几乎总是加** |
| `Clone` | `.clone()` | 要复制的时候 |
| `PartialEq` | `==`、`assert_eq!` | 要比较、要写测试的时候 |
| `Default` | `Task::default()` | 要「全是默认值的一个」的时候 |
| `Copy` | 赋值不搬走（第 06 课） | 只有小的、纯数字的类型才配加 |

**写测试时 `assert_eq!(a, b)` 要求两边能比较**，所以那时候得补上 `PartialEq`。
第 12 课会撞上。

`derive` 生成的代码是最朴素的那种（逐字段比较、逐字段复制）。
不满意就自己写 —— 第 15 课讲怎么手写一个。

## 5. 多个 impl 块、多个方法

一个类型可以有好几个 `impl` 块，也可以一个块里放十个方法。
惯例是**按职责分块**：

```rust
impl Task {          // 造和读
    fn new(...) -> Self { }
    fn line(&self) -> String { }
}

impl Task {          // 改
    fn finish(&mut self) { }
    fn rename(&mut self, title: &str) { }
}
```

小程序不用分。知道可以分就行 —— 真实项目里一个类型有几百行方法时，
分块是唯一能读下去的办法。

## 6. 动手任务

改 `src/main.rs`：

1. 定义 `struct Task { id, title, done }`
2. 写 `Task::new(id, title)` 和 `fn line(&self) -> String`
3. 把两个平行 `Vec` 换成一个 `Vec<Task>`
4. 遍历打印用 `task.line()`
5. 统计已完成条数：`tasks.iter().filter(|t| t.done).count()`

额外挑战：加一个 `fn finish(&mut self)`，在 `main` 里把第一条标成完成。

## 7. 验收

```bash
cargo run
```

- [ ] 程序里只剩一个 `Vec<Task>`，没有平行列表了
- [ ] 用 `{:?}` 打印过一个 `Task`，看到了 `Task { id: 1, title: "...", done: false }`
- [ ] 能说清 `Task::new(...)` 和 `task.line()` 这两种调用形式的区别
- [ ] 知道 `Self` 是什么，也知道 `Task { id, ... }` 里那个光秃秃的 `id` 是什么意思
- [ ] 看第一个参数就能说出这个方法会不会改数据

## 8. 常见错误

```
error[E0599]: no method named `line` found ... `&mut` is required
```
→ 要改数据的方法必须是 `&mut self`，而且调用它的变量得是 `mut` 的。

```
error[E0277]: `Task` doesn't implement `Debug`
```
→ 结构体上面漏了 `#[derive(Debug)]`。

```
error[E0308]: mismatched types  expected `String`, found `&str`
```
→ 字段是 `String`，参数收的是 `&str`。加 `.to_string()`。

```
error[E0425]: cannot find value `title` in this scope
```
→ 想用字段简写 `Task { title }`，但局部变量不叫 `title`。
简写只在**名字一样**时能用，否则老实写 `title: 那个变量`。

参考答案：`solutions/lesson07.md`

---

## 练习：让测试替你验收

```bash
cargo test --example ex07
```

`exercises/ex07_*.rs` 里有 5 道小题（5 个测试），每道题的函数体都是 `todo!()`，你把它换成实现。

- **一次只做一道**，做完就跑一次。红变绿的那一下就是你学会了
- 失败信息会指到具体哪一行、期望什么、实际什么
- 全绿了这一课才算过 —— 上面的验收清单靠自觉，测试不会放水

答案：`solutions/exercises/ex07.rs`（先自己写 15 分钟）。想看答案跑起来什么样：
`cargo test --example ans07`。
