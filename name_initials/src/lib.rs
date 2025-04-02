pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res:Vec<String> = Vec::new();
    for n in names {
    
        let parts: Vec<&str> = n.split_whitespace().collect();
        let first = parts[0].chars().next().unwrap_or(' ');
        let last = if parts.len()> 1 {
            parts[parts.len()-1].chars().next().unwrap_or(' ')
        }else {
            ' '
        };
        res.push(format!("{}. {}.", first, last));
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        assert_eq!(initials(names), vec!["H. P.", "S. E.", "J. L.", "B. O."]);
    }
}
