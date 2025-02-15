use crate::lexer;
#[derive(Debug)]
enum ASTNode {
    Program(Vec<ASTNode>),
    FuncDec {
        func_type: String,
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

    //TODO parse statement
    fn parse_if(&mut self) {
        //let _condition: Box<ASTNode> = Box::new(self.parse_rel_operation());
        println!("{:?}", self.parse_operation());
        //ASTNode::IfStmt { condition: (condition), if_branch: (if_branch), else_branch: () }
    }

    fn parse_return(&mut self) -> ASTNode {
        //TODO check for unexpected tokens and check if it is an identifier or a condtion or other
        let term = Box::new(self.parse_term());
        ASTNode::ReturnStmt(term)
    }

    //TODO parse expression

    fn parse_term(&mut self) -> ASTNode {
        let term = match self.cur_token() {
            lexer::TokType::IDENTIFIER(ident) => ASTNode::Identifier(ident),
            lexer::TokType::STRING(string) => ASTNode::StringLiteral(string),
            lexer::TokType::NUMBER(num) => ASTNode::IntLiteral(num.parse::<i64>().unwrap()),
            _ => panic!("Not an identifier token")
        };
        self.parser_advance();
        term
    }
    //TODO parse all operations
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

    fn parse_func(&mut self) -> ASTNode {
        let func_keyword: Vec<&str> = Vec::from(["char", "double", "float", "int", "long", "short", "void"]);
        let func_type_tok = self.cur_token();
        let mut equal: bool = false;
        let mut type_func: String = String::new();
        for keyword in func_keyword {
            if func_type_tok == lexer::TokType::KEYWORD(keyword.to_string()) {
                self.parser_advance();
                type_func.push_str(keyword);
                equal = true;
            }
        }
        if !equal {
            panic!("Not a valid function");
        }
        if self.cur_token() == lexer::TokType::OPERATOR("*".to_string()) {
            type_func.push('*');
            self.parser_advance();
        }

        let name_func: String = String::new();

        let params: Vec<(String, String)> = Vec::new();
        ASTNode::FuncDec { func_type: type_func, name: name_func, params: params, body: Box::new(ASTNode::StringLiteral("".to_string())) }
    }

    fn parse_instruction(&mut self) {
        let cur_token: lexer::TokType = self.cur_token();
        if cur_token == lexer::TokType::KEYWORD("if".to_string()) {
            self.parser_advance();
            self.parse_if();
        }
        if cur_token == lexer::TokType::KEYWORD("return".to_string()) {
            self.parser_advance();
            println!("{:?}", self.parse_return());
        }
        println!("{:?}", self.parse_func());
    }
    //TODO parse variables
    //TODO parse functions
    //TODO parse blocks
}

//TODO parse program
pub fn parse_program(tokens_list: Vec<lexer::TokType>) {
    let mut parser = Parser::new(tokens_list.clone());
    while parser.pos < tokens_list.len() {
        parser.parse_instruction();
    }
}

