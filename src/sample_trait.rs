struct RightTrangle {
    base: f64,
    perpendicular: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

trait GeoCalculator {
    fn area(&self) -> f64;
    fn length(&self) -> f64;
}

impl GeoCalculator for RightTrangle {
    fn area(&self) -> f64 {
        (self.base * self.perpendicular) * 0.5
    }
    fn length(&self) -> f64 {
        self.base + self.perpendicular +
            (self.base.powi(2) + self.perpendicular.powi(2)).sqrt()
    }
}

impl GeoCalculator for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn length(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

fn printval<T: GeoCalculator>(poly: &T) {
    println!("{}", poly.area());
    println!("{}", poly.length());
}

fn myprint<T: std::fmt::Display>(msg: &T) {
    print!("{}", msg);
}

fn main() {
    let tri = RightTrangle { base: 3.0, perpendicular: 4.0 };
    printval(&tri);
    let rec = Rectangle { width: 3.0, height: 4.0 };
    printval(&rec);

    let tri2 = RightTrangle { base: 3.0, perpendicular: 4.0 };
    let rec2 = Rectangle { width: 3.0, height: 4.0 };

    let mut vv:Vec<&dyn GeoCalculator>=vec![];
    vv.push(&tri2);
    vv.push(&rec2);

    for v in vv {
        println!("{}", v.area());
        println!("{}", v.length());
    }
}