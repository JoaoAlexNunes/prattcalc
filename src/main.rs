mod includes;

use includes::{lexer::lex, parser::parse};
use std::io::Write;

pub fn user_input() -> String {
    let mut input = String::new();
    print!("Enter your number: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    return input;
}

fn main() {
    loop {
        let source = user_input();

        if source.contains("exit") {
            break;
        }

        let tokens = lex(&source);
        if tokens.len() == 1 {
            println!("Please only right numbers and operators");
            continue;
        }
        let ast = parse(&tokens, &mut 0, 0);

        println!("{}", ast);
        println!("Eval: {}", ast.eval())
    }
}
