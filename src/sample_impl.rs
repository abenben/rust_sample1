struct RightTriangle {
    base: f64,
    // 底辺の長さ
    perpendicular: f64,
    // 高さ（垂直）
}

impl RightTriangle {
    fn area(&self) -> f64 {
        (self.base * self.perpendicular) * 0.5
    }
    fn lenhth(&self) -> f64 {
        self.base + self.perpendicular + (self.base.powi(2) + self.perpendicular.powi(2).sqrt())
    }
}

struct Rectangle {
    width: f64,
    // 横の長さ
    height: f64,// 縦の長さ
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn lenhth(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

fn main() {
    let tri = RightTriangle {
        base: 3.0,
        perpendicular: 4.0,
    };
    println!("{}", tri.area());
    println!("{}", tri.lenhth());

    let rec = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("{}", rec.area());
    println!("{}", rec.lenhth());
}