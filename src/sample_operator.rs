fn main() {
    let answer1 = (10 + 20) * 30 / 4;
    println!("answer1 = {}", answer1);

    let mut answer2 = 5_000;
    answer2 += 600_000;
    println!("answer2 = {}", answer2);

    //let answer3 = 70 + 8.9;
    //println!("answer3 = {}", answer3);

    let answer4 = 70.0 + 8.9;
    println!("answer4 = {}", answer4);

    let answer5 = !((true || false) && false);
    println!("answer5 = {}", answer5);

    let answer6 = 0b11110000 & 0b01010000 | 0b00001010;
    println!("answer6 = {:b}", answer6);

    let unsigned: u8 = 0b11111111;
    // todo 調査必要
    //let signed: i8 = 0b11111111;
    println!("{:08b}", unsigned >> 2);
    //println!("{:08b}", signed >> 2);
}