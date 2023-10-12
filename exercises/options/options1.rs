// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.



// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(

// 定义一个函数 maybe_icecream，接受一个时间参数 time_of_day，返回一个 Option<u16> 类型。
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    if time_of_day < 22 {
        // 如果时间早于 22 时，返回 Some(5)，表示还剩下 5 个冰淇淋。
        Some(5)
    } else if time_of_day >= 22 && time_of_day <= 23 {
        // 如果时间介于 22 时和 23 时之间，返回 Some(0)，表示没有冰淇淋剩下。
        Some(0)
    } else {
        // 其他情况，返回 None，表示无法确定冰淇淋的数量。
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        // 检查 maybe_icecream 函数的不同输入是否返回了正确的 Option 值。
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // 测试获取 Option 中的值，这里使用 unwrap() 方法。
        let icecreams = maybe_icecream(12).unwrap(); // 获取值，这里返回 5
        assert_eq!(icecreams, 5);
    }
}