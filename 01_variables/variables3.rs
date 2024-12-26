#![allow(clippy::needless_late_init)]

fn main() {
    let x: i32 = 42;

    println!("Number {x}");
    let y: i32;
    y = 42;
    println!("Number {y}");
}
