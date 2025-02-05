int porcodio() {
	printf("Ciao a tutti");
}
use std::fs::File;
mod lexer;
mod preprocessor;
fn main() {
    let mut preprocessor = preprocessor::Preprocessor::new("src/main.c");
    preprocessor.process_file("src/main.c");
    //lexer::parse_file("src/main.c".to_string());
}
int main() {
	return 3&&4;
	//Ciao
}
