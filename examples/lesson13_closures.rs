//! 第 13 课：闭包与迭代器 —— 把 for 循环换成一句话
//!
//!     cargo run --example lesson13_closures
//!
//! 前 12 课你写的都是 `for`。真实项目里更常见的是这个：
//!
//!     tasks.iter().filter(|t| !t.done).count()
//!
//! 这一课讲那个 `|t| ...` 是什么，以及 `.iter()` 后面能接什么。

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
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
            title: "练习函数".into(),
            done: false,
        },
        Task {
            id: 3,
            title: "搞懂所有权".into(),
            done: false,
        },
    ];

    println!("=== 1. 闭包就是一个没名字的函数 ===\n");

    // 这两个东西是一回事：
    fn is_done_fn(t: &Task) -> bool {
        t.done
    }
    let is_done_closure = |t: &Task| t.done;

    println!("函数：  {}", is_done_fn(&tasks[0]));
    println!("闭包：  {}", is_done_closure(&tasks[0]));

    // 闭包的类型标注几乎总是能省掉，因为编译器从上下文推得出来：
    let is_done = |t: &&Task| t.done;
    println!("省掉标注：{}", is_done(&&tasks[0]));

    println!("\n=== 2. 「闭」在哪里：它能抓住外面的变量 ===\n");

    let keyword = "所有权";
    // 这个闭包用到了外面的 keyword —— 函数做不到这件事
    let matches = |t: &Task| t.title.contains(keyword);

    for t in &tasks {
        println!("  {} 包含「{keyword}」？{}", t.title, matches(t));
    }

    println!("\n=== 3. for 循环 → 迭代器 ===\n");

    // 老写法
    let mut count = 0;
    for t in &tasks {
        if !t.done {
            count += 1;
        }
    }
    println!("for 循环数出来：   {count}");

    // 新写法。读法是从左往右：「拿到每一条 → 只留没做完的 → 数一数」
    let count = tasks.iter().filter(|t| !t.done).count();
    println!("迭代器数出来：     {count}");

    println!("\n=== 4. 五个最常用的 ===\n");

    // map：每一条变成别的东西
    let titles: Vec<String> = tasks.iter().map(|t| t.title.clone()).collect();
    println!("map      标题列表：{titles:?}");

    // filter：只留符合条件的
    let pending: Vec<&Task> = tasks.iter().filter(|t| !t.done).collect();
    println!("filter   未完成的：{} 条", pending.len());

    // find：找第一个，找不到就是 None（第 08 课那个 Option）
    let first = tasks.iter().find(|t| !t.done);
    println!("find     第一条未完成：{:?}", first.map(|t| &t.title));

    // sum：加起来
    let total: usize = tasks.iter().map(|t| t.title.chars().count()).sum();
    println!("sum      标题总字数：{total}");

    // any / all：有没有 / 是不是全都
    println!("any      有没做完的吗？{}", tasks.iter().any(|t| !t.done));
    println!("all      全做完了吗？  {}", tasks.iter().all(|t| t.done));

    println!("\n=== 5. 最容易踩的坑：迭代器是懒的 ===\n");

    // 这一行什么都不会发生 —— 没有 collect / count / sum，它根本不动
    let _lazy = tasks.iter().map(|t| {
        println!("  （如果你看见这行，说明它跑了）");
        t.id
    });
    println!("上面定义了一个 map，但什么都没打印 —— 因为没人要结果。");

    let ids: Vec<u32> = tasks
        .iter()
        .map(|t| {
            println!("  map 正在处理 id={}", t.id);
            t.id
        })
        .collect(); // ← 加上 collect，它才动
    println!("加了 collect 之后：{ids:?}");

    println!("\n=== 6. collect 要你说清楚收成什么 ===\n");

    // 下面这行编译不过：collect 不知道要收成 Vec 还是别的
    //     let ids = tasks.iter().map(|t| t.id).collect();
    //
    // 两种写法任选一种告诉它：
    let a: Vec<u32> = tasks.iter().map(|t| t.id).collect();
    let b = tasks.iter().map(|t| t.id).collect::<Vec<u32>>();
    println!("写在变量上：{a:?}");
    println!("写在 collect 上：{b:?}");

    println!("\n=== 7. 为什么值得学 ===\n");
    println!("真实项目 cpe-test 里 .iter() 出现 841 次、.map( 609 次。");
    println!("不认识这些写法，那份代码你一行都读不下去。");
}
