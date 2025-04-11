pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }
   
    // Check if it's a yelling exclamation (uppercase + !)
    if text.trim().ends_with("!") && text.trim().chars().any(|c| c.is_alphabetic()) && 
       text.trim().chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
        return "There is no need to yell, calm down!";
    }

    // Check if it's a question
    if text.trim().ends_with("?") {
        // Check if the question is in all uppercase
        if text.trim().chars().any(|c| c.is_alphabetic()) && 
           text.trim().chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
            return "Quiet, I am thinking!";
        } else {
            return "Sure.";
        }
    }
    
    // Default response
    "Interesting"
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