use std::io;

fn main() {
    println!("Enter expression:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut num = String::new();
    let mut result = 0;
    let mut sign = 1;

    for c in input.chars() {
        if c.is_ascii_digit() {
            num.push(c);
        } else if c == '+' || c == '-' {
            if !num.is_empty() {
                let val: i32 = num.parse().unwrap();
                result += sign * val;
                num.clear();
            }
            sign = if c == '+' { 1 } else { -1 };
        }
    }

    if !num.is_empty() {
        let val: i32 = num.parse().unwrap();
        result += sign * val;
    }

    println!("Result: {}", result);
}
