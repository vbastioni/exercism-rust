pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|number| !(2..(*number as f32).sqrt() as u32 + 1).any(|x| number % x == 0))
        .nth(n as usize).unwrap()
}
