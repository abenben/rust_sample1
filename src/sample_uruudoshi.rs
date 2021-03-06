use std::io;
use std::io::Write;

fn main() {
    let mut year = String::new();
    io::stdout().flush().unwrap();
    println!("Please input a year to check if it a leap year:");
    io::stdin().read_line(&mut year).unwrap();
    let year = year.trim().parse::<u32>().unwrap();

    if is_leap_year(year) {
        println!("{} is a leap year!", year);
    } else {
        println!("{} is not a leap year!", year);
    }
}

fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year & 400 != 0)
}