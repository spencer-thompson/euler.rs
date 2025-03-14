#[derive(Debug)]
struct CollatzIter {
    num: usize,
}

#[derive(Debug)]
struct Collatz {
    current: usize,
}

impl Iterator for CollatzIter {
    // we will be counting with usize
    type Item = usize;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        if self.num % 2 == 0 {
            self.num /= 2
        } else if self.num != 1 {
            self.num = (self.num * 3) + 1
        }

        // Check to see if we've finished counting or not.
        if self.num > 1 {
            Some(self.num)
        } else {
            None
        }
    }
}

impl Collatz {
    fn new(x: usize) -> Collatz {
        Collatz { current: x }
    }

    fn iter(&self) -> CollatzIter {
        CollatzIter { num: self.current }
    }
}

pub fn run() {
    let problem_input = Collatz::new(13);

    let nums: Vec<usize> = (2..=1_000_000).collect();

    // let answer = problem_input.iter().collect::<Vec<usize>>();
    let answer = nums
        .iter()
        .map(|x| Collatz::new(*x).iter().collect::<Vec<usize>>().len())
        .reduce(usize::max);

    println!("{:?}", answer)
}
