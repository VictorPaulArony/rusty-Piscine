pub fn factorial(num: u64) -> u64 {
    let mut res = 1;
    if num == 0 {
        return res;
    };
    for i in 1..= num {
        res *= i;
    };
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(19),121645100408832000 );
        assert_eq!(factorial(5),120 );
        assert_eq!(factorial(10),3628800);
        assert_eq!(factorial(1),1);
    }
}
