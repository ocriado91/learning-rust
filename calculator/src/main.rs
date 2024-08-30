use std::io; // Use the I/O library
use calculator::calculator::parse_to_float;
use std::process; // To return error codes


fn get_input(prompt: &str) -> String {
    // Define a mutable input variable (the true value is defined later).
    let mut input = String::new();

    // Show the prompt to the user
    println!("{}", prompt);

    // Retrieve the incoming value and stored into input variable.
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Return value
    input
}

fn main() {
    println!("Welcome to the Rust Calculator!");

    // Enter the fist number
    let first_operator = get_input("Enter the first number: ");
    let num1 = match parse_to_float(&first_operator) {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
            // continue;
        }
    };

    // Enter the operator
    let operator = get_input("Enter an operator (+, -, *, /): ");

    // Enter the second number
    let second_operator = get_input("Enter the second number: ");
    let num2 = match parse_to_float(&second_operator) {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
            // continue;
        }
    };

    // Perform the calculation
    let result = match operator.trim() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("ERROR: Division by zero not allowed!");
                process::exit(1);
            }
        }
        _ => {
            println!("Invalid operator.");
            process::exit(1);
        }
    };

    println!("Result: {}", result);
}