struct Circle {
    radius: u32,
}

impl Circle {
    fn diameter(&self) -> u32 {
        self.radius * 2
    }

    fn smart_circle() -> Circle {
        Circle { radius: 1 }
    }
}

fn main() {
    //let circle1 = Circle { radius: 10 };
    let circle1 = Circle::smart_circle();
    println!("{}", circle1.diameter());
}