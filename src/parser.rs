use crate::operations::{add, subtract, multiply, divide};
use crate::tokenizer::{Token, tokenize};

pub fn evaluate_expression(input: &str) -> Result<f64, String> {
    let tokens = tokenize(input)?;
    let rpn = shunting_yard(tokens)?;
    evaluate_rpn(rpn)
}

fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Plus | Token::Minus => {
                while let Some(op) = operators.last() {
                    if matches!(op, Token::Plus | Token::Minus | Token::Multiply | Token::Divide) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
            Token::Multiply | Token::Divide => {
                while let Some(op) = operators.last() {
                    if matches!(op, Token::Multiply | Token::Divide) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
            Token::LeftParen => operators.push(token),
            Token::RightParen => {
                while let Some(op) = operators.pop() {
                    if let Token::LeftParen = op {
                        break;
                    }
                    output.push(op);
                }
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    Ok(output)
}

fn evaluate_rpn(rpn: Vec<Token>) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in rpn {
        match token {
            Token::Number(num) => stack.push(num),
            Token::Plus => {
                let b = stack.pop().ok_or("Insufficient operands")?;
                let a = stack.pop().ok_or("Insufficient operands")?;
                stack.push(add(a, b)?);
            }
            Token::Minus => {
                let b = stack.pop().ok_or("Insufficient operands")?;
                let a = stack.pop().ok_or("Insufficient operands")?;
                stack.push(subtract(a, b)?);
            }
            Token::Multiply => {
                let b = stack.pop().ok_or("Insufficient operands")?;
                let a = stack.pop().ok_or("Insufficient operands")?;
                stack.push(multiply(a, b)?);
            }
            Token::Divide => {
                let b = stack.pop().ok_or("Insufficient operands")?;
                let a = stack.pop().ok_or("Insufficient operands")?;
                stack.push(divide(a, b)?);
            }
            _ => return Err("Invalid token in RPN evaluation.".to_string()),
        }
    }

    stack.pop().ok_or("No result".to_string())
}