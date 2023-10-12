// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// This exercise uses some concepts that we won't get to until later in the
// course, like `Box` and the `From` trait. It's not important to understand
// them in detail right now, but you can read ahead if you like. For now, think
// of the `Box<dyn ???>` type as an "I want anything that does ???" type, which,
// given Rust's usual standards for runtime safety, should strike you as
// somewhat lenient!
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The Box is declared as of type Box<dyn Trait> where Trait is
// the trait the compiler looks for on any value used in that context. For this
// exercise, that context is the potential errors which can be returned in a
// Result.
//
// What can we use to describe both errors? In other words, is there a trait
// which both errors implement?
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.


use std::error; // 导入错误处理相关的标准库模块
use std::fmt;   // 导入格式化相关的标准库模块
use std::num::ParseIntError; // 导入解析整数错误相关的标准库模块

// 定义一个结构体 PositiveNonzeroInteger 用于表示正的非零整数
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

// 定义一个枚举 CreationError 用于表示创建 PositiveNonzeroInteger 时可能的错误
#[derive(PartialEq, Debug)]
enum CreationError {
    Negative, // 负数错误
    Zero,     // 零错误
}

impl PositiveNonzeroInteger {
    // 定义一个关联函数 new，它尝试创建 PositiveNonzeroInteger 实例
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative), // 如果值为负数，返回 Negative 错误
            x if x == 0 => Err(CreationError::Zero),     // 如果值为零，返回 Zero 错误
            x => Ok(PositiveNonzeroInteger(x as u64)),   // 否则返回包含非零正整数的 Result
        }
    }
}

// 实现 Display trait 用于将 CreationError 转化为字符串表示
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 根据错误类型返回相应的描述信息
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description) // 写入描述信息到 Formatter 中
    }
}

// 实现 Error trait 用于将 CreationError 映射为标准错误类型
impl error::Error for CreationError {}

fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?; // 尝试解析输入字符串为整数，可能会返回 ParseIntError

    // 调用 PositiveNonzeroInteger::new(x) 尝试创建 PositiveNonzeroInteger 实例，可能会返回 CreationError
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);

    Ok(()) // main 函数成功返回 Ok(())
}
