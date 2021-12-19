fn main() {
    let mut counter = 0;
    while counter != 10 {
        println!("{}", counter);
        counter += 1;
    }

    let mut counter = Some(0);
    while let Some(i) = counter {
        if i == 10 {
            counter = None;
        } else {
            println!("{}", i);
            counter = Some(i + 1);
        }
    }
}