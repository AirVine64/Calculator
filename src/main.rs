//Crates and modules
use std::io;
use std::process;

fn main() {
    //Variables declarations
    let mut value1 = String::new();
    let mut value2 = String::new();
    let mut operation = String::new();
    let mut number1: i128;
    let mut number2: i128;
    let mut outcome: i128;

    //Main program loop
    loop {
        //Values input and type casting
        println!("Input the first value");
        io::stdin().read_line(&mut value1).expect("Failed to read line");
        if value1.trim() == "exit" {
            process::exit(0);
        }
        else {
            number1 = value1.trim().parse().unwrap();
        }

        println!("Input the second value");
        io::stdin().read_line(&mut value2).expect("Failed to read line");
        if value2.trim() == "exit" {
            process::exit(0);
        }
        else {
            number2 = value2.trim().parse().unwrap();
        }

        //Operation input
        println!("Input the operation of your choice (+/-/*/:/^)");
        io::stdin().read_line(&mut operation).expect("Failed to read line");
        if operation.trim() == "exit" {
            process::exit(0);
        }

        //Result calculation
        loop {
            if operation.trim() == "+"{
                outcome = number1 + number2;
                break;
            }
            else if operation.trim() == "-" {
                outcome = number1 - number2;
                break;
            }
            else if operation.trim() == "*" {
                outcome = number1 * number2;
                break;
            }
            else if operation.trim() == "/" || operation.trim() == ":" {
                outcome = number1 / number2;
                break;
            }
            else if operation.trim() == "^"{
                outcome = number1.pow(number2 as u32);
                break;
            }
            else{
                println!("Invalid operation");
            } 
        }

        //Result printout
        println!("{}", outcome);
    }
}
