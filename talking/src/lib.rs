pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    // Check if the input is empty or contains only whitespace
    if trimmed.is_empty() {
        return "Just say something!";
    }

    // Check if the text is yelling (all uppercase and ends with '!')
    if trimmed.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()) && trimmed.ends_with('!') {
        return "There is no need to yell, calm down!";
    }

    // Check if the text is a question (ends with '?')
    if trimmed.ends_with('?') {
        // Check if the question is yelled (contains uppercase letters)
        if trimmed.chars().any(|c| c.is_uppercase()) {
            return "Quiet, I am thinking!";
        }
        return "Sure.";
    }

    // Default response for other cases
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
        assert_eq!(talking("    "), "Just say something!"); // Handles whitespace-only input
    }
}