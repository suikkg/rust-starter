# 第 09 课：命令行参数、Result 与 `?`

**目标**：程序开始真正能用 —— 从终端接命令，并且出错时给人话而不是崩溃。

## 1. 先跑实验

```bash
cargo run --example lesson09_args -- add "学习 Rust"
cargo run --example lesson09_args -- done 1
cargo run --example lesson09_args -- done abc
cargo run --example lesson09_args
```

注意 `--`：它之前的参数是给 cargo 的，之后的才是给你程序的。

## 2. 读参数

```rust
use std::env;

let args: Vec<String> = env::args().skip(1).collect();
```

`args()` 的第 0 个是程序自己的路径，所以 `skip(1)`。

取第一个参数的惯用写法：

```rust
let cmd = args.first().map(|s| s.as_str()).unwrap_or("");
```

`first()` 返回 `Option<&String>`，`map` 转成 `Option<&str>`，`unwrap_or("")` 给个兜底。一行里用到了上一课的全部内容。

## 3. Result

```rust
enum Result<T, E> {
    Ok(T),     // 成功，带结果
    Err(E),    // 失败，带原因
}
```

`Option` 是"有或没有"，`Result` 是"成了或失败了，失败有理由"。

## 4. `?` 运算符

```rust
let id: u32 = raw.parse().map_err(|_| format!("编号必须是数字，你给的是 {raw}"))?;
```

末尾的 `?` 意思是：**成功就取出值继续走，失败就立刻从当前函数返回这个错误**。

没有 `?` 就得这么写：

```rust
let id: u32 = match raw.parse() {
    Ok(v) => v,
    Err(_) => return Err(format!("编号必须是数字，你给的是 {raw}")),
};
```

`?` 只能用在返回 `Result`（或 `Option`）的函数里。

## 5. 关键结构：main 只管显示

```rust
fn main() {
    match run(&args) {
        Ok(msg) => println!("{msg}"),
        Err(msg) => {
            eprintln!("错误：{msg}");
            std::process::exit(1);
        }
    }
}

fn run(args: &[String]) -> Result<String, String> { ... }
```

真正干活的函数返回 `Result`，`main` 负责把结果变成给人看的输出。这样"出错怎么显示"只写一遍。**这是 Rust 命令行程序最常见的骨架，记住它。**

### `println!` 和 `eprintln!` 差在哪

程序有**两条**输出通道：

| 宏 | 去哪 | 装什么 |
|---|---|---|
| `println!` | 标准输出 stdout | **结果**：清单、报表、给下一个程序吃的数据 |
| `eprintln!` | 标准错误 stderr | **过程和错误**：警告、报错、进度 |

为什么要分？因为管道只接 stdout：

```bash
cargo run -- list > 清单.txt     # 只有清单进了文件
                                 # 错误照样显示在屏幕上，不会混进文件
```

把错误写进 stdout，用户导出的清单里就会夹着一行「错误：找不到第 3 条」。
**结果走 stdout，别的一律走 stderr。**

### 退出码：写给脚本看的

```rust
std::process::exit(1);
```

程序退出时留下一个数字：**0 = 成功，非 0 = 出问题了**。
人看不见它，但脚本靠它做判断：

```bash
cargo run -- done 3 && echo "改完了"     # && 只在退出码为 0 时才执行后面
echo $?                                  # 看上一条命令的退出码
```

`main` 正常结束就是 0。所以上面那个骨架里，只有 `Err` 分支需要 `exit(1)`。

> 隔壁 `cpe-mini` 把退出码当成正经的对外接口：
> 有测试项不达标返回 1，全通过返回 0，而且有测试专门钉住这条规则。
> 命令行程序的退出码**是 API，不是随手写的**。

### 报错要说人话

对比一下：

```text
错误：参数不对
错误：编号必须是数字，你给的是 `三`
```

第二句多写了十个字，省掉用户一次猜谜。写报错时问自己两句：

1. **哪里错了？**（`specs[0].transport`，不是「配置」）
2. **他敲的是什么？**（把原话带上，不要让用户自己回想）

这件事在 `cpe-mini` 里是专门的一课，现在先养成习惯。

## 6. 动手任务

改 `src/main.rs`，让它真的能用：

```bash
cargo run -- add "学习 Rust"
cargo run -- list
cargo run -- done 1
cargo run -- rm 2
```

要求：
1. `main` 里只有参数读取 + `match run(...)`
2. `run` 返回 `Result<String, String>`
3. 参数缺失、编号不是数字、命令不认识 —— 三种情况都给清楚的中文提示，退出码为 1
4. 任何输入都不能出现 `panicked at`

（数据这一课还是存在内存里，跑完就没了。下一课解决。）

## 7. 验收

```bash
cargo run -- done abc ; echo "退出码=$?"
cargo run -- 飞天       ; echo "退出码=$?"
cargo run -- add        ; echo "退出码=$?"
```

- [ ] 三条都输出中文错误提示，退出码都是 1
- [ ] 没有任何一条出现 `panicked at`
- [ ] 能解释 `?` 帮你省掉了什么

## 8. 常见错误

```
error[E0277]: the `?` operator can only be used in a function that returns `Result`
```
→ 你在 `main` 里用了 `?`。把逻辑搬进返回 `Result` 的函数。

```
error[E0308]: mismatched types  expected `Result<...>`, found `()`
```
→ 函数声明了返回 `Result`，但某条路径上没返回东西。补 `Ok(())`。

参考答案：`solutions/lesson09.md`

---

## 练习：让测试替你验收

```bash
cargo test --example ex09
```

`exercises/ex09_*.rs` 里有 4 道小题（5 个测试），每道题的函数体都是 `todo!()`，你把它换成实现。

- **一次只做一道**，做完就跑一次。红变绿的那一下就是你学会了
- 失败信息会指到具体哪一行、期望什么、实际什么
- 全绿了这一课才算过 —— 上面的验收清单靠自觉，测试不会放水

答案：`solutions/exercises/ex09.rs`（先自己写 15 分钟）。想看答案跑起来什么样：
`cargo test --example ans09`。
