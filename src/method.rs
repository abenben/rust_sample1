struct Circle {
    radius: u32,
}

impl Circle {
    fn diameter(&self) -> u32 {
        self.radius * 2
    }
}

fn main() {
    let circle1 = Circle { radius: 10 };
    println!("{}", circle1.diameter());
}