# 第 14 课：HashMap

`Vec` 是「第 3 个」，`HashMap` 是「叫『工作』的那个」。

真实项目 `cpe-test` 里 `HashMap` 出现 **111 次**。

## 1. 先跑实验

```bash
cargo run --example lesson14_hashmap
```

第 4 节的输出每次都不一样 —— 那是这一课的重点。

## 2. 基本用法

```rust
use std::collections::HashMap;

let mut counts: HashMap<String, u32> = HashMap::new();
counts.insert("工作".to_string(), 3);

counts.get("工作")      // Some(&3)
counts.get("旅游")      // None      ← 又是第 08 课那个 Option
counts["旅游"]          // 💥 崩溃
```

方括号是「我保证有」的写法，和第 05 课 `Vec` 的 `[i]` 一模一样。
**不确定有没有就用 `.get()`。**

取不到给个默认值：

```rust
counts.get("旅游").copied().unwrap_or(0)
```

（`copied()` 把 `Option<&u32>` 变成 `Option<u32>`，这样才和 `0` 类型对得上。）

## 3. 两个惯用写法

### 计数

```rust
for tag in tags {
    *counts.entry(tag).or_insert(0) += 1;
}
```

`entry(k)` 读作「找 k 这一格」：有就给你，没有就按你说的先建一个。
比「先 get 看看，没有再 insert」少一次查找，也少一次写错的机会。

前面那个 `*` 是因为 `or_insert` 给的是 `&mut u32`，要解引用才能加。

### 分组

```rust
grouped.entry(tag).or_default().push(title);
```

`or_default()` 对 `Vec` 就是「没有就给个空 Vec」。

## 4. 最大的坑：遍历顺序是乱的

跑一下实验的第 4 节：

```text
第一个 map 的顺序：["a", "c", "b", "f", "d", "e"]
第二个 map 的顺序：["c", "f", "d", "e", "b", "a"]
内容完全一样，顺序不一样！
```

**同一份数据，建两个 map，顺序都可能不同。** 不只是「和插入顺序不同」，
是**每次都可能不同**（Rust 故意这么设计，为了防一类攻击）。

后果：

> 你的报告按 `for (k, v) in &map` 打印。今天跑出来 A、B、C，
> 明天跑出来 C、A、B。数据一模一样，diff 却是满屏的红绿。

规矩：

> **任何要给人看、要存文件、要对比的输出，都必须先定顺序。**

```rust
let mut keys: Vec<_> = counts.keys().collect();
keys.sort();
for k in keys { ... }
```

## 5. 要顺序就用 BTreeMap

```rust
use std::collections::BTreeMap;
```

用法几乎一样，但它**永远按 key 排好**。

| 场景 | 用哪个 |
|---|---|
| 只是查一下，结果不输出 | `HashMap`（快一点） |
| 结果要输出 / 存文件 / 做对比 | `BTreeMap` |

后者省掉一次排序，更重要的是**省掉一次忘记排序**。

隔壁 `cpe-mini` 的 `src/compare.rs` 用的就是 `BTreeMap`，理由正是这个 ——
对比报告的行顺序要是随机的，这个功能就没法用了。

## 6. key 可以是什么

能当 key 的类型要实现 `Hash + Eq`。常见的都行：
`String`、`&str`、整数、`char`、元组、`#[derive(Hash, Eq, PartialEq)]` 的
你自己的类型。

**`f64` 不行** —— 因为 `NaN != NaN`，浮点数当 key 会出乱子。

## 7. 动手任务

给任务加标签，让 `list` 能按标签分组显示。

1. `Task` 加一个 `tag: String` 字段（第 10 课那个 JSON 也要跟着变 ——
   加 `#[serde(default)]` 让旧文件还能读）
2. `add` 支持 `cargo run -- add "买菜" --tag 生活`，不给就用 `"未分类"`
3. `list` 按标签分组显示，**标签按名字排序**

```text
生活
  [ ] 3  买菜

学习
  [✓] 1  学习变量
  [ ] 2  练习函数
```

第 3 步是这一课的考点：不排序的话，每次跑出来的分组顺序都可能不同。

## 8. 验收

- [ ] `cargo run --example lesson14_hashmap` 跑过，第 4 节的乱序看见了
- [ ] `list` 按标签分组，**连跑三次顺序完全一样**
- [ ] 旧的 `tasks.json`（没有 tag 字段）还能读得出来
- [ ] 说得出 `HashMap` 和 `BTreeMap` 各该用在什么时候
- [ ] `cargo test` 全绿

## 9. 常见错误

```
error[E0433]: failed to resolve: use of undeclared type `HashMap`
```
→ 漏了 `use std::collections::HashMap;`

```
error[E0277]: the trait bound `f64: Eq` is not satisfied
```
→ 浮点数不能当 key。见第 6 节。

```
error[E0614]: type `u32` cannot be dereferenced
```
→ `*counts.entry(k).or_insert(0) += 1` 那个 `*` 加错地方了。

参考答案：`solutions/lesson14.md`

---

## 练习：让测试替你验收

```bash
cargo test --example ex14
```

`exercises/ex14_hashmap.rs` 里有 5 道小题（9 个测试）。
第 4、5 题是这一课真正的考点。

答案：`solutions/exercises/ex14.rs`。
