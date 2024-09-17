use std::{collections::HashMap, str::Chars};

use crate::transition_table::{
    build_transition_table, char_to_input, is_transitional_state, Input, State,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    BlockType(String),
    SubBlockType(String),
    Identifier(String),
    Digit(u8),
    StringValue(String),
    Delimeter(char),
    AsKeyword,
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
    current_char: char,
    lookahead_char: char,
    start_index: usize,
    end_index: usize,
    transitional_table: HashMap<(State, Input), State>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            chars: input.chars(),
            current_char: '\0',
            lookahead_char: '\0',
            start_index: 0,
            end_index: 0,
            transitional_table: build_transition_table(),
        };
        // initialize the lexer character position
        lexer.advance();
        // Fill lookahead
        lexer.advance();
        // reset the end index after populating the current and lookahead characters.
        lexer.end_index = 1;
        lexer
    }

    /// Grab the next token that can be identified in the input.
    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_char == '\0' {
            return None;
        }

        self.skip_whitespaces_or_newline();

        let mut ch = self.current_char;
        let mut input = char_to_input(ch);
        let mut state = self.get_next_state(State::Start, input);

        println!("=> Initial");
        println!("=> ch: '{}', state: {:?}, input: {:?}", ch, state, input);

        while is_transitional_state(state) {
            // println!("ch: {:?}, input: {:?}, state: {:?}", ch, input, state);
            self.advance();
            ch = self.current_char;
            input = char_to_input(ch);
            state = self.get_next_state(state, input);
        }

        match state {
            State::EndIdentifier | State::EndSubBlockType => {
                let slice = self.get_literal(self.start_index, self.end_index - 1);
                self.reset_slice_pointers();
                Some(self.match_ident_to_keyword(slice))
            }
            State::EndDelimeter => {
                // have to advanced once since single end states do not trigger the while loop
                self.advance();
                self.reset_slice_pointers();
                // println!("delimeter: {}", ch);
                Some(Token::Delimeter(ch))
            }
            State::EndDigit => {
                let digit = if ch == '1' { 1 } else { 0 };
                self.advance();
                self.reset_slice_pointers();
                Some(Token::Digit(digit))
            }
            State::EndString => {
                let slice = self.get_literal(self.start_index + 1, self.end_index - 1);
                // ended on a tilt, need to advance
                if ch == '`' {
                    self.advance();
                }
                self.reset_slice_pointers();
                Some(Token::StringValue(slice))
            }
            State::EndSpecialIdentifier => {
                let slice = self.get_literal(self.start_index + 1, self.end_index - 1);
                // ended on a double quote, need to advance to avoid infinite special identifier
                // read
                if ch == '"' {
                    self.advance();
                }
                self.reset_slice_pointers();
                Some(Token::Identifier(slice))
            }
            _ => None,
        }
    }

    /// Move onto the next character, may be None.
    fn advance(&mut self) {
        // move to end index to later grab the desired input string
        self.end_index += 1;
        self.current_char = self.lookahead_char;
        self.lookahead_char = match self.chars.next() {
            Some(ch) => ch,
            None => '\0',
        };
    }

    /// Skip all characters that have the White_Space property. Read Rust documentation for more
    /// information.
    fn skip_whitespaces_or_newline(&mut self) {
        while self.current_char.is_whitespace() || self.current_char == '\n' {
            self.advance();
        }
        // set the starting pointer to the end now after skipping through whitespaces and newlines.
        self.reset_slice_pointers();
    }

    fn get_literal(&mut self, s: usize, e: usize) -> String {
        let slice = match self.input.get(s..e) {
            Some(s) => String::from(s),
            None => String::new(),
        };
        slice
    }

    fn reset_slice_pointers(&mut self) {
        self.start_index = self.end_index - 1;
    }

    fn get_next_state(&self, current_state: State, input: Input) -> State {
        match self.transitional_table.get(&(current_state, input)) {
            Some(new_state) => *new_state,
            None => State::Error,
        }
    }

    /// Tries to match the given identifier to a keyword (block type, sub block type, and reserved
    /// keywords). If none is matched, it returns an Identifier token.
    fn match_ident_to_keyword(&self, ident: String) -> Token {
        match ident.as_str() {
            "collection" | "request" | "environment" | "body" | "headers" | "queries" => {
                Token::BlockType(ident)
            }
            "as" => Token::AsKeyword,
            ".json" | ".text" | ".form-urlencoded" | ".multipart-form" => {
                Token::SubBlockType(ident)
            }
            _ => Token::Identifier(ident),
        }
    }
}
