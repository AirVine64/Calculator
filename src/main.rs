//Crates and modules
use std::io;
use std::process;

fn exit_test(var: String) {
    if var.trim() == "exit" || var.trim() == "Exit" {
        process::exit(0);
    }
}

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
        //Values input
        println!("Input the first value");
        io::stdin().read_line(&mut value1).expect("Failed to read line");
        //exit_test(value2);

        println!("Input the second value");
        io::stdin().read_line(&mut value2).expect("Failed to read line");
        //exit_test(value2);

        //Type casting
        number1 = value1.trim().parse().unwrap();
        number2 = value2.trim().parse().unwrap();

        //Operation input
        println!("Input the operation of your choice (+/-/*/:/^)");
        io::stdin().read_line(&mut operation).expect("Failed to read line");
        //exit_test(operation);

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
