// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.


fn main() {
  
    let radius = 5.00f32;

    // To fix Clippy warnings:
    // 1. Use std::f32::consts::PI for the value of pi.
    // 2. Use the f32::powi method to raise radius to the power of 2.
    // 3. Use {:.2} for pi and {:.5} for area in the format string.
    let area = std::f32::consts::PI * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}
