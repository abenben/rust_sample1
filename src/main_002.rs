// 型宣言
fn main() {
    let name = "abenben";
    let age = 30;
    println!("{}", name);
    println!("{}", age);
    println!("{}", add(5, 10));

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = String::from("World.");
    let s = s1 + " " + &s2 + " " + &s3;
    println!("{}", s);
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = String::from("World.");
    let s = format!("{} {} {}", s1, s2, s3);
    print!("{}", s);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}