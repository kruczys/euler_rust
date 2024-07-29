pub fn solve_1(max: i32) -> i32 {
    let three_multiplies: Vec<i32> = (1..max).map(|x| 3 * x).filter(|x| x < &max).collect();
    let five_multiples: Vec<i32> = (1..max).map(|x| 5 * x).filter(|x| x < &max).collect();
    three_multiplies.iter().sum::<i32>() + five_multiples.iter().sum::<i32>()
}
