pub fn run() {
    let upper = 100;
    let answer = (1..=upper).sum::<i32>().pow(2) - (1..=upper).map(|x| x * x).sum::<i32>();
    println!("6 | {:?}", answer)
}
