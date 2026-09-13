# Rust 语法速查卡

每天开工前扫一眼，写代码时忘了就翻这里。**只收这 15 课用得到的**，
不全面是故意的 —— 速查卡一长就没人看了。

括号里是「这在第几课讲」。

---

## 变量（02）

```rust
let x = 5;              // 不可变，默认就是这个
let mut x = 5;          // 可变，要改才加 mut
let x: u32 = 5;         // 显式写类型
const MAX: u32 = 100;   // 常量，必须写类型，名字全大写
```

**默认不可变**是 Rust 的立场：能不改就不改，改动的地方越少越好定位问题。

## 基本类型（02）

| 类型 | 是什么 | 用在哪 |
|---|---|---|
| `i32` | 有符号整数 | 可能是负数的计数 |
| `u32` | 无符号整数 | 编号、条数 —— **减成负数会崩溃** |
| `usize` | 下标专用整数 | `Vec` 的长度和下标 |
| `f64` | 小数 | 比率、速率 |
| `bool` | `true` / `false` | 状态 |
| `char` | 一个字符 | `'a'`、`'中'` |
| `String` | own 的字符串 | 要存下来、要改 |
| `&str` | 借来的字符串 | **只读的参数一律用它** |

类型转换要显式写：

```rust
let n: u32 = 3;
let f = n as f64 / 4.0;     // 先转再除；n / 4 是整数除法，得 0
```

### 数字上的两个坑（02）

```rust
3 / 4                      // 0  ← 整数除法把小数扔掉，不是四舍五入
done as f64 / total as f64 // 0.75 ✓  先转再除
(done / total) as f64      // 0.00 ✗  先除再转，已经晚了

let c = a as f64 + b;      // 类型不会自动转，要自己写 as
```

「完成率永远是 0」十次有九次是这个。**编译器不会提醒**——它是合法代码。

## 判断与循环（03）

```rust
if done { } else if overdue { } else { }

let mark = if done { "✓" } else { " " };   // 当值用；else 不能省，里面不加分号

a == b   a != b   a > b   a >= b          // == 是两个等号
a && b   a || b   !a                      // 都成立 / 至少一个 / 取反

for i in 1..=3 { }        // 1 2 3     ..= 含末尾
for i in 1..3  { }        // 1 2       ..  不含
for t in &titles { }      // 遍历集合，记得加 &

while left > 0 { left -= 1; }   // 忘了 -= 就是死循环，Ctrl+C 停

loop {                    // 无条件转，出口靠 break
    if done { break; }
}

continue;                 // 跳过这一轮
break;                    // 整个循环到此为止
```

**数数的三步**：循环**外面**定义 `let mut n = 0;` → 循环里面 `n += 1;` →
循环外面用它。定义写进循环里，出了大括号就不存在（E0425）。

## 函数（04）

```rust
fn show(id: u32, title: &str) { }               // 无返回值
fn summary(total: u32) -> String {
    format!("共 {total} 项")                     // 最后一行不带分号 = 返回它
}
```

- 参数**必须**写类型，Rust 不推断参数
- 有返回值就写 `-> 类型`
- 多写一个分号 = 什么都不返回，报错是 `expected String, found ()`

## 字符串（04 / 06）

```rust
let s = String::from("abc");
let s = "abc".to_string();
let s = format!("{a} 和 {b}");        // 拼串首选

s.len()                  // 字节数！一个汉字是 3
s.chars().count()        // 字符数
s.trim()                 // 去首尾空白
s.is_empty()
s.starts_with("前缀")
s.contains("中")
s.to_uppercase()
s.split(' ')             // 切成迭代器
s.split_once(' ')        // Option<(&str, &str)>，切不到是 None
s.find(' ')              // Option<usize>
s.replace("a", "b")
s.push_str("追加")        // 要 mut
&s                       // String → &str（自动的）
```

比长短、截断显示、判断「超过 N 个字」，**一律 `chars().count()`**。
用 `len()` 的话四个汉字（12）会被判成比八个字母（8）还长。
中文也不能 `s[0..3]` 这样切，会切到半个字然后崩。

## Vec（05）

```rust
let mut v: Vec<String> = Vec::new();
let v = vec![1, 2, 3];

v.push(x);
v.len()  v.is_empty()
v.get(i)                 // 返回 Option，越界不崩溃 ← 首选
v[i]                     // 越界直接 panic
v.remove(i)              // 删第 i 条并返回它
v.iter()                 // 遍历借用
v.iter().filter(|t| t.done).count()
v.iter().map(|t| t.id).max()
v.iter().find(|t| t.id == 3)         // 返回 Option
v.iter().position(|t| t.id == 3)     // 返回 Option<usize>
```

## 所有权（06）

三条规则：

1. 每个值有且只有一个 owner
2. owner 离开作用域，值就被丢掉
3. 同一时刻，要么有**多个不可变借用**，要么有**一个可变借用**，不能同时有

