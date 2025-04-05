pub fn capitalize_first(input: &str) -> String {
    let mut ch = input.chars();
    match ch.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + ch.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    input
    .split_whitespace()
    .map(|word| {
        let mut chars = word.chars();
        match chars.next() {
            None => String::new(),
            Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
        }
    })
    .collect::<Vec<_>>()
    .join(" ")
}

pub fn change_case(input: &str) -> String {
    input.chars().map(|ch| {
        if ch.is_lowercase(){
            ch.to_uppercase().to_string()
        }else{
            ch.to_lowercase().to_string()
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        assert_eq!(capitalize_first("joe is missing"),"Joe is missing");
        assert_eq!(title_case("jill is leaving A"), "Jill Is Leaving A");
        assert_eq!(change_case("heLLo THere"), "HEllO thERE");
    }
}
