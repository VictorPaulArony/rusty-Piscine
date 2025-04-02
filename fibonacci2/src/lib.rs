pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }else if n == 1 {
        return 1;
    }else {
        return (fibonacci(n-1)) + fibonacci(n-2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(22), 17711);
        assert_eq!(fibonacci(20), 6765);
        assert_eq!(fibonacci(0), 0);
    }
}
