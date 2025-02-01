#[derive(PartialEq)]
#[derive(Debug)]
pub enum tok_type {
    ILLEGAL,
    EOF,
    IDENTIFIER(Vec<char>),
    INT(Vec<char>),
    ASSIGN(char),
    PLUS(char),
    COMMA(char),
    SEMICOLON(char),
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    MINUS(char),
    BANG(char),
    ASTERISK(char),
    SLASH(char),
    LT(char),
    GT(char)
}

fn get_keyword_token(ident: &Vec<char>) -> Result<tok_type, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "true" => Ok(tok_type::TRUE),
        "false" => Ok(tok_type::FALSE),
        "if" => Ok(tok_type::IF),
        "else" => Ok(tok_type::ELSE),
        "return" => Ok(tok_type::RETURN),
        _ => Err(String::from("Not a keyword"))
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0'
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn skip_whitespace(&mut self) {
        let ch = self.ch;
        if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> tok_type {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_letter(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_digit(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let token: tok_type;
        self.skip_whitespace();
        match self.ch {
            '=' => token = tok_type::ASSIGN(self.ch),
            '+' => token = tok_type::PLUS(self.ch),
            '-' => token = tok_type::MINUS(self.ch),
            '!' => token = tok_type::BANG(self.ch),
            '/' => token = tok_type::SLASH(self.ch),
            '*' => token = tok_type::ASTERISK(self.ch),
            '<' => token = tok_type::LT(self.ch),
            '>' => token = tok_type::GT(self.ch),
            ';' => token = tok_type::SEMICOLON(self.ch),
            '(' => token = tok_type::LPAREN(self.ch),
            ')' => token = tok_type::RPAREN(self.ch),
            ',' => token = tok_type::COMMA(self.ch),
            '{' => token = tok_type::LBRACE(self.ch),
            '}' => token = tok_type::RBRACE(self.ch),
            '0' => token = tok_type::EOF,
            _ => {
                if is_letter(self.ch) {
                    let ident: Vec<char> = read_identifier(self);
                    match get_keyword_token(&ident) {
                        Ok(keyword_token) => {
                            return keyword_token;
                        },
                        Err(_err) => {
                            return tok_type::IDENTIFIER(ident);
                        }
                    }
                } else if is_digit(self.ch) {
                    let ident: Vec<char> = read_number(self);
                    return tok_type::INT(ident);
                } else {
                    return tok_type::ILLEGAL
                }
            }
        }
        self.read_char();
        token
    }
}

