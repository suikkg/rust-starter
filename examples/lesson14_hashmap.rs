//! 第 14 课：HashMap —— 按名字找，不是按位置找
//!
//!     cargo run --example lesson14_hashmap
//!
//! `Vec` 是「第 3 个」，`HashMap` 是「叫 tag 的那个」。
//! 真实项目 cpe-test 里 HashMap 出现 111 次。

use std::collections::{BTreeMap, HashMap};

fn main() {
    println!("=== 1. 建一个，放东西，取东西 ===\n");

    let mut counts: HashMap<String, u32> = HashMap::new();
    counts.insert("工作".to_string(), 3);
    counts.insert("学习".to_string(), 5);

    // get 返回 Option —— 又是第 08 课那个东西
    println!("学习：{:?}", counts.get("学习"));
    println!("买菜：{:?}", counts.get("买菜"));

    // 取不到就给个默认值
    println!(
        "买菜（默认 0）：{}",
        counts.get("买菜").copied().unwrap_or(0)
    );

    // 这样写会崩：
    //     println!("{}", counts["买菜"]);
    // 和第 05 课 Vec 的 [i] 一样，方括号是「我保证有」的写法。

    println!("\n=== 2. 计数：entry().or_insert() ===\n");

    let tags = ["工作", "学习", "工作", "生活", "工作", "学习"];

    // 笨写法
    let mut a: HashMap<&str, u32> = HashMap::new();
    for tag in tags {
        if let Some(n) = a.get_mut(tag) {
            *n += 1;
        } else {
            a.insert(tag, 1);
        }
    }

    // 惯用写法。读作：「找 tag 这一格，没有就先放个 0 进去，然后加一」
    let mut b: HashMap<&str, u32> = HashMap::new();
    for tag in tags {
        *b.entry(tag).or_insert(0) += 1;
    }

    println!("两种写法结果一样吗？{}", a == b);
    println!("工作出现 {} 次", b["工作"]);

    println!("\n=== 3. 分组：entry().or_default().push() ===\n");

    let items = [("工作", "写周报"), ("学习", "看 Rust"), ("工作", "开会")];

    let mut grouped: HashMap<&str, Vec<&str>> = HashMap::new();
    for (tag, title) in items {
        grouped.entry(tag).or_default().push(title);
    }
    // or_default() 是 or_insert(Vec::new()) 的省事写法

    println!("工作下面有：{:?}", grouped["工作"]);

    println!("\n=== 4. 最大的坑：遍历顺序是乱的 ===\n");

    let mut m1: HashMap<&str, u32> = HashMap::new();
    let mut m2: HashMap<&str, u32> = HashMap::new();
    for (i, k) in ["a", "b", "c", "d", "e", "f"].iter().enumerate() {
        m1.insert(k, i as u32);
        m2.insert(k, i as u32);
    }

    let order1: Vec<&str> = m1.keys().copied().collect();
    let order2: Vec<&str> = m2.keys().copied().collect();
    println!("第一个 map 的顺序：{order1:?}");
    println!("第二个 map 的顺序：{order2:?}");
    println!(
        "内容完全一样，顺序{}",
        if order1 == order2 {
            "碰巧也一样"
        } else {
            "不一样！"
        }
    );

    println!();
    println!("所以：**任何要给人看、要存文件、要对比的输出，都必须先排序。**");
    println!("不排序的话，同一份数据跑两次，报告里的行顺序可能不同 ——");
    println!("拿去做 diff 就全是噪声。");

    println!("\n=== 5. 要顺序就用 BTreeMap ===\n");

    let mut bt: BTreeMap<&str, u32> = BTreeMap::new();
    for (i, k) in ["d", "a", "f", "b"].iter().enumerate() {
        bt.insert(k, i as u32);
    }
    println!(
        "BTreeMap 永远按 key 排好：{:?}",
        bt.keys().collect::<Vec<_>>()
    );

    println!();
    println!("用哪个？");
    println!("  只是查一下           → HashMap（快一点）");
    println!("  结果要输出 / 要对比   → BTreeMap（省掉一次排序，也省掉一次忘记排序）");
    println!();
    println!("cpe-mini 的 src/compare.rs 用的就是 BTreeMap，理由正是这个。");

    println!("\n=== 6. 排序输出 ===\n");

    let mut keys: Vec<&&str> = b.keys().collect();
    keys.sort();
    for k in keys {
        println!("  {k}: {}", b[k]);
    }
}
