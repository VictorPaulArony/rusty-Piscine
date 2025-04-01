
pub fn sum(left: u8, right: u8) -> u8 {
    left + right
}

pub fn diff(left: i32, right: i32) -> i32 {
    left - right
}

pub fn pro(left: i8, right: i8) -> i8 {
    left * right
}

pub fn quo(left: f32, right: f32) -> f32 {
    left / right
}

pub fn rem(left: f32, right: f32) -> f32 {
    left % right
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(234, 2), 236);
        // cause an overflow panic
        // assert_eq!(sum(1, 255), 256);
    }

    #[test]
    fn test_diff() {
        assert_eq!(diff(234, 2), 232);
        // cause an overflow panic
        // assert_eq!(diff(-32768, 32766), -65534);
    }

    #[test]
    fn test_pro() {
        assert_eq!(pro(23, 2), 46);
        //cause an overflow panic
        // assert_eq!(pro(-128, 2), -256);
    }

    #[test]
    fn test_quo() {
        assert_eq!(quo(22.0, 2.0), 11.0);
        assert_eq!(quo(-128.23, 2.0), -64.115);
    }

    #[test]
    fn test_rem() {
        assert_eq!(rem(22.0, 2.0), 0.0);
        assert!((rem(-128.23, 2.0) - -0.22999573).abs() < 1e-6);
    }
}