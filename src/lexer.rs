use std::fs::{self, File};

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

static PREPROCESSOR_DIR: [&str; 11] = [
    "#include", "#define", "#undef", "#ifdef", "#ifndef",
    "#if", "#else", "#elif", "#endif", "#error", "#pragma"
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
    COMMENT(String),
    PREPROCESSOR(String),
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

fn get_preprocessor_dir(ident: &Vec<char>) -> Result<TokType, String> {
    let identifier: String = ident.into_iter().collect();
    for preprocessor_dir in PREPROCESSOR_DIR.iter() {
        if identifier == preprocessor_dir.to_string() {
            return Ok(TokType::PREPROCESSOR(identifier))
        }
    }
    Err(String::from("Not a preprocessor dir"))
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

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
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

    pub fn next_token(&mut self) -> TokType {
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

        let read_preprocessor_dir = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_letter(l.ch) || l.ch == '#' {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_operator = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && !is_letter(l.ch) && !is_digit(l.ch) && l.ch != '#' && l.ch != ' ' {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let token: TokType;
        self.skip_whitespace();
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
                if !is_letter(self.ch) && !is_digit(self.ch) && self.ch != '#' && self.ch != ' '{
                    let operator: Vec<char> = read_operator(self);
                    match get_operator_token(&operator) {
                        Ok(operator_token) => {
                            return operator_token;
                        }
                        Err(_err) => {}
                    }
                }

                if self.ch == '#' {
                    let ident: Vec<char> = read_preprocessor_dir(self);
                    match get_preprocessor_dir(&ident) {
                        Ok(preprocessor_token) => {
                            return preprocessor_token;
                        }
                        Err(_err) => {},
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