```rust
let a = String::from("x");
let b = a;              // move：a 之后不能再用了
let b = a.clone();      // 复制一份，两个都能用（有代价）
let b = &a;             // 借用：a 还归自己

fn f(s: &str) { }       // 只读 ← 默认先想它
fn f(s: &mut String) { }// 要改调用方那个值
fn f(s: String) { }     // 把值吃掉，调用方之后用不了
```

**参数类型怎么选**：

| 想做什么 | 参数写法 |
|---|---|
| 只读一个字符串 | `&str` |
| 只读一个列表 | `&[T]` |
| 要改列表（push/remove） | `&mut Vec<T>` |
| 只改元素内容，不增删 | `&mut [T]` |
| 要把值留下 | `T` |

### 动原件 还是 造新件

| 方法 | 动原件？ | 返回 |
|---|---|---|
| `s.push_str("x")` / `s.push('x')` / `s.clear()` | **动** | 不返回 |
| `s.to_uppercase()` / `to_lowercase()` / `replace()` | 不动 | 新 `String` |
| `s.trim()` | 不动 | `&str` |

```rust
fn add_suffix(t: &mut String, x: &str)   // 改原件：&mut，不返回
fn shout(t: &str) -> String              // 造新件：&，返回新的
```

**看签名就知道它会不会动你的数据。**

## struct（07）

```rust
struct Task {
    id: u32,
    title: String,
    done: bool,
}

impl Task {
    fn new(id: u32, title: &str) -> Task {      // 没有 self = 关联函数，用 Task::new() 调
        Task { id, title: title.to_string(), done: false }
    }
    fn line(&self) -> String { ... }            // &self  = 只读
    fn toggle(&mut self) { self.done = !self.done; }   // &mut self = 要改
}

let t = Task::new(1, "买菜");
t.line();
```

`#[derive(Debug, Clone)]` 加在 struct 上面：`Debug` 让 `{:?}` 能打印，
`Clone` 让 `.clone()` 可用。

## enum 与 match（08）

```rust
enum Command {
    List,                   // 不带数据
    Done(u32),              // 带一个
    Add(String),
}

match cmd {
    Command::List => ...,
    Command::Done(id) => ...,
    Command::Add(title) => ...,
}
```

**`match` 必须覆盖所有分支**，漏了是编译错误 `non-exhaustive patterns`。
能不写 `_ =>` 就不写：将来加了新变体，编译器才会提醒你这里也得改。

## Option（08）

「可能没有」。Rust 没有 null，用它代替。

```rust
let found: Option<&Task> = tasks.iter().find(|t| t.id == id);

match found {
    Some(t) => println!("{}", t.title),
    None => println!("没找到"),
}

if let Some(t) = found { ... }          // 只关心一种情况时

found.unwrap_or(&default)               // 没有就用默认值
found.map(|t| t.title.clone())          // 有值才变换
found.is_some()  found.is_none()
found.unwrap()                          // None 时崩溃 ← 尽量别用
```

```rust
"42".parse::<u32>()          // Result<u32, _>
"42".parse::<u32>().ok()     // Option<u32>，不关心失败原因时用
let n: u32 = s.parse().unwrap_or(0);
```

**别用 `.unwrap()`**：那是「我保证有，没有就崩」。
用 `unwrap_or` / `match` / `if let`。

## Result 与 ?（09）

「可能失败」。

```rust
fn parse_id(s: &str) -> Result<u32, String> {
    s.parse::<u32>().map_err(|_| format!("编号要是数字，收到的是「{s}」"))
}

let id = parse_id(arg)?;        // 失败就带着错误立刻返回
```

`?` 只能用在返回 `Result` 的函数里。在 `main` 里用会报
`the ? operator can only be used in a function that returns Result` ——
把逻辑搬进一个返回 `Result` 的函数，`main` 只负责调用和报错。

```rust
result.map_err(|e| ...)         // 换个错误类型/文案
result.ok()                     // Result → Option，扔掉错误
result.unwrap_or(默认值)
result.is_ok()  result.is_err()
```

## 命令行参数（09）

```rust
let args: Vec<String> = std::env::args().skip(1).collect();   // skip(1) 跳过程序名
let cmd = args.first().map(|s| s.as_str()).unwrap_or("");

std::process::exit(1);          // 给脚本用：0 成功，非 0 出事
```

## 文件与 JSON（10）

```rust
use std::fs;
use std::path::Path;

let text = fs::read_to_string(path)?;
fs::write(path, text)?;
fs::create_dir_all(dir)?;       // 目录不存在就建
path.exists()
```

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task { ... }

let text = serde_json::to_string_pretty(&tasks)?;   // 存盘用 pretty，人能读
let tasks: Vec<Task> = serde_json::from_str(&text)?;
```

## 模块（11）

```rust
// src/lib.rs
pub mod model;
pub mod store;

// src/store.rs
use crate::model::Task;         // crate = 本项目的根
use super::model::Task;         // super = 上一层

