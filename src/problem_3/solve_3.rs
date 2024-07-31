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

fn find_prime_factors(n: u64) -> u64 {
    let max = n.sqrt();
    for i in (2..max).rev() {
        match n % i == 0 {
            true => {
                if is_prime(i) {
                    return i;
                }
            }
            false => continue,
        }
    }
    0
}

pub fn solve_3(n: u64) -> u64 {
    find_prime_factors(n)
}
