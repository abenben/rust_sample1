fn main() {
    let x = 1;
    // x = x + 1; コンパイルエラー（不変のため）
    println!("{}", x);

    let mut y = 1;
    y = y + 1;
    println!("{}", y);

    // シャドーイング
    let z = 1;
    {
        let z = z + 1;
        println!("{}", z);
    }
    println!("{}", z);

    let s = "Hello".to_string();
    let t = s;
    // println!("{}", s); ←コンパイルエラー
    println!("{}", t);

    let v = 1;
    myprint(v);
    myprint(v);

    let v2 = "Hello".to_string();
    myprint(v2);
    //myprint(v2); ←　コンパイルエラー

    let v3 = "Hello".to_string();
    myprint2(&v3);
    myprint2(&v3);

    let mut ibuf = [0i32; BUFSIZE];
    for ii in 0..BUFSIZE {
        ibuf[ii] = ii as i32;
    }
}

const BUFSIZE: usize = 1024;

fn myprint<T: std::fmt::Display>(msg: T) {
    println!("{}", msg)
}

fn myprint2<T: std::fmt::Display>(msg: &T) {
    println!("{}", msg)
}