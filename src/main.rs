use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {
    game();
}
fn game() {
    let correct =rand::rng().random_range(1..101);
    println!("The correct number is: {}", correct);
loop {
    println!("Hey, guess a number:");
    let mut guessed_number= String::new();
    io::stdin()
        .read_line(&mut guessed_number)
        .expect("Failed to read line");
    let guessed_number:u32 = guessed_number.trim().parse().expect("Please type a number!");

    match guessed_number.cmp(&correct)
    {
        Ordering::Greater=> println!("You guessed too high"),
        Ordering::Less => println!("you guessed too low"),
        Ordering::Equal =>  {
           println!("you guessed correctly");
            break;
        }
    };
}
}
fn practice() {
    let name = "Tommy";
    let last:&str = "MyLastNameski";
    //immutable = cannot change
    //default string I.E let mystring = mytext IS IMUTIBLE CANNOT CHANGE STATIC
    //to make dynamic let mystring = String::from("my text")
    //only put variable names inside the placeholder {Variable_name} if it is the only thing
    //anything else like .tostring means put var outside placeholder {}, Variable_name
    println!("Hello my full name is, {name} {}!", last.to_lowercase());

    let data = [1,2,3,4,5];
    println!("{data:?} 67");
}
