mod lexer;

fn main() {
    let input: String = String::from("if { } else { }");
    let mut lexer = lexer::Lexer::new(input.chars().collect());
    lexer.read_char();
    loop {
        let token = lexer.next_token();
        if token == lexer::tok_type::EOF {
            break;
        } else {
            println!("{:?}", token);
        }
    }
}
