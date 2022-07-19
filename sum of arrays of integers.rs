use std::io;
fn main() {
    println!("Program to find the sum of n numbers of integers");
//input are the integers 
let mut input = String::new();
println!("input the numbers separated by a single space");

io::stdin()
.read_line (&mut input )
.expect ("unable to read line");

 let array: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

 let mut sum:i32 = 0;
 //method one
 let sum_2 :i32 = array.iter().sum();
 //method two
 for n in &array{
     sum += n
 }
 println!("{}", sum_2);
}
