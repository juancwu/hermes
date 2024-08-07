use std::collections::HashMap;
use std::str::Chars;

use crate::parser::token::Token;
use crate::parser::transition_table::{build_transition_table, char_to_input, Input, State};

use super::transition_table::is_definitive_state;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
    current_char: Option<char>,
    lookahead_char: Option<char>,
    transition_table: HashMap<(State, Input), State>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            chars: input.chars(),
            current_char: None,
            lookahead_char: None,
            transition_table: build_transition_table(),
        };
        // initialize the lexer character position
        lexer.advance();
        // Fill lookahead
        lexer.advance();
        lexer
    }

    /// Grab the next token that can be identified in the input.
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces_or_newline();

        let mut token: Option<Token> = None;
        let mut state = State::Start;
        while let Some(ch) = self.current_char {
            let input_type = char_to_input(ch);
            state = match self.transition_table.get(&(state, input_type)) {
                Some(&new_state) => new_state,
                None => State::Error,
            };
            if state == State::Error {
                token = Some(Token::Illegal);
                break;
            } else if is_definitive_state(state) {
                // do something here
            } else {
                self.advance();
            }
        }

        token
    }

    /// Move onto the next character, may be None.
    fn advance(&mut self) {
        self.current_char = self.lookahead_char;
        self.lookahead_char = self.chars.next();
    }

    /// Skip all characters that have the White_Space property. Read Rust documentation for more
    /// information.
    fn skip_whitespaces_or_newline(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skips all the characters that are comments.
    fn skip_comment(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// Reads an identifier, this can be a word with a combination of any characters that does not
    /// include any numbers, colon and whitespaces.
    fn read_identifier(&mut self) -> String {
        let mut identifier: String = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() || ch.is_numeric() || ch == ':' {
                break;
            }
            identifier.push(ch);
            self.advance();
        }
        identifier
    }
}
