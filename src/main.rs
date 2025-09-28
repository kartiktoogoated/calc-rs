use std::io;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Num(f64),
    Op(char),
    LParen,
    RParen,
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut num = String::new();

    for c in expr.chars() {
        if c.is_ascii_digit() || c == '.' {
            num.push(c);
        } else {
            if !num.is_empty() {
                tokens.push(Token::Num(num.parse().unwrap()));
                num.clear();
            }

            match c {
                '+' | '-' | '*' | '/' | '^' => tokens.push(Token::Op(c)),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                ' ' => (), // ignore spaces
                _ => panic!("Unexpected character: {}", c),
            }
        }
    }

    if !num.is_empty() {
        tokens.push(Token::Num(num.parse().unwrap()));
    }

    tokens
}

fn eval(tokens: Vec<Token>) -> i32 {
    let mut temp: Vec<Token> = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Num(n) => temp.push(Token::Num(*n)),
            Token::Op(op) if *op == '*' || *op == '/' => {
                if let Token::Num(left) = temp.pop().unwrap() {
                    if let Token::Num(right) = tokens[i + 1] {
                        let val = if *op == '*' {
                            left * right
                        } else {
                            left / right
                        };
                        temp.push(Token::Num(val));
                    }
                }
                i += 1;
            }
            Token::Op(op) => temp.push(Token::Op(*op)),
        }
        i += 1;
    }

    let mut result = if let Token::Num(n) = temp[0] { n } else { 0 };
    i = 1;
    while i < temp.len() {
        if let Token::Op(op) = temp[i] {
            if let Token::Num(n) = temp[i + 1] {
                result = match op {
                    '+' => result + n,
                    '-' => result - n,
                    _ => result,
                }
            }
        }
        i += 2;
    }
    result
}

fn main() {
    println!("Enter an expression:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let tokens = tokenize(input.trim());
    println!("Tokens: {:?}", tokens);

    let result = eval(tokens);
    println!("Result: {}", result);
}
