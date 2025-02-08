use std::fs::File;
mod preprocessor;
mod parser;
mod lexer;
fn main() {
    //let mut preprocessor = preprocessor::Preprocessor::new("src/main.c");
    //preprocessor.process_file("src/main.c");
    //lexer::parse_file("src/main.c".to_string());
    parser::parse_file("src/main.c".to_string());
}
