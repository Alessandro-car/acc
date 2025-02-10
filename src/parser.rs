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
        return tokens[index];
    }

    fn parser_advance(&mut self) {
        pos = pos + 1;
    }

    fn expected_token(&mut self, expected: lexer::TokType) {
        if self.cur_token() != expected {
            panic!("Expected {:?} but got {:?}", expected, self.cur_token());
        }
    }

    //TODO parse statement
    fn parse_if(&mut self) -> ASTNode {
        /*let condition = parse_rel_operation();
        let if_branch;
        let else_branch;
        ASTNode::IfStmt { condition: (condition), if_branch: (if_branch), else_branch: (else_branch) }*/
    }

    //TODO parse expression
    //TODO parse literal
    //TODO parse operations
    //TODO parse variables
    //TODO parse functions
    //TODO parse blocks
    //TODO parse program
}

