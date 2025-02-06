use std::fs::{self};

static KEYWORDS: [&str; 34] = [
   "auto", "break", "case", "char", "const", "continue",
   "default", "do", "double",  "else", "enum", "extern",
   "float", "for", "goto", "if", "inline", "int",
   "long", "register", "restrict", "return", "short", "signed",
   "sizeof", "static", "struct", "switch", "typedef", "union",
   "unsigned", "void", "volatile", "while"
];

static OPERATORS: [&str; 37] = [
    "+", "-", "*", "/", "%", "++", "--", //Arithmetic operators
    "==", "!=", ">", "<", ">=", "<=", //Relational operators
    "&&", "||", "!", //Logical operators
    "&", "|", "^", "~", "<<", ">>", //Bitwise operators
    "=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", //Assignment operators
    "?:", ".", "->", "&"  //Other operators
];

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokType {
    EOF,
    ILLEGAL,
    IDENTIFIER(String),
    NUMBER(String),
    OPERATOR(String),
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),
    LSQUARE(char),
    RSQUARE(char),
    SEMICOLON(char),
    COMMA(char),
    KEYWORD(String),
    STRING(String),
}

fn get_keyword_token(ident: &Vec<char>) -> Result<TokType, String> {
    let identifier: String = ident.into_iter().collect();
    for keyword in KEYWORDS.iter() {
        if keyword.to_string() == identifier {
            return Ok(TokType::KEYWORD(identifier))
        }
    }

    Err(String::from("Not a keyword"))
}

fn get_operator_token(ident: &Vec<char>) -> Result<TokType, String> {
    let identifier: String = ident.into_iter().collect();
    for operator in OPERATORS.iter() {
        if identifier == operator.to_string() {
            return Ok(TokType::OPERATOR(identifier));
        }
    }
    Err(String::from("Not an operator"))
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
    comment: bool,
}

impl Lexer {
    fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: '0',
            comment: false,
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    fn skip_whitespace(&mut self) {
        let ch = self.ch;
        if is_whitespace(ch) {
            self.read_char();
        }
    }

    fn handle_comments(&mut self, type_comment: &str) -> Result<TokType, String>{
        match type_comment {
            "//" => {
                self.ch = '0';
                return Ok(TokType::EOF);
            },
            "/*" => {
                self.comment = true;
                return Ok(TokType::EOF);
            },
            _ => {},
        }
        return Ok(TokType::EOF);
    }

    fn next_token(&mut self) -> TokType {
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

       let read_operator = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && !is_letter(l.ch) && !is_digit(l.ch) && !is_whitespace(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let token: TokType;
        while is_whitespace(self.ch) {
            self.skip_whitespace();
        }

        if self.comment == true {
            return TokType::EOF;
        }
        match self.ch {
            '0' => token = TokType::EOF,
            '(' => token = TokType::LPAREN(self.ch),
            ')' => token = TokType::RPAREN(self.ch),
            '{' => token = TokType::LBRACE(self.ch),
            '}' => token = TokType::RBRACE(self.ch),
            '[' => token = TokType::LSQUARE(self.ch),
            ']' => token = TokType::RSQUARE(self.ch),
            ';' => token = TokType::SEMICOLON(self.ch),
            ',' => token = TokType::COMMA(self.ch),
            _ => {
                if !is_letter(self.ch) && !is_digit(self.ch) && !is_whitespace(self.ch){
                    let operator: Vec<char> = read_operator(self);
                    match get_operator_token(&operator) {
                        Ok(operator_token) => {
                            return operator_token;
                        }
                        Err(_err) => {
                            let op_string: String = operator.iter().collect();
                            if op_string == "//" || op_string == "/*" {
                                match self.handle_comments(op_string.as_str()) {
                                    Ok(comment_token) => {
                                        return comment_token;
                                    }
                                    Err(_err) => { }
                                }
                            }
                            if op_string == "*/" {
                                self.comment = false;
                            }
                        }
                    }
                }

               if is_letter(self.ch) {
                    let ident: Vec<char> = read_identifier(self);
                    match get_keyword_token(&ident) {
                        Ok(keyword_token) => {
                            return keyword_token;
                        },
                        Err(_err) => {
                            return TokType::IDENTIFIER(ident.into_iter().collect());
                        }
                    }
                } else if is_digit(self.ch) {
                    let ident: String = read_number(self).into_iter().collect();
                    return TokType::NUMBER(ident);
                }
                return TokType::ILLEGAL
            }
        }
        self.read_char();
        token
    }
}

pub fn parse_file(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to open the file");
    let mut lexer = Lexer::new(contents.chars().collect());
    lexer.read_char();
    loop {
        let token = lexer.next_token();
        if token == TokType::EOF {
            break;
        } else {
            println!("{:?}", token);
        }
    }
}
