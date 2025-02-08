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
    lex: Lexer,
    current_token: lexer::TokType,
}

impl Parser {
    fn new(input: Vec<char>) -> Self {
        let mut lexer = lexer::Lexer::new(input);
        let current_token = lexer.next_token();
        Parser {
            lex: lexer,
            current_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.lex.next_token();
    }

    fn expect_token(&mut self, expected: lexer::TokType) {
        if self.current_token == expected {
            self.next_token();
        } else {
            panic!("Expected {:?}, got {:?}", expected, self.current_token);
        }
    }

    fn parse_primary_expression(&mut self) -> ASTNode {
        match self.current_token.clone() {
            lexer::TokType::NUMBER(value) => {
                self.next_token();
                ASTNode::IntLiteral(value.parse::<i64>().unwrap())
            }
            lexer::TokType::STRING(value) => {
                self.next_token();
                ASTNode::StringLiteral(value)
            }
            lexer::TokType::IDENTIFIER(name) => {
                self.next_token();
                ASTNode::Identifier(name)
            }
            /*TokType::LPAREN => {
                self.next_token();
                let expr = self.parse_expression();
                self.expect_token(TokType::RPAREN);
                expr
            }*/
            _ => panic!("Unxpected token in primary expression"),
        }
    }
}


