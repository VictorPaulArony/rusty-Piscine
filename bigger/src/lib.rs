use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut bigger:i32 = 0;
    for (_, v) in h {
        if v > bigger {
            bigger = v;
        }
    }
    bigger
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let hash = HashMap::from_iter([
            ("Daniel", 122),
            ("Ashley", 333),
            ("Katie", 334),
            ("Robert", 14),
        ]);
        assert_eq!(bigger(hash), 334);
    }
}
