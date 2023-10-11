// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


/*string_slice() 接受一个字符串 slice(即 &str)类型的参数。字符串字面量如"blue"就是&str类型。
string() 接受一个 String 类型的参数。想把字符串字面量转换成 String,需要 to_string() 或 String::from()。
main()函数展示了多种字符串操作:
1. "blue" 直接传给 string_slice(),因为它已是 &str 类型。
2. "red".to_string() 创建一个 String 传给 string()。
3. String::from("hi") 也创建了一个 String。
4. to_owned() 可以把 &str 转换成 String。
5. into() 也可以把 &str 转换成 String。
6. format! 宏可以格式化字符串并返回 String。
7. &String::from 返回字符串的一部分,为 &str。
8. trim() 去除字符串两端的空白字符。
9. replace() 替换字符串的一部分。
10. to_lowercase() 把字符串变为小写。
综上,Rust 为字符串处理提供了丰富的方法,我们可以灵活地在 &str 和 String 间转换。  */
fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");//这里用string_slice的原因是因为blue是一个字符串字面量，而不是一个String类型的变量
    string("red".to_string());//这里用string的原因是因为"red".to_string()是一个String类型的变量
    string(String::from("hi"));//这里用string的原因是因为String::from("hi")是一个String类型的变量
    string("rust is fun!".to_owned());//这里用
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);//这里用string_slice的原因是因为&String::from("abc")[0..1]是一个&str类型的变量
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));//这里用string的原因是因为"Happy Monday!".to_string().replace("Mon", "Tues")是一个String类型的变量
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());//这里用string的原因是因为"mY sHiFt KeY iS sTiCkY".to_lowercase()是一个String类型的变量
}
