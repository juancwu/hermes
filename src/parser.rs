use crate::lexer::Lexer;

pub fn parse(input: &str) {
    let mut lexer = Lexer::new(input);
    // for _ in 0..6 {
    //     println!("{:?}", lexer.next_token());
    // }
    while let Some(t) = lexer.next_token() {
        println!("Token: {:?}", t);
    }
}
