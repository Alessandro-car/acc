use std::fs;
use crate::lexer;
use crate::lexer::TokType;
use crate::lexer::Lexer;
#[derive(Debug)]
enum ASTNode {
    Program(Vec<ASTNode>),
    FuncDec {
        name: String,
        params: Vec<(String, String)>,
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
        if_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>,
    },
    WhileStmt {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    }
}

struct Parser {
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
        self.pos = self.pos + 1;
    }

    fn expected_token(&mut self, expected: lexer::TokType) {
        if self.cur_token() != expected {
            panic!("Expected {:?} but got {:?}", expected, self.cur_token());
        }
    }

    //TODO parse statement
    fn parse_if(&mut self) {
        println!("{:?}", self.parse_rel_operation());
        //ASTNode::IfStmt { condition: (condition), if_branch: (if_branch), else_branch: () }
    }

    fn parse_instruction(&mut self) {
        let cur_token: lexer::TokType = self.cur_token();
        if cur_token == lexer::TokType::KEYWORD("if".to_string()) {
            self.parse_if();
        } else {
            self.expected_token(lexer::TokType::KEYWORD("if".to_string()));
        }

    }

    //TODO parse expression
    //TODO parse literal
    //TODO parse operations
    fn parse_rel_operation(&mut self) -> ASTNode {
        while self.cur_token() != lexer::TokType::LPAREN('(') {
            self.parser_advance();
        }
        self.parser_advance();
        let left_op = match self.cur_token() {
            lexer::TokType::IDENTIFIER(ident) => Box::new(ASTNode::Identifier(ident)),
            _ => panic!("Not an identfier token")
        };
        self.parser_advance();
        let op_token = self.cur_token();
        let mut right_op: Box<ASTNode> = Box::new(ASTNode::Identifier("".to_string()));
        if op_token == lexer::TokType::OPERATOR("<".to_string()) {
            self.parser_advance();
            right_op = match self.cur_token() {
                lexer::TokType::IDENTIFIER(ident) => Box::new(ASTNode::Identifier(ident)),
                _ => panic!("Not an identifier token")
            };
            self.parser_advance();
        }
        ASTNode::BinaryOP { operator: (op_token), left: (left_op), right: (right_op) }
    }
    //TODO parse variables
    //TODO parse functions
    //TODO parse blocks
    //TODO parse program
}

pub fn parse_program(tokens_list: Vec<lexer::TokType>) {
    let mut parser = Parser::new(tokens_list.clone());
    for token in tokens_list {
        parser.parse_instruction();
    }
}

