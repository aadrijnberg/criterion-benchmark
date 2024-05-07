pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

pub fn factorial_with_fold(n: u64) -> u64 {
    (1..=n).fold(1, |a,c| a * c)
}
