//Crates and modules
use std::io;
use std::process;

fn main() {
    //Values input
    println!("Input the first value");
    let mut value1 = String::new();
    io::stdin().read_line(&mut value1).expect("Failed to read line");

    println!("Input the second value");
    let mut value2 = String::new();
    io::stdin().read_line(&mut value2).expect("Failed to read line");

    //Type casting
    let number1: i64 = value1.trim().parse().unwrap();
    let number2: i64 = value2.trim().parse().unwrap();

    //Operation input
    println!("Input the operation of your choice (+/-/*/:)");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");

    //Result calculation
    let outcome: i64;
    if operation.trim() == "+"{
        outcome = number1 + number2;
    }
    else if operation.trim() == "-" {
        outcome = number1 - number2;
    }
    else if operation.trim() == "*" {
        outcome = number1 * number2;
    }
    else if operation.trim() == "/" || operation.trim() == ":" {
        outcome = number1 / number2;
    }
    else{
        println!("Invalid operation");
        process::exit(0);
    }

    //Result printout
    println!("{}", outcome);

    //Exit
    println!("Press enter to exit");
    let mut exit = String::new();
    io::stdin().read_line(&mut exit).expect("Failed to read line");
}
