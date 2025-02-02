mod lexer;

fn main() {
    let input: String = String::from("#include int a = 5;");
    let mut lexer = lexer::Lexer::new(input.chars().collect());
    lexer.read_char();
    loop {
        let token = lexer.next_token();
        if token == lexer::TokType::EOF {
            break;
        } else {
            println!("{:?}", token);
        }
    }
}
