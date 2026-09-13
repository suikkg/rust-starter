//! 第 03 课：判断与循环
//!
//!     cargo run --example lesson03_flow
//!
//! if 做判断，for 做遍历，while 做"条件满足就一直做"。

fn main() {
    let done = true;

    // if 判断。注意条件不用写括号，但大括号不能省。
    if done {
        println!("[✓] 学习 Rust");
    } else {
        println!("[ ] 学习 Rust");
    }

    // if 也可以当成"值"用，整个 if 表达式的结果赋给变量。
    // 注意 { "✓" } 里面没有分号 —— 没有分号才是"返回这个值"。
    let mark = if done { "✓" } else { " " };
    println!("[{mark}] 学习 Rust");

    println!("---");

    // for 循环：把 1、2、3 依次拿出来。1..=3 含 3，1..3 不含 3。
    for i in 1..=3 {
        println!("第 {i} 条任务");
    }

    println!("---");

    // while 循环：条件为真就一直转
    let mut left = 3;
    while left > 0 {
        println!("还剩 {left} 条没做");
        left -= 1; // ← 忘了这一行就是死循环，按 Ctrl+C 停
    }
    println!("全部做完了");

    println!("---");

    // else if：分支多于两个
    for score in [100, 60, 10] {
        let level = if score >= 90 {
            "优"
        } else if score >= 60 {
            "及格"
        } else {
            "不及格"
        };
        println!("{score} 分 → {level}");
    }

    println!("---");

    // 条件组合：&& 两个都要成立，|| 至少一个，! 取反
    let id = 3;
    let finished = false;
    if !finished && id > 0 {
        println!("id={id} 是一条还没做的有效任务");
    }

    println!("---");

    // loop + break：先转起来，出口自己定
    let mut n = 0;
    loop {
        n += 1;
        if n >= 3 {
            break; // 整个循环到此为止
        }
    }
    println!("loop 转了 {n} 轮");

    // continue：跳过这一轮，进下一轮
    print!("1..=5 里跳过 2、遇到 4 就停：");
    for i in 1..=5 {
        if i == 2 {
            continue;
        }
        if i == 4 {
            break;
        }
        print!("{i} ");
    }
    println!();

    println!("---");

    // 数数：这一课动手任务要用的模式。
    // 三步——循环外面定义、循环里面累加、循环外面使用。
    let states = [true, false, true];
    let mut done_count = 0; // ① 循环外面
    for task_done in states {
        if task_done {
            done_count += 1; // ② 循环里面
        }
    }
    println!("共 {} 项，已完成 {done_count} 项。", states.len()); // ③ 循环外面

    // 【动手】1. 把 1..=3 改成 1..3，运行看少了哪一行，想清楚为什么。
    //
    //        2. 把 `let mut done_count = 0;` 挪到 for 的大括号**里面**，
    //           看编译器说什么。报错是 E0425 cannot find value ——
    //           变量活在它所在的那对大括号里，出了就没了。
    //
    //        3. 把 while 里的 `left -= 1;` 注释掉，运行。
    //           屏幕开始刷屏就按 Ctrl+C。这就是死循环长什么样。
}
