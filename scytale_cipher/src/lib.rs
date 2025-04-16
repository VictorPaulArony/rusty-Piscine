
pub fn scytale_cipher(s: String, i: u32) -> String {
    if i > (s.len() as u32) {
        panic!("Index out of bounds");
    }
    let (s1, s2) = s.split_at(i as usize);
    (s1.to_string(), s2.to_string());
    let res = interleave_strings(&s1, &s2);
    res

}

pub fn interleave_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    let mut iter1 = s1.chars();
    let mut iter2 = s2.chars();

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(c1), Some(c2)) => {
                result.push(c1);
                result.push(c2);
            }
            (Some(c1), None) =>{
                result.push(c1);
                result.push(' ');
            } 
            (None, Some(c2)) => {
                result.push(' ');
                result.push(c2);
            }
            (None, None) => break,
        }
    }

    result.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(scytale_cipher(String::from("scytale Code"), 6), "sec yCtoadle");
        assert_eq!(scytale_cipher(String::from("scytale Code"), 8), "sCcoydtea l e");
    }
}