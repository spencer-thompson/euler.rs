// smallest Pythagorean triplet
// what a + b + c = 1000

pub fn run() {
    let mut answer = 0;
    (1..=1000).for_each(|a: u32| {
        let b: Vec<u32> = (a..=1000)
            .filter(|b| a as f64 + *b as f64 + ((a.pow(2) + b.pow(2)) as f64).sqrt() == 1000.0)
            .collect();
        if !b.is_empty() {
            let c = 1000 - (a + b[0]);
            answer = a * b[0] * c;
            // println!("{:?} {:?} {:?}", a, b[0], c)
        }
    });
    println!("9 | {:?}", answer)
}