pub fn add(...) { }             // 外面能调
fn next_id(...) { }             // 内部细节，外面调不到
```

默认私有。**先不加 `pub`，等真有别处要用了再加** —— 加得越少，
将来想改实现越自由。

## 测试（12）

```rust
#[cfg(test)]                    // 只有 cargo test 时才编译
mod tests {
    use super::*;               // 把外面的东西引进来

    #[test]
    fn 空列表从1开始编号() {
        assert_eq!(next_id(&[]), 1);
        assert!(条件, "失败时打印这句：{值}");
        assert_ne!(a, b);
    }
}
```

写在被测代码同一个文件里 = 单元测试，**能测到私有函数**。
放在 `tests/` 目录 = 集成测试，只能用 `pub` 的东西。

## 闭包与迭代器（13）

```rust
let f = |x: u32| x + 1;         // 闭包 = 没名字的函数
let keyword = "所有权";
let g = |t: &Task| t.title.contains(keyword);   // 能抓外面的变量

tasks.iter()                    // &T    原列表还能用
tasks.iter_mut()                // &mut T
tasks.into_iter()               // T     原列表被吃掉

.count()                        // 有几条
.filter(|t| !t.done)            // 只留符合的（闭包拿到的是 &&T）
.map(|t| t.title.clone())       // 每条变成别的
.find(|t| t.id == 3)            // 第一个 → Option<&T>
.any(|t| !t.done)               // 有没有 → bool
.all(|t| t.done)                // 是不是全都 → bool
.max_by_key(|t| t.title.chars().count())
.sum::<usize>()                 // 加起来
.for_each(|t| println!("{t}"))
.collect::<Vec<_>>()            // 收成 Vec

fn f(g: impl Fn(&str) -> String)    // 把闭包当参数收
```

**迭代器是懒的**：没有 `collect` / `count` / `sum` / `for_each`，它根本不动。

**`collect` 要你说清楚收成什么**：`let v: Vec<u32> = ...` 或 `.collect::<Vec<u32>>()`。

**`.map(|t| t.title)` 编译不过**：`t` 是借来的，搬不走 → `.clone()`。

## HashMap（14）

```rust
use std::collections::{HashMap, BTreeMap};

let mut m: HashMap<String, u32> = HashMap::new();
m.insert("工作".into(), 3);

m.get("工作")                    // Option<&u32>
m.get("旅游").copied().unwrap_or(0)   // 取不到给 0
m["旅游"]                        // 💥 键不存在会崩

*m.entry(k).or_insert(0) += 1;        // 计数
m.entry(k).or_default().push(v);      // 分组

m.len()  m.contains_key(k)  m.remove(k)
for (k, v) in &m { }             // ⚠️ 顺序是乱的
```

**`HashMap` 的遍历顺序每次都可能不同。** 要输出 / 存文件 / 做对比，
要么先 `keys()` 收出来 `sort()`，要么直接用 `BTreeMap`（永远按 key 排好）。

`f64` 不能当 key（`NaN != NaN`）。

## trait（15）

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]

// Display 要自己写；写了之后 to_string() 白送
use std::fmt;
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.id, self.title)   // ← 没有分号
    }
}

// From 让 ? 能跨错误类型
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}

fn f<T: fmt::Display>(x: T)     // 泛型约束
fn f(x: impl fmt::Display)      // 一样的意思
```

| trait | 给你什么 |
|---|---|
| `Debug` | `{:?}` |
| `Display` | `{}` + `.to_string()`（**要自己写**） |
| `Clone` / `Copy` | `.clone()` / 赋值不搬走 |
| `PartialEq` / `Ord` | `==` / 能排序 |
| `From` | `?` 自动转错误 |
| `Default` | `Default::default()` |

**enum 的声明顺序就是排序顺序。** 挪一个变体，所有排序行为都变，
而且编译器不会说话。

## 命令

```bash
cargo check                     # 只检查能不能编译，最快
cargo run                       # 编译并运行
cargo run -- add "任务"          # -- 后面的才是给程序的参数
cargo test                      # 跑测试
cargo test 编号                  # 只跑名字里有"编号"的测试
cargo test -- --nocapture       # 让测试里的 println! 也显示出来
cargo fmt                       # 自动排版
cargo clippy                    # 写法建议
cargo run --example lesson06_ownership    # 跑某一课的小实验
cargo test --example ex06                 # 跑某一课的练习
```

## 打印

```rust
println!("{}", x);          // 要求实现 Display
println!("{:?}", x);        // Debug，struct 上加 #[derive(Debug)]
println!("{:#?}", x);       // Debug 但换行缩进，调试大结构体用
println!("{x} 和 {y}");      // 直接写变量名（Rust 2021）
println!("{:.2}", 3.14159); // 保留两位小数
eprintln!("出错了");         // 写到 stderr，不会被 > 重定向吃掉
```

---

**找不到的东西**：`cargo doc --open` 能打开本项目和依赖的完整文档，
标准库文档在 https://doc.rust-lang.org/std/ ，搜索框直接搜方法名。
