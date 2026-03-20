#[allow(unused_variables)]
use std::io::{self,Write};

fn main() {
    // let y :i32;
    // let mut x:i32 = 1;
    // x += 2;
    // println!("The value of x is {}",x);
    // assert_eq!(x,3);
    // println!("success");
    guess();

}

fn guess() {
    println!("Guess The Number!");
    print!("Enter your Guess: ");

    io::stdout().flush().expect("could not flush");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
}