//! 怎么显示给人看。

use crate::model::Task;

pub fn render(tasks: &[Task]) -> String {
    let mut out = String::from("我的待办清单\n\n");

    if tasks.is_empty() {
        out.push_str("（清单是空的，用 add 加一条）\n");
    } else {
        for t in tasks {
            out.push_str(&t.line());
            out.push('\n');
        }
    }

    let done = tasks.iter().filter(|t| t.done).count();
    out.push_str(&format!("\n共 {} 项，已完成 {} 项。", tasks.len(), done));
    out
}
