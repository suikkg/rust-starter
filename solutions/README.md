# 参考答案

## 怎么用

**先自己写，卡住超过 15 分钟再看。** 看答案之前，至少先把编译器的报错完整读一遍。

- `steps/lessonNN_main.rs` —— 第 NN 课结束时 `src/main.rs` 应该长的样子（全部验证过能编译）
- `lessonNN.md` —— 这一课的关键点和常见坑
- `final/` —— 第 11 课拆分模块之后的完整程序（5 个文件）

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

## 关于第 12 课

`lesson12.md` 只有**任务 A（写测试）**的要点。

**任务 B（`list --pending`）故意没有答案** —— 那是毕业考，前面 11 课教的东西够你自己做出来了。
