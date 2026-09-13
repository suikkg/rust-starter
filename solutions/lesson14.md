# 第 14 课答案要点

**`Task` 加字段**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
    #[serde(default = "default_tag")]   // ← 旧文件没有这个字段也能读
    pub tag: String,
}

fn default_tag() -> String {
    "未分类".to_string()
}
```

不加 `#[serde(default)]` 的话，你现有的 `tasks.json` 会直接读不出来 ——
**给已经存在的数据加字段，必须想好旧数据怎么办**。
（隔壁 cpe-mini 第 08 课专门讲这件事。）

**解析 `--tag`**

```rust
let tag = args
    .iter()
    .position(|a| a == "--tag")
    .and_then(|i| args.get(i + 1))
    .cloned()
    .unwrap_or_else(|| "未分类".to_string());
```

`unwrap_or_else` 而不是 `unwrap_or`：后者不管用不用得上都会先把
那个 `String` 造出来。这里差别可以忽略，但习惯要对。

**分组显示**

```rust
use std::collections::BTreeMap;

let mut grouped: BTreeMap<&str, Vec<&Task>> = BTreeMap::new();
for t in &self.tasks {
    grouped.entry(&t.tag).or_default().push(t);
}

for (tag, tasks) in &grouped {     // BTreeMap，自动按 tag 排好
    println!("{tag}");
    for t in tasks {
        println!("  [{}] {}  {}", if t.done { "✓" } else { " " }, t.id, t.title);
    }
    println!();
}
```

**用 `BTreeMap` 就不用自己排序了** —— 这是这一课的重点。

用 `HashMap` 也行，但要多写三行：

```rust
let mut tags: Vec<&&str> = grouped.keys().collect();
tags.sort();
for tag in tags { ... }
```

多写三行不要紧，**忘了写才要紧** —— 而且忘了之后测试大概率还是绿的
（小 map 碰巧顺序对了），上线才出问题。

## 怎么验证「连跑三次顺序一样」

```bash
for i in 1 2 3; do cargo run -q -- list; done | sort | uniq -c
```

顺序稳定的话，三次输出完全一样。

## 练习里的坑

**第 3 题**：`counts.get(tag)` 给的是 `Option<&usize>`，
`unwrap_or(0)` 要的是 `Option<usize>` —— 中间差一个 `.copied()`。

报错会说 `expected &usize, found integer`，读懂它就知道该加什么。

**第 5 题**：`max_by_key(|(_, n)| *n)` 在平局时返回哪个，
**取决于遍历顺序**，也就是不确定。

两种解法：

```rust
// 甲：先排序再找
let mut items: Vec<_> = counts.iter().collect();
items.sort();
items.into_iter().max_by_key(|(_, n)| **n).map(|(t, _)| t.clone())

// 乙：把名字写进比较（答案用的这个）
counts.iter()
    .max_by_key(|(tag, n)| (**n, std::cmp::Reverse((*tag).clone())))
    .map(|(tag, _)| tag.clone())
```

`Reverse` 是「反着比」：条数大的赢，条数相同时名字**小**的赢。

这类「平局怎么办」的问题在真实项目里到处都是，
而且**不写出来就等于随机**。
