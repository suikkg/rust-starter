//! 第 14 课练习的**答案**：HashMap
//!
//!     cargo test --example ans14
//!
//! 先自己写 15 分钟。骨架在 `exercises/ex14_hashmap.rs`。
#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    tag: String,
}

/// 【1】每个标签下面有几条。
///
/// 提示：`*map.entry(k).or_insert(0) += 1`
///
/// `entry` 读作「找 k 这一格」：有就给你，没有就按你说的先建一个。
/// 比「先 get 看看有没有，没有再 insert」少一次查找，也少一次写错的机会。
fn count_by_tag(tasks: &[Task]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for t in tasks {
        *counts.entry(t.tag.clone()).or_insert(0) += 1;
    }
    counts
}

/// 【2】每个标签下面都有哪些标题。
///
/// 提示：`map.entry(k).or_default().push(v)`
///
/// `or_default()` 对 `Vec` 就是「没有就给个空 Vec」。
/// 每个标签里的标题，顺序要和 `tasks` 里一样。
fn group_titles_by_tag(tasks: &[Task]) -> HashMap<String, Vec<String>> {
    let mut grouped: HashMap<String, Vec<String>> = HashMap::new();
    for t in tasks {
        grouped
            .entry(t.tag.clone())
            .or_default()
            .push(t.title.clone());
    }
    grouped
}

/// 【3】查某个标签有几条。**这个标签不存在就返回 0。**
///
/// 提示：`.get(tag).copied().unwrap_or(0)`
///
/// 不要写 `counts[tag]` —— 和第 05 课 `Vec` 的 `[i]` 一样，键不存在会崩溃。
///
/// 顺便想一想：这里「不存在 = 0」是对的（没有这个标签，就是零条）。
/// 但不是所有场合都这样 —— 隔壁 cpe-mini 第 04 课讲的正相反：
/// 「没测到速率」和「速率是 0」必须分开。**默认值对不对，要看那个 0 有没有意义。**
fn count_of(counts: &HashMap<String, usize>, tag: &str) -> usize {
    // get 给的是 Option<&usize>；copied() 把它变成 Option<usize>，
    // 这样 unwrap_or(0) 才对得上类型。
    counts.get(tag).copied().unwrap_or(0)
}

/// 【4】把所有标签按名字排好返回。
///
/// 提示：`.keys().cloned().collect()` 然后 `.sort()`
///
/// **`.keys()` 出来的顺序是乱的**，而且同一份数据建两次 map，
/// 两次的顺序可能都不一样。直接 collect 就交差的话，
/// 这个函数每次跑的结果都可能不同 —— 拿去写文件、做 diff 全是噪声。
fn sorted_tags(counts: &HashMap<String, usize>) -> Vec<String> {
    let mut tags: Vec<String> = counts.keys().cloned().collect();
    tags.sort();
    tags
}

/// 【5】条数最多的标签是哪个。空 map 返回 `None`。
///
/// **平局时返回名字排在前面的那个。**
///
/// 这一题是陷阱：直接 `.max_by_key(|(_, n)| *n)` 在平局时返回哪个，
/// 取决于遍历顺序 —— 也就是说**不确定**。同一份数据跑两次可能给出不同答案，
/// 而且大概率在你本地怎么跑都对，上了别人的机器才出问题。
///
/// 提示：先排序，或者在比较里显式地把名字也算进去。
fn busiest_tag(counts: &HashMap<String, usize>) -> Option<String> {
    // max_by_key 在平局时返回**最后一个**最大值——而「最后一个」取决于
    // 遍历顺序，也就是不确定。所以把名字也写进比较里：
    // 先比条数（大的赢），条数相同时比名字（小的赢，所以取反）。
    counts
        .iter()
        .max_by_key(|(tag, n)| (**n, std::cmp::Reverse((*tag).clone())))
        .map(|(tag, _)| tag.clone())
}

fn main() {
    println!("这是答案，用 cargo test --example ans14 跑");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    fn 样例() -> Vec<Task> {
        let mk = |id, title: &str, tag: &str| Task {
            id,
            title: title.into(),
            tag: tag.into(),
        };
        vec![
            mk(1, "写周报", "工作"),
            mk(2, "看 Rust", "学习"),
            mk(3, "开会", "工作"),
            mk(4, "买菜", "生活"),
            mk(5, "练习题", "学习"),
            mk(6, "改代码", "工作"),
        ]
    }

    #[test]
    fn 按标签计数() {
        let c = count_by_tag(&样例());
        assert_eq!(c.len(), 3);
        assert_eq!(c["工作"], 3);
        assert_eq!(c["学习"], 2);
        assert_eq!(c["生活"], 1);
    }

    #[test]
    fn 空列表是空map() {
        assert!(count_by_tag(&[]).is_empty());
    }

    #[test]
    fn 按标签分组() {
        let g = group_titles_by_tag(&样例());
        // 组内顺序要和原列表一致
        assert_eq!(g["工作"], vec!["写周报", "开会", "改代码"]);
        assert_eq!(g["生活"], vec!["买菜"]);
    }

    #[test]
    fn 查一个存在的标签() {
        let c = count_by_tag(&样例());
        assert_eq!(count_of(&c, "学习"), 2);
    }

    #[test]
    fn 查一个不存在的标签返回零而不是崩溃() {
        let c = count_by_tag(&样例());
        assert_eq!(count_of(&c, "旅游"), 0);
        assert_eq!(count_of(&HashMap::new(), "任何"), 0);
    }

    #[test]
    fn 标签按名字排好() {
        let c = count_by_tag(&样例());
        // 按 Unicode 码位排：学(5B66) < 工(5DE5) < 生(751F)
        assert_eq!(sorted_tags(&c), vec!["学习", "工作", "生活"]);
    }

    #[test]
    fn 排序结果跑两次一样() {
        // 同一份数据建两个 map，两个 map 的遍历顺序很可能不同。
        // 排过序的话，这个断言永远成立。
        let a = count_by_tag(&样例());
        let b = count_by_tag(&样例());
        assert_eq!(sorted_tags(&a), sorted_tags(&b));
    }

    #[test]
    fn 最忙的标签() {
        let c = count_by_tag(&样例());
        assert_eq!(busiest_tag(&c), Some("工作".to_string()));
        assert_eq!(busiest_tag(&HashMap::new()), None);
    }

    #[test]
    fn 平局时按名字取前面那个() {
        // 三个标签都是 1 条。没有显式的平局规则，返回哪个全看运气。
        let mut c = HashMap::new();
        c.insert("生活".to_string(), 1);
        c.insert("学习".to_string(), 1);
        c.insert("工作".to_string(), 1);
        assert_eq!(busiest_tag(&c), Some("学习".to_string()));
    }
}
