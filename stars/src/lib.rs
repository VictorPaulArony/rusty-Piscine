pub fn stars(n: u32) -> String {
    let result = String::from("*");
    let s = 2u32.pow(n);
    result.repeat(s.try_into().unwrap())
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    assert_eq!(stars(1), "**");
    assert_eq!(stars(4), "****************");
    }
}
