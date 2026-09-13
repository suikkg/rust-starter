//! 第 15 课练习：trait 入门
//!
//!     cargo test --example ex15
//!
//! 四道题：Display、派生排序、From + `?`、泛型。
//!
//! **先做第 2 题。** 它不做的话整个文件**编译不过**（不是测试红，是编译错误），
//! 别的题一道也跑不了。这是 Rust 的常态：编译错误永远排在测试失败前面。
//! 好消息是那条报错会直接把该加的 derive 写给你看。
//!
//! 答案：`solutions/exercises/ex15.rs`
#![allow(dead_code, unused_variables)]

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

/// 【1】让 `{}` 能打印 `Task`。
///
/// 格式：`[✓] 1  学习变量`（做完是 `✓`，没做完是一个空格）
///
/// 两个空格隔开编号和标题。
///
/// 提示：函数体里用 `write!(f, "...")`，最后**不要**加分号 ——
/// `write!` 的返回值就是这个函数要返回的 `fmt::Result`。
///
/// 实现了它，`task.to_string()` 会自动一起有 —— 白送的。
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// 【2】给这个 enum 加上派生，让它**能排序**，而且顺序是 High < Medium < Low。
///
/// 提示：`#[derive(...)]` 里要加四个：`PartialEq, Eq, PartialOrd, Ord`。
/// （`Ord` 要求 `Eq`，`PartialOrd` 要求 `PartialEq` —— 编译器会告诉你少了哪个。）
///
/// **变体的声明顺序就是大小顺序**，所以下面的顺序不用改。
/// 记住这件事：哪天有人把 `Low` 挪到最前面，所有排序的行为都会变，
/// 而编译器一个字都不会说。
#[derive(Debug, Clone, Copy)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, PartialEq)]
pub enum AppError {
    /// 编号不是数字，把原话带上
    BadNumber(String),
    /// 输入是空的
    Empty,
}

/// 【3a】让 `?` 能把 `ParseIntError` 自动转成 `AppError::BadNumber`。
///
/// `BadNumber` 里装的是 `e.to_string()`。
///
/// 提示：`impl From<A> for B` 要写的方法是 `fn from(e: A) -> Self`。
impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        todo!()
    }
}

/// 【3b】把字符串解析成编号。
///
/// - `""`     → `Err(AppError::Empty)`
/// - `"四十二"` → `Err(AppError::BadNumber(...))`
/// - `"42"`   → `Ok(42)`
///
/// **要求用 `?`，不要用 `match` 手动转。** 上面那个 `From` 写对了，
/// `s.parse::<u32>()?` 就能直接用 —— `?` 会自己调 `From::from`。
///
/// 这就是第 09 课里 `?` 能跨类型工作的原因。
fn parse_id(s: &str) -> Result<u32, AppError> {
    todo!()
}

/// 【4】把一串**任何能打印的东西**连成一行，用 `、` 隔开。
///
/// - `join_all(&[1, 2, 3])`      → `"1、2、3"`
/// - `join_all(&["甲", "乙"])`    → `"甲、乙"`
/// - `join_all::<Task>(&[])`     → `""`
///
/// 提示：签名里的 `T: fmt::Display` 读作「任何实现了 Display 的类型」。
/// 函数体里 `.map(|x| x.to_string())` 就能用 —— 因为 Display 白送了 to_string。
fn join_all<T: fmt::Display>(items: &[T]) -> String {
    todo!()
}

fn main() {
    println!("这是练习题，用 cargo test --example ex15 跑");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    fn 做完的() -> Task {
        Task {
            id: 1,
            title: "学习变量".into(),
            done: true,
        }
    }
    fn 没做的() -> Task {
        Task {
            id: 2,
            title: "搞懂 trait".into(),
            done: false,
        }
    }

    #[test]
    fn 花括号能打印了() {
        assert_eq!(format!("{}", 做完的()), "[✓] 1  学习变量");
        assert_eq!(format!("{}", 没做的()), "[ ] 2  搞懂 trait");
    }

    #[test]
    fn toString是白送的() {
        // 实现了 Display 之后，to_string() 自动就有了，一行都不用写
        assert_eq!(做完的().to_string(), "[✓] 1  学习变量");
    }

    #[test]
    fn 能排序而且High最小() {
        let mut ps = vec![Priority::Low, Priority::High, Priority::Medium];
        ps.sort();
        assert_eq!(format!("{ps:?}"), "[High, Medium, Low]");
    }

    #[test]
    fn 声明顺序就是大小顺序() {
        assert!(Priority::High < Priority::Medium);
        assert!(Priority::Medium < Priority::Low);
    }

    #[test]
    fn 解析正常编号() {
        assert_eq!(parse_id("42"), Ok(42));
    }

    #[test]
    fn 空串是Empty() {
        assert_eq!(parse_id(""), Err(AppError::Empty));
    }

    #[test]
    fn 不是数字的时候把原话带上() {
        match parse_id("四十二") {
            Err(AppError::BadNumber(msg)) => {
                // 报错内容来自 ParseIntError，具体措辞不该钉死，
                // 但它必须非空 —— 否则用户看到的是「出错了」三个字
                assert!(!msg.is_empty(), "错误信息不能是空的");
            }
            other => panic!("应该是 BadNumber，实际是 {other:?}"),
        }
    }

    #[test]
    fn 泛型函数吃三种类型() {
        assert_eq!(join_all(&[1, 2, 3]), "1、2、3");
        assert_eq!(join_all(&["甲", "乙"]), "甲、乙");
        assert_eq!(
            join_all(&[做完的(), 没做的()]),
            "[✓] 1  学习变量、[ ] 2  搞懂 trait"
        );
    }

    #[test]
    fn 空列表连出空串() {
        let empty: [Task; 0] = [];
        assert_eq!(join_all(&empty), "");
    }
}
