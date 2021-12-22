fn main() {
    let x = String::from("Hello");
    let y = string_length(&x);
    println!("{} is {}", x, y);
    let name = "masuda";
    let mut age = get_person_age();
    if age > 20 {
        age = 100;
    }
    println!("name is {}, age {}", name, age);
}

fn string_length(x: &String) -> usize {
    let length = x.len();
    length
}

fn get_person_age() -> i32 {
    30
}
