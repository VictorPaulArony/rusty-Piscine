pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let num: Vec<u32> = s.split_whitespace()
        .filter_map(|c| { 
            if c.contains('k'){
                let digits: String = c.chars().filter(|ch| ch.is_digit(10) || *ch == '.').collect();
                digits.parse::<f32>().ok().map(|n| (n * 1000.0) as u32) 
            }else {
                c.parse::<u32>().ok()
            }
        })
        .collect();
        Box::new(num)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a//dereference the box
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let new_str = String::from("5.5k 8.9k 32");
        
        let a_h = transform_and_save_on_heap(new_str);
      
        let a_b_v = take_value_ownership(a_h);
       
        assert_eq!(a_b_v, [5500, 8900, 32]);
    }
}
