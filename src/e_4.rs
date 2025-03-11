pub fn run() {
    let mut answer: Vec<i32> = Vec::new();
    for i in 100..1000 {
        if let Some(max_palindrome) = (100..1000)
            .map(|x| x * i)
            .filter(|x| {
                let s = x.to_string();
                s.chars().eq(s.chars().rev())
            })
            .max()
        {
            answer.push(max_palindrome);
        }
    }

    println!(
        "4 | {:?}",
        answer.iter().max().expect("Array should not be empty")
    )
}
