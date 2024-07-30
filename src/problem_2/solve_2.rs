fn nth_fib(n: u64, a: u64, b: u64) -> u64 {
    match n {
        0 => a,
        1 => b,
        _ => nth_fib(n - 1, b, a + b)
    }
}

pub fn solve_2() -> u64 {
    (1..34)
        .map(|x| nth_fib(x, 0, 1))
        .filter(|x| x % 2 == 0)
        .collect::<Vec<u64>>()
        .iter()
        .sum::<u64>()
}
