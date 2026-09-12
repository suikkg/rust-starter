//! 第 09 课练习的参考答案。
//!
//!     cargo test --example ans09
#![allow(dead_code)]

fn parse_id(arg: &str) -> Result<u32, String> {
    // parse 失败时给的是 ParseIntError（"invalid digit found in string"），
    // 那句话对用户没用。这里换成一句他看得懂的，并且**把原文带上**。
    arg.parse::<u32>()
        .map_err(|_| format!("编号要是数字，收到的是「{arg}」"))
}

fn pick_command(args: &[String]) -> Result<&str, String> {
    // ok_or_else 而不是 ok_or：后者会无条件先把那个 String 造出来，
    // 哪怕根本用不上。这里差别微乎其微，但习惯要从小处养。
    args.first()
        .map(|s| s.as_str())
        .ok_or_else(|| "请给一个命令，例如：add 买菜".to_string())
}

fn parse_two(args: &[String]) -> Result<(String, u32), String> {
    // ? = 出错就带着错误立刻返回，成功就把里面的值取出来继续往下走。
    // 没有 ? 的话这三步要写成三层嵌套的 match。
    let cmd = pick_command(args)?;

    let rest = args
        .get(1)
        .ok_or_else(|| format!("{cmd} 后面要跟编号，例如：done 1"))?;

    let id = parse_id(rest)?;

    Ok((cmd.to_string(), id))
}

fn exit_code(result: &Result<(), String>) -> i32 {
    // 脚本靠这个判断该不该继续。0 = 成功，非 0 = 出事了。
    if result.is_ok() {
        0
    } else {
        1
    }
}

fn main() {
    println!("这是第 09 课练习的答案：cargo test --example ans09");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    fn 参数(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn 解析编号() {
        assert_eq!(parse_id("3"), Ok(3));
        assert_eq!(parse_id("0"), Ok(0));
        assert!(parse_id("三").is_err());
        assert!(parse_id("").is_err());
        assert!(parse_id("-1").is_err(), "u32 装不下负数");
    }

    #[test]
    fn 报错要带上原文() {
        let err = parse_id("三").unwrap_err();
        assert!(
            err.contains('三'),
            "错误信息里要有用户敲的原文，当前是：{err}"
        );
    }

    #[test]
    fn 没有参数时报错() {
        assert_eq!(pick_command(&参数(&["add"])), Ok("add"));
        assert!(pick_command(&[]).is_err());
        let err = pick_command(&[]).unwrap_err();
        assert!(err.contains("add"), "错误信息里要给个例子，当前是：{err}");
    }

    #[test]
    fn 两段式命令() {
        assert_eq!(
            parse_two(&参数(&["done", "3"])),
            Ok(("done".to_string(), 3))
        );
        assert!(parse_two(&参数(&["done"])).is_err(), "少了编号");
        assert!(parse_two(&参数(&["done", "三"])).is_err(), "编号不是数字");
        assert!(parse_two(&[]).is_err());
    }

    #[test]
    fn 退出码() {
        assert_eq!(exit_code(&Ok(())), 0);
        assert_eq!(exit_code(&Err("出错了".to_string())), 1);
    }
}
