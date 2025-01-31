use std::io::{self, Write};

fn main() {
    println!("Welcome to the Rust CLI Calculator!");
    let mut history: Vec<String> = Vec::new();

    loop {
        // Get user input
        let num1 = get_number("Enter first number: ");
        let operation = get_operation();
        let num2 = if operation != "!" {
            Some(get_number("Enter second number (if applicable): "))
        } else {
            None // Factorial doesn't require a second number
        };

        // Perform calculation
        match calculate(num1, &operation, num2) { // Pass operation as a reference
            Ok(result) => {
                let log = if operation == "!" {
                    format!("{}! = {}", num1, result)
                } else {
                    format!(
                        "{} {} {} = {}",
                        num1,
                        operation,
                        num2.unwrap_or(0.0),
                        result
                    )
                };
                println!("Result: {}", result);
                history.push(log);
            }
            Err(e) => println!("Error: {}", e),
        }

        // Show history
        println!("\nCalculation History:");
        for entry in &history {
            println!("{}", entry);
        }

        // Ask to continue or exit
        print!("\nDo you want to perform another calculation? (y/n): ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        if choice.trim().to_lowercase() != "y" {
            break;
        }
    }

    println!("Thank you for using the Rust CLI Calculator!");
}

fn get_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn get_operation() -> String {
    loop {
        print!("Enter operation (+, -, *, /, ^, %, !): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let operation = input.trim();
        if ["+", "-", "*", "/", "^", "%", "!"].contains(&operation) {
            return operation.to_string();
        } else {
            println!("Invalid operation. Please enter one of +, -, *, /, ^, %, !");
        }
    }
}

fn calculate(num1: f64, operation: &str, num2: Option<f64>) -> Result<f64, String> {
    match operation {
        "+" => num2.map_or(Err("Missing second number.".to_string()), |n| Ok(num1 + n)),
        "-" => num2.map_or(Err("Missing second number.".to_string()), |n| Ok(num1 - n)),
        "*" => num2.map_or(Err("Missing second number.".to_string()), |n| Ok(num1 * n)),
        "/" => {
            num2.map_or(
                Err("Missing second number.".to_string()),
                |n| {
                    if n == 0.0 {
                        Err("Division by zero is not allowed.".to_string())
                    } else {
                        Ok(num1 / n)
                    }
                },
            )
        }
        "^" => num2.map_or(Err("Missing second number.".to_string()), |n| Ok(num1.powf(n))),
        "%" => {
            num2.map_or(
                Err("Missing second number.".to_string()),
                |n| {
                    if n == 0.0 {
                        Err("Modulus by zero is not allowed.".to_string())
                    } else {
                        Ok(num1 % n)
                    }
                },
            )
        }
        "!" => {
            if num1 < 0.0 || num1.fract() != 0.0 {
                Err("Factorial is only defined for non-negative integers.".to_string())
            } else {
                Ok(factorial(num1 as u64) as f64)
            }
        }
        _ => Err("Invalid operation.".to_string()),
    }
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}