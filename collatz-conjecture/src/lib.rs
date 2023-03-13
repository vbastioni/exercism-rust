pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0u64 => None,
        1u64 => Some(0u64),
        n if n & 1u64 == 0u64 => collatz(n / 2u64).map(|n| n + 1u64),
        n => collatz(n.checked_mul(3)?.checked_add(1)?).map(|n| n + 1),
    }
}
