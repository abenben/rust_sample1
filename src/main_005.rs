fn main() {
    let s1 = "Hello, ";
    let s2 = "World!";
    let s3 = s1.to_string() + s2;
    assert_eq!(s3, "Hello, World!");

    let ret = hello();
    assert_eq!(ret, ());

    assert_eq!(std::mem::size_of::<()>(), 0);
    assert_eq!(std::mem::size_of::<bool>(), 1);

    // aaaa
    /* aaaa
       nnn
     */
    /*
    /* aaaa */
    */
}

fn hello() {
    println!("Hello");
}