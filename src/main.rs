use std::fs::File;
mod lexer;
mod preprocessor;
fn main() {
    let mut preprocessor = preprocessor::Preprocessor::new("src/main.c");
    preprocessor.process_file("src/main.c");
}
