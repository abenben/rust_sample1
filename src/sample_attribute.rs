#[test]
fn test1() {
    println!("test1");
}

// fn test2(){
//     #![test]
//     println!("test2");
// }

//#[macro_use]mod macros;

//use log::[debug,error];

#[cfg(unix)]
fn something_for_unix() {
    println!("unix")
}

#[cfg(windows)]
fn something_for_windows() {
    println!("windows")
}

fn main() {
    //test1();
    //test2();
}