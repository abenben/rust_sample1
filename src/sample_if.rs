fn main() {
    let a = 11;
    if a % 2 == 0 {
        println!("{} is an even number", a);
    } else {
        println!("{} is an odd number", a);
    }

    let b = 12;
    let even_or_odd = if a % 2 == 0 {
        "an even"
    } else {
        "an odd"
    };
    println!("{} is {} number", b, even_or_odd);

    let d=14;
    if d % 2==0 {
        println!("{} ia an even number", d);
    }
}