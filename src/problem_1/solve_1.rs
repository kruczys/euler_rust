pub fn solve_1(max: i32) -> i32 {
    let mut result: Vec<i32> = (1..max).map(|x| 3 * x).filter(|x| x < &max).collect();
    result.append(&mut (1..max).map(|x| 5 * x).filter(|x| x < &max).collect());
    result.sort();
    result.dedup();
    result.iter().sum::<i32>()
}
