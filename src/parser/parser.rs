use crate::parser::lexer::Lexer;
use crate::parser::token::Token;

pub fn parse(input_string: String) -> Vec<Token> {
    let mut lexer = Lexer::new(input_string);
    let mut tokens: Vec<Token> = Vec::new();
    let mut token: Token = lexer.next_token();
    tokens.push(token.clone());
    while match token {
        Token::EOF => false,
        _ => true,
    } {
        token = lexer.next_token();
        tokens.push(token.clone())
    }

    tokens
}
