use crate::lexer::Lexer;

pub fn parse(input: &str) {
    let mut lexer = Lexer::new(input);
    lexer.next_token();
}

