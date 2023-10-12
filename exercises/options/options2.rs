// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        // 创建一个字符串 `target` 和一个包含 `target` 的 `Some` 类型的 `optional_target`
        let target = "rustlings";
        let optional_target = Some(target);

        // 使用 `if let` 语句来检查 `optional_target` 是否包含 `Some` 类型
        if let Some(word) = optional_target {
            // 如果 `optional_target` 包含 `Some` 类型，将 `word` 绑定到 `target`
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        // 创建一个范围 `range`，并初始化一个包含 `None` 的 `optional_integers` 向量
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        // 将从 1 到 `range` 的整数包装为 `Some` 类型，并添加到 `optional_integers` 向量中
        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        // 初始化一个游标 `cursor` 为 `range`
        let mut cursor = range;

        // 使用 `while let` 语句和 `if let` 语句来处理嵌套的 `Option<i8>`
        while let Some(Some(integer)) = optional_integers.pop() {
            // `optional_integers.pop()` 移除向量中的元素，并返回 `Some(Some(integer))`，将其解构
            // 如果 `integer` 不为空，执行以下代码块
            assert_eq!(integer, cursor);
            // 检查 `integer` 是否等于 `cursor`，即当前游标值
            cursor -= 1;
            // 游标递减
        }

        // 断言游标 `cursor` 等于 0，表明所有元素已经处理完
        assert_eq!(cursor, 0);
    }
}

