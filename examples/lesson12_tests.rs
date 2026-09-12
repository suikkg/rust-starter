//! 第 12 课：写测试
//!
//! 这个文件本身是给 `cargo test` 跑的，不是用来 run 的：
//!
//!     cargo test --example lesson12_tests
//!
//! 测试的价值：改完代码敢按回车。手工点一遍会累会忘，测试不会。

fn next_id(ids: &[u32]) -> u32 {
    ids.iter().copied().max().unwrap_or(0) + 1
}

fn pending_count(done_flags: &[bool]) -> usize {
    done_flags.iter().filter(|d| !**d).count()
}

fn main() {
    println!("这一课请运行：cargo test --example lesson12_tests");
    println!("next_id([1,2]) = {}", next_id(&[1, 2]));
    println!(
        "pending_count([true,false]) = {}",
        pending_count(&[true, false])
    );
}

// #[cfg(test)] = 只有跑测试时才编译这一块，正式程序里不带它。
#[cfg(test)]
mod tests {
    use super::*; // 把上面的函数引进来

    #[test]
    fn 空列表从1开始编号() {
        assert_eq!(next_id(&[]), 1);
    }

    #[test]
    fn 编号取最大值加一() {
        assert_eq!(next_id(&[1, 2, 5]), 6);
        // 就算顺序乱了也要对
        assert_eq!(next_id(&[5, 1, 2]), 6);
    }

    #[test]
    fn 统计未完成条数() {
        assert_eq!(pending_count(&[]), 0);
        assert_eq!(pending_count(&[true, true]), 0);
        assert_eq!(pending_count(&[true, false, false]), 2);
    }

    // 【动手】故意把 next_id 里的 + 1 删掉，跑 cargo test，
    // 看失败信息长什么样（它会告诉你 left 和 right 分别是多少），再改回来。
}
