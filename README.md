# rust-starter —— 零基础 Rust 入门

用 12 课，从 `println!` 写到一个真能用的命令行待办清单。

```bash
cargo run -- add "学习 Rust 变量"
cargo run -- list
cargo run -- done 1
```

```text
我的待办清单

[✓] 1  学习 Rust 变量
[ ] 2  练习函数

共 2 项，已完成 1 项。
```

**不需要任何编程基础。** 全部练习和答案都在这个目录里，不依赖别的教程或仓库。

---

## 目录

- [先跑起来](#先跑起来)
- [这个项目由三部分组成](#这个项目由三部分组成)
- [每一课怎么上](#每一课怎么上)
- [12 课](#12-课)
- [进度表](#进度表)
- [常用命令](#常用命令)
- [速查卡](#速查卡)
- [卡住的时候](#卡住的时候)
- [报错速查](#报错速查)
- [环境](#环境)
- [学完之后](#学完之后)

---

## 先跑起来

```bash
cd rust-starter
cargo run
```

看到「我的待办清单」就说明环境没问题。

第一次跑会花几十秒下载并编译依赖，之后就快了。

然后打开 **`lessons/01_第一次运行.md`**，从那里开始。

## 这个项目由三部分组成

这是最该先搞清楚的事：

| | 是什么 | 怎么跑 | 会不会变 |
|---|---|---|---|
| **`src/main.rs`** | 你的待办清单程序 | `cargo run` | **每一课都在改它**，从 20 行长到 200 行 |
| **`examples/`** | 12 个独立小实验，**读的** | `cargo run --example lesson06_ownership` | 不变，随时可以回头再跑 |
| **`exercises/`** | 每课 4–5 道小题，**写的** | `cargo test --example ex06` | 你把里面的 `todo!()` 换成实现 |

每一课的流程是：先跑那一课的 example 看清楚概念 → 做练习让测试变绿 →
再回到 `src/main.rs` 把它用上。

**例子是拿来读和改坏的，练习是拿来写的，`src/main.rs` 才是你的作品。**

### 练习为什么用测试

前面的验收清单是「自己对着打勾」，容易放水，而且写错了不一定当场看得出来。
练习不一样：

```bash
cargo test --example ex04
```

第一次跑，全部是红的 —— 因为函数体还是 `todo!()`：

```text
thread 'tests::剩余条数' panicked at exercises/ex04_functions.rs:24:5:
not yet implemented
```

然后你顺手写了 `total - done`，再跑一次：

```text
---- tests::数据坏了也不崩溃 stdout ----
thread 'tests::数据坏了也不崩溃' panicked at exercises/ex04_functions.rs:24:5:
attempt to subtract with overflow
```

`u32` 减成负数会崩溃 —— 这个坑你自己盯着代码看十分钟也未必想得到，
测试三秒钟就替你撞出来了。哪一行、什么原因，一目了然。

**红变绿的那一下，就是你真的学会了那个点。**

普通的 `cargo test` **不会**跑这些练习，所以作业没做完也不会看到一片红。

## 每一课怎么上

固定五步，别跳：

1. **跑实验** —— `cargo run --example lessonNN_xxx`，先看到这个概念在干什么
2. **读课程** —— `lessons/NN_*.md`，只讲今天这一个概念
3. **做练习** —— `cargo test --example exNN`，一次做一道，跑到全绿
4. **改程序** —— 按课程末尾的「动手任务」改 `src/main.rs`
5. **验收** —— 对着「验收」清单逐条打勾，全过了再进下一课

第 3 步和第 4 步的区别：练习是**单个概念的小题**，`src/main.rs` 是
**把概念用进自己的作品**。只做练习不改程序，学到的东西是散的。

**每天 30–60 分钟，一课通常一到两天。** 第 06 课（所有权）花三天也正常。

课程里凡是写着「动手」「必做实验」的地方，都要真的动手跑一遍 ——
特别是那些让你**故意把代码改坏**的步骤。看着编译器报错、读懂它、再修好，
这个循环就是学 Rust 最快的路。跳过它，后面一定会卡。

## 12 课

| 课 | 学什么 | 学完之后程序能做什么 |
|---|---|---|
| 01 | cargo、`fn main`、`println!` | 能跑，能打印 |
| 02 | `let`、`mut`、基本类型 | 数据存进变量 |
| 03 | `if`、`for`、`while` | 按状态显示 ✓，循环打印 |
| 04 | 函数、参数、返回值 | 逻辑收进函数，`main` 变短 |
| 05 | `Vec`、遍历、`get` | 能装任意多条，不再写死 3 条 |
| 06 | **所有权、`&`、`&mut`** | 显示和修改都改成借用 |
| 07 | `struct`、`impl` | 一条任务是一个 `Task` |
| 08 | `enum`、`match`、`Option` | 命令类型化，找不到编号也不崩溃 |
| 09 | 命令行参数、`Result`、`?` | **从终端真的能用了** |
| 10 | 文件、JSON、serde | 关掉程序数据还在 |
| 11 | `mod`、`pub`、`lib.rs` | 拆成 5 个文件 |
| 12 | 测试 + 独立加功能 | **毕业考，没有标准答案** |

第 02–11 课每课都配了练习（共 42 道、49 个测试），课程文档末尾有入口。

### 关于第 06 课

所有权是 Rust 和别的语言最不一样的地方，也是前期唯一真正的难关。
**卡住是正常的**，每个学 Rust 的人都在这里慢下来过。

这一课的课程文档专门配了一张报错对照表。慢慢来，可以拆成两三天。

### 关于第 12 课

任务 A（补测试）有答案要点。**任务 B（`list --pending`）故意没有答案** ——
前面 11 课教的东西够你自己做出来了。做完还要回答三个问题，答得上来才算过。

## 进度表

做完一课就打个勾：

- [ ] 01 第一次运行
- [ ] 02 变量与可变性
- [ ] 03 判断与循环
- [ ] 04 函数
- [ ] 05 Vec 列表
- [ ] 06 所有权与借用 ← 最难的一关
- [ ] 07 struct 结构体
- [ ] 08 enum 与 Option
- [ ] 09 命令行与 Result
- [ ] 10 文件与 JSON
- [ ] 11 模块拆分
- [ ] 12 测试与独立扩展 ← 毕业考

打勾的标准：验收清单全过 **且** 那一课的 `cargo test --example exNN` 全绿。

**上一课的验收清单没全过，就不要进下一课。** 后面每一课都建立在前面之上。

## 常用命令

```bash
cargo check          # 只检查能不能编译，最快 —— 写代码时一直用它
cargo run            # 编译并运行
cargo test           # 跑测试（第 12 课才用得上）
cargo fmt            # 自动排版。这个项目已经是规范格式，跑它不会有任何改动
cargo clippy         # 代码建议，见下
```

跑你的程序时，参数要写在 `--` 后面：

```bash
cargo run -- add "任务名"      # -- 后面的才是给程序的
cargo run -- list
```

跑某一课的小实验和练习：

```bash
cargo run --example lesson06_ownership   # 读的例子
cargo test --example ex06                # 写的练习（第 02–11 课）
cargo test --example ans06               # 练习的答案，跑起来是全绿的
cargo test --example lesson12_tests      # 第 12 课那个例子要用 test 跑
```

`exNN` 和 `ansNN` 里的 NN 就是课号。

### clippy 是你的第二个老师

```bash
cargo clippy
```

编译器管「能不能跑」，clippy 管「该不该这么写」。它会告诉你
「这里用 `&[T]` 比 `&Vec<T>` 好」「这个 `match` 可以简化」这类事。

**这个项目现在是 clippy 零告警的。** 你改完代码如果冒出告警，
多半真的有更好的写法 —— 值得看一眼。

## 速查卡

**[`CHEATSHEET.md`](CHEATSHEET.md)** —— 12 课用得到的全部语法，按课号排。

写代码时忘了 `Option` 有哪些方法、参数该写 `&str` 还是 `String`、
`match` 怎么写全分支，翻它比翻课程快。刻意只收这 12 课的东西，
**不全面是故意的** —— 速查卡一长就没人看了。

## 卡住的时候

按这个顺序来：

**1. 完整读一遍报错。** 不要只看第一行。Rust 的报错通常包含：
出错位置、为什么错、**怎么改**。很多时候它直接给出了正确代码。

```text
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:5:16
  |
2 |     let a = String::from("x");
  |         - move occurs because `a` has type `String`,
  |           which does not implement the `Copy` trait    ← 为什么会这样
3 |     let b = a;
  |             - value moved here                         ← 在哪被移走的
4 |     println!("b = {b}");
5 |     println!("{a}");
  |                ^ value borrowed here after move        ← 哪一行出的错
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let b = a.clone();
  |              ++++++++                                  ← 直接给了改法
```

一条报错里有四样东西：**哪一行错了、在哪被移走的、为什么、怎么改**。
养成每条都看完的习惯，你会发现大部分问题不用查资料。

**2. 回头看那一课的「常见错误」。** 每课末尾都有，`CHEATSHEET.md` 里也有语法速查。

**3. 看 `solutions/`。** 但先自己试 15 分钟。
练习的答案在 `solutions/exercises/exNN.rs`，可以直接 `diff`：

```bash
diff exercises/ex06_ownership.rs solutions/exercises/ex06.rs
```

**4. 改坏了想重来**：

```bash
cp solutions/steps/lesson07_main.rs src/main.rs     # 回到第 07 课的完成态
```

## 报错速查

前期最常撞见的几个，以及它们在哪一课讲：

| 报错 | 意思 | 怎么修 | 哪一课 |
|---|---|---|---|
| `cannot assign twice to immutable variable` | 变量不能改 | 加 `mut` | 02 |
| `` `if` and `else` have incompatible types`` | 两个分支类型不一样 | 多半是一边多了分号 | 03 |
| `mismatched types: expected String, found ()` | 函数没返回值 | 最后一行的分号去掉 | 04 |
| `borrow of moved value` | 数据被移走了还在用 | 传参加 `&` | 06 |
| `cannot borrow as mutable` | 没声明成可变 | `let mut x = ...` | 06 |
| `expected &str, found String` | 字符串两种类型混了 | 传 `&s` | 06 |
| `non-exhaustive patterns` | `match` 漏了分支 | 按提示补上 | 08 |
| `the ? operator can only be used in a function that returns Result` | 在 `main` 里用了 `?` | 逻辑搬进返回 `Result` 的函数 | 09 |
| `index out of bounds` | 下标越界（运行时崩溃） | 用 `.get()` 代替 `[i]` | 05 |

## 环境

```bash
rustc --version    # 1.96.0
cargo --version    # 1.96.0
```

两个都能打印出版本号就行。

`command not found` 的话：Rust 没装好，或者装完没重开终端。先试着重开一个终端窗口。

这个项目用 **edition 2021**（和后面要学的真实项目一致）。
依赖只有 `serde` 和 `serde_json`，到第 10 课才用得上。

## 目录

```
rust-starter/
├── lessons/          12 课的课程文档 ← 主线，按顺序看
├── examples/         12 个独立小实验，读的
├── exercises/        第 02–11 课的练习，写的（cargo test --example exNN）
├── src/main.rs       你的待办清单程序，每课都在改它
├── solutions/
│   ├── steps/        第 01–10 课结束时 main.rs 该长什么样
│   ├── final/        第 11 课拆完模块的完整程序（5 个文件）
│   ├── exercises/    练习的答案（cargo test --example ansNN）
│   ├── lessonNN.md   每课的要点和常见坑
│   └── README.md     怎么用答案
├── tests/            第 12 课才会用到
├── CHEATSHEET.md     语法速查卡
├── check.sh          一键自检：fmt + clippy + 全部答案
└── Cargo.toml
```

## 学完之后

你能做到：

- 从空文件写出一个能跑的 Rust 程序
- 用 `struct` 和 `Vec` 组织数据
- 看懂常见的借用错误并自己修掉
- 处理非法输入而不是让程序崩溃
- 读写文件、存取 JSON
- 把程序拆成几个模块
- 写测试保护自己的改动

**下一站**：隔壁的 `cpe-mini` —— 同样的能力，用到真实业务上。
那边不是从零写，而是读懂一份现成代码再改它，这正是维护真实项目的工作方式。

不必等这里全部做完。**学到第 07 课（struct）之后就可以并行开始。**
