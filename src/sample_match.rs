fn main() {
    let value = 100;
    match value {
        1 => println!("One"),
        10 => println!("Ten"),
        100 => println!("One hundred"),
        _ => println!("Something else"),
    }
    enum Light {
        Red,
        Yellow,
        Green,
    }
    let light = Light::Green;

    let action = match light {
        Light::Red => "Stop",
        Light::Yellow => "Proceed with caution",
        Light::Green => "Go",
    };
    println!("Green:{}", action);
}