use std::io;
fn main()
{
    println!("Hey, guess a number:");
    let mut guessed_number:String = String::new();
    io::stdin().read_line(&mut guessed_number).expect("Failed to read line");
    println!("You guessed: {}", guessed_number.trim());
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
