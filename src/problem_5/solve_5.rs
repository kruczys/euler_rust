fn is_evenly_divisible(n: u64) -> bool {
    for i in 1..21 {
        if n % i != 0 {
            return false;
        }
    }
    true
}

pub fn solve_5() -> u64 {
    let mut result = 1;
    while !is_evenly_divisible(result) {
        result += 1
    }
    result
}
