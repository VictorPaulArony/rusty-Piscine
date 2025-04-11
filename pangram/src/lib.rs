use std::collections::HashMap;
pub fn is_pangram(s: &str) -> bool {
    // (b'a'..=b'z').all(|c| s.to_ascii_lowercase().contains(c as char))
    let mut map = HashMap::new();
    for c in s.chars() {
        if c.is_ascii_alphabetic(){
            let ch = c.to_ascii_lowercase();
            *map.entry(ch).or_insert(0) +=1;

        }
    }
    map.len() == 26

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_pangram("The quick brown fox jumps over the lazy dog!"), true);
        assert_eq!(is_pangram("victor paul arony"), false)
    }
}
