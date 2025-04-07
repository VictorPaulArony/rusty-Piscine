pub fn is_permutation(s1: &str, s2: &str) -> bool {
  // If the lengths are different, they cannot be permutations
  if s1.len() != s2.len() {
    return false;
}

// Convert the strings to vectors of characters, sort them, and compare
let mut s1_chars: Vec<char> = s1.chars().collect();
let mut s2_chars: Vec<char> = s2.chars().collect();

s1_chars.sort_unstable();
s2_chars.sort_unstable();

s1_chars == s2_chars    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "avsde";
        let word1 = "edbca";
        assert_eq!(!is_permutation(word, word1), true);
    }
}
