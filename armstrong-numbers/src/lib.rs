pub fn is_armstrong_number(num: u32) -> bool {
    //    let s = num.to_string();
    //    s.chars().into_iter().fold(0, |acc, cur| {
    //        acc + cur.to_digit(10).unwrap().pow(s.len() as u32)
    //    }) == num

    let l = ((num as f64).log10() + 1.).floor() as u32;
    (0..l)
        .map(|i| (num / 10u32.pow(i) % 10).pow(l))
        .sum::<u32>()
        == num
}
