 use std::io;

fn main(){ 

    println!("guess the number");
    
    let mut guess = String::new();

    io::stdin()
 .read_line(&mut guess);
 
  println!("u guessed right,{guess}");
}