pub fn square_of_sum(n: u32) -> u32 {
    let sum = (1..=n).sum::<u32>(); 
    sum * sum 
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x * x).sum::<u32>() 
}

pub fn difference(n: u32) -> u32 {
    let square_of_sum = square_of_sum(n); 
    let sum_of_squares = sum_of_squares(n); 
    square_of_sum - sum_of_squares 
}
