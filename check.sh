#!/usr/bin/env bash
# 一键自检：排版、写法建议、全部答案是否还能跑。
#
#     ./check.sh
#
# 改完代码跑一下，三样全过就说明没把项目改坏。
# 注意它**不跑你的练习**（那些没做完本来就是红的），只跑 solutions/ 里的答案。
set -euo pipefail
cd "$(dirname "$0")"

echo "== cargo fmt --check =="
cargo fmt --check

echo "== cargo clippy --all-targets =="
cargo clippy --all-targets -- -D warnings

echo "== cargo test =="
cargo test --quiet

echo "== 练习答案（第 02–15 课）=="
for n in 02 03 04 05 06 07 08 09 10 11 13 14 15; do
    printf '  ans%s  ' "$n"
    cargo test --quiet --example "ans$n" 2>&1 | grep -E '^test result' || {
        echo "失败"
        exit 1
    }
done

echo "== 例子能编译 =="
cargo build --examples --quiet

# 练习骨架里的 todo!() 必须是「能编译、跑起来才 panic」，
# 不能是「连编译都过不去」——否则学生一道题也开不了头。
# 唯一的例外是 ex15 第 2 题（少 derive 就是编译错误），那一课的文件头写明了。
echo "== 练习骨架能编译 =="
for n in 02 03 04 05 06 07 08 09 10 11 13 14; do
    cargo build --quiet --example "ex$n" || {
        echo "  ex$n 编译不过"
        exit 1
    }
done
echo "  ok"

echo
echo "全部通过。"
