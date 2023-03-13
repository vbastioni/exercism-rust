pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m => {
            let is_upper = m.contains(char::is_alphabetic)
                && m.chars()
                    .all(|c| c.is_ascii_alphabetic() ^ c.is_uppercase() ^ true);
            match (is_upper, m.ends_with("?")) {
                (true, true) => "Calm down, I know what I'm doing!",
                (true, _) => "Whoa, chill out!",
                (_, true) => "Sure.",
                _ => "Whatever.",
            }
        }
    }
}
