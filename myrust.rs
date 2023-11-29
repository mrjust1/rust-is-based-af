use std::io;
fn main()
{
    println!("pick a new number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed");
    println!("your number is: {guess}");

}
