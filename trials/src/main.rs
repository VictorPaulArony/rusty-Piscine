fn main() {
    let nums = vec![1,2,3,4,5,6,7,8,9,0];
    let s:Vec<_> = nums.into_iter().map(|d| d.to_string()).collect();
    println!("{:?}", s);

    let mut  dig:Vec<i32> = s.iter()
        .flat_map(|c| c.chars())
        .filter_map(|ch| ch.to_digit(10))
        .map(|d| (d)as i32).collect();
    println!("{:?}", dig);


    let borrows_prime:Vec<i32> = dig.clone()
        .into_iter()
        .filter(|&d| is_prime(d)).collect();
        println!("{:?}", borrows_prime);


    let prime:Vec<i32> = dig.iter().filter(|&&d| is_prime(d)).map(|&d| d).collect();
    println!("{:?}", prime);



    let odd:Vec<i32> = dig.iter_mut()
        .filter(|d| **d%2 != 0)// Dereference mutable reference
        .map(|d| *d)// Dereference to get the value
        .collect();
    println!("{:?}", odd);

    let n:Vec<i32> = dig.iter().filter(|&&d| d%2 == 0).map(|&d| d).collect();
    println!("{:?}", n);
    //an iter gives a reference to the original value (&value)
    //to use a referencer as value you dereference it (&&value)
    //map(iterator) and dereferance the value(&value) to get the actual value for storage


    //filter => is a boolean return checker
    //filter_map => is an option<> and keeps only elements that return Some(value)
    //and transform them into value and discard the None.

    //into_iter() => takes the ownership ot the elements(used when the original data is not needed)
    //iter() => borrows the elements of the collection
}

//function to check for prime numbers
pub fn is_prime(n: i32) -> bool {
    if n <= 2 {
        return false;
    }
    !(2..=(n.isqrt() as i32 )).any(|i| n % i == 0)
}
