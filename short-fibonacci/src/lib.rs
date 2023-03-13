/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut buf = create_buffer(5);
    for i in 0..5 {
        match i {
            0..=1 => buf[i] = 1,
            _ => {
                let a = *buf.get(i - 2).unwrap_or(&1);
                let b = *buf.get(i - 1).unwrap_or(&1);
                buf[i] = a + b;
            }
        }
    }
    buf
}
