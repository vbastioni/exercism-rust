use std::collections::HashMap;
use std::ops::AddAssign;

fn map_chars_mut(s: &str, h: &mut HashMap<char, usize>) {
    for c in s.chars().filter(|c| c.is_alphabetic()) {
        h.entry(c.to_ascii_lowercase()).or_insert(0).add_assign(1);
    }
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut s = HashMap::new();
    if input.len() == 0 {
        return s;
    } else if input.len() == 1 {
        map_chars_mut(input[0], &mut s);
        return s;
    }

    let input = input.join("");
    if input.len() == 0 {
        return s;
    }
    let mut work = input.chars();
    let mut handles = vec![];
    let wc = worker_count.clamp(1, input.len());
    let chunk = {
        let mut size = input.len() / wc;
        if input.len() % wc != 0 {
            size += 1;
        }
        size
    };
    for _ in 0..wc {
        let part: String = work.by_ref().take(chunk).collect();
        handles.push(std::thread::spawn(move || {
            let mut h: HashMap<char, usize> = HashMap::new();
            part.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
                h.entry(c.to_ascii_lowercase()).or_insert(0).add_assign(1);
            });
            h
        }));
    }
    for h in handles {
        let m = h.join().unwrap();
        m.iter().for_each(|(k, v)| {
            s.entry(*k).or_insert(0).add_assign(*v);
        });
    }
    s
}
