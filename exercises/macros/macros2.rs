// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    my_macro!();

}
#[macro_export]//宏定义,这可以让宏在整个 crate 中可用
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
