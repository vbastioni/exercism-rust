// code.chars()
//     .fold(Some(vec![]), |acc, cur| match (acc, cur) {
//         (Some(mut v), c) if c.is_ascii_digit() => {
//             v.push(c as u8 - b'0');
//             Some(v)
//         }
//         (Some(v), c) if c == ' ' => Some(v),
//         _ => return None,
//     })
//     .filter(|codes| codes.len() > 1)
//     .map(|codes| {
//         codes.iter().enumerate().fold(0, |acc, (i, v)| {
//             acc + {
//                 let mut n = *v;
//                 if i & 1 == 1 {
//                     n *= 2;
//                     if n > 9 {
//                         n -= 9;
//                     }
//                 }
//                 println!("{} + {}", acc, n);
//                 n as u32
//             }
//         })
//     })
//     .filter(|sum| sum % 10 == 0)
//     .is_some()

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
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
