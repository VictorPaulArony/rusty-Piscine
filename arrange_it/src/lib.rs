pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    
    // Sort the words based on the embedded number
    words.sort_by_key(|word| {
        word.chars()
            .find(|c| c.is_ascii_digit())
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0)
    });
    
    // Remove the numbers from each word
    let processed_words: Vec<String> = words
        .iter()
        .map(|word| {
            word.chars()
                .filter(|c| !c.is_ascii_digit())
                .collect()
        })
        .collect();
    
    processed_words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(arrange_phrase("is2 Thi1s T4est 3a"),"This is a Test");
    }
}
