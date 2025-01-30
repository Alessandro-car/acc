mod lexer;

fn main() {
    let input: String = String::from("{()()};");
    let tokens: Vec<String> = lexer::lex(input);
    println!("{:#?}", tokens);
}
