use std::collections::HashSet;

fn sort<'a>(s: &'a str) -> (String, Vec<char>) {
    let srt = s.to_lowercase();
    let mut v = srt.chars().collect::<Vec<_>>();
    v.sort_unstable();
    (srt, v)
}

// pub fn anagrams_for<'a>(oword: &str, open: &'a [&'a str]) -> HashSet<&'a str> {
//     let (lower, sorted) = sort(oword);
//     HashSet::from_iter(
//         open.iter()
//             .filter(|o| {
//                 let (op_lower, op_sorted) = sort(w);
//                 op_lower.len() == lower.len() && op_lower != lower && op_sorted == sorted
//             })
//             .into(),
//     )
// }

pub fn anagrams_for<'a>(word: &str, open: &'a [&'a str]) -> HashSet<&'a str> {
    let (lower, sorted) = sort(word);
    open.iter()
        .filter(|w| {
            let (op_lower, op_sorted) = sort(w);
            op_lower.len() == lower.len() && op_lower != lower && op_sorted == sorted
        })
        .copied()
        .collect()
}
