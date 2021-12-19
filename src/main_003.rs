// タプル
fn main1() {
    let t = ("masuda", 30);
    println!("name is {} age {}", t.0, t.1);
}

fn main() {
    let name="maduda";
    let age=30;
    let t=(name,age);
    println!("name is {} age {}", t.0, t.1);

    //main1()
}