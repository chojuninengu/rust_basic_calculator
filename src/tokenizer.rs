#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_whitespace() {
            continue; // Ignore whitespace
        } else if c.is_digit(10) || c == '.' {
            current_number.push(c); // Build number
        } else if "+-*/()".contains(c) {
            if !current_number.is_empty() {
                tokens.push(Token::Number(
                    current_number.parse::<f64>().map_err(|_| "Invalid number")?,
                ));
                current_number.clear();
            }
            tokens.push(match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Multiply,
                '/' => Token::Divide,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                _ => unreachable!(),
            });
        } else {
            return Err(format!("Invalid character: {}", c));
        }
    }

    if !current_number.is_empty() {
        tokens.push(Token::Number(
            current_number.parse::<f64>().map_err(|_| "Invalid number")?,
        ));
    }

    Ok(tokens)
}