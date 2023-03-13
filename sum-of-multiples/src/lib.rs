pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|u| factors.iter().any(|fa| fa != &0 && u % fa == 0))
        .sum()
}
