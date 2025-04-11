pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let is_question = text.trim_end().ends_with('?');
    let is_exclamation = text.trim_end().ends_with('!');
    let is_all_upper = text.chars().any(|c| c.is_alphabetic())
        && text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());

    match (is_all_upper, is_exclamation, is_question) {
        (true, true, _) => "There is no need to yell, calm down!",
        (true, false, true) => "Quiet, I am thinking!",
        (_, _, true) => "Sure.",
        _ => "Interesting",
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        assert_eq!(talking("JUST DO IT!"), "There is no need to yell, calm down!");
        assert_eq!(talking("Hello how are you?"), "Sure.");
        assert_eq!(talking("WHAT'S GOING ON?"), "Quiet, I am thinking!");
        assert_eq!(talking("something"), "Interesting");
        assert_eq!(talking(""), "Just say something!");
    }
}
