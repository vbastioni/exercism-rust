// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut m = magazine.iter().fold(HashMap::new(), |mut m, cur| {
        m.entry(cur)
            .and_modify(|t: &mut (i32, i32)| *t = (t.0 + 1, 0))
            .or_insert((1, 0));
        m
    });
    note.iter().for_each(|w| {
        m.entry(w)
            .and_modify(|t| *t = (t.0, t.1 + 1))
            .or_insert((0, 1));
    });
    m.values().all(|(m, n)| *m >= *n)
}
