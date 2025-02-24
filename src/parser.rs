use regex::Replacer;
use regex::Regex;
use crate::lexer;
#[derive(Debug)]
enum ASTNode {
    Program(Vec<ASTNode>),
    FuncDec {
        name: String,
        params: Vec<(String, String)>,
        ret_type: String,
        body: Box<ASTNode>,
    },
    Block(Vec<ASTNode>),
    VarDec {
        var_type: String,
        name: String,
        initializer: Option<Box<ASTNode>>,
    },
    BinaryOP {
        operator: lexer::TokType,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Identifier(String),
    IntLiteral(i64),
    StringLiteral(String),
    ReturnStmt(Box<ASTNode>),
    IfStmt {
        condition: Box<ASTNode>,
        if_branch: Box<Vec<ASTNode>>,
        else_branch: Option<Box<Vec<ASTNode>>>,
    },
    WhileStmt {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    }
}

pub struct Parser {
    tokens: Vec<lexer::TokType>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<lexer::TokType>) -> Self {
        Parser {
            tokens,
            pos: 0,
        }
    }

   fn cur_token(&mut self) -> lexer::TokType {
        return self.tokens.get(self.pos).unwrap().clone();
    }

    fn parser_advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos = self.pos + 1;
        }
    }

    fn expected_token(&mut self, expected: lexer::TokType) {
        if self.cur_token() != expected {
            panic!("Expected {:?} but got {:?}", expected, self.cur_token());
        } else {
            self.parser_advance();
        }
    }

   //TODO parse all operations

    fn parse_instruction(&mut self) -> ASTNode {
        let int_keyword   = lexer::TokType::KEYWORD("int".to_string());
        let float_keyword = lexer::TokType::KEYWORD("float".to_string());
        let char_keyword  = lexer::TokType::KEYWORD("char".to_string());
        let str_keyword   = lexer::TokType::KEYWORD("string".to_string());
        let data_keyword: Vec<lexer::TokType> = Vec::from([int_keyword, float_keyword, char_keyword, str_keyword]);
        let cur_token: lexer::TokType = self.cur_token();

        if data_keyword.contains(&cur_token) {
            return self.parse_var();
        }
        if cur_token == lexer::TokType::KEYWORD("fn".to_string()) {
            return self.parse_func();
        }
        if cur_token == lexer::TokType::KEYWORD("return".to_string()) {
            return self.parse_return_stmt();
        }
        if cur_token == lexer::TokType::KEYWORD("if".to_string()) {
            return self.parse_if_stmt();
        }

        panic!("Not a valid keyword");
    }

    fn parse_operation(&mut self) -> ASTNode {
        let rel_operators: Vec<&str> = Vec::from(["==", "!=", ">", "<", ">=", "<=", "&&", "||"]);
        self.expected_token(lexer::TokType::LPAREN('('));
        let left_op = self.parse_term();
        let op_token = self.cur_token();
        let mut right_op: ASTNode = ASTNode::Identifier("".to_string());
        for operator in rel_operators {
            if op_token == lexer::TokType::OPERATOR(operator.to_string()) {
                self.parser_advance();
                right_op = self.parse_term();
            }
        }
        self.expected_token(lexer::TokType::RPAREN(')'));
        ASTNode::BinaryOP { operator: (op_token), left: Box::new(left_op), right: Box::new(right_op) }
    }

    fn parse_term(&mut self) -> ASTNode {
        let term = match self.cur_token() {
            lexer::TokType::IDENTIFIER(ident) => ASTNode::Identifier(ident),
            lexer::TokType::STRING(string) => ASTNode::StringLiteral(string),
            lexer::TokType::NUMBER(num) => ASTNode::IntLiteral(num.parse::<i64>().unwrap()),
            _ => panic!("Expected a term or initializer but got {:?}", self.cur_token())
        };
        self.parser_advance();
        term
    }

    //TODO parse statements
    fn parse_return_stmt(&mut self) -> ASTNode {
        self.parser_advance();
        let term = Box::new(self.parse_term());
        self.expected_token(lexer::TokType::SEMICOLON(';'));
        ASTNode::ReturnStmt(term)
    }

    fn parse_if_stmt(&mut self) -> ASTNode {
        self.parser_advance();
        let condition = Box::new(self.parse_operation());
        let if_branch = Box::new(self.parse_block(false));
        //TODO check if there is an else statement
        let else_branch = if self.cur_token() == lexer::TokType::KEYWORD("else".to_string()) {
            self.parser_advance();
            Some(Box::new(self.parse_block(false)))
        } else {
            None
        };
        ASTNode::IfStmt { condition, if_branch , else_branch }
    }

    //TODO function to control the variable declaration
    fn parse_var(&mut self) -> ASTNode {
        let var_type: String = self.cur_token().as_keyword().unwrap().to_string();
        self.parser_advance();
        let mut name: String = String::new();
        match self.cur_token() {
            lexer::TokType::IDENTIFIER(str) => name.push_str(&str),
            _ => panic!("Expected an identifier token but got {:?}", self.cur_token()),
        };
        self.parser_advance();
        let initializer = if self.cur_token() == lexer::TokType::OPERATOR("=".to_string()) {
            self.parser_advance();
            self.parse_term()
        } else {
            panic!("Expected an initializer but found {:?}", self.cur_token());
        };
        self.expected_token(lexer::TokType::SEMICOLON(';'));
        ASTNode::VarDec { var_type, name, initializer: Some(Box::new(initializer)) }
    }

    fn parse_func(&mut self) -> ASTNode {
        self.parser_advance();
        let name = match self.cur_token() {
            lexer::TokType::IDENTIFIER(ident) => String::from(ident),
            _ => panic!("Expected an identifier token but got {:?}", self.cur_token())
        };
        self.parser_advance();
        self.expected_token(lexer::TokType::LPAREN('('));
        let mut params: Vec<(String, String)> = Vec::new();
        let valid_param_types: Vec<&str> = Vec::from(["int", "float", "char", "string"]);
        while self.cur_token() != lexer::TokType::RPAREN(')') {
            let mut param_type: String = String::new();
            match self.cur_token() {
                lexer::TokType::KEYWORD(data_type) => {
                    if !valid_param_types.contains(&data_type.as_str()) {
                        panic!("Not a valid data type, got {data_type}")
                    }
                    param_type.push_str(&data_type);
                }
                _ => panic!("Expected a keyword but got {:?}", self.cur_token()),
            };
            self.parser_advance();
            let mut param_name: String = String::new();
            match self.cur_token() {
                lexer::TokType::IDENTIFIER(par_name) => param_name.push_str(&par_name),
                _ => panic!{"Expected an identifier but got {:?}", self.cur_token()},
            };
            params.push(( param_type, param_name ));
            self.parser_advance();
            if self.cur_token() != lexer::TokType::RPAREN(')') {
                self.expected_token(lexer::TokType::COMMA(','));
            }
        }
        self.parser_advance();
        self.expected_token(lexer::TokType::OPERATOR("->".to_string()));
        let mut ret_type: String = String::new();
        let valid_ret_types: Vec<&str> = Vec::from(["void", "int", "float", "char", "string"]);
        match self.cur_token() {
            lexer::TokType::KEYWORD(return_type) => {
                if !valid_ret_types.contains(&return_type.as_str()) {
                    panic!("Not a valid return type, got {return_type}");
                }
                ret_type.push_str(&return_type);
            }
            _ => panic!{"Expected a keyword but got {:?}", self.cur_token()}
        }
        self.parser_advance();
        let mut need_return: bool = false;
        if !ret_type.contains("void") {
            need_return = true;
        }
        let body = Box::new(ASTNode::Block(self.parse_block(need_return)));
        ASTNode::FuncDec { name, params, ret_type, body }
    }

    fn parse_block(&mut self, need_return: bool ) -> Vec<ASTNode> {
        self.expected_token(lexer::TokType::LBRACE('{'));
        let mut block: Vec<ASTNode> = Vec::new();
        let mut return_keyword: bool = false;
        while self.cur_token() != lexer::TokType::RBRACE('}') && !return_keyword {
            if self.cur_token() == lexer::TokType::KEYWORD("return".to_string()) {
                return_keyword = true;
            }
            let instr = self.parse_instruction();
            block.push(instr);
        }

        if return_keyword {
            while self.cur_token() != lexer::TokType::RBRACE('}') {
                self.parser_advance();
            }
        }

        if need_return && !return_keyword {
            panic!("Expected a return statement");
        }

        self.parser_advance();
        block
    }

    //TODO function to control the block
}

pub fn parse_program(tokens_list: Vec<lexer::TokType>) {
    let mut parser = Parser::new(tokens_list.clone());
    while parser.pos < tokens_list.len(){
        //println!("{:?}", tokens_list);
        println!("{:?}", parser.parse_instruction());
    }
}

