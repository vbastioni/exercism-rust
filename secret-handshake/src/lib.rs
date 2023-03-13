pub fn actions(n: u8) -> Vec<&'static str> {
    let r = ["wink", "double blink", "close your eyes", "jump"]
        .iter()
        .enumerate()
        .filter_map(|(o, s)| if n & (1 << o) != 0 { Some(*s) } else { None });
    match n & (1 << 4) {
        0 => r.collect(),
        _ => r.rev().collect(),
    }
}
