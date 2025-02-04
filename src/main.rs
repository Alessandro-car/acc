mod lexer;
fn main() {
    lexer::parse_file("src/main.c".to_string());
}
