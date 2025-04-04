
pub fn min_max(n1:i32, n2:i32, n3:i32) -> (i32, i32) {
    let mut min = n1;
    let mut max = n1;
    
    if n1 > max {
        max = n1;
    }
    if n3 > max {
        max = n3;
    }
    if n2 > max {
        max = n2;
    }

    if n1 < min {
        min = n1;
    }
    if n2 < min {
        min = n2;
    }
    if n3 < min {
        min = n3;
    }
(max, min)
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        assert_eq!(min_max(10, 130, 20), (130, 10));
    }
}
