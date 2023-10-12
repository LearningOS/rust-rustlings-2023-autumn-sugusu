// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

fn main() {
    // 初始化玩家的令牌数量为 100
    let mut tokens = 100;
    
    // 假设玩家输入的购买数量
    let pretend_user_input = "8";

    // 调用 total_cost 函数，处理结果
    match total_cost(pretend_user_input) {
        // 如果 total_cost 返回 Ok(cost)，表示解析成功
        Ok(cost) => {
            // 检查购买的物品总价是否超过了玩家的令牌数量
            if cost > tokens {
                println!("You can't afford that many!");
            } else {
                // 扣除令牌费用，更新令牌数量
                tokens -= cost;
                println!("You now have {} tokens.", tokens);
            }
        }
        // 如果 total_cost 返回 Err(error)，表示解析失败
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

// total_cost 函数接受一个字符串 item_quantity，尝试将其解析为整数
// 如果解析成功，返回 Ok 包含购买物品的总成本
// 如果解析失败，返回 Err 包含 ParseIntError 错误
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    
    // 尝试将输入字符串解析为整数，如果解析失败，返回 Err
    let qty = item_quantity.parse::<i32>()?;
    
    // 计算购买物品的总成本，包括处理费
    Ok(qty * cost_per_item + processing_fee)
}
