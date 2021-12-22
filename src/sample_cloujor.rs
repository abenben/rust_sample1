fn main() {
    let one = 1;
    let plus_one = |x| {
        x + one
    };
    println! {"10 + 1 = {}", plus_one(10)};

    let mut one = 1;
    let plus_one = move |x| {
        x + one
    };
    one += 1;
    println!("10 + 1 = {}", plus_one(10));

    const N_THREADS: usize = 3;
    let series_range = 0..30;
    let add = 1;

    let chunks = (0..N_THREADS)
        .map(|ii| series_range.clone().skip(ii).step_by(N_THREADS));

    let handles: Vec<_> = chunks
        .map(|vv| std::thread::spawn(move || {
            vv.for_each(|nn| println!("{}", nn + add));
        })
        ).collect();

    handles.into_iter().for_each(|hh| hh.join().unwrap());
}