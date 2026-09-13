# 第 15 课：trait 入门

`println!("{}", 3)` 能打印数字，`"a" < "b"` 能比字符串，`vec.sort()` 能排序。

这些本事**不是语言内置的** —— 是这些类型实现了对应的 trait。
你的类型实现同一个 trait，就有同样的本事。

## 1. 先跑实验

```bash
cargo run --example lesson15_trait
```

## 2. trait 是什么

一句话：**「能做某件事」的约定**。

```rust
trait 会打印 {
    fn 打印出来(&self) -> String;
}
```

标准库里已经有一堆现成的，你基本不用自己定义 trait，
但**要会实现**它们。

## 3. 你已经在用了：`#[derive(...)]`

前面十二课每个 struct 上面都有这个：

```rust
#[derive(Debug, Clone, PartialEq)]
struct Task { ... }
```

`derive` 就是「这个 trait 的实现太机械了，编译器你替我写」。

| trait | 给你什么 | 怎么来 |
|---|---|---|
| `Debug` | `{:?}` 能打印 | derive |
| `Clone` | `.clone()` | derive |
| `Copy` | 赋值不搬走（第 06 课） | derive，只有小的纯数据类型才配 |
| `PartialEq` | `==` | derive |
| `Ord` | 能排序 | derive |
| `Default` | `Default::default()` | derive |
| **`Display`** | **`{}` 能打印** | **要自己写** |
| `From` | `?` 号自动转 | 自己写 |

`Display` 为什么不能 derive？因为**只有你知道给人看的时候该长什么样**。

## 4. `Display`：让 `{}` 能打印你的类型

```rust
use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mark = if self.done { "✓" } else { " " };
        write!(f, "[{mark}] {}  {}", self.id, self.title)
    }
}
```

三件事要记：

1. 函数体里用 `write!(f, ...)`，参数和 `println!` 一样
2. **最后一行不加分号** —— `write!` 的返回值就是这个函数要返回的
3. 实现了它，`.to_string()` **自动一起有了**，一行都不用写

`{}` 和 `{:?}` 的分工：

```rust
println!("{}", task);    // [✓] 1  学习变量        给人看
println!("{:?}", task);  // Task { id: 1, ... }    给你自己调试看
```

## 5. 派生排序：声明顺序就是大小顺序

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    High,      // 最小
    Medium,
    Low,       // 最大
}
```

`High < Medium < Low`。**变体的声明顺序就是它们的大小顺序。**

这件事很容易忘，而且后果很隐蔽：

> 哪天有人把 `Low` 挪到最前面，所有排序的行为立刻就变了 ——
> **编译器一个字都不会说**，测试如果没盯住顺序也不会红。

隔壁 `cpe-mini` 的 `src/compare.rs` 里 `DeltaKind` 就是这么设计的：
「判定变坏」声明在最前面，所以对比报告里它排在最上面。
那个 enum 上专门写了一行注释说「**声明顺序就是报告的排序顺序，别乱动**」。

四个 derive 少一个都不行，但编译器会告诉你少了哪个（`Ord` 要 `Eq`，
`PartialOrd` 要 `PartialEq`）。

## 6. `From`：第 09 课那个 `?` 号的秘密

第 09 课你写过这个：

```rust
let text = std::fs::read_to_string(path)?;   // io::Error
let n: u32 = text.trim().parse()?;           // ParseIntError
```

两种完全不同的错误，同一个 `?`，都能往上抛。为什么？

因为 `?` 不只是「出错就 return」，它还会**调 `From::from` 转一次类型**：

```rust
impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::BadNumber(e.to_string())
    }
}
```

写了这个之后：

```rust
fn parse_id(s: &str) -> Result<u32, AppError> {
    let id: u32 = s.parse()?;   // ← 自动变成了 AppError
    Ok(id)
}
```

> `?` 能跨类型工作，靠的全是 `From`。

## 7. 泛型：一个函数吃所有实现了某个 trait 的类型

```rust
fn print_all<T: fmt::Display>(items: &[T]) {
    for item in items {
        println!("{item}");
    }
}
```

`T: fmt::Display` 读作「任何一个实现了 `Display` 的类型」。
这一个函数能吃 `&[Task]`、`&[i32]`、`&[&str]`。

两种写法一回事，短的那种更常见：

```rust
fn f<T: Display>(x: T)      // 泛型参数
fn f(x: impl Display)       // 一样的意思
```

## 8. 动手任务

1. 给 `Task` 实现 `Display`，然后把 `view.rs`（或者 `main.rs`）里
   手拼字符串的地方换成 `{}`
2. 加一个 `Priority` 枚举（`High` / `Medium` / `Low`），派生排序，
   让 `list` 按优先级排
3. （难）把 `main.rs` 里那些 `Result<_, String>` 换成一个
   `enum AppError`，用 `From` 让 `?` 还能照常用

第 3 步是真实项目的标准做法：错误有类型，不是一坨字符串。

## 9. 验收

- [ ] `println!("{}", task)` 能打印了
- [ ] `task.to_string()` 也能用（没有额外写代码）
- [ ] `list` 按优先级排序，High 在最前
- [ ] 说得出为什么 `Display` 不能 derive
- [ ] 说得出 `?` 号是怎么把一种错误变成另一种的
- [ ] `cargo test` 全绿

## 10. 常见错误

```
error[E0277]: `Task` doesn't implement `std::fmt::Display`
```
→ 用 `{}` 打印了一个没实现 `Display` 的类型。要么实现它，要么改用 `{:?}`。

```
error[E0277]: the trait bound `Priority: Ord` is not satisfied
```
→ derive 少了。按报错提示补，通常是 `Eq` 或 `PartialOrd`。

```
error[E0308]: mismatched types: expected `()`, found `Result<...>`
```
→ `impl Display` 里 `write!` 后面多写了分号。

```
error[E0277]: `?` couldn't convert the error to `AppError`
```
→ 缺 `impl From<那个错误类型> for AppError`。见第 6 节。

参考答案：`solutions/lesson15.md`

---

## 练习：让测试替你验收

```bash
cargo test --example ex15
```

`exercises/ex15_trait.rs` 里有 4 道小题（9 个测试）。

**先做第 2 题** —— 它不做的话整个文件编译不过，别的题一道也跑不了。
这是 Rust 的常态：编译错误永远排在测试失败前面。

答案：`solutions/exercises/ex15.rs`。

---

## 三门补充课到这里就完了

你现在读得懂 `.iter().filter().map().collect()`、
`entry().or_insert()`、`impl Display`、`T: Display` 这些写法了。

**这是隔壁 `cpe-mini` 的入场券** —— 那份代码里这四样东西满屏都是。
