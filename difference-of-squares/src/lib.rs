pub fn square_of_sum(n: u32) -> u32 {
    //    (1..=n).sum::<u32>().pow(2)
    (n * (1 + n) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    //   (1..=n).map(|x| x.pow(2)).sum::<u32>()
    (n * (n + 1) * (2 * n + 1)) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
