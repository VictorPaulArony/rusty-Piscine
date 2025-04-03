pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let num: f64= c as f64;
    let expo = num.exp();
    let log = num.abs().ln();
    (c, expo, log)

}

pub fn str_function(a: String) -> (String, String) {
    let mut expo = String::new();
    for n in a.chars() {
        if n == ' ' {
            continue 
        }
        let num = n.to_string().parse::<f64>().ok();
        let st = num.expect("Fail").exp();
        expo.push_str(&st.to_string());
        expo.push_str(" ")
    }
    (a, expo.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res:Vec<f64> = Vec::new();
    for n in &b {
        let num: f64= *n as f64;
        let log = num.abs().ln();
        res.push(log)
    }
    (b, res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = "1 2 4 5 6".to_owned();
        let b = vec![1, 2, 4, 5];
        let c = 0;
        assert_eq!(str_function(a), ("1 2 4 5 6".to_string(), "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_string()));
        assert_eq!(nbr_function(c), (0, 1.0, f64::NEG_INFINITY));
        assert_eq!(vec_function(b), (vec![1, 2, 4, 5], vec![0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003]));
    }
}
