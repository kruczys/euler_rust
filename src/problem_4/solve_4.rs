fn is_palindrome(n: u64) -> bool {
    let n_string = n.to_string();
    let n_reversed = n_string.chars().rev().collect::<String>();
    n_string == n_reversed
}

pub fn solve_4() -> u64 {
    let mut result = 0;

    for i in 100..1000 {
        for j in i..1000 {
            match is_palindrome(i * j) {
                true => {
                    if i * j > result {
                        result = i * j
                    }
                }
                false => continue,
            }
        }
    }

    result
}
