pub fn number_logic(num: u32) -> bool {
    let size = num.to_string().len() as u32;
    let st = num.to_string();
    let sum: u32= st.chars().filter_map(|d| d.to_digit(10)).map(|n| n.pow(size)).sum();
    sum == num

}
