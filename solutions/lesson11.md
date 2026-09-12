# 第 11 课答案要点

代码：`final/`（5 个文件）

**拆成了什么**

| 文件 | 职责 | 行数量级 |
|---|---|---|
| `model.rs` | `Task` 长什么样 | 30 |
| `store.rs` | 增删改查 + 存盘 + 测试 | 100 |
| `view.rs` | 怎么显示 | 20 |
| `lib.rs` | 串起来，对外给 `run()` | 100 |
| `main.rs` | 读参数 → 调 run → 显示 | 15 |

按**职责**拆，不按大小拆。

**语法要点**

```rust
// lib.rs
pub mod model;          // 声明有这个模块
pub fn run(...) { }

// store.rs
use crate::model::Task; // crate = 本包的根

// main.rs
use rust_starter::run;  // 包名：Cargo.toml 里是 rust-starter，代码里是 rust_starter
```

**为什么要有 lib.rs**

拆出 `lib.rs` 之后，`run()` 能被测试直接调用 —— `main.rs` 里的东西测试够不着。这也是真实 Rust 项目的通行做法：**`main.rs` 只做入口，业务在 lib 那一侧**。

**pub 不是麻烦，是设计**

`store.rs` 里的 `next_id` 故意不写 `pub`。它是内部细节，将来想换算法（比如改成 UUID）不用管外面有没有人在用。**没标 `pub` 的东西你可以随便改** —— 这是可见性真正的价值。

**拆的时候怎么不出错**

一次搬一个模块，每搬完一步就 `cargo check`。不要一次全搬完再编译，那样报错会堆成一片，很难定位。
