pub fn rotate(input: &str, key: i8) -> String {
    let res:String = input.chars()
        .map(|c| match c{
                'a'..='z' => {
                    ((c as u8 - b'a' + ((key %26 + 26)%26) as u8) %26 + b'a')as char
                }
                'A'..='Z' => {
                    ((c as u8 - b'A' + ((key %26 + 26)%26) as u8)%26 +b'A')as char
                }
                _ => c,
        })
        .collect();
        res as String
}
