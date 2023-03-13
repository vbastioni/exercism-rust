pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        self.to_string()
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .try_fold((0u32, 0u32), |(count, sum), cur| {
                cur.to_digit(10)
                    .map(|d| d * if count & 1 == 1 { 2 } else { 1 })
                    .map(|d| if d > 9 { d - 9 } else { d })
                    .map(|d| (count + 1, sum + d))
            })
            .map_or(false, |(count, sum)| count > 1 && sum % 10 == 0)
    }
}
