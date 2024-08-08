use crate::parser::lexer::Lexer;
use crate::parser::token::Token;

pub fn parse(input_string: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input_string);
    let mut tokens: Vec<Token> = Vec::new();
    let mut token = lexer.next_token();
    while token.is_some() {
        tokens.push(token.unwrap());
        token = lexer.next_token();
    }
    tokens
}
