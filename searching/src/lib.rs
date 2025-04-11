pub fn search(array: &[i32], key: i32) -> Option<usize> {
   Some(array.binary_search(&key).ok()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ar = [1, 3, 4, 6, 8, 9, 11];
        let f = search(&ar, 6);
        assert_eq!(f, Some(3));
    }
}
