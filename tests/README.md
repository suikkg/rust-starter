# tests/

这个目录第 12 课才会用到。

Rust 有两种测试位置：

- **单元测试**：写在被测代码同一个文件里的 `#[cfg(test)] mod tests`，能测到私有函数
- **集成测试**：放在这个 `tests/` 目录，只能用 `pub` 的东西，模拟"别人怎么用你的库"

第 12 课主要写单元测试（在 `src/store.rs` 底部）。想写集成测试的话，在这里建一个 `cli.rs`：

```rust
use rust_starter::run;

#[test]
fn 不认识的命令会报错() {
    let args = vec!["飞天".to_string()];
    assert!(run(&args).is_err());
}
```

注意集成测试要等第 11 课拆出 `lib.rs` 之后才能写 —— `main.rs` 里的东西外面调不到。
