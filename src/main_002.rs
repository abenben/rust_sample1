// 型宣言
fn main() {
    let name = "abenben";
    let age = 30;
    println!("{}", name);
    println!("{}", age);
    println!("{}", add(5, 10));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}