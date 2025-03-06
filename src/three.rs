fn generate(n: usize) -> Vec<usize> {
    let mut primes = vec![true; n + 1];
    let mut result = Vec::new();
    for num in 2..=n {
        if primes[num] {
            result.push(num);
            let mut multiple = num * num;
            while multiple <= n {
                primes[multiple] = false;
                multiple += num;
            }
        }
    }
    result
}

fn factor(num: usize, primes: Vec<usize>) -> Vec<usize> {
    let factors: Vec<usize> = primes.iter().cloned().filter(|&x| num % x == 0).collect();
    factors
}

pub fn run() {
    let num: usize = 600851475143;
    println!("3 | {:?}", factor(num, generate(10000)));
}
