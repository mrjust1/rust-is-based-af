use std::io;
use rand::Rng;

fn main()
{

    let num_to_g = rand::thread_rng()
        .gen_range(1..=100);
    let mut guess = String::new();

    while (num_to_g != guess)
    {
    println!("pick a new number:");

    io::stdin()
        .read_line(&mut guess)
        .expect("failed");
    
    println!("your guess is wrong but");
    }
    println!("oh you win!!");

}
