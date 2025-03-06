pub fn run() {
    let numbers = 3..1000;
    let total: i32 = numbers.filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    println!("1 | {:?}", total)
}
