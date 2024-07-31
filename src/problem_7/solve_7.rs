fn is_prime(n: u128) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        _ => {
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 {
                    return false;
                }
                i += 1
            }
            true
        }
    }
}

fn nth_prime(n: u64) -> u128 {
    let mut prime_vec: Vec<u128> = Vec::new();
    let mut i = 0;

    while prime_vec.len() != n as usize {
        if is_prime(i) {
            prime_vec.push(i)
        }
        i += 1
    }
    *prime_vec.last().unwrap()
}

pub fn solve_7(n: u64) -> u128 {
    nth_prime(n)
}
