pub fn first_subword( s: String) -> String {
    let mut res = String::new();
    for ch in s.chars() {
        if ch == '_' || (ch.is_uppercase() && res.len() > 0){
            break;
        }else {
            res.push(ch);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = "helloWorld".to_owned();
        let s2 = "snake_case".to_owned();
        let s3 = "CamelCase".to_owned();
        let s4 = "just".to_owned();

        assert_eq!(first_subword(s1), "hello");
        assert_eq!(first_subword(s2), "snake");
        assert_eq!(first_subword(s3), "Camel");
        assert_eq!(first_subword(s4), "just");
    }
}
