pub fn mean(list: &[i32]) -> f64 {
   let total:i32 =  list.iter().sum();
    total as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort_unstable();

    let middle = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        ((sorted[middle - 1] + sorted[middle]) / 2) as i32
    }else {
        sorted[middle] as i32
    }
}

use std::collections::HashMap;
pub fn mode(list: &[i32]) -> i32 {
    let mut pop = HashMap::new();

    // Count the frequency of each element in the list
    for &v in list {
        let count = pop.entry(v).or_insert(0);
        *count += 1;
    }

    // Find the element with the maximum frequency
    let mut max_count = 0;
    let mut mode_value = list[0];
    for (&k, &v) in &pop {
        if v > max_count {
            max_count = v;
            mode_value = k;
        }
    }

    mode_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        assert_eq!(mean(&v), 3.857142857142857);
        assert_eq!(median(&v), 4);
        assert_eq!(mode(&v), 5);
    }
}
