pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let  div:i32;
    let  rem:i32;
    div = (x/y).abs();
    rem = x%y;
    return (div, rem);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(9,4),(2,1));
    }
}
