pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return "Just say something!";
    }

    let is_all_upper = trimmed.chars().any(|c| c.is_alphabetic())
        && trimmed.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());

    if is_all_upper && trimmed.ends_with('!') {
        return "There is no need to yell, calm down!";
    }

    if trimmed.ends_with('?') {
        if is_all_upper {
            return "Quiet, I am thinking!";
        } else {
            return "Sure.";
        }
    }

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