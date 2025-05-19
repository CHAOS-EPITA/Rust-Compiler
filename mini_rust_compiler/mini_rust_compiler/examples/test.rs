// filepath: /mini_rust_compiler/mini_rust_compiler/examples/test.rs
fn main() {
    let x: i32 = 10;
    let y: i32 = 20;
    println!("x = {}, y = {}", x, y);
    println!("Les opérateurs de base");
    println!("{} + {} = {}", x, y, sum(x, y));
    println!("{} - {} = {}", x, y, diff(x, y));
    println!("{} / {} = {} (division entière)", x, y, divide(x, y));
    println!("{} * {} = {}", x, y, mult(x, y));
}

fn sum(x: i32, y: i32) -> i32 { return x + y; }
fn diff(x: i32, y: i32) -> i32 { return x - y; }
fn divide(x: i32, y: i32) -> i32 { return x / y; }
fn mult(x: i32, y: i32) -> i32 { return x * y; }