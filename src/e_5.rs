pub fn run() {
    let mut i = 20;
    loop {
        if (2..=20).any(|x| i % x != 0) {
            i += 20;
            continue;
        } else {
            break;
        }
    }
    // Correct: 232792560

    println!("5 | {:?}", i);
    // let nums: i32 = (1..=10).rev().reduce(|x, y| x + y).unwrap();
    // println!("{:?}", nums)
}
