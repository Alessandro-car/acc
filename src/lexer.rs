use core::panic;
use std::fs::File;

#[derive(Debug)]
enum tok_type {
    OP_BRACE, CLOSE_BRACE,
    OP_PAR, CLOSE_PAR,
    SEMICOLON,
    INT,
    RETURN,
    IDENTIFIER,
    INT_LITERAL,
}

pub fn lex(input: String) -> Vec<String> {
    let mut list_tokens: Vec<String> = Vec::new();
    let mut iter = input.chars();

    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            '{' => list_tokens.push(tok_type_to_string(tok_type::OP_BRACE)),
            '}' => list_tokens.push(tok_type_to_string(tok_type::CLOSE_BRACE)),
            '(' => list_tokens.push(tok_type_to_string(tok_type::OP_PAR)),
            ')' => list_tokens.push(tok_type_to_string(tok_type::CLOSE_PAR)),
            ';' => list_tokens.push(tok_type_to_string(tok_type::SEMICOLON)),
            _ => {
                panic!("Unrecognized char");
            }
        }
    }
    return list_tokens;
}

fn tok_type_to_string(tok_type: tok_type) -> String {
    match tok_type {
        tok_type::OP_BRACE => return String::from("{"),
        tok_type::CLOSE_BRACE => return String::from("}"),
        tok_type::OP_PAR => return String::from("("),
        tok_type::CLOSE_PAR => return String::from(")"),
        tok_type::SEMICOLON => return String::from(";"),
        tok_type::INT => return String::from("int"),
        tok_type::RETURN => return String::from("return"),
        _ => return String::from("Unrecognized character"),
    }
}

