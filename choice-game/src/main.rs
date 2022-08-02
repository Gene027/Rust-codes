use std::io;
fn main() { 
println!("comparing the triplet game");
loop{
 let mut player_1_in = String::new();
 println!("player 1 \t(input three numbers separated with a space)"); 
 io::stdin()
 .read_line(&mut player_1_in)
 .expect("unable to read");

let player_1:Vec<u32> = player_1_in.split_whitespace()
.map(|s| s.parse().expect("parse error"))
.collect();

 let mut player_2_in = String::new();
 println!("player 2 \t(input three numbers separated with a space)"); 
 io::stdin()
 .read_line(&mut player_2_in)
 .expect("unable to read, separate the numbers with only one space");

let player_2:Vec<u32> = player_2_in.split_whitespace()
.map(|s| s.parse().expect("parse error"))
.collect();

let mut acct_player_1:u32 = 50;
let mut acct_player_2:u32 = 50;
if acct_player_1 >= 100{
    panic! ("player one account maxed out");
}
if acct_player_2 >= 100{
    panic! ("player two account maxed out");
}

if player_1.len() != player_2.len(){
    panic! ("input only three numbers");
}
//GAME LOGIC
let mut player_1_score:u32 = 0;
let mut player_2_score:u32 = 0;

for i in 0..player_1.len(){
 if player_1[i] > player_2[i] {
player_1_score += 1;
 }
 else if player_1[i] < player_2[i] {
        player_2_score += 1;
 }
 else {
     continue;
 }
}

if player_1_score > player_2_score{
acct_player_1 += 10;
    acct_player_2 -= 10;
    println! ("player 1 wins!");
    }
else if player_1_score < player_2_score{
    acct_player_1 -= 10;
    acct_player_2 += 10;
    println! ("player 2 wins!");
    }
else if player_1_score == player_2_score{
    println! ("Draw Game!");
    }
 println!("Account balances are player_1: {} player_2: {}", acct_player_1, acct_player_2);

 //REPLAY
 let mut reply = String:: new();
 println!("Do you want to replay? \t(input yes or no)");
 io::stdin()
 .read_line(&mut reply)
 .expect("unable to read");

let _yes = ["yes", "Yes", "Y"];
let no = ["no","No","N"];
let mut logic = 9;
for a in no.iter(){
    if reply.contains(a){
        logic += 1;
    }
}
if logic >=10 {
    break;
}
}
}