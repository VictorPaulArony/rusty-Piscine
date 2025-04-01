pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val)
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    &slice[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    //helper func for the testing
    fn create_vector() -> Vec<String> {
        vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "bread".to_string(),
            "cheese".to_string(),
        ]
    }

    #[test]
    fn test_insert(){
        let mut groceries = create_vector();
        insert(&mut groceries , "nuts".to_string());
        assert_eq!( groceries,
            vec![
                "yogurt".to_string(),
                "panettone".to_string(),
                "bread".to_string(),
                "cheese".to_string(),
                "nuts".to_string()
            ]
        );
    }

    #[test]
    fn test_index(){
        let groceries = create_vector();
        assert_eq!(at_index(&groceries, 1),"panettone" )
    }
    
}
