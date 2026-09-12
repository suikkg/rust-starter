# 第 12 课答案要点

## 任务 A（写测试）：有答案

参考 `final/store.rs` 底部的 `#[cfg(test)] mod tests`，那里有 7 个测试，
覆盖了课程要求的四项。挑三个值得说的：

**测边界，不测显而易见的东西**

```rust
#[test]
fn 删掉中间一条后编号不重复() {
    let mut tasks = vec![Task::new(1, "a"), Task::new(2, "b"), Task::new(3, "c")];
    remove(&mut tasks, 2);
    // 条数是 2，但新编号必须是 4，不能是 3
    assert_eq!(next_id(&tasks), 4);
}
```

这个测试保护的是"`next_id` 用最大值加一，不是用条数加一"。
如果你当初写的是 `tasks.len() + 1`，这个测试会立刻抓到。

**测"不该发生的事没发生"**

```rust
#[test]
fn 完成不存在的编号返回None() {
    let mut tasks = vec![Task::new(1, "a")];
    assert!(finish(&mut tasks, 99).is_none());
    assert!(!tasks[0].done);     // ← 这一行才是重点
}
```

第一行测返回值，第二行测**原数据没被误改**。只写第一行的话，
一个"找不到就把第一条标记完成"的错误实现也能过。

**往返测试**

```rust
save(&path, &tasks).expect("存盘应该成功");
let loaded = load(&path).expect("读取应该成功");
assert_eq!(loaded.len(), 2);
assert!(loaded[0].done);
```

存进去再读出来，数据一致。这类测试能一次抓住序列化、反序列化、
文件读写三处的问题。

**测试要用临时目录**

```rust
let path = std::env::temp_dir().join("rust-starter-test-roundtrip.json");
```

不要用项目里的 `todos.json` —— 测试会把你自己的清单洗掉，
而且多个测试同时跑会互相干扰。

---

## 任务 B（`list --pending`）：没有答案

**这是毕业考，自己做完。**

如果完全卡住，这里只给方向，不给代码：

- 参数解析在 `lib.rs` 的 `"list"` 分支里。现在它直接 `Ok(view::render(&tasks))`，
  你需要先看一眼 `args.get(1)`
- 未知选项要报错。想清楚：`None`（没给选项）、`Some("--pending")`、
  `Some(别的)` 是三种情况，用 `match` 比用 `if` 清楚
- `view::render` 要多一个参数。改完记得所有调用点都要改 —— 编译器会告诉你在哪
- 测试放 `lib.rs` 的 `mod tests` 里，`run(&args(&["list", "--pendign"]))`
  应该 `is_err()`

**一个容易忽略的判断**：过滤之后清单可能是空的（全都完成了）。
这时该显示什么？显示"（清单是空的）"是错的 —— 清单不空，只是没有未完成的。

做完记得回答课程里那三个问题。答不上来，就说明是抄通的不是想通的。
