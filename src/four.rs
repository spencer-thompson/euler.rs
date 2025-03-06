pub fn run() {
    let low = 100..1000;
    let high = 100..1000;
    let answer: Vec<i32> = low
        .zip(high)
        .map(|(x, y)| x * y)
        .filter(|x| {
            x.to_string().chars().eq(x.to_string().chars().rev())
            // .zip(x.to_string().chars().rev())
            // .all(|(a, b)| a == b)
        })
        .collect();

    println!("4 | {:?}", answer)
}
