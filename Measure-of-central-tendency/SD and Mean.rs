use std::io;
fn main (){
    println!("Program to compute the average and Standard deviation of numbers");
    let mut numbers = String::new();

    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");

    let numbers: Vec<f32> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    
    let sum :f32 = numbers.iter().sum();
    println! ("The sum of the numbers is {}", sum); 
    let mut n = 0.0;
    for _num in 0..numbers.len(){
        n += 1.0
    }
    println!("the value of n is {}", n);
    
    let mean :f32 = sum/n;
    println!("the mean is {}", mean);
    
    let mut square:f32 = 0.0;
    for _num in numbers.iter(){
        square += (_num - mean).powf(2.0);
} 
    let variance:f32 = square/n;
    
    let std_deviation:f32 = variance.sqrt();
    println!("The standard deviation is : {}", std_deviation);
}
