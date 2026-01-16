use std::cmp::Ordering;
use std::io;
use rand::Rng;
use std::vec::Vec;
fn main() {
    chap_1();
}
fn chap_1() {
    let mut how_many = String::new();
    println!("how many numbers would you like to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Failed to read line");
    let how_many: u8 =  how_many.trim().parse().expect("error reading input");
    let mut correct = Vec::new();

    for _u8 in 0..how_many {
        correct.push(rand::rng().random_range(1..101));
    }
    println!("{correct:?}");
/*
    //println!("The correct number is: {}", correct);
loop {
    println!("Hey, guess a number:");
    let mut guessed_number= String::new();
    io::stdin()
        .read_line(&mut guessed_number)
        .expect("Failed to read line");

    let guessed_number:u32 = match guessed_number.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Error with parse please try again");
            continue;
        }
    };

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

fn chap_2() {
    println!("Hello, world!");
    */
}
