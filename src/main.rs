//Crates and modules
use std::io;
use std::process;

fn main() {
    //Variables declarations
    let mut value1;
    let mut value2;
    let mut operation;
    let mut number1: f64;
    let mut number2: f64;
    let mut outcome: f64;

    //Main program loop
    loop {
        //Values input and type casting
        println!("Input the first value");
        value1 = String::new();
        io::stdin().read_line(&mut value1).expect("Failed to read line");
        if value1.trim() == "exit" {
            process::exit(0);
        }
        else {
            number1 = value1.trim().parse().unwrap();
        }

        println!("Input the second value");
        value2 = String::new();
        io::stdin().read_line(&mut value2).expect("Failed to read line");
        if value2.trim() == "exit" {
            process::exit(0);
        }
        else {
            number2 = value2.trim().parse().unwrap();
        }

        //Operation input and result calculation
        loop {
            println!("Input the operation of your choice (+/-/*/:/^)");
            operation = String::new();
            io::stdin().read_line(&mut operation).expect("Failed to read line");
            if operation.trim() == "exit" {
                process::exit(0);
            }

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
                outcome = number1.powf(number2);
                break;
            }
            else{
                println!("Invalid operation");
            } 
        }

        //Result printout
        println!("Result:{}", outcome);
    }
}
