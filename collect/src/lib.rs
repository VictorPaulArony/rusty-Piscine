pub fn bubble_sort(arr: &mut [i32]) {
    let l = arr.len();
    for i in 0..l{
        for j in 0..l-i-1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = [3, 2, 4, 5, 1, 7];
        let mut v_clone = v;

        bubble_sort(&mut v);
        v_clone.sort_unstable();
        assert_eq!(v, v_clone);
    }
}
