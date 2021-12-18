fn main() {
    println!("Hello world!");
    let english = "Hey!";
    let japanese = "こんにちは";
    let ary = [english, japanese];
    for region in ary.iter() {
        println!("{}", &region)
    }
}