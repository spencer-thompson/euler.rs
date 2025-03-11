use std::collections::HashMap;

fn generate(n: usize, memo: &mut HashMap<usize, Vec<usize>>) -> Vec<usize> {
    if let Some(primes) = memo.get(&n) {
        return primes.clone();
    }

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

    memo.insert(n, result.clone());
    result
}

fn correct_num(n: usize) -> usize {
    let mut m = n;
    let mut memo = HashMap::new();
    loop {
        let primes = generate(m, &mut memo);
        if primes.len() >= n {
            return primes[n - 1];
        } else {
            m += 1
        }
    }
}

pub fn run() {
    let answer = correct_num(10001);
    println!("7 | {:?}", answer)
}
