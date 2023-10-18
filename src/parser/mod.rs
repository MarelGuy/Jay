use crate::lexer::token::{Token, TokenType};

use self::ast::{
    math::{EitherIntOrMathNode, Int, MathNode, OpNode, Operator, ProcessedMathNode},
    Nodes,
};

pub(crate) mod ast;

#[derive(Debug, PartialEq, Clone)]
pub struct Parser<'a> {
    token_stream: Vec<Token<'a>>,
    file_name: String,
    lines: Vec<String>,
    current_token: Token<'a>,
    tok_i: usize,
    pub ast: Vec<Nodes<'a>>,
}

impl<'a> Parser<'a> {
    // * Main functions

    pub fn new(token_stream: Vec<Token<'a>>, file_name: String, lines: Vec<String>) -> Self {
        let init_tok: Token = token_stream[0].clone();

        Self {
            file_name: file_name.clone(),
            current_token: init_tok,
            token_stream,
            tok_i: 0,
            lines,
            ast: vec![],
        }
    }

    fn next(&mut self, count: usize) {
        for _ in 0..count {
            if self.tok_i < self.token_stream.len() {
                self.current_token = self.token_stream[self.tok_i].clone();
            }

            self.tok_i += 1;
        }
    }

    fn peek(&self, add: usize) -> Token {
        self.token_stream[self.tok_i + add].clone()
    }

    fn parse_list(&mut self, token: Token) -> Nodes<'a> {
        match token.token_type {
            TokenType::Number | TokenType::NegativeNumber => {
                if self.peek(0).token_type == TokenType::Plus
                    || self.peek(0).token_type == TokenType::Minus
                    || self.peek(0).token_type == TokenType::Multiply
                    || self.peek(0).token_type == TokenType::Divide
                {
                    return Nodes::ProcessedMathNode(self.parse_math());
                }

                println!("{:?}", Int::new(&self.current_token.slice).neg_to_minus());

                return Nodes::Int(Int::new(&self.current_token.slice));
            }
            TokenType::Semicolon => Nodes::NextLine,
            _ => Nodes::Null,
        }
    }

    pub fn parse(&mut self) {
        while self.tok_i < self.token_stream.len() {
            self.next(1);

            let new_node: Nodes = self.parse_list(self.current_token.clone()).clone();

            self.ast.push(new_node);
        }
    }

    // Math

    fn parse_math(&mut self) -> ProcessedMathNode<'a> {
        let mut operations: Vec<OpNode<'a>> = vec![];
        let mut prio: usize = 1;

        loop {
            let math_node: MathNode<'a> = self.parse_math_node();

            match &math_node.op {
                Operator::Multiply | Operator::Divide => prio += 1,
                _ => (),
            }

            operations.push(OpNode::new(prio, math_node));
            self.next(1);

            if self.current_token.token_type == TokenType::Semicolon {
                break;
            }
        }

        ProcessedMathNode::new(operations)
    }

    fn parse_math_node(&mut self) -> MathNode<'a> {
        let lhs: Int<'_> = Int::new(&self.current_token.slice);

        self.next(1);

        let op: Operator = Operator::get_op(self.current_token.token_type);

        self.next(1);

        let rhs: EitherIntOrMathNode = if (self.peek(0).token_type == TokenType::Plus
            || self.peek(0).token_type == TokenType::Minus
            || self.peek(0).token_type == TokenType::Multiply
            || self.peek(0).token_type == TokenType::Divide)
            && (self.peek(2).token_type == TokenType::Semicolon)
        {
            EitherIntOrMathNode::MathNode(self.parse_math_node())
        } else {
            EitherIntOrMathNode::Int(Int::new(&self.current_token.slice))
        };

        return MathNode::new(lhs, op, rhs);
    }
}
