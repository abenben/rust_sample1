fn main() {
    let mut counter = 10;
    let ten = loop {
        println!("{}", counter);
        if counter == 0 {
            break counter;
        }
        counter -= 1;
    };
    println!("{}", ten);
}