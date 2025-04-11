pub fn num_to_ordinal(x: u32) -> String {
    if x == 0 {
        return "0".to_string();
    }

    let nth = match x%100 {
        11| 12|13 => "th",
        _ => match x%10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        },
    };
    x.to_string() + nth

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        assert_eq!(num_to_ordinal(1), "1st");
        assert_eq!(num_to_ordinal(22), "22nd");
        assert_eq!(num_to_ordinal(43), "43rd");
        assert_eq!(num_to_ordinal(47), "47th");
    }
}
