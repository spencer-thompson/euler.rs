pub fn run() {
    let mut fibs: Vec<u64> = vec![0, 1];
    let mut total = 0;
    loop {
        let next = fibs.iter().sum();
        fibs.push(next);
        fibs.remove(0);

        if next >= 4_000_000 {
            break;
        }

        if next % 2 == 0 {
            total += next;
        }
    }
    println!("2 | {:?}", total);
}
