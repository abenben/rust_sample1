struct Rectangle<T, S> {
    witdh: T,
    height: S,
}

impl<T, S> Rectangle<T, S> {
    fn new(witdh: T, height: S) -> Rectangle<T, S> {
        Rectangle {
            witdh: witdh,
            height: height,
        }
    }
}

fn gen<T>(x:T)->T{
    x
}


fn main() {
    let rec = Rectangle {
        witdh: 3.0,
        height: 4.0,
    };

    let x = gen(10);
    println!("{}", x);
}