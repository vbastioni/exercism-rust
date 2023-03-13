pub fn factors(mut n: u64) -> Vec<u64> {
    let mut v = vec![];
    while n != 1 {
        let d = (2..=(n / 2)).find(|i| n % i == 0).unwrap_or(n);
        v.push(d);
        n /= d;
    }
    v
}
