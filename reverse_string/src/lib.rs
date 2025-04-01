pub fn rev_str(input: &str) -> String {
    let mut reved = String::new();

    //for loop to reverse the string 
    for c in input.chars().rev() {
        reved += &c.to_string();
    }
   return reved;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_str() {
        assert_eq!(rev_str("hello"), "olleh");
    }
}
