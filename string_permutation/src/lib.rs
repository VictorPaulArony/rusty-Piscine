pub fn is_permutation(s1: &str, s2: &str) -> bool {
    s1.chars().count() == s2.chars().count()
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "thought";
        let word1 = "thougth";
        assert_eq!(is_permutation(word, word1), true);
    }
}
