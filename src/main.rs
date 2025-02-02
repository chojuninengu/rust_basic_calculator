// src/main.rs
mod operations;
mod parser;
mod tokenizer;
use std::io::{self, Write};

fn main() {
    println!("Welcome to the Rust CLI Calculator!");
    loop {
        // Get user input
        print!("Enter expression (e.g., '3 + 5 * 2 - 8 / 4'): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            println!("Input cannot be empty. Please try again.");
            continue;
        }

        // Parse and evaluate the expression
        match parser::evaluate_expression(input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
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