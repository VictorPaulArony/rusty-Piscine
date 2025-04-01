use std::io;

fn main() {
    let mut input = String::new();
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let ans = "the letter e";
    let mut atmp = 0;

    loop {
        println!("{}", riddle);
        //read the input 
        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read");

        atmp += 1;

        let mut  userinput = input.trim().to_lowercase();
        if checker(&mut userinput, ans) {
            println!("Number of trials: {}", atmp);
            break;
        }else{
            input.clear();
        }
    }
   
}

//function to check if input and answer are same
fn checker(input: &str, ans:&str) -> bool {
    input.trim().to_lowercase() == ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker(){
        assert_eq!(checker("the letter e", "the letter e"), true);
        assert_eq!(checker("letter e", "the letter e"), false);
    }
}

