use num_integer::Roots;

fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        _ => {
            let mut i = 2;
            while i * i < n {
                if n % i == 0 {
                    return false;
                }
                i += 1
            }
            true
        }
    }
}

fn find_prime_factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let max = n.sqrt();
    factors
}
