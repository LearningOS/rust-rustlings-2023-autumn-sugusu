

// iterators3.rs
//
// 这是一个比大多数其他练习更大的练习！你可以做到的！以下是你的任务，如果你选择接受的话：
// 1. 完成divide函数，以使前四个测试通过。
// 2. 通过完成result_with_list和list_of_results函数来使其余测试通过。
//
// 执行 `rustlings hint iterators3` 或使用 `hint` 观看子命令获取提示。

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// 如果a可以被b整除，则计算a除以b。
// 否则，返回适当的错误。
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        // 如果b为0，返回DivideByZero错误。
        return Err(DivisionError::DivideByZero);
    }

    if a % b == 0 {
        // 如果a可以被b整除，返回a除以b的结果。
        Ok(a / b)
    } else {
        // 如果a不能被b整除，返回NotDivisible错误。
        Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b,
        }))
    }
}

// 完成函数并返回正确类型的值，以使测试通过。
// 期望输出：Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27)).collect();
    division_results
}

// 完成函数并返回正确类型的值，以使测试通过。
// 期望输出：[Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27)).collect();
    division_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
