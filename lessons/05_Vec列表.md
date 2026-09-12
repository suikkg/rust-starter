# 第 05 课：Vec —— 装多条数据

**目标**：能在程序里存任意多条任务，而不是写死 3 条。

## 1. 先跑实验

```bash
cargo run --example lesson05_vec
```

## 2. 写法

```rust
let mut titles: Vec<String> = Vec::new();
titles.push(String::from("学习 Rust"));

titles.len()        // 有几条
titles[0]           // 按下标取，越界直接崩溃
titles.get(0)       // 安全版，返回 Option（第 08 课细讲）
titles.remove(0)    // 删掉第 1 条
```

`Vec<String>` 读作"装 String 的列表"。尖括号里是元素类型。

## 3. 遍历

```rust
for title in &titles {                       // & = 借来看，不夺走
    println!("- {title}");
}

for (i, title) in titles.iter().enumerate() {  // 要序号就用 enumerate
    println!("{} {}", i + 1, title);           // i 从 0 开始，显示时 +1
}
```

**注意**：写 `for title in titles`（不带 `&`）会把整个列表"吃掉"，循环结束后 `titles` 就不能用了。下一课会讲清楚为什么。现在先记住：**遍历加 `&`**。

## 4. 下标 vs 编号

列表下标从 0 开始，但给人看的任务编号从 1 开始。这两套数字要在脑子里分清楚，混了就会出现"完成第 1 条，结果第 2 条打了勾"。

本课先用 `下标 + 1` 当编号；第 07 课会给任务一个真正的 `id` 字段，从此不再依赖下标。

## 5. 动手任务

改 `src/main.rs`：

1. 用 `Vec<String>` 存任务名，至少放 3 条
2. 用 `Vec<bool>` 存对应的完成状态（先这样凑合，第 07 课会合并成一个 struct）
3. 遍历打印，编号用 `i + 1`
4. 统计行的总数改成 `titles.len()`，不要写死

## 6. 验收

```bash
cargo run
```

- [ ] 再往 `titles` 里 push 一条，输出自动多一行，统计数字自动变
- [ ] 用了 `.len()` 而不是写死的数字
- [ ] 试过 `titles[99]`，见过崩溃信息 `index out of bounds`，并改回了 `.get()`

## 7. 常见错误

```
error[E0382]: borrow of moved value: `titles`
```
→ `for` 后面漏了 `&`，列表被循环吃掉了。

参考答案：`solutions/lesson05.md`

---

## 练习：让测试替你验收

```bash
cargo test --example ex05
```

`exercises/ex05_*.rs` 里有 4 道小题，每道题的函数体都是 `todo!()`，你把它换成实现。

- **一次只做一道**，做完就跑一次。红变绿的那一下就是你学会了
- 失败信息会指到具体哪一行、期望什么、实际什么
- 全绿了这一课才算过 —— 上面的验收清单靠自觉，测试不会放水

答案：`solutions/exercises/ex05.rs`（先自己写 15 分钟）。想看答案跑起来什么样：
`cargo test --example ans05`。
