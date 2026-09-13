//! 第 15 课：trait 入门 —— 让你自己的类型也会那些「内置」的本事
//!
//!     cargo run --example lesson15_trait
//!
//! `println!("{}", 3)` 能打印数字，`"a" < "b"` 能比字符串。
//! 这些本事不是语言内置的，是这些类型**实现了对应的 trait**。
//! 你的类型实现同一个 trait，就有同样的本事。

use std::fmt;

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

// ---------------------------------------------------------------------------
// 1. Display：让 {} 能打印它
// ---------------------------------------------------------------------------

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mark = if self.done { "✓" } else { " " };
        write!(f, "[{mark}] {}  {}", self.id, self.title)
    }
}

// ---------------------------------------------------------------------------
// 2. 派生来的比较：声明顺序就是排序顺序
// ---------------------------------------------------------------------------

/// **变体的声明顺序就是它们的大小顺序**：High < Medium < Low。
///
/// 把 `Low` 挪到最前面，所有用到排序的地方行为立刻就变了 ——
/// 而且不会有任何编译错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    High,
    Medium,
    Low,
}

// ---------------------------------------------------------------------------
// 3. From：? 号自动转错误，靠的就是它
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum AppError {
    BadNumber(String),
    Empty,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadNumber(s) => write!(f, "不是个数字：{s}"),
            AppError::Empty => write!(f, "输入是空的"),
        }
    }
}

/// 有了这个，`?` 就能把 `ParseIntError` 自动变成 `AppError`。
impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::BadNumber(e.to_string())
    }
}

fn parse_id(s: &str) -> Result<u32, AppError> {
    if s.is_empty() {
        return Err(AppError::Empty);
    }
    // s.parse() 返回的是 Result<u32, ParseIntError>，
    // 但这个函数要返回 AppError —— ? 号在这里悄悄调了 From::from
    let id: u32 = s.parse()?;
    Ok(id)
}

// ---------------------------------------------------------------------------
// 4. 泛型：一个函数吃所有「会打印」的东西
// ---------------------------------------------------------------------------

/// `T: fmt::Display` 读作「任何一个实现了 Display 的类型」。
fn print_all<T: fmt::Display>(label: &str, items: &[T]) {
    println!("{label}：");
    for item in items {
        println!("    {item}");
    }
}

fn main() {
    let tasks = vec![
        Task {
            id: 1,
            title: "学习变量".into(),
            done: true,
        },
        Task {
            id: 2,
            title: "搞懂 trait".into(),
            done: false,
        },
    ];

    println!("=== 1. Display：{{}} 能打印你的类型了 ===\n");

    println!("{}", tasks[0]);
    println!("{}", tasks[1]);

    // 白送的好处：实现了 Display，to_string() 就自动有了
    let s: String = tasks[0].to_string();
    println!("\nto_string() 也一起有了：{s:?}");

    println!("\n{{}} 和 {{:?}} 的分工：");
    println!("    {{}}   给人看：{}", tasks[0]);
    println!("    {{:?}} 给你自己调试看：{:?}", tasks[0]);

    println!("\n=== 2. 派生比较：声明顺序就是排序顺序 ===\n");

    let mut ps = vec![Priority::Low, Priority::High, Priority::Medium];
    ps.sort();
    println!("排完序：{ps:?}");
    println!("High < Low ？{}", Priority::High < Priority::Low);
    println!();
    println!("这件事很容易忘：enum 的**声明顺序**决定了排序结果。");
    println!("把变体挪个位置，排序行为立刻变，而且编译器一个字都不会说。");
    println!();
    println!("cpe-mini 的 src/compare.rs 里 DeltaKind 就是这样 ——");
    println!("「判定变坏」声明在最前面，所以对比报告里它排在最上面。");

    println!("\n=== 3. From：? 号自动转错误 ===\n");

    for input in ["42", "四十二", ""] {
        match parse_id(input) {
            Ok(id) => println!("  {input:?} → {id}"),
            Err(e) => println!("  {input:?} → 出错了：{e}"),
        }
    }
    println!();
    println!("第 09 课里 ? 号能用，靠的就是这个 —— 它会调 From::from");
    println!("把下层的错误转成本函数要返回的错误类型。");

    println!("\n=== 4. 泛型：一个函数吃所有会打印的东西 ===\n");

    print_all("任务", &tasks);
    print_all("数字", &[1, 2, 3]);
    print_all("字符串", &["甲", "乙"]);
    println!();
    println!("同一个函数，三种完全不同的类型。要求只有一条：实现了 Display。");

    println!("\n=== 5. 最常用的几个 trait ===\n");
    println!("  Debug        {{:?}} 能打印        几乎总是 derive");
    println!("  Clone        .clone() 能复制     derive");
    println!("  Copy         赋值不搬走          只有小的、纯数据的类型才配");
    println!("  PartialEq    == 能比             derive");
    println!("  Ord          能排序              derive，注意声明顺序");
    println!("  Display      {{}} 能打印          **要自己写**，因为只有你知道给人看该长什么样");
    println!("  From         ? 号自动转          自己写");
    println!("  Default      Default::default()  derive 或自己写");
}
