// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


#[macro_use]//#[macro_use]作用与#[macro_export]在于,#[macro_use]可以让宏在整个 crate 中可用,而#[macro_export]只能让宏在当前模块中可用   
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
