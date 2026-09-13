# 参考答案

## 怎么用

**先自己写，卡住超过 15 分钟再看。** 看答案之前，至少先把编译器的报错完整读一遍。

- `steps/lessonNN_main.rs` —— 第 NN 课结束时 `src/main.rs` 应该长的样子（全部验证过能编译）
- `lessonNN.md` —— 这一课的关键点和常见坑
- `final/` —— 第 11 课拆分模块之后的完整程序（5 个文件）
- `exercises/exNN.rs` —— `exercises/` 里那些练习的答案，**带注释解释为什么这么写**

对比自己的写法：

```bash
# 看答案代码
cat solutions/steps/lesson07_main.rs

# 和自己的比
diff src/main.rs solutions/steps/lesson07_main.rs
```

想直接跑答案：

```bash
cp solutions/steps/lesson07_main.rs src/main.rs
cargo run
```

（跑完记得把自己的版本换回来。）

## 跑 final 版

`final/` 是多文件版本，要一起复制：

```bash
cp solutions/final/*.rs src/
cargo test
cargo run -- add "试一下"
cargo run -- list
```

恢复成单文件：

```bash
rm src/lib.rs src/model.rs src/store.rs src/view.rs
cp solutions/steps/lesson10_main.rs src/main.rs
```

## 练习的答案

`exercises/exNN_*.rs` 里的练习，答案在 `solutions/exercises/exNN.rs`。

直接跑一遍（全绿）：

```bash
cargo test --example ans06
```

和自己的比：

```bash
diff exercises/ex06_ownership.rs solutions/exercises/ex06.rs
```

答案里的注释比代码本身更值得看 —— 它们解释的是**为什么选这个写法**，
比如为什么 `pending` 要用 `saturating_sub`、为什么 `longest` 要用
`chars().count()` 而不是 `len()`。这些是题目里问不出来、只能讲的东西。

## 补充课 13–15

主线是 01–12。13–15 是**毕业之后**的补充课，讲前面刻意没讲、
但真实 Rust 代码里满屏都是的三样东西：

| 课 | 答案 |
|---|---|
| 13 闭包与迭代器 | `lesson13.md` + `exercises/ex13.rs` |
| 14 HashMap | `lesson14.md` + `exercises/ex14.rs` |
| 15 trait 入门 | `lesson15.md` + `exercises/ex15.rs` |

这三课的答案文档里，「练习里的坑」那一节最值得看 ——
每一条都是真会撞上的编译错误，以及它到底在说什么。

## 关于第 12 课

`lesson12.md` 只有**任务 A（写测试）**的要点。

**任务 B（`list --pending`）故意没有答案** —— 那是毕业考，前面 11 课教的东西够你自己做出来了。
